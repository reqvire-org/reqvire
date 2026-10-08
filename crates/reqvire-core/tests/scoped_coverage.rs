use reqvire::element::{ElementType, RequirementType, VerificationType};
use reqvire::exclusions::ExclusionSetBuilder;
use reqvire::relation::{LinkType, Relation, RelationTarget, RELATION_TYPES};
use reqvire::report::coverage::{generate_coverage_report, CoverageReport};
use reqvire::{Element, GraphRegistry, ModelManager};
use serde_json::{json, Value};
use serial_test::serial;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

// Count only this test thread's allocations inside the operation under test.
// Fixture construction, report generation and serialization are excluded.
thread_local! {
    static ALLOCATIONS: Cell<Option<(usize, usize)>> = const { Cell::new(None) };
}

struct CountingAllocator;

fn record_allocation(bytes: usize) {
    let _ = ALLOCATIONS.try_with(|counter| {
        if let Some((calls, total)) = counter.get() {
            counter.set(Some((calls + 1, total + bytes)));
        }
    });
}

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        record_allocation(layout.size());
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        record_allocation(new_size);
        unsafe { System.realloc(pointer, layout, new_size) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

fn index_allocations(report: &CoverageReport, registry: &GraphRegistry) -> (usize, usize) {
    struct StopCounting;
    impl Drop for StopCounting {
        fn drop(&mut self) {
            ALLOCATIONS.with(|counter| counter.set(None));
        }
    }
    let _guard = StopCounting;
    ALLOCATIONS.with(|counter| counter.set(Some((0, 0))));
    let index = report.scope_index(registry);
    std::hint::black_box(&index);
    ALLOCATIONS.with(|counter| {
        counter
            .replace(None)
            .expect("test fixture operation should succeed")
    })
}

fn relate(source: &mut Element, name: &'static str, target: &str) {
    source.relations.push(Relation {
        relation_type: &RELATION_TYPES[name],
        target: RelationTarget {
            text: target.into(),
            link: if target.starts_with("https:") {
                LinkType::ExternalUrl(target.into())
            } else {
                LinkType::Identifier(target.into())
            },
            element_id: None,
        },
        user_created: true,
    });
}

fn flat_fixture(branches: usize, evidence_bytes: usize) -> GraphRegistry {
    let mut registry = GraphRegistry::new();
    let mut root = Element::new(
        "Root",
        "model.md#root",
        "model.md",
        1,
        Some(ElementType::Capability),
    );
    let evidence = format!("https://example.org/{}", "e".repeat(evidence_bytes));
    for index in 0..branches {
        let capability_id = format!("model.md#c{index:04}");
        let requirement_id = format!("model.md#r{index:04}");
        let verification_id = format!("model.md#v{index:04}");
        let mut capability = Element::new(
            &capability_id,
            &capability_id,
            "model.md",
            1,
            Some(ElementType::Capability),
        );
        let mut requirement = Element::new(
            &requirement_id,
            &requirement_id,
            "model.md",
            1,
            Some(ElementType::Requirement(RequirementType::System)),
        );
        let mut verification = Element::new(
            &verification_id,
            &verification_id,
            "model.md",
            1,
            Some(ElementType::Verification(VerificationType::Test)),
        );
        relate(&mut root, "derive", &capability_id);
        relate(&mut capability, "specifiedBy", &requirement_id);
        relate(&mut requirement, "verifiedBy", &verification_id);
        relate(&mut requirement, "satisfiedBy", &evidence);
        relate(&mut verification, "verify", &requirement_id);
        relate(&mut verification, "satisfiedBy", &evidence);
        for element in [capability, requirement, verification] {
            registry
                .register_element(element, "model.md")
                .expect("test fixture operation should succeed");
        }
    }
    registry
        .register_element(root, "model.md")
        .expect("test fixture operation should succeed");
    registry
}

#[test]
#[serial]
fn compact_scope_index_allocations_follow_memberships() {
    let measure = |branches| {
        let registry = flat_fixture(branches, 32);
        let report = generate_coverage_report(&registry);
        index_allocations(&report, &registry)
    };
    let small = measure(64);
    let large = measure(128);
    eprintln!("scope index allocations (calls, bytes): 64={small:?}, 128={large:?}");
    assert!(
        large.0 < small.0 * 3,
        "doubling flat membership must not quadruple allocation calls: {small:?} -> {large:?}"
    );
    assert!(
        large.1 < small.1 * 3,
        "doubling flat membership must not quadruple allocated bytes: {small:?} -> {large:?}"
    );
}

#[test]
#[serial]
fn compact_scope_index_does_not_copy_evidence() {
    let measure = |evidence_bytes| {
        let registry = flat_fixture(8, evidence_bytes);
        let report = generate_coverage_report(&registry);
        index_allocations(&report, &registry)
    };
    let small = measure(256);
    let large = measure(16_384);
    eprintln!("scope index allocations (calls, bytes): small evidence={small:?}, large evidence={large:?}");
    assert!(large.1 < small.1 * 2, "compact memberships/summaries must not copy detailed evidence payloads: {small:?} -> {large:?}");
}

fn assert_scope_parity(registry: &GraphRegistry, expected_scopes: &Value) -> Value {
    let report = generate_coverage_report(registry);
    let index = serde_json::to_value(report.scope_index(registry))
        .expect("test fixture operation should succeed");
    assert_eq!(
        index
            .as_object()
            .expect("expected an object in the test response")
            .len(),
        expected_scopes
            .as_object()
            .expect("expected an object in the test response")
            .len()
    );
    for (name, scope) in expected_scopes
        .as_object()
        .expect("expected an object in the test response")
    {
        let compact = &index[scope["capability_identifier"]
            .as_str()
            .expect("expected a string in the test response")];
        let detailed = serde_json::to_value(
            report
                .clone()
                .with_scope(registry, Some(name))
                .expect("test fixture operation should succeed"),
        )
        .expect("test fixture operation should succeed");
        assert_eq!(&compact["scope"], scope, "membership for {name}");
        assert_eq!(
            compact["scope"], detailed["scope"],
            "scope metadata for {name}"
        );
        assert_eq!(
            compact["summary"], detailed["summary"],
            "summary for {name}"
        );
        // Independent record counts prevent both paths from sharing a counting error.
        for field in [
            "verified_leaf_requirements",
            "unverified_leaf_requirements",
            "satisfied_test_verifications",
            "unsatisfied_test_verifications",
            "covered_requirements",
            "uncovered_requirements",
        ] {
            let count: usize = detailed[field]["files"]
                .as_object()
                .expect("expected an object in the test response")
                .values()
                .map(|rows| {
                    rows.as_array()
                        .expect("expected an array in the test response")
                        .len()
                })
                .sum();
            assert_eq!(compact["summary"][field], json!(count), "{name}: {field}");
        }
    }
    index
}

#[test]
#[serial]
fn compact_scopes_preserve_shared_subjects_and_global_contract_evidence() {
    let fixture =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tests/test-scoped-coverage");
    let workspace = tempfile::tempdir().expect("test fixture operation should succeed");
    for directory in ["specifications", "evidence"] {
        fs::create_dir(workspace.path().join(directory))
            .expect("test fixture operation should succeed");
        for entry in fs::read_dir(fixture.join(directory)).expect("read test fixture") {
            let entry = entry.expect("test fixture operation should succeed");
            fs::copy(
                entry.path(),
                workspace.path().join(directory).join(entry.file_name()),
            )
            .expect("test fixture operation should succeed");
        }
    }
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(workspace.path())
        .status()
        .expect("test fixture operation should succeed")
        .success());
    struct RestoreDirectory(PathBuf);
    impl Drop for RestoreDirectory {
        fn drop(&mut self) {
            std::env::set_current_dir(&self.0).expect("test fixture operation should succeed");
        }
    }
    let _restore =
        RestoreDirectory(std::env::current_dir().expect("test fixture operation should succeed"));
    std::env::set_current_dir(workspace.path()).expect("test fixture operation should succeed");
    let exclusions = ExclusionSetBuilder::new()
        .build()
        .expect("test fixture operation should succeed");
    let scopes: Value = serde_json::from_str(
        &fs::read_to_string(fixture.join("expected/scopes.json")).expect("read test fixture"),
    )
    .expect("read test fixture");
    let mut model = ModelManager::new();
    model
        .parse_and_validate(None, &exclusions)
        .expect("test fixture operation should succeed");
    let initial = assert_scope_parity(&model.graph_registry, &scopes);
    let alpha_id = "specifications/Capabilities.md#alpha-root";
    let alpha = &initial[alpha_id]["summary"];
    assert_eq!(alpha["total_requirements_in_scope"], 6);
    assert_eq!(alpha["total_verifications"], 5);
    assert_eq!(alpha["coverage_sources"]["contract_consumer_rollup"], 1);
    assert_eq!(
        alpha["verification_types"],
        json!({"test":2,"formal_proof":1,"analysis":0,"inspection":1,"demonstration":1})
    );
    assert_eq!(
        initial["specifications/Capabilities.md#beta-root"]["summary"]["verification_types"]
            ["analysis"],
        1
    );
    assert_eq!(
        initial["specifications/Capabilities.md#empty-branch"]["summary"]
            ["implementation_coverage_percentage"],
        0.0
    );
    let whole = serde_json::to_value(generate_coverage_report(&model.graph_registry))
        .expect("test fixture operation should succeed");
    assert_eq!(whole["summary"]["orphaned_verifications"], 1);
    assert_eq!(alpha["orphaned_verifications"], 0);
    assert_eq!(alpha["total_terminal_requirements"], 3);
    assert_eq!(alpha["covered_terminal_requirements"], 1);

    fs::copy(
        fixture.join("fixtures/BetaWithoutEvidence.md"),
        "specifications/Beta.md",
    )
    .expect("test fixture operation should succeed");
    model
        .parse_and_validate(None, &exclusions)
        .expect("test fixture operation should succeed");
    let changed = assert_scope_parity(&model.graph_registry, &scopes);
    assert_eq!(
        changed[alpha_id]["summary"]["coverage_sources"]["contract_consumer_rollup"],
        0
    );
    assert_eq!(changed[alpha_id]["summary"]["covered_requirements"], 2);
    assert_eq!(
        changed[alpha_id]["summary"]["total_terminal_requirements"],
        3
    );
    // Regenerating an index cannot mutate a previously returned snapshot.
    assert_eq!(
        initial[alpha_id]["summary"]["coverage_sources"]["contract_consumer_rollup"],
        1
    );
}
