#!/bin/bash

# Test: Diagram Generation Functionality
# --------------------------------------
# Acceptance Criteria:
# - System should generate diagrams embedded in markdown files
# - System should add/update mermaid diagrams under section headers
# - System should preserve existing custom mermaid diagrams
#
# Test Criteria:
# - Command exits with success (0) return code
# - Modified files contain the expected mermaid diagrams
# - Custom diagrams are preserved
# - Generated diagrams have proper content and relationships

# Create a reqvire.yaml configuration
cat > "$TEST_DIR/reqvire.yaml" << EOF
paths:
  specifications_folder: "specifications"
  output_folder: "output"
  excluded_filename_patterns: []
style:
  theme: "default"
  max_width: 1200
  diagram_direction: "LR"
EOF

# Make backup copies of original files for comparison
mkdir -p "$TEST_DIR/backup"
cp -r "$TEST_DIR/specifications" "$TEST_DIR/backup/"

# Run reqvire to generate diagrams
OUTPUT=$(cd "$TEST_DIR" && "$REQVIRE_BIN" --subdirectory tests/test-diagram-generation --config "$TEST_DIR/reqvire.yaml" --generate-diagrams 2>&1)
EXIT_CODE=$?

# Save output to log
printf "%s\n" "$OUTPUT" > "${TEST_DIR}/test_results.log"

# Check for basic success
if [ $EXIT_CODE -ne 0 ]; then
  echo "❌ FAILED: Diagram generation command returned error: $EXIT_CODE"
  cat "${TEST_DIR}/test_results.log"
  exit 1
fi

# List of files to check
FILES_TO_CHECK=(
  "Requirements.md"
  "Requirements-with-custom-mermaid.md"
  "SystemRequirements.md" 
  "UserRequirements.md"
  "VerificationTests.md"
)

# Check if files were modified and contain mermaid diagrams
FAILED_CHECKS=0
for file in "${FILES_TO_CHECK[@]}"; do
  file_path="$TEST_DIR/specifications/$file"
  backup_path="$TEST_DIR/backup/specifications/$file"
  
  # Check that files were modified
  if cmp -s "$file_path" "$backup_path"; then
    echo "❌ NOT MODIFIED: Expected file to be modified with diagrams: $file"
    FAILED_CHECKS=$((FAILED_CHECKS + 1))
    continue
  fi
  
  # Check for mermaid content
  if ! grep -q '```mermaid' "$file_path"; then
    echo "❌ NO DIAGRAM: Expected file to contain mermaid diagram: $file"
    FAILED_CHECKS=$((FAILED_CHECKS + 1))
    continue
  fi
  
  
  # For files with diagrams, check if they contain expected elements
  case "$file" in
    "Requirements.md")
      if ! grep -q 'Element 1' "$file_path"; then
        echo "❌ MISSING CONTENT: Expected diagram to contain 'Element 1' in $file"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
      fi
      ;;
    "Requirements-with-custom-mermaid.md")
      # For the custom mermaid file, ensure the custom flowchart diagram is preserved
      if ! grep -q 'flowchart TD' "$file_path"; then
        echo "❌ LOST CONTENT: Custom flowchart diagram not preserved in $file"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
      fi
      if ! grep -q 'A\[Start\] --> B{Is it?}' "$file_path"; then
        echo "❌ LOST CONTENT: Custom flowchart content not preserved in $file"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
      fi
      ;;
    *)
      # Generic checks for other files
      # Look for any node/class definition
      if ! grep -q 'class [a-zA-Z0-9]' "$file_path"; then
        echo "❌ MISSING CONTENT: Expected diagram to contain class definitions in $file"
        FAILED_CHECKS=$((FAILED_CHECKS + 1))
      fi
      ;;
  esac
done

if [ $FAILED_CHECKS -gt 0 ]; then
  echo "❌ FAILED: $FAILED_CHECKS diagram checks failed"
  exit 1
fi


# Perform a specific check for relationships in diagrams and if rendered with right arrow
if ! grep -q -- "-.->|verifies|" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Missing relationships in Requirements.md diagram"
  exit 1
fi

# Perform a specific check for relationships in diagrams and if rendered with right arrow
if ! grep -q -- "-.->|trace|" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Missing relationships in Requirements.md diagram"
  exit 1
fi

# Perform a specific check for relationships in diagrams and if rendered with right arrow
if ! grep -q -- "-->|refines|" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Missing relationships in Requirements.md diagram"
  exit 1
fi

# Perform a specific check for relationships in diagrams and if rendered with right arrow
if ! grep -q -- "--o|contains" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Missing relationships in Requirements.md diagram"
  exit 1
fi

# Perform a specific check for relationships in diagrams and if rendered with right arrow
if ! grep -q -- "-.->|deriveReqT" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Missing relationships in Requirements.md diagram"
  exit 1
fi

# Perform a specific check for relationships in diagrams and if rendered with right arrow
if ! grep -q -- "-->|satisfies|" "$TEST_DIR/specifications/Requirements.md"; then
  echo "❌ FAILED: Missing relationships in Requirements.md diagram"
  exit 1
fi
  

echo "✅ All diagram generation tests PASSED"
exit 0
