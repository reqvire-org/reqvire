use crate::graph_registry::GraphRegistry;
use maud::{html, Markup, PreEscaped};

pub fn render(registry: &GraphRegistry, nav_prefix: &str) -> Markup {
    let graph_json = super::knowledgegraph::project_graph_json(registry);

    let content = html! {
        style { (PreEscaped(KN2_CSS)) }
        section class="kn2-page" aria-label="Cytoscape knowledge graph POC" {
            div class="kn2-canvas" {
                div class="kn2-panel" aria-label="KN2 Cytoscape controls" {
                    div class="kn2-title" { "KN2 Cytoscape POC" }
                    div class="kn2-group" {
                        div class="kn2-group-title" { "Layout" }
                        button type="button" class="kn2-control active" data-kn2-group="layout" data-kn2-value="structural" { "CoSE structural" }
                        button type="button" class="kn2-control" data-kn2-group="layout" data-kn2-value="concentric" { "Concentric" }
                        button type="button" class="kn2-control" data-kn2-group="layout" data-kn2-value="breadthfirst" { "Breadthfirst" }
                        button type="button" class="kn2-control" data-kn2-group="layout" data-kn2-value="circle" { "Circle" }
                        button type="button" class="kn2-control" data-kn2-group="layout" data-kn2-value="grid" { "Grid" }
                        div class="kn2-group-title kn2-group-title-secondary" { "Clusters" }
                        button type="button" class="kn2-control active" data-kn2-group="cluster" data-kn2-value="structural" { "Structural islands" }
                        button type="button" class="kn2-control" data-kn2-group="cluster" data-kn2-value="modularity" { "Modularity-style" }
                    }
                    div class="kn2-group" {
                        div class="kn2-group-title" { "Focus" }
                        label class="kn2-slider-control" {
                            span { "Selection radius " strong id="kn2-radius-value" { "1" } }
                            input id="kn2-radius-slider" type="range" min="1" max="4" step="1" value="1" oninput="kn2SetFocusRadius(this.value)";
                            span class="kn2-slider-hint" { "1-4 hops from the selected node" }
                        }
                        label class="kn2-check" {
                            input id="kn2-focus-only" type="checkbox" onchange="kn2ToggleFocusOnly(this.checked)";
                            "Show focus only"
                        }
                    }
                    div class="kn2-group" {
                        div class="kn2-group-title" { "Relations" }
                        label class="kn2-check" { input class="kn2-relation-toggle" type="checkbox" checked data-relation="derive" onchange="kn2ToggleRelation(this.dataset.relation, this.checked)"; "derive" }
                        label class="kn2-check" { input class="kn2-relation-toggle" type="checkbox" checked data-relation="specify" onchange="kn2ToggleRelation(this.dataset.relation, this.checked)"; "specify" }
                        label class="kn2-check" { input class="kn2-relation-toggle" type="checkbox" checked data-relation="refine" onchange="kn2ToggleRelation(this.dataset.relation, this.checked)"; "refine" }
                    }
                    div class="kn2-group" {
                        div class="kn2-group-title" { "Overlays" }
                        label class="kn2-check" {
                            input id="kn2-cross-subgraph-overlay" type="checkbox" onchange="kn2ToggleCrossSubgraphOverlay(this.checked)";
                            "Cross-subgraph overlays"
                        }
                        label class="kn2-check" {
                            input id="kn2-verification-overlay" type="checkbox" onchange="kn2ToggleVerificationOverlay(this.checked)";
                            "Verification / satisfy"
                        }
                        label class="kn2-check" {
                            input id="kn2-trace-overlay" type="checkbox" onchange="kn2ToggleTraceOverlay(this.checked)";
                            "Trace"
                        }
                    }
                    div class="kn2-group" {
                        div class="kn2-group-title" { "Display" }
                        label class="kn2-check" {
                            input id="kn2-label-toggle" type="checkbox" checked onclick="kn2ToggleLabels(this.checked)";
                            "Labels"
                        }
                    }
                    div id="kn2-status" class="kn2-status" {}
                }
                div id="kn2-cytoscape" role="img" aria-label="Cytoscape project graph POC" {}
            }
            aside class="kn2-sidebar" {
                div class="kn2-search-panel" {
                    input id="kn2-search"
                        type="search"
                        placeholder="Search KN2 graph, facts, overlays"
                        class="kn2-search"
                        oninput="kn2Search(this.value)";
                    ul id="kn2-results" class="kn2-results" {}
                }
                div class="kn2-inspector-header" {
                    h2 id="kn2-inspector-title" { "Cytoscape Inspector" }
                    button type="button" onclick="kn2ClearSelection()" aria-label="Clear selection" { "x" }
                }
                div id="kn2-inspector-body" class="kn2-inspector-body" {
                    p class="kn2-empty" {
                        "Select a node to inspect project facts. Structural subgraphs ignore cross-subgraph overlays."
                    }
                }
                div class="kn2-summary" {
                    span { "Nodes " strong id="kn2-node-count" { "0" } }
                    span { "Edges " strong id="kn2-edge-count" { "0" } }
                    span { "Overlays " strong id="kn2-overlay-count" { "0" } }
                    span { "Focus " strong id="kn2-focus-state" { "all" } }
                }
            }
        }
        script src="https://cdn.jsdelivr.net/npm/cytoscape@3.29.2/dist/cytoscape.min.js" {}
        script {
            "const kn2GraphData = ";
            (PreEscaped(graph_json));
            ";"
        }
        script { (PreEscaped(KN2_JS)) }
    };

    crate::html::layouts::base("KN2", content, nav_prefix)
}

const KN2_CSS: &str = r#"
body:has(.kn2-page) > div.w-full {
  max-width: none;
  width: 100%;
  height: calc(100vh - 50px);
  margin: 0;
  padding: 0;
}
body:has(.kn2-page) > div.w-full > div.bg-white {
  height: 100%;
  padding: 0;
  border: 0;
  border-radius: 0;
  box-shadow: none;
  background: #d8d8d4;
}
.kn2-page {
  display: grid;
  grid-template-columns: minmax(0, 1fr) 390px;
  height: calc(100vh - 50px);
  min-height: 520px;
  background: #f7f7f4;
  color: #172027;
}
.kn2-canvas {
  position: relative;
  min-width: 0;
  height: 100%;
  min-height: 520px;
  background: #d8d8d4;
  overflow: hidden;
}
#kn2-cytoscape {
  width: 100%;
  height: 100%;
  min-height: 520px;
  display: block;
}
.kn2-panel {
  position: absolute;
  top: 12px;
  left: 12px;
  z-index: 3;
  width: 220px;
  max-height: calc(100% - 24px);
  overflow: auto;
  padding: 10px;
  border: 1px solid #c7c7bf;
  border-radius: 6px;
  background: rgba(247, 247, 244, 0.94);
  box-shadow: 0 2px 6px rgba(28, 28, 28, 0.10);
  color: #334155;
  font-size: 11px;
  line-height: 1.25;
}
.kn2-title {
  margin-bottom: 8px;
  color: #0f172a;
  font-size: 12px;
  font-weight: 800;
}
.kn2-group {
  display: grid;
  gap: 4px;
  margin-top: 9px;
}
.kn2-group-title {
  color: #111827;
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.kn2-group-title-secondary {
  margin-top: 6px;
  color: #52605b;
  font-size: 10px;
  letter-spacing: 0;
}
.kn2-panel button,
.kn2-check,
.kn2-slider-control {
  display: flex;
  align-items: center;
  gap: 5px;
  width: 100%;
  padding: 3px 5px;
  border: 1px solid var(--reqvire-surface-border, #c7c1b7);
  border-radius: 4px;
  background: var(--reqvire-surface-muted, #f1efe9);
  color: #172027;
  font: inherit;
  text-align: left;
  cursor: pointer;
}
.kn2-slider-control {
  display: grid;
  cursor: default;
}
.kn2-slider-control input[type="range"] {
  width: 100%;
  accent-color: #172027;
}
.kn2-slider-hint {
  color: #66736d;
  font-size: 10px;
}
.kn2-panel button:hover,
.kn2-check:hover {
  background: var(--reqvire-surface-hover, #e8e4da);
}
.kn2-panel button.active {
  border-color: #172027;
  background: #172027;
  color: #f7f7f4;
  font-weight: 800;
}
.kn2-panel button.active::before {
  content: "✓";
  display: inline-block;
  width: 10px;
  color: inherit;
  font-weight: 900;
}
.kn2-status {
  margin-top: 9px;
  padding-top: 8px;
  border-top: 1px solid #d8d8d2;
  color: #52605b;
}
.kn2-sidebar {
  display: flex;
  flex-direction: column;
  min-height: 0;
  border-left: 1px solid #c7c7bf;
  background: #f7f7f4;
  overflow: hidden;
}
.kn2-search-panel {
  flex: 0 0 auto;
  padding: 14px;
  border-bottom: 1px solid #d8d8d2;
}
.kn2-search {
  width: 100%;
  box-sizing: border-box;
  padding: 8px 10px;
  border: 1px solid #bfc3c7;
  border-radius: 4px;
  background: #fff;
  color: #172027;
  font-size: 13px;
}
.kn2-results {
  max-height: 140px;
  margin: 8px 0 0;
  padding: 0;
  overflow: auto;
  list-style: none;
}
.kn2-results button {
  width: 100%;
  padding: 6px 8px;
  border: 0;
  border-bottom: 1px solid rgba(37, 48, 58, 0.11);
  background: transparent;
  color: #172027;
  text-align: left;
}
.kn2-results button:hover {
  background: var(--reqvire-surface-hover, #e8e4da);
}
.kn2-inspector-header {
  flex: 0 0 auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 14px;
  border-bottom: 1px solid #d8d8d2;
  background: var(--reqvire-surface-hover, #e8e4da);
}
.kn2-inspector-header h2 {
  margin: 0;
  color: #111827;
  font-size: 16px;
  line-height: 1.3;
  font-weight: 800;
}
.kn2-inspector-header button {
  border: 0;
  background: transparent;
  color: #52606a;
  font-size: 22px;
  cursor: pointer;
}
.kn2-inspector-body {
  flex: 1;
  min-height: 0;
  overflow: auto;
  padding: 14px;
  background: #f7f7f4;
  color: #374151;
  font-size: 13px;
  line-height: 1.5;
}
.kn2-empty {
  margin: 0;
  color: #58646b;
  font-style: italic;
}
.kn2-section {
  padding: 13px 0;
  border-top: 1px solid rgba(37, 48, 58, 0.12);
}
.kn2-section:first-child {
  border-top: 0;
  padding-top: 0;
}
.kn2-section h3 {
  margin: 0 0 8px;
  font-size: 12px;
  text-transform: uppercase;
}
.kn2-fact {
  display: grid;
  gap: 3px;
  margin: 0 0 6px;
  padding: 7px 9px;
  border: 1px solid var(--reqvire-surface-border, #c7c1b7);
  border-radius: 5px;
  background: var(--reqvire-surface-muted, #f1efe9);
}
.kn2-fact-name {
  color: #52605b;
  font-size: 10px;
  font-weight: 700;
  letter-spacing: 0.04em;
  text-transform: uppercase;
}
.kn2-fact-value {
  overflow-wrap: anywhere;
}
.kn2-fact a {
  color: #174a75;
  text-decoration: none;
}
.kn2-pill {
  display: inline-flex;
  padding: 5px 8px;
  border: 1px solid #c7c1b7;
  border-radius: 5px;
  color: #fff;
  font-size: 12px;
  line-height: 1.25;
}
.kn2-summary {
  flex: 0 0 auto;
  display: flex;
  gap: 8px;
  align-items: center;
  justify-content: center;
  overflow-x: auto;
  padding: 5px 8px;
  border-top: 1px solid #d8d8d2;
  background: var(--reqvire-surface-hover, #e8e4da);
  color: #64748b;
  font-size: 10px;
  line-height: 1.2;
  white-space: nowrap;
}
.kn2-summary strong {
  color: #111827;
  font-size: 11px;
  font-weight: 700;
}
"#;

const KN2_JS: &str = r#"
(function() {
  const KN2_LOG_PREFIX = '[Reqvire KN2]';
  const log = (...args) => console.info(KN2_LOG_PREFIX, ...args);
  const warn = (...args) => console.warn(KN2_LOG_PREFIX, ...args);
  const container = document.getElementById('kn2-cytoscape');
  const status = document.getElementById('kn2-status');
  const data = {
    nodes: kn2GraphData.nodes.map(d => ({ ...d, node_type: d.node_type || d.type || 'other' })),
    edges: kn2GraphData.edges.map(d => ({ ...d })),
    submodels: kn2GraphData.submodels || []
  };
  const nodeById = new Map(data.nodes.map(d => [d.id, d]));
  const submodelRoots = new Map(data.submodels.map(submodel => [submodel.root_id, submodel]));
  data.edges = data.edges.filter(d => nodeById.has(d.source) && nodeById.has(d.target));
  let cy = null;
  let selectedId = null;
  let labelsEnabled = true;
  let focusRadius = 1;
  let focusOnly = false;
  let communityOnly = false;
  let currentLayout = 'structural';
  let clusterMode = 'structural';
  let communityMap = new Map();
  let crossSubgraphOverlay = false;
  let verificationOverlay = false;
  let traceOverlay = false;
  const activeRelations = new Set(['derive', 'specify', 'refine']);
  const communityPalette = ['#1976D2', '#673AB7', '#4CAF50', '#B08A00', '#8D6E63', '#00838F', '#C62828', '#5D4037', '#7B1FA2', '#2E7D32'];

  if (!window.cytoscape || !container) {
    warn('Cytoscape is unavailable or container is missing');
    return;
  }

  window.kn2ToggleLabels = toggleLabels;
  window.kn2SetFocusRadius = setFocusRadius;
  window.kn2ToggleFocusOnly = toggleFocusOnly;
  window.kn2ToggleRelation = toggleRelation;
  window.kn2ToggleCrossSubgraphOverlay = toggleCrossSubgraphOverlay;
  window.kn2ToggleVerificationOverlay = toggleVerificationOverlay;
  window.kn2ToggleTraceOverlay = toggleTraceOverlay;
  window.kn2Search = search;
  window.kn2ClearSelection = clearSelection;

  bindControlPanel();
  render();

  function bindControlPanel() {
    const panel = document.querySelector('.kn2-panel');
    if (!panel) return;
    panel.addEventListener('click', event => {
      if (!event.target || typeof event.target.closest !== 'function') return;
      const button = event.target.closest('.kn2-control');
      if (!button) return;
      event.preventDefault();
      event.stopPropagation();
      executeControl(button.dataset.kn2Group, button.dataset.kn2Value);
    }, true);
  }

  function executeControl(group, value) {
    if (!group || !value) return;
    try {
      if (group === 'community') {
        if (value === 'color') colorCommunities();
        else if (value === 'only') showSelectedCommunityOnly();
        else clearCommunities();
      } else if (group === 'layout') {
        switchLayout(value);
      } else if (group === 'cluster') {
        switchClusterMode(value);
      }
    } catch (error) {
      warn('control action failed', { group, value, error });
      setStatus(`${group} ${value} failed: ${error && error.message ? error.message : error}`);
    }
  }

  function render() {
    const elements = [
      ...data.nodes.map(node => ({
        group: 'nodes',
        data: {
          ...node,
          color: nodeFill(node),
          borderColor: nodeBorder(node),
          size: nodeSize(node),
          shortLabel: truncate(node.label, nodeLabelLimit(node))
        }
      })),
      ...data.edges.map((edge, index) => ({
        group: 'edges',
        data: {
          ...edge,
          id: `e${index}`,
          relCategory: relationCategory(edge),
          width: edge.kind === 'attachment' || edge.kind === 'concept-reference' ? 1 : 1.4
        }
      }))
    ];

    cy = cytoscape({
      container,
      elements,
      minZoom: 0.08,
      maxZoom: 4,
      wheelSensitivity: 0.18,
      style: cytoscapeStyle(),
      layout: { name: 'grid', fit: false }
    });

    cy.on('tap', 'node', event => selectNode(event.target));
    cy.on('tap', event => {
      if (event.target === cy) clearSelection();
    });
    cy.on('mouseover', 'node', event => focusNeighborhood(event.target, selectedId));
    cy.on('mouseout', 'node', () => {
      applyViewFilters();
    });

    runSelectedLayout();
    colorCommunities();
    updateSummary();
    updateRadiusLabel();
    applyViewFilters();
    setStatus(`Cytoscape loaded ${data.nodes.length} nodes / ${data.edges.length} edges. Structural subgraphs ignore cross-subgraph overlays.`);
    log('rendered', { nodes: data.nodes.length, edges: data.edges.length });
  }

  function cytoscapeStyle() {
    return [
      {
        selector: 'node',
        style: {
          'background-color': 'data(color)',
          'border-color': 'data(borderColor)',
          'border-width': 2,
          'width': 'data(size)',
          'height': 'data(size)',
          'label': 'data(shortLabel)',
          'font-size': 10,
          'font-weight': 600,
          'color': '#111827',
          'text-background-color': '#ffffff',
          'text-background-opacity': 0.88,
          'text-background-padding': 2,
          'text-border-color': '#111827',
          'text-border-width': 0.5,
          'text-border-opacity': 0.35,
          'text-valign': 'center',
          'text-halign': 'right',
          'text-margin-x': 5,
          'overlay-opacity': 0
        }
      },
      {
        selector: 'node[node_type = "capability"], node[node_type = "requirement"]',
        style: { 'shape': 'ellipse' }
      },
      {
        selector: 'node[node_type = "refinement"], node[node_type = "ontology"], node[node_type = "verification"], node[node_type = "resource"], node[node_type = "concept"], node[node_type = "other"]',
        style: { 'shape': 'round-rectangle' }
      },
      {
        selector: 'edge',
        style: {
          'curve-style': 'bezier',
          'target-arrow-shape': 'triangle',
          'target-arrow-color': '#4b5563',
          'line-color': '#4b5563',
          'width': 'data(width)',
          'label': 'data(label)',
          'font-size': 7,
          'color': '#334155',
          'text-background-color': '#d8d8d4',
          'text-background-opacity': 0.74,
          'text-background-padding': 1,
          'arrow-scale': 0.72
        }
      },
      {
        selector: '.faded',
        style: {
          'opacity': 0.18,
          'text-opacity': 0
        }
      },
      {
        selector: '.focused',
        style: {
          'opacity': 1,
          'text-opacity': 1,
          'border-width': 4,
          'z-index': 10
        }
      },
      {
        selector: '.community-colored',
        style: {
          'border-width': 5,
          'border-color': 'data(communityColor)'
        }
      },
      {
        selector: '.labels-off',
        style: {
          'label': ''
        }
      }
    ];
  }

  function coseLayout() {
    return {
      name: 'cose',
      animate: false,
      fit: true,
      padding: 80,
      nodeRepulsion: 7800,
      idealEdgeLength: 90,
      edgeElasticity: 120,
      nestingFactor: 1.25,
      gravity: 0.65,
      numIter: 1100
    };
  }

  function runStructuralSubgraphLayout() {
    if (!cy) return;
    structuralLayoutCollection().layout(coseLayout()).run();
  }

  function structuralLayoutCollection() {
    const nodes = cy.nodes().filter(node => node.data('node_type') !== 'concept');
    const edges = cy.edges().filter(edge => edgeParticipatesInSubgraphLayout(edge));
    return nodes.union(edges);
  }

  function runSelectedLayout() {
    if (!cy) return;
    const collection = structuralLayoutCollection();
    const layout = layoutFor(currentLayout);
    collection.layout(layout).run();
  }

  function layoutFor(mode) {
    if (mode === 'concentric') {
      return {
        name: 'concentric',
        animate: false,
        fit: true,
        padding: 80,
        minNodeSpacing: 14,
        concentric: node => node.degree(false),
        levelWidth: nodes => Math.max(1, nodes.maxDegree(false) / 7)
      };
    }
    if (mode === 'breadthfirst') {
      return {
        name: 'breadthfirst',
        animate: false,
        fit: true,
        padding: 80,
        directed: true,
        spacingFactor: 1.15,
        avoidOverlap: true
      };
    }
    if (mode === 'circle') {
      return {
        name: 'circle',
        animate: false,
        fit: true,
        padding: 80,
        avoidOverlap: true
      };
    }
    if (mode === 'grid') {
      return {
        name: 'grid',
        animate: false,
        fit: true,
        padding: 80,
        avoidOverlap: true
      };
    }
    return coseLayout();
  }

  function switchLayout(mode) {
    const allowed = new Set(['structural', 'concentric', 'breadthfirst', 'circle', 'grid']);
    currentLayout = allowed.has(mode) ? mode : 'structural';
    setActiveControl('layout', currentLayout);
    runSelectedLayout();
    applyViewFilters();
    setStatus(`Layout switched to ${currentLayout}; cross-subgraph overlays remain excluded from layout.`);
  }

  function setFocusRadius(value) {
    const next = Number.parseInt(value, 10);
    focusRadius = Number.isFinite(next) ? Math.max(1, Math.min(4, next)) : 1;
    communityOnly = false;
    clearActiveControl('community');
    updateRadiusLabel();
    applyViewFilters();
  }

  function updateRadiusLabel() {
    const label = document.getElementById('kn2-radius-value');
    if (label) label.textContent = String(focusRadius);
  }

  function toggleFocusOnly(enabled) {
    focusOnly = Boolean(enabled);
    applyViewFilters();
  }

  function toggleRelation(relation, enabled) {
    if (enabled) activeRelations.add(relation);
    else activeRelations.delete(relation);
    recolorCommunities();
    runSelectedLayout();
    applyViewFilters();
  }

  function toggleCrossSubgraphOverlay(enabled) {
    crossSubgraphOverlay = Boolean(enabled);
    applyViewFilters();
  }

  function toggleVerificationOverlay(enabled) {
    verificationOverlay = Boolean(enabled);
    applyViewFilters();
  }

  function toggleTraceOverlay(enabled) {
    traceOverlay = Boolean(enabled);
    applyViewFilters();
  }

  function applyViewFilters() {
    if (!cy) return;
    cy.elements().show();
    cy.elements().removeClass('faded focused');

    cy.edges().forEach(edge => {
      const category = edge.data('relCategory');
      if (category === 'attach' || category === 'concept-reference') {
        if (!crossSubgraphOverlay) edge.hide();
        return;
      }
      if (category === 'verify' || category === 'satisfy') {
        if (!verificationOverlay) edge.hide();
        return;
      }
      if (category === 'trace') {
        if (!traceOverlay) edge.hide();
        return;
      }
      if (!activeRelations.has(category)) edge.hide();
    });

    if (!crossSubgraphOverlay) {
      cy.nodes('[node_type = "concept"]').hide();
    }

    if (communityOnly) {
      const root = selectedCommunityRoot();
      const community = root ? communityMap.get(root.id()) : undefined;
      if (community === undefined) {
        setStatus('Color subgraphs first, then select a node or focus a structural subgraph.');
      } else {
        cy.nodes().forEach(node => {
          if (communityMap.get(node.id()) !== community) node.hide();
        });
      }
    }

    if (selectedId) {
      const root = cy.getElementById(selectedId);
      if (root && root.nonempty() && !root.hidden()) {
        const focus = egoCollection(root, focusRadius);
        if (focusOnly) {
          cy.nodes().forEach(node => {
            if (!focus.contains(node)) node.hide();
          });
        } else {
          cy.elements().addClass('faded');
          focus.removeClass('faded').addClass('focused');
        }
      } else {
        selectedId = null;
      }
    }

    cy.edges().forEach(edge => {
      if (edge.source().hidden() || edge.target().hidden()) edge.hide();
    });

    updateSummary();
    const visibleNodes = cy.nodes(':visible').length;
    const visibleEdges = cy.edges(':visible').length;
    const focusLabel = selectedId ? `r${focusRadius}${focusOnly ? ' only' : ''}` : 'all';
    const overlays = [
      crossSubgraphOverlay ? 'cross-subgraph' : null,
      verificationOverlay ? 'verification' : null,
      traceOverlay ? 'trace' : null
    ].filter(Boolean).join(', ') || 'off';
    setStatus(`View: ${visibleNodes} visible nodes / ${visibleEdges} visible edges; focus ${focusLabel}; structural relations ${Array.from(activeRelations).join(', ') || 'none'}; overlays ${overlays}.`);
  }

  function egoCollection(root, radius) {
    let keep = cy.collection(root);
    let frontier = [root];
    const seen = new Set([root.id()]);
    for (let depth = 0; depth < radius; depth += 1) {
      const nextFrontier = [];
      frontier.forEach(node => {
        node.connectedEdges().forEach(edge => {
          if (edge.hidden() || !edgeParticipatesInSubgraphLayout(edge)) return;
          const other = edge.connectedNodes().difference(node)[0];
          if (!other || other.hidden()) return;
          keep = keep.union(edge).union(other);
          if (!seen.has(other.id())) {
            seen.add(other.id());
            nextFrontier.push(other);
          }
        });
      });
      frontier = nextFrontier;
    }
    return keep;
  }

  function colorCommunities() {
    communityOnly = false;
    recolorCommunities();
    applyViewFilters();
    setStatus(clusterStatusMessage());
  }

  function switchClusterMode(mode) {
    clusterMode = mode === 'modularity' ? 'modularity' : 'structural';
    communityOnly = false;
    clearActiveControl('community');
    setActiveControl('cluster', clusterMode);
    recolorCommunities();
    applyViewFilters();
    setStatus(clusterStatusMessage());
  }

  function recolorCommunities() {
    communityMap = detectCommunities();
    applyCommunityColors();
  }

  function clusterStatusMessage() {
    const count = new Set(communityMap.values()).size;
    if (clusterMode === 'modularity') {
      return `Colored ${count} modularity-style clusters using submodel structural relations only. Attachments, verification, satisfaction, trace, and concept overlays are ignored for cluster detection.`;
    }
    const source = submodelRoots.size > 0 ? `${submodelRoots.size} Reqvire root submodels` : 'structural islands';
    return `Colored ${count} ${source}. Attachments, verification, satisfaction, trace, and concept overlays are ignored for cluster detection.`;
  }

  function applyCommunityColors() {
    cy.nodes().forEach(node => {
      const community = communityMap.get(node.id());
      if (community === undefined) {
        node.removeClass('community-colored');
        return;
      }
      node.data('communityColor', communityPalette[community % communityPalette.length]);
      node.addClass('community-colored');
    });
  }

  function showSelectedCommunityOnly() {
    const selected = selectedId ? cy.getElementById(selectedId) : null;
    if (!selected || selected.empty()) {
      communityOnly = false;
      clearActiveControl('community');
      applyViewFilters();
      setStatus('Select a node before focusing a structural subgraph.');
      return;
    }
    if (communityMap.size === 0) colorCommunities();
    if (communityMap.get(selected.id()) === undefined) {
      communityOnly = false;
      clearActiveControl('community');
      applyViewFilters();
      setStatus('Selected node is not part of a structural subgraph.');
      return;
    }
    communityOnly = true;
    setActiveControl('community', 'only');
    applyViewFilters();
  }

  function clearCommunities() {
    communityOnly = false;
    communityMap = new Map();
    cy.nodes().removeClass('community-colored');
    cy.nodes().removeData('communityColor');
    clearActiveControl('community');
    applyViewFilters();
    setStatus('Subgraph colors cleared.');
  }

  function selectedCommunityRoot() {
    if (selectedId) {
      const selected = cy.getElementById(selectedId);
      if (selected.nonempty()) return selected;
    }
    return null;
  }

  function detectCommunities() {
    if (clusterMode === 'modularity') return detectModularityStyleCommunities();
    if (submodelRoots.size > 0) return detectReqvireRootSubmodels();
    return detectStructuralIslands();
  }

  function detectReqvireRootSubmodels() {
    const result = new Map();
    const seen = new Set();
    let community = 0;

    data.submodels.forEach(submodel => {
      const root = cy.getElementById(submodel.root_id);
      if (!root || root.empty() || seen.has(root.id()) || root.data('node_type') === 'concept') return;
      community = visitStructuralComponent(root, result, seen, community);
    });

    cy.nodes().forEach(start => {
      if (seen.has(start.id()) || start.data('node_type') === 'concept') return;
      community = visitStructuralComponent(start, result, seen, community);
    });

    return result;
  }

  function visitStructuralComponent(start, result, seen, community) {
    const queue = [start];
    seen.add(start.id());
    while (queue.length) {
      const node = queue.shift();
      result.set(node.id(), community);
      node.connectedEdges().forEach(edge => {
        if (!edgeParticipatesInCommunity(edge)) return;
        const other = edge.connectedNodes().difference(node)[0];
        if (!other || seen.has(other.id()) || other.data('node_type') === 'concept') return;
        seen.add(other.id());
        queue.push(other);
      });
    }
    return community + 1;
  }

  function detectStructuralIslands() {
    const result = new Map();
    const seen = new Set();
    let community = 0;
    cy.nodes().forEach(start => {
      if (seen.has(start.id()) || start.data('node_type') === 'concept') return;
      community = visitStructuralComponent(start, result, seen, community);
    });
    return result;
  }

  function detectModularityStyleCommunities() {
    const nodes = cy.nodes().filter(node => node.data('node_type') !== 'concept');
    const labels = new Map(nodes.map(node => [node.id(), node.id()]));
    for (let iteration = 0; iteration < 24; iteration += 1) {
      let changed = false;
      nodes.forEach(node => {
        const counts = new Map();
        node.connectedEdges().forEach(edge => {
          if (!edgeParticipatesInCommunity(edge)) return;
          const other = edge.connectedNodes().difference(node)[0];
          if (!other || other.data('node_type') === 'concept') return;
          const label = labels.get(other.id());
          if (!label) return;
          counts.set(label, (counts.get(label) || 0) + 1);
        });
        if (counts.size === 0) return;
        const next = Array.from(counts.entries())
          .sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]))[0][0];
        if (next !== labels.get(node.id())) {
          labels.set(node.id(), next);
          changed = true;
        }
      });
      if (!changed) break;
    }
    const compressed = new Map();
    const result = new Map();
    let community = 0;
    nodes.forEach(node => {
      const label = labels.get(node.id());
      if (!compressed.has(label)) {
        compressed.set(label, community);
        community += 1;
      }
      result.set(node.id(), compressed.get(label));
    });
    return result;
  }

  function edgeParticipatesInCommunity(edge) {
    return edgeParticipatesInSubgraphLayout(edge);
  }

  function edgeParticipatesInSubgraphLayout(edge) {
    const category = edge.data('relCategory');
    return isSubmodelStructureCategory(category) && activeRelations.has(category);
  }

  function isSubmodelStructureCategory(category) {
    return category === 'derive' || category === 'specify' || category === 'refine';
  }

  function isCrossSubgraphOverlay(edge) {
    const category = edge.data ? edge.data('relCategory') : relationCategory(edge);
    return category === 'attach' || category === 'concept-reference';
  }

  function topDegreeNodes(limit) {
    return cy.nodes(':visible')
      .sort((a, b) => b.degree(false) - a.degree(false))
      .slice(0, limit);
  }

  function toggleLabels(enabled) {
    labelsEnabled = enabled;
    if (!cy) return;
    cy.elements().toggleClass('labels-off', !enabled);
  }

  function search(value) {
    const term = (value || '').trim().toLowerCase();
    const results = document.getElementById('kn2-results');
    if (!results) return;
    results.innerHTML = '';
    if (!term) return;
    data.nodes
      .filter(node => searchCorpus(node).includes(term))
      .slice(0, 24)
      .forEach(node => {
        const item = document.createElement('li');
        const button = document.createElement('button');
        button.type = 'button';
        button.textContent = node.label;
        button.onclick = () => {
          const cyNode = cy.getElementById(node.id);
          if (cyNode.nonempty()) selectNode(cyNode);
        };
        item.appendChild(button);
        results.appendChild(item);
      });
  }

  function selectNode(node) {
    selectedId = node.id();
    applyViewFilters();
    cy.animate({ center: { eles: node }, zoom: Math.min(1.4, Math.max(cy.zoom(), 0.75)) }, { duration: 260 });
    renderInspector(node.data());
  }

  function clearSelection() {
    selectedId = null;
    applyViewFilters();
    document.getElementById('kn2-inspector-title').textContent = 'Cytoscape Inspector';
    document.getElementById('kn2-inspector-body').innerHTML = '<p class="kn2-empty">Select a node to inspect project facts. Structural subgraphs ignore cross-subgraph overlays.</p>';
  }

  function focusNeighborhood(node, pinnedId) {
    let focus = node.closedNeighborhood();
    if (pinnedId && pinnedId !== node.id()) {
      const pinned = cy.getElementById(pinnedId);
      if (pinned.nonempty()) focus = focus.union(pinned.closedNeighborhood());
    }
    cy.elements().addClass('faded').removeClass('focused');
    focus.removeClass('faded').addClass('focused');
    node.connectedEdges().removeClass('faded').addClass('focused');
  }

  function renderInspector(d) {
    document.getElementById('kn2-inspector-title').textContent = d.label || d.id;
    document.getElementById('kn2-inspector-body').innerHTML = [
      section('Kind', `<span class="kn2-pill" style="background:${nodeFill(d)};border-color:${nodeBorder(d)};color:${readableTextColor(nodeFill(d))}">${escapeHtml(d.element_type || d.node_type)}</span>`),
      section('Identifier', fieldHtml('id', `<span>${escapeHtml(d.identifier || d.id)}</span>`)),
      d.file_path ? section('Source', fieldHtml('file', `<a href="${escapeAttr(d.link)}">${escapeHtml(d.file_path)}:${d.line_number}</a>`)) : '',
      section('Description', `<p>${escapeHtml(d.description || 'None specified.')}</p>`),
      factsSection('Governance', d.governance),
      factsSection('Metadata', d.metadata),
      factsSection('Outgoing Facts', d.outgoing),
      factsSection('Incoming Facts', d.incoming),
      factsSection('Attachments', d.attachments),
      factsSection('Concept References', d.concept_references)
    ].filter(Boolean).join('');
  }

  function factsSection(title, facts) {
    if (!facts || facts.length === 0) return '';
    return section(title, facts.map(f => {
      const value = f.link ? `<a href="${escapeAttr(f.link)}">${escapeHtml(f.value)}</a>` : escapeHtml(f.value);
      return fieldHtml(f.name, value);
    }).join(''));
  }

  function fieldHtml(name, value) {
    return `<div class="kn2-fact"><span class="kn2-fact-name">${escapeHtml(name)}</span><span class="kn2-fact-value">${value}</span></div>`;
  }

  function section(title, body) {
    return `<div class="kn2-section"><h3>${escapeHtml(title)}</h3>${body}</div>`;
  }

  function updateSummary() {
    document.getElementById('kn2-node-count').textContent = cy ? cy.nodes(':visible').length : data.nodes.length;
    document.getElementById('kn2-edge-count').textContent = cy ? cy.edges(':visible').length : data.edges.length;
    document.getElementById('kn2-overlay-count').textContent = cy
      ? cy.edges(':visible').filter(edge => isCrossSubgraphOverlay(edge)).length
      : data.edges.filter(edge => {
          const category = relationCategory(edge);
          return category === 'attach' || category === 'concept-reference';
        }).length;
    const focusLabel = selectedId ? `r${focusRadius}${focusOnly ? ' only' : ''}` : 'all';
    document.getElementById('kn2-focus-state').textContent = communityOnly ? `${focusLabel}+subgraph` : focusLabel;
  }

  function setStatus(message) {
    if (status) status.textContent = message;
  }

  function setActiveControl(group, value) {
    const buttons = document.querySelectorAll(`[data-kn2-group="${escapeSelector(group)}"]`);
    buttons.forEach(button => {
      button.classList.toggle('active', button.dataset.kn2Value === value);
    });
  }

  function clearActiveControl(group) {
    const buttons = document.querySelectorAll(`[data-kn2-group="${escapeSelector(group)}"]`);
    buttons.forEach(button => button.classList.remove('active'));
  }

  function searchCorpus(d) {
    const facts = [].concat(d.metadata || [], d.governance || [], d.outgoing || [], d.incoming || [], d.attachments || [], d.concept_references || []);
    return [d.label, d.element_type, d.identifier, d.file_path, d.description]
      .concat(facts.flatMap(f => [f.name, f.value, f.kind]))
      .join(' ').toLowerCase();
  }

  function nodeLabelLimit(node) {
    return ['capability', 'requirement', 'ontology'].includes(node.node_type) ? 26 : 34;
  }

  function relationCategory(edge) {
    const label = String(edge.label || '').toLowerCase();
    const kind = String(edge.kind || '').toLowerCase();
    if (kind === 'attachment' || label === 'attaches') return 'attach';
    if (kind === 'concept-reference' || label === 'conceptref') return 'concept-reference';
    if (label.includes('derive')) return 'derive';
    if (label.includes('specif')) return 'specify';
    if (label.includes('refine')) return 'refine';
    if (label.includes('verif')) return 'verify';
    if (label.includes('satisf')) return 'satisfy';
    if (label.includes('trace')) return 'trace';
    return 'trace';
  }

  function nodeSize(node) {
    const degree = data.edges.filter(edge => edge.source === node.id || edge.target === node.id).length;
    return Math.min(34, 11 + Math.sqrt(degree + 1) * 3.8);
  }

  function nodeFill(d) {
    return ({
      capability: '#1976D2',
      requirement: '#673AB7',
      refinement: '#673AB7',
      verification: '#4CAF50',
      ontology: '#B08A00',
      resource: '#FFCA28',
      concept: '#8D6E63',
      other: '#424242'
    })[d.node_type] || '#424242';
  }

  function nodeBorder(d) {
    return ({
      capability: '#0f4d8a',
      requirement: '#452480',
      refinement: '#452480',
      verification: '#2f6f32',
      ontology: '#775d00',
      resource: '#b88c00',
      concept: '#5f493f',
      other: '#232323'
    })[d.node_type] || '#232323';
  }

  function readableTextColor(background) {
    if (!background || !background.startsWith('#')) return '#172027';
    const value = background.slice(1).padEnd(6, '0').slice(0, 6);
    const r = parseInt(value.slice(0, 2), 16);
    const g = parseInt(value.slice(2, 4), 16);
    const b = parseInt(value.slice(4, 6), 16);
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
    return luminance > 0.55 ? '#172027' : '#ffffff';
  }

  function truncate(value, max) {
    value = value || '';
    return value.length > max ? value.slice(0, Math.max(1, max - 1)) + '…' : value;
  }

  function escapeHtml(value) {
    return String(value ?? '').replace(/[&<>"']/g, c => ({'&':'&amp;','<':'&lt;','>':'&gt;','"':'&quot;',"'":'&#39;'}[c]));
  }

  function escapeAttr(value) {
    return escapeHtml(value).replace(/`/g, '&#96;');
  }

  function escapeSelector(value) {
    if (window.CSS && typeof window.CSS.escape === 'function') return window.CSS.escape(value);
    return String(value).replace(/["\\]/g, '\\$&');
  }
})();
"#;
