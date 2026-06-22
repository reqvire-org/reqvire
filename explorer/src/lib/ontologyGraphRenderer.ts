// @ts-nocheck
import Graph from "graphology";
import Sigma from "sigma";
import forceAtlas2 from "graphology-layout-forceatlas2";
import { createDrawCurvedEdgeLabel, createEdgeCurveProgram, indexParallelEdgesIndex } from "@sigma/edge-curve";
import { createNodeImageProgram } from "@sigma/node-image";
import { EdgeProgram } from "sigma/rendering";
import { floatColor } from "sigma/utils";
import type { OntologyGraphData, OntologyGraphNode } from "../store/types";
import { cssVar } from "@ds";

export interface OntologyGraphRendererHandle {
  destroy: () => void;
  filter: (query: string) => void;
  focusNode: (nodeId: string) => void;
  clearSelection: () => void;
  resetLayout: () => void;
  setFilter: (category: string, value: string, active: boolean) => void;
}

export function mountOntologyGraph(
  container: HTMLElement,
  ontologyGraphData: OntologyGraphData,
  options: {
    onSelect?: (node: OntologyGraphNode | null) => void;
  } = {},
): OntologyGraphRendererHandle {
    const onSelect = options.onSelect;
    ontologyGraphData = {
        nodes: ontologyGraphData?.nodes ?? [],
        edges: ontologyGraphData?.edges ?? [],
    };
    const noopHandle = {
        destroy() {},
        filter() {},
        focusNode() {},
        clearSelection() {},
        resetLayout() {},
        setFilter() {},
    };

function createOntologyNotationEdgeProgram({ drawLabel, marker = 'diamond' } = {}) {
    const { UNSIGNED_BYTE, FLOAT } = WebGLRenderingContext;
    const markerKind = marker === 'hollowTriangle' ? 2 : 1;
    const targetArrowFill = marker === 'diamond' ? 1 : 0;
    const dashLength = 8.0;
    const dashPeriod = 16.0;
    const vertexShader = `
attribute vec4 a_id;
attribute vec4 a_color;
attribute float a_direction;
attribute float a_thickness;
attribute vec2 a_source;
attribute vec2 a_target;
attribute float a_current;
attribute float a_curvature;
attribute float a_sourceSize;
attribute float a_targetSize;

uniform mat3 u_matrix;
uniform float u_sizeRatio;
uniform float u_pixelRatio;
uniform vec2 u_dimensions;
uniform float u_minEdgeThickness;
uniform float u_feather;
uniform float u_widenessToThicknessRatio;

varying vec4 v_color;
varying float v_thickness;
varying float v_feather;
varying vec2 v_cpA;
varying vec2 v_cpB;
varying vec2 v_cpC;
varying vec2 v_sourcePoint;
varying vec2 v_targetPoint;
varying float v_sourceSize;
varying float v_targetSize;

const float bias = 255.0 / 254.0;
const float epsilon = 0.7;

vec2 clipspaceToViewport(vec2 pos, vec2 dimensions) {
  return vec2((pos.x + 1.0) * dimensions.x / 2.0, (pos.y + 1.0) * dimensions.y / 2.0);
}

vec2 viewportToClipspace(vec2 pos, vec2 dimensions) {
  return vec2(pos.x / dimensions.x * 2.0 - 1.0, pos.y / dimensions.y * 2.0 - 1.0);
}

void main() {
  vec2 position = a_source * max(0.0, a_current) + a_target * max(0.0, 1.0 - a_current);
  position = (u_matrix * vec3(position, 1)).xy;

  vec2 source = (u_matrix * vec3(a_source, 1)).xy;
  vec2 target = (u_matrix * vec3(a_target, 1)).xy;

  vec2 viewportPosition = clipspaceToViewport(position, u_dimensions);
  vec2 viewportSource = clipspaceToViewport(source, u_dimensions);
  vec2 viewportTarget = clipspaceToViewport(target, u_dimensions);

  vec2 delta = viewportTarget.xy - viewportSource.xy;
  float len = max(1.0, length(delta));
  vec2 normal = vec2(-delta.y, delta.x) * a_direction;
  vec2 unitNormal = normal / len;
  float boundingBoxThickness = len * a_curvature;

  float curveThickness = max(u_minEdgeThickness, a_thickness);
  v_thickness = curveThickness * u_pixelRatio;
  v_feather = u_feather;

  v_cpA = viewportSource;
  v_cpB = 0.5 * (viewportSource + viewportTarget) + unitNormal * a_direction * boundingBoxThickness;
  v_cpC = viewportTarget;
  v_sourcePoint = viewportSource;
  v_targetPoint = viewportTarget;
  v_sourceSize = a_sourceSize * u_pixelRatio / u_sizeRatio;
  v_targetSize = a_targetSize * u_pixelRatio / u_sizeRatio;

  vec2 viewportOffsetPosition = (
    viewportPosition +
    unitNormal * (boundingBoxThickness / 2.0 + sign(boundingBoxThickness) * (curveThickness * u_widenessToThicknessRatio + 28.0 + epsilon)) *
    max(0.0, a_direction)
  );

  position = viewportToClipspace(viewportOffsetPosition, u_dimensions);
  gl_Position = vec4(position, 0, 1);

  #ifdef PICKING_MODE
  v_color = a_id;
  #else
  v_color = a_color;
  #endif
  v_color.a *= bias;
}
`;
    const fragmentShader = `
precision highp float;

varying vec4 v_color;
varying float v_thickness;
varying float v_feather;
varying vec2 v_cpA;
varying vec2 v_cpB;
varying vec2 v_cpC;
varying vec2 v_sourcePoint;
varying vec2 v_targetPoint;
varying float v_sourceSize;
varying float v_targetSize;

uniform float u_lengthToThicknessRatio;
uniform float u_widenessToThicknessRatio;
uniform float u_markerKind;
uniform float u_targetArrowFill;
uniform float u_dashLength;
uniform float u_dashPeriod;
uniform float u_pixelRatio;

const vec4 transparent = vec4(0.0, 0.0, 0.0, 0.0);

float det(vec2 a, vec2 b) {
  return a.x * b.y - b.x * a.y;
}

vec2 getDistanceVector(vec2 b0, vec2 b1, vec2 b2) {
  float a = det(b0, b2), b = 2.0 * det(b1, b0), d = 2.0 * det(b2, b1);
  float f = b * d - a * a;
  vec2 d21 = b2 - b1, d10 = b1 - b0, d20 = b2 - b0;
  vec2 gf = 2.0 * (b * d21 + d * d10 + a * d20);
  gf = vec2(gf.y, -gf.x);
  vec2 pp = -f * gf / dot(gf, gf);
  vec2 d0p = b0 - pp;
  float ap = det(d0p, d20), bp = 2.0 * det(d10, d0p);
  float t = clamp((ap + bp) / (2.0 * a + b + d), 0.0, 1.0);
  return mix(mix(b0, b1, t), mix(b1, b2, t), t);
}

float distToQuadraticBezierCurve(vec2 p, vec2 b0, vec2 b1, vec2 b2) {
  return length(getDistanceVector(b0 - p, b1 - p, b2 - p));
}

float distToSegment(vec2 p, vec2 a, vec2 b) {
  vec2 pa = p - a;
  vec2 ba = b - a;
  float h = clamp(dot(pa, ba) / max(dot(ba, ba), 1.0), 0.0, 1.0);
  return length(pa - ba * h);
}

float approximateCurveT(vec2 p) {
  vec2 chord = v_cpC - v_cpA;
  float chordLength = max(1.0, dot(chord, chord));
  return clamp(dot(p - v_cpA, chord) / chordLength, 0.0, 1.0);
}

void main(void) {
  vec2 p = gl_FragCoord.xy;
  vec2 sourceTangentSeed = mix(v_cpB - v_cpA, v_cpC - v_cpB, 0.08);
  if (length(sourceTangentSeed) < 0.5) sourceTangentSeed = v_cpC - v_cpA;
  vec2 sourceTangent = normalize(sourceTangentSeed);
  vec2 sourceNormal = vec2(-sourceTangent.y, sourceTangent.x);
  vec2 targetTangentSeed = mix(v_cpB - v_cpA, v_cpC - v_cpB, 0.92);
  if (length(targetTangentSeed) < 0.5) targetTangentSeed = v_cpC - v_cpA;
  vec2 targetTangent = normalize(targetTangentSeed);
  vec2 targetNormal = vec2(-targetTangent.y, targetTangent.x);

  float diamondRadius = 7.5 * u_pixelRatio;
  float diamondStroke = max(1.15 * u_pixelRatio, v_thickness * 0.45);
  vec2 diamondCenter = v_sourcePoint + sourceTangent * (v_sourceSize + diamondRadius + 3.0 * u_pixelRatio);
  vec2 diamondRel = p - diamondCenter;
  float diamondSignedDistance = abs(dot(diamondRel, sourceTangent)) + abs(dot(diamondRel, sourceNormal)) - diamondRadius;
  bool diamondStrokeHit = u_markerKind < 1.5 && abs(diamondSignedDistance) <= diamondStroke;

  float triangleLength = 11.0 * u_pixelRatio;
  float triangleWidth = 6.6 * u_pixelRatio;
  float triangleStroke = max(1.05 * u_pixelRatio, v_thickness * 0.4);
  vec2 triangleTip = v_targetPoint - targetTangent * (v_targetSize + 3.0 * u_pixelRatio);
  vec2 triangleBase = triangleTip - targetTangent * triangleLength;
  vec2 triangleLeft = triangleBase + targetNormal * triangleWidth;
  vec2 triangleRight = triangleBase - targetNormal * triangleWidth;
  float triangleDistance = min(
    distToSegment(p, triangleTip, triangleLeft),
    min(distToSegment(p, triangleTip, triangleRight), distToSegment(p, triangleLeft, triangleRight))
  );
  bool triangleStrokeHit = u_markerKind > 1.5 && triangleDistance <= triangleStroke;

  float dist = distToQuadraticBezierCurve(p, v_cpA, v_cpB, v_cpC);
  float thickness = v_thickness;
  float distToTarget = length(p - v_targetPoint);
  float targetArrowLength = v_targetSize + thickness * u_lengthToThicknessRatio;
  if (u_targetArrowFill > 0.5 && distToTarget < targetArrowLength) {
    thickness = (distToTarget - v_targetSize) / (targetArrowLength - v_targetSize) * u_widenessToThicknessRatio * thickness;
  }

  float t = approximateCurveT(p);
  float dashPosition = mod(t * length(v_cpC - v_cpA), u_dashPeriod);
  bool dashVisible = dashPosition < u_dashLength || (u_targetArrowFill > 0.5 && distToTarget < targetArrowLength);
  float halfThickness = thickness / 2.0;

  if (diamondStrokeHit || triangleStrokeHit || (dashVisible && dist < halfThickness)) {
    #ifdef PICKING_MODE
    gl_FragColor = v_color;
    #else
    float edgeAlpha = 1.0 - smoothstep(max(halfThickness - v_feather, 0.0), halfThickness, dist);
    float diamondAlpha = 1.0 - smoothstep(max(diamondStroke - v_feather, 0.0), diamondStroke, abs(diamondSignedDistance));
    float triangleAlpha = 1.0 - smoothstep(max(triangleStroke - v_feather, 0.0), triangleStroke, triangleDistance);
    gl_FragColor = vec4(v_color.rgb, v_color.a * max(edgeAlpha, max(diamondAlpha, triangleAlpha)));
    #endif
  } else {
    gl_FragColor = transparent;
  }
}
`;

    return class OntologyNotationEdgeProgram extends EdgeProgram {
        drawLabel = drawLabel;

        getDefinition() {
            return {
                VERTICES: 6,
                VERTEX_SHADER_SOURCE: vertexShader,
                FRAGMENT_SHADER_SOURCE: fragmentShader,
                METHOD: WebGLRenderingContext.TRIANGLES,
                UNIFORMS: [
                    'u_matrix',
                    'u_sizeRatio',
                    'u_dimensions',
                    'u_pixelRatio',
                    'u_feather',
                    'u_minEdgeThickness',
                    'u_lengthToThicknessRatio',
                    'u_widenessToThicknessRatio',
                    'u_markerKind',
                    'u_targetArrowFill',
                    'u_dashLength',
                    'u_dashPeriod'
                ],
                ATTRIBUTES: [
                    { name: 'a_source', size: 2, type: FLOAT },
                    { name: 'a_target', size: 2, type: FLOAT },
                    { name: 'a_sourceSize', size: 1, type: FLOAT },
                    { name: 'a_targetSize', size: 1, type: FLOAT },
                    { name: 'a_thickness', size: 1, type: FLOAT },
                    { name: 'a_curvature', size: 1, type: FLOAT },
                    { name: 'a_color', size: 4, type: UNSIGNED_BYTE, normalized: true },
                    { name: 'a_id', size: 4, type: UNSIGNED_BYTE, normalized: true }
                ],
                CONSTANT_ATTRIBUTES: [
                    { name: 'a_current', size: 1, type: FLOAT },
                    { name: 'a_direction', size: 1, type: FLOAT }
                ],
                CONSTANT_DATA: [
                    [0, 1],
                    [0, -1],
                    [1, 1],
                    [0, -1],
                    [1, 1],
                    [1, -1]
                ]
            };
        }

        processVisibleItem(edgeIndex, startIndex, sourceData, targetData, data) {
            const color = floatColor(data.color);
            const curvature = Number(data.curvature ?? 0.25);
            const thickness = Number(data.size || 1);
            const array = this.array;

            array[startIndex++] = sourceData.x;
            array[startIndex++] = sourceData.y;
            array[startIndex++] = targetData.x;
            array[startIndex++] = targetData.y;
            array[startIndex++] = sourceData.size;
            array[startIndex++] = targetData.size;
            array[startIndex++] = thickness;
            array[startIndex++] = curvature;
            array[startIndex++] = color;
            array[startIndex++] = edgeIndex;
        }

        setUniforms(params, { gl, uniformLocations }) {
            gl.uniformMatrix3fv(uniformLocations.u_matrix, false, params.matrix);
            gl.uniform1f(uniformLocations.u_pixelRatio, params.pixelRatio);
            gl.uniform1f(uniformLocations.u_sizeRatio, params.sizeRatio);
            gl.uniform1f(uniformLocations.u_feather, params.antiAliasingFeather);
            gl.uniform2f(uniformLocations.u_dimensions, params.width * params.pixelRatio, params.height * params.pixelRatio);
            gl.uniform1f(uniformLocations.u_minEdgeThickness, params.minEdgeThickness);
            gl.uniform1f(uniformLocations.u_lengthToThicknessRatio, 3.2);
            gl.uniform1f(uniformLocations.u_widenessToThicknessRatio, 2.1);
            gl.uniform1f(uniformLocations.u_markerKind, markerKind);
            gl.uniform1f(uniformLocations.u_targetArrowFill, targetArrowFill);
            gl.uniform1f(uniformLocations.u_dashLength, dashLength * params.pixelRatio);
            gl.uniform1f(uniformLocations.u_dashPeriod, dashPeriod * params.pixelRatio);
        }
    };
}

function createConstructDiamondEdgeProgram(options = {}) {
    return createOntologyNotationEdgeProgram({ ...options, marker: 'diamond' });
}

function createSubclassTriangleEdgeProgram(options = {}) {
    return createOntologyNotationEdgeProgram({ ...options, marker: 'hollowTriangle' });
}


    if (!ontologyGraphData || !ontologyGraphData.nodes.length) {
        return noopHandle;
    }

    const reqvireSurfaceHover = cssVar('--bg-sunken');
    const textStrong = cssVar('--text-strong');
    const textBody = cssVar('--text-body');
    const textMuted = cssVar('--text-muted');
    const textInverse = cssVar('--slate-0');
    const labelOnLight = cssVar('--slate-950');
    const defaultEdge = cssVar('--edge-default');
    const colorBySemanticType = {
        class: { fill: cssVar('--rdf-class'), stroke: textStrong, text: textStrong },
        'object-property': { fill: cssVar('--rdf-objprop'), stroke: textBody, text: textStrong },
        'datatype-property': { fill: cssVar('--rdf-dtprop'), stroke: textBody, text: textInverse },
        'rdf-property': { fill: cssVar('--rdf-rdfprop'), stroke: textBody, text: textInverse },
        'skos-concept': { fill: cssVar('--rdf-concept'), stroke: cssVar('--ontology-ink'), text: textStrong },
        'named-individual': { fill: cssVar('--rdf-individual'), stroke: cssVar('--requirement-ink'), text: textInverse },
        datatype: { fill: cssVar('--rdf-datatype'), stroke: cssVar('--ontology-ink'), text: textStrong },
        restriction: { fill: cssVar('--rdf-restriction'), stroke: textStrong, text: textStrong },
        'class-expression': { fill: cssVar('--rdf-classexpr'), stroke: textStrong, text: textStrong },
        'node-shape': { fill: cssVar('--rdf-nodeshape'), stroke: cssVar('--rdf-propshape'), text: textInverse },
        'property-shape': { fill: cssVar('--rdf-propshape'), stroke: cssVar('--rdf-nodeshape'), text: textInverse },
        resource: { fill: reqvireSurfaceHover, stroke: cssVar('--other-ink'), text: textStrong }
    };
    const colorByLayer = {
        concepts: { fill: cssVar('--rdf-concept'), stroke: cssVar('--ontology-ink'), text: textStrong },
        'reqvire-context': { fill: cssVar('--info'), stroke: cssVar('--info-border'), text: textInverse },
        'external-source': { fill: cssVar('--other'), stroke: cssVar('--other-ink'), text: textStrong }
    };
    const ontologyZIndex = {
        mutedNode: -10,
        base: 0,
        focusedEdge: 100,
        focusedNeighborNode: 200,
        focusedNode: 300
    };
    const drawSigmaCurvedEdgeLabel = createDrawCurvedEdgeLabel({
        curvatureAttribute: 'curvature',
        defaultCurvature: 0.08,
        keepLabelUpright: true
    });
    function nodePalette(nodeData) {
        if (nodeData.layer && nodeData.layer !== 'authored') {
            return colorByLayer[nodeData.layer] || colorBySemanticType.resource;
        }
        return colorBySemanticType[nodeData.semantic_type] || colorBySemanticType.resource;
    }
    const rawConnectionCounts = new Map();
    ontologyGraphData.edges.forEach(edge => {
        rawConnectionCounts.set(edge.source, (rawConnectionCounts.get(edge.source) || 0) + 1);
        rawConnectionCounts.set(edge.target, (rawConnectionCounts.get(edge.target) || 0) + 1);
    });
    const rawNodes = ontologyGraphData.nodes.map(node => {
        const displayLabel = graphNodeDisplayLabel(node);
        const shape = nodeShapeType(node);
        const labelLength = String(displayLabel || '').length;
        const connectionCount = rawConnectionCounts.get(node.id) || 0;
        const diameter = Math.max(46, Math.min(92, 46 + Math.sqrt(connectionCount) * 9));
        const boxWidth = Math.max(86, Math.min(238, labelLength * 6.5 + 26));
        return {
            ...node,
            display_label: displayLabel,
            shape,
            width: shape === 'class-anchor' ? diameter : boxWidth,
            height: shape === 'class-anchor' ? diameter : 34
        };
    });
    const rawNodeById = new Map(rawNodes.map(node => [node.id, node]));
    const propertyNodes = rawNodes.filter(isOntologyPropertyNode);
    const nodes = rawNodes.filter(node => !isOntologyPropertyNode(node));
    const links = buildRenderedOntologyLinks(ontologyGraphData.edges, rawNodeById, propertyNodes);
    const nodeById = new Map(nodes.map(node => [node.id, node]));
    const connectionCounts = computeRenderedNodeConnections(nodes, links);
    const adjacency = new Map(nodes.map(node => [node.id, new Set([node.id])]));
    const linkAdjacency = new Map(nodes.map(node => [node.id, []]));
    links.forEach(link => {
        const source = typeof link.source === 'string' ? link.source : link.source.id;
        const target = typeof link.target === 'string' ? link.target : link.target.id;
        if (adjacency.has(source)) adjacency.get(source).add(target);
        if (adjacency.has(target)) adjacency.get(target).add(source);
        if (linkAdjacency.has(source)) linkAdjacency.get(source).push(link);
        if (target !== source && linkAdjacency.has(target)) linkAdjacency.get(target).push(link);
    });
    const filterState = {
        role: new Set([
            'ontology-term',
            'shacl-shape',
            'resource',
            'external-reference'
        ]),
        origin: new Set([
            'authored',
            'registry',
            'construct'
        ]),
        layer: new Set([
            'layer-authored',
            'layer-concepts'
        ]),
        construct: new Set([
            'domain-range',
            'subclass',
            'membership',
            'disjoint',
            'equivalence',
            'inverse',
            'property-chain',
            'property-characteristic',
            'class-expression',
            'shape-overlay'
        ])
    };
    const relationFilterState = new Set([
        'class-membership',
        'class-disjointness',
        'class-expressions'
    ]);
    rawNodes.forEach(node => {
        node._ontologyRoles = nodeRoleValues(node);
        node._ontologyConstructs = nodeConstructValues(node);
        node._ontologyOrigins = nodeOriginValues(node);
        node._ontologyLayers = nodeLayerValues(node);
    });
    links.forEach(link => {
        link._ontologyConstructs = edgeConstructValues(link);
        link._ontologyLayers = edgeLayerValues(link);
    });
    let visibleNodeIds = new Set(nodes.map(node => node.id));
    let selectedNodeId = null;

    let graph = null;
    let renderer = null;
    let hoveredNodeId = null;
    let draggedNodeId = null;
    let isDraggingNode = false;
    let dragMovedNode = false;
    let suppressNextStageClear = false;
    let suppressNextNodeClick = false;
    let suppressStageClearTimer = null;
    let graphFilterRevision = 0;
    let focusNeighborhoodCacheKey = '';
    let focusNeighborhoodCache = new Set();

    function setGraphCursor(cursor) {
        container.style.cursor = cursor;
        container.querySelectorAll('canvas').forEach(canvas => {
            canvas.style.cursor = cursor;
        });
    }

    renderOntologyGraph();

    function renderOntologyGraph() {
        if (!container) {
            return;
        }
        ensureOntologyCanvasSize();
        graph = new Graph({ type: 'directed', multi: true, allowSelfLoops: true });
        assignInitialSigmaPositions(nodes);
        nodes.forEach(nodeData => {
            const palette = nodePalette(nodeData);
            const constructGlyph = isConstructGlyphNode(nodeData);
            graph.addNode(nodeData.id, {
                ...nodeData,
                type: constructGlyph ? 'constructGlyph' : 'circle',
                image: constructGlyph ? constructGlyphImage(nodeData) : undefined,
                mutedImage: constructGlyph ? constructGlyphImage(nodeData, true) : undefined,
                label: sigmaNodeLabel(nodeData),
                fullLabel: fullSigmaNodeLabel(nodeData),
                x: nodeData.x,
                y: nodeData.y,
                size: sigmaNodeSize(nodeData),
                color: palette.fill,
                borderColor: palette.stroke,
                hidden: !nodePassesOwnFilters(nodeData)
            });
        });
        links.forEach((linkData, index) => {
            const source = endpointId(linkData.source);
            const target = endpointId(linkData.target);
            if (!graph.hasNode(source) || !graph.hasNode(target)) {
                return;
            }
            graph.addDirectedEdgeWithKey(`o${index}`, source, target, {
                ...linkData,
                source,
                target,
                type: ontologyEdgeProgramType(linkData),
                label: edgeDisplayLabel(linkData),
                size: sigmaEdgeSize(linkData),
                color: sigmaEdgeColor(linkData),
                hidden: !isEdgeVisible(linkData)
            });
        });
        applySigmaParallelEdgeCurvature();
        applyOntologyLayout();
        renderer = new Sigma(graph, container, {
            allowInvalidContainer: true,
            defaultEdgeType: 'curvedArrow',
            zIndex: true,
            nodeProgramClasses: {
                constructGlyph: createNodeImageProgram({
                    objectFit: 'contain',
                    keepWithinCircle: true,
                    correctCentering: true,
                    padding: 0.08,
                    drawingMode: 'background',
                    size: { mode: 'force', value: 256 }
                })
            },
            edgeProgramClasses: {
                curvedArrow: createEdgeCurveProgram({
                    arrowHead: {
                        extremity: 'target',
                        lengthToThicknessRatio: 2.5,
                        widenessToThicknessRatio: 2
                    },
                    drawLabel: renderOntologySigmaEdgeLabel
                }),
                subclassTriangleArrow: createSubclassTriangleEdgeProgram({
                    drawLabel: renderOntologySigmaEdgeLabel
                }),
                restrictionConnectorArrow: createEdgeCurveProgram({
                    arrowHead: {
                        extremity: 'target',
                        lengthToThicknessRatio: 2.8,
                        widenessToThicknessRatio: 1.8
                    },
                    drawLabel: renderOntologySigmaEdgeLabel
                }),
                constructDiamondArrow: createConstructDiamondEdgeProgram({
                    drawLabel: renderOntologySigmaEdgeLabel
                })
            },
            renderEdgeLabels: true,
            labelColor: { color: labelOnLight },
            labelWeight: '600',
            edgeLabelSize: 12,
            edgeLabelWeight: '600',
            edgeLabelColor: { color: textStrong },
            labelDensity: 0.14,
            labelGridCellSize: 88,
            labelRenderedSizeThreshold: 8,
            nodeReducer: (nodeId, attributes) => {
                const result = { ...attributes };
                const focusIds = activeOntologyFocusIds();
                const focusNeighborhoodIds = activeOntologyFocusNeighborhoodIds();
                const hoverIds = activeOntologyHoverIds();
                const hoverNeighborhoodIds = activeOntologyHoverNeighborhoodIds();
                const hasSelection = focusIds.length > 0;
                const hoverRefinesSelection = hasSelection
                    && hoverIds.some(hoverId => focusNeighborhoodIds.has(hoverId));
                const hasAnyFocus = hasSelection || hoverIds.length > 0;
                const constructGlyph = isConstructGlyphNode(attributes);
                const hovered = hoveredNodeId === nodeId;
                const dragged = draggedNodeId === nodeId;
                if (hasSelection) {
                    const inSelectionTree = focusNeighborhoodIds.has(nodeId);
                    const inHoverTree = hoverRefinesSelection && hoverNeighborhoodIds.has(nodeId);
                    if (!inSelectionTree && !inHoverTree && !dragged) {
                        result.hidden = true;
                        result.label = '';
                        result.forceLabel = false;
                        result.highlighted = false;
                        result.zIndex = ontologyZIndex.base;
                        return result;
                    }
                    result.hidden = false;
                    result.focused = hoverRefinesSelection
                        ? hoverIds.includes(nodeId)
                        : focusIds.includes(nodeId);
                    result.inFocusNeighborhood = hoverRefinesSelection ? inHoverTree : inSelectionTree;
                } else {
                    result.focused = hoverIds.includes(nodeId);
                    result.inFocusNeighborhood = hoverNeighborhoodIds.has(nodeId);
                }
                const muted = hasSelection && hoverRefinesSelection
                    ? focusNeighborhoodIds.has(nodeId) && !hoverNeighborhoodIds.has(nodeId) && !dragged
                    : hasAnyFocus && !result.inFocusNeighborhood && !dragged;
                result.highlighted = hasAnyFocus && (result.inFocusNeighborhood || dragged || hovered);
                result.zIndex = result.focused || dragged
                    ? ontologyZIndex.focusedNode
                    : result.inFocusNeighborhood
                        ? ontologyZIndex.focusedNeighborNode
                        : ontologyZIndex.base;
                if (!hasAnyFocus && !dragged) {
                    result.label = '';
                    result.forceLabel = false;
                } else if (result.inFocusNeighborhood || dragged || hovered) {
                    result.label = constructGlyph
                        ? attributes.label || constructNodeSymbolLabel(attributes)
                        : attributes.fullLabel || attributes.label || '';
                    result.forceLabel = true;
                }
                if (muted) {
                    result.color = dimColor(attributes.color || cssVar('--other'), 0.2);
                    result.label = '';
                    result.forceLabel = false;
                    result.zIndex = ontologyZIndex.mutedNode;
                }
                if (constructGlyph) {
                    result.image = muted
                        ? attributes.mutedImage || constructGlyphImage(attributes, true)
                        : attributes.image || constructGlyphImage(attributes);
                    result.color = muted
                        ? result.color
                        : attributes.color || nodePalette(attributes).fill;
                    if (hasAnyFocus && result.inFocusNeighborhood && !muted && !result.label) {
                        result.label = attributes.label || constructNodeSymbolLabel(attributes);
                        result.forceLabel = true;
                    }
                }
                return result;
            },
            edgeReducer: (_edgeId, attributes) => {
                const result = { ...attributes };
                const focusIds = activeOntologyFocusIds();
                const focusNeighborhoodIds = activeOntologyFocusNeighborhoodIds();
                const hoverIds = activeOntologyHoverIds();
                const hoverNeighborhoodIds = activeOntologyHoverNeighborhoodIds();
                const hasSelection = focusIds.length > 0;
                const hoverRefinesSelection = hasSelection
                    && hoverIds.some(hoverId => focusNeighborhoodIds.has(hoverId));
                const hasAnyFocus = hasSelection || hoverIds.length > 0;
                const activeFocusIds = hoverRefinesSelection ? hoverIds : focusIds;
                const activeNeighborhoodIds = hoverRefinesSelection
                    ? hoverNeighborhoodIds
                    : focusNeighborhoodIds;
                result.hidden = !isEdgeVisible(attributes)
                    || !hasAnyFocus
                    || (hasSelection
                        ? !isEdgeInFocusNeighborhood(attributes, activeFocusIds, activeNeighborhoodIds)
                        : !isEdgeInFocusNeighborhood(attributes, hoverIds, hoverNeighborhoodIds));
                result.zIndex = result.hidden ? ontologyZIndex.base : ontologyZIndex.focusedEdge;
                if (result.hidden) {
                    result.label = '';
                    result.forceLabel = false;
                    return result;
                }
                result.color = focusSigmaEdgeColor(attributes);
                result.size = Math.max(0.8, Number(attributes.size || sigmaEdgeSize(attributes)));
                result.label = hasAnyFocus ? edgeDisplayLabel(attributes) : '';
                result.forceLabel = hasAnyFocus;
                return result;
            }
        });
        renderer.on('clickNode', event => {
            if (suppressNextNodeClick) {
                suppressNextNodeClick = false;
                armStageClearSuppression();
                return;
            }
            armStageClearSuppression();
            window.focusOntologyNode(event.node);
        });
        renderer.on('clickStage', () => {
            setGraphCursor('');
            if (suppressNextStageClear) {
                suppressNextStageClear = false;
                if (suppressStageClearTimer) {
                    window.clearTimeout(suppressStageClearTimer);
                    suppressStageClearTimer = null;
                }
                return;
            }
            window.clearOntologySelection();
        });
        renderer.on('enterNode', event => {
            hoveredNodeId = event.node;
            setGraphCursor('pointer');
            renderer.refresh();
        });
        renderer.on('leaveNode', event => {
            if (hoveredNodeId === event.node) {
                hoveredNodeId = null;
                renderer.refresh();
            }
            setGraphCursor('');
        });
        initializeOntologyNodeDragging();
    }

    function initializeOntologyNodeDragging() {
        if (!renderer || !graph) {
            return;
        }
        renderer.on('downNode', event => {
            if (!visibleNodeIds.has(event.node)) {
                return;
            }
            setGraphCursor('grabbing');
            isDraggingNode = true;
            draggedNodeId = event.node;
            dragMovedNode = false;
            armStageClearSuppression();
            if (!renderer.getCustomBBox()) {
                renderer.setCustomBBox(renderer.getBBox());
            }
            refreshOntologyRenderer();
        });
        renderer.on('moveBody', ({ event }) => {
            if (!isDraggingNode || !draggedNodeId || !graph.hasNode(draggedNodeId)) {
                return;
            }
            const position = renderer.viewportToGraph(event);
            graph.mergeNodeAttributes(draggedNodeId, {
                x: position.x,
                y: position.y
            });
            dragMovedNode = true;
            refreshOntologyRenderer();
            if (event.preventSigmaDefault) {
                event.preventSigmaDefault();
            }
            if (event.original) {
                event.original.preventDefault();
                event.original.stopPropagation();
            }
        });
        const handleOntologyNodeDragEnd = () => {
            if (!isDraggingNode && !draggedNodeId) {
                return;
            }
            if (dragMovedNode) {
                suppressNextNodeClick = true;
                armStageClearSuppression();
            }
            isDraggingNode = false;
            draggedNodeId = null;
            dragMovedNode = false;
            setGraphCursor(hoveredNodeId ? 'pointer' : '');
            refreshOntologyRenderer();
        };
        renderer.on('upNode', handleOntologyNodeDragEnd);
        renderer.on('upStage', handleOntologyNodeDragEnd);
    }

    function armStageClearSuppression() {
        suppressNextStageClear = true;
        if (suppressStageClearTimer) {
            window.clearTimeout(suppressStageClearTimer);
        }
        suppressStageClearTimer = window.setTimeout(() => {
            suppressNextStageClear = false;
            suppressStageClearTimer = null;
        }, 0);
    }

    function ensureOntologyCanvasSize() {
        const minimumHeight = Math.max(window.innerHeight - 50, 520);
        if (!container.clientHeight || container.clientHeight < 20) {
            container.style.height = `${minimumHeight}px`;
        }
    }

    function assignInitialSigmaPositions(renderedNodes) {
        const buckets = new Map();
        renderedNodes.forEach(nodeData => {
            const bucket = nodeLayoutBand(nodeData);
            if (!buckets.has(bucket)) {
                buckets.set(bucket, []);
            }
            buckets.get(bucket).push(nodeData);
        });
        const centers = {
            shacl: [-7, -5],
            concept: [-1, 0],
            value: [6, 2],
            construct: [2, 6]
        };
        for (const [bucketName, bucket] of buckets.entries()) {
            const [cx, cy] = centers[bucketName] || centers.concept;
            const radius = Math.max(2.5, Math.sqrt(bucket.length) * 0.85);
            bucket.forEach((nodeData, index) => {
                const angle = (index / Math.max(bucket.length, 1)) * Math.PI * 2;
                const ring = radius * (0.42 + (index % 13) / 13);
                nodeData.x = cx + Math.cos(angle) * ring;
                nodeData.y = cy + Math.sin(angle) * ring;
            });
        }
    }

    function applyOntologyLayout() {
        try {
            const settings = forceAtlas2.inferSettings(graph);
            forceAtlas2.assign(graph, {
                iterations: graph.order > 650 ? 90 : graph.order > 350 ? 130 : 190,
                settings: {
                    ...settings,
                    adjustSizes: true,
                    barnesHutOptimize: true,
                    gravity: 1.45,
                    scalingRatio: 16,
                    slowDown: 2
                }
            });
            separateOverlappingSigmaNodes();
        } catch {
            // Keep deterministic initial positions if layout cannot run.
        }
    }

    function separateOverlappingSigmaNodes() {
        const seen = new Map();
        graph.forEachNode((nodeId, attributes) => {
            const key = `${Math.round(attributes.x * 10)}:${Math.round(attributes.y * 10)}`;
            const count = seen.get(key) || 0;
            seen.set(key, count + 1);
            if (count > 0) {
                graph.mergeNodeAttributes(nodeId, {
                    x: attributes.x + Math.cos(count) * count * 0.12,
                    y: attributes.y + Math.sin(count) * count * 0.12
                });
            }
        });
    }

    function sigmaNodeSize(nodeData) {
        const connectionCount = connectionCounts.get(nodeData.id) || 0;
        const base = nodeData.shape === 'class-anchor' ? 8 : 6;
        return Math.max(5, Math.min(18, base + Math.sqrt(connectionCount) * 1.7));
    }

    function sigmaLabelWidth(nodeData) {
        return nodeData.shape === 'class-anchor' ? 120 : 160;
    }

    function sigmaEdgeSize(edgeData) {
        if (edgeData.rendered_kind === 'property') {
            return 1.55;
        }
        if (isSetOperatorEdge(edgeData)) {
            return 1.35;
        }
        if (edgeHasConstruct(edgeData, 'subclass')) {
            return 1.35;
        }
        if (edgeHasConstruct(edgeData, 'disjoint')) {
            return 1.2;
        }
        return 1;
    }

    function sigmaEdgeColor(edgeData) {
        if (edgeData.rendered_kind === 'property') {
            return textStrong;
        }
        if (isSetOperatorEdge(edgeData)) {
            return textStrong;
        }
        if (edgeHasConstruct(edgeData, 'subclass')) {
            return textStrong;
        }
        if (edgeHasConstruct(edgeData, 'disjoint')) {
            return cssVar('--rdf-nodeshape');
        }
        if (edgeHasConstruct(edgeData, 'inverse')) {
            return cssVar('--rdf-individual');
        }
        return defaultEdge;
    }

    function focusSigmaEdgeColor(edgeData) {
        if (isSetOperatorEdge(edgeData)) {
            return textStrong;
        }
        if (edgeData.rendered_kind === 'property') {
            return textStrong;
        }
        if (edgeHasConstruct(edgeData, 'subclass')) {
            return textStrong;
        }
        if (edgeHasConstruct(edgeData, 'disjoint')) {
            return cssVar('--rdf-nodeshape');
        }
        return textMuted;
    }

    function ontologyEdgeProgramType(edgeData) {
        if (isClassExpressionMemberEdge(edgeData)) return 'constructDiamondArrow';
        if (edgeHasConstruct(edgeData, 'subclass')) return 'subclassTriangleArrow';
        if (edgeHasConstruct(edgeData, 'restriction')) return 'restrictionConnectorArrow';
        return 'curvedArrow';
    }

    function isSetOperatorEdge(edgeData) {
        return isClassExpressionMemberEdge(edgeData);
    }

    function isClassExpressionMemberEdge(edgeData) {
        return edgeHasConstruct(edgeData, 'class-expression');
    }

    function sigmaEdgeCurvature(edgeData) {
        const index = Number(edgeData.parallelIndex);
        if (!Number.isFinite(index)) {
            return 0.22;
        }
        const max = Math.max(
            1,
            Math.abs(Number(edgeData.parallelMaxIndex) || 0),
            Math.abs(Number(edgeData.parallelMinIndex) || 0)
        );
        const normalized = index / max;
        if (Math.abs(normalized) < 0.01) {
            return 0.08;
        }
        return Math.max(-0.48, Math.min(0.48, normalized * 0.28));
    }

    function applySigmaParallelEdgeCurvature() {
        indexParallelEdgesIndex(graph);
        graph.forEachEdge((edgeId, attributes) => {
            graph.setEdgeAttribute(edgeId, 'curvature', sigmaEdgeCurvature(attributes));
        });
    }

    function renderOntologySigmaEdgeLabel(context, data, source, target, settings) {
        if (!data.label) return;
        const label = ontologyEdgeLabelText(data);
        if (!label) return;

        drawSigmaCurvedEdgeLabel(
            context,
            { ...data, label },
            source,
            target,
            {
                ...settings,
                edgeLabelColor: {
                    color: edgeHasConstruct(data, 'subclass') ? textMuted : textBody
                }
            }
        );
    }

    function constructGlyphImage(nodeData, muted = false) {
        const symbol = constructNodeSymbolLabel(nodeData);
        const isRestriction = nodeHasConstructClass(nodeData, 'restriction');
        const fill = muted ? textMuted : textStrong;
        const opacity = muted ? '0.2' : '1';
        const body = isRestriction
            ? restrictionGlyphSvg(symbol, fill, opacity)
            : classExpressionGlyphSvg(setOperatorSymbol({ label: symbol }), fill, opacity);
        const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="256" height="256" viewBox="0 0 256 256">
${body}
</svg>`;
        return `data:image/svg+xml;charset=utf-8,${encodeURIComponent(svg)}`;
    }

    function classExpressionGlyphSvg(symbol, fill, opacity) {
        const safeSymbol = escapeXml(symbol || 'U');
        return `<text x="128" y="142" font-family="Arial, Helvetica, sans-serif" font-size="112" font-weight="800" text-anchor="middle" fill="${escapeXml(fill)}" fill-opacity="${escapeXml(opacity)}">${safeSymbol}</text>`;
    }

    function restrictionGlyphSvg(symbol, fill, opacity) {
        const safeSymbol = escapeXml(symbol || 'R');
        return `<text x="128" y="142" font-family="Arial, Helvetica, sans-serif" font-size="112" font-weight="800" text-anchor="middle" fill="${escapeXml(fill)}" fill-opacity="${escapeXml(opacity)}">${safeSymbol}</text>`;
    }

    function isConstructGlyphNode(nodeData) {
        return nodeHasConstructClass(nodeData, 'class-expression')
            || nodeHasConstructClass(nodeData, 'restriction');
    }

    function nodeHasConstructClass(nodeData, constructClass) {
        return nodeConstructClasses(nodeData).has(constructClass);
    }

    function nodeConstructClasses(nodeData) {
        const classes = new Set();
        if (!nodeData) return classes;
        const semanticType = String(nodeData.semantic_type || '');
        if (semanticType === 'restriction') classes.add('restriction');
        if (semanticType === 'class-expression') classes.add('class-expression');
        (nodeData.constructs || []).forEach(item => {
            constructFilterValues(item).forEach(value => classes.add(value));
        });
        return classes;
    }

    function constructNodeSymbolLabel(nodeData) {
        if (nodeHasConstructClass(nodeData, 'restriction')) {
            return restrictionNodeSymbolLabel(nodeData);
        }
        return classExpressionEdgeSymbolLabel(nodeData);
    }

    function constructNodeDisplayLabel(nodeData) {
        if (nodeHasConstructClass(nodeData, 'restriction')) {
            return restrictionNodeDisplayLabel(nodeData);
        }
        const label = String(classExpressionEdgeSymbolLabel(nodeData) || '').toLowerCase();
        if (label.includes('intersection')) return 'Intersection';
        if (label.includes('complement')) return 'Complement';
        if (label.includes('union')) return 'Union';
        return 'Class expression';
    }

    function classExpressionEdgeSymbolLabel(nodeData) {
        const expression = (nodeData.constructs || []).find(item => {
            const kind = String(item.kind || '');
            const family = String(item.family || '');
            const label = String(item.label || item.class_expression_kind || item.classExpressionKind || '').toLowerCase();
            return kind === 'class-expression'
                || family === 'class-expression'
                || label === 'union'
                || label === 'intersection'
                || label === 'complement';
        });
        return expression
            ? expression.label || expression.class_expression_kind || expression.classExpressionKind || ''
            : graphNodeDisplayLabel(nodeData);
    }

    function restrictionNodeSymbolLabel(nodeData) {
        const restriction = (nodeData.constructs || []).find(item => {
            const kind = String(item.kind || '');
            const family = String(item.family || '');
            return kind === 'restriction' || family === 'restriction';
        });
        const label = String(
            (restriction && (restriction.label || restriction.restriction_kind || restriction.restrictionKind))
            || graphNodeDisplayLabel(nodeData)
            || ''
        ).toLowerCase();
        if (label.includes('universal')) return '∀';
        if (label.includes('existential')) return '∃';
        if (label.includes('min')) return '≥';
        if (label.includes('max')) return '≤';
        if (label.includes('cardinality')) return '=';
        return 'R';
    }

    function restrictionNodeDisplayLabel(nodeData) {
        const restriction = (nodeData.constructs || []).find(item => {
            const kind = String(item.kind || '');
            const family = String(item.family || '');
            return kind === 'restriction' || family === 'restriction';
        });
        const label = String(
            (restriction && (restriction.label || restriction.restriction_kind || restriction.restrictionKind))
            || graphNodeDisplayLabel(nodeData)
            || ''
        ).toLowerCase();
        if (label.includes('universal')) return 'Universal restriction';
        if (label.includes('existential')) return 'Existential restriction';
        if (label.includes('min')) return 'Min cardinality';
        if (label.includes('max')) return 'Max cardinality';
        if (label.includes('cardinality')) return 'Cardinality restriction';
        return 'Restriction';
    }

    function setOperatorSymbol(edgeData) {
        const label = String(
            edgeDisplayLabel(edgeData)
            || edgeData.class_expression_kind
            || edgeData.classExpressionKind
            || ''
        ).toLowerCase();
        if (label.includes('intersection')) return '∩';
        if (label.includes('complement')) return '¬';
        if (label.includes('union')) return '∪';
        return 'U';
    }

    function ontologyEdgeLabelText(edgeData) {
        const raw = edgeDisplayLabel(edgeData);
        if (isSetOperatorEdge(edgeData)) return '';
        if (edgeHasConstruct(edgeData, 'shape-overlay') && isGenericShapeOverlayLabel(raw)) return '';
        if (edgeHasConstruct(edgeData, 'subclass')) return 'Subclass of';
        if (edgeHasConstruct(edgeData, 'membership')) return 'member';
        return raw;
    }

    function isGenericShapeOverlayLabel(value) {
        return String(value || '').toLowerCase().includes('shape overlay');
    }

    function sigmaNodeLabel(nodeData) {
        if (isConstructGlyphNode(nodeData)) {
            return constructNodeSymbolLabel(nodeData);
        }
        const text = truncateLabel(nodeData.display_label || graphNodeDisplayLabel(nodeData), sigmaLabelWidth(nodeData));
        const badges = visibleBadgeSymbols(nodeData);
        return badges ? `${text} ${badges}` : text;
    }

    function fullSigmaNodeLabel(nodeData) {
        if (isConstructGlyphNode(nodeData)) {
            return constructNodeDisplayLabel(nodeData);
        }
        const text = nodeData.display_label || graphNodeDisplayLabel(nodeData);
        const badges = visibleBadgeSymbols(nodeData);
        return badges ? `${text} ${badges}` : text;
    }

    function dimColor(color, alpha) {
        const foreground = parseCssColor(resolveColorValue(color));
        const background = parseCssColor(cssVar('--bg-canvas'));
        if (!foreground || !background) return color;
        const r = Math.round(foreground.r * alpha + background.r * (1 - alpha));
        const g = Math.round(foreground.g * alpha + background.g * (1 - alpha));
        const b = Math.round(foreground.b * alpha + background.b * (1 - alpha));
        return `#${[r, g, b].map(component => component.toString(16).padStart(2, '0')).join('')}`;
    }

    function resolveColorValue(color) {
        if (!color) return '';
        const value = String(color).trim();
        if (value.startsWith('var(')) {
            const match = value.match(/^var\((--[a-z0-9-]+)(?:,\s*([^)]+))?\)$/i);
            if (match) {
                return cssVar(match[1]);
            }
        }
        return value;
    }

    function parseCssColor(color) {
        const value = resolveColorValue(color);
        if (!value) return null;
        if (value.startsWith('#')) return parseHexColor(value);
        const rgb = value.match(/rgba?\(\s*([\d.]+)(?:,|\s+)\s*([\d.]+)(?:,|\s+)\s*([\d.]+)(?:\s*[,/]\s*([\d.]+%?))?/i);
        if (!rgb) return null;
        const r = Number(rgb[1]);
        const g = Number(rgb[2]);
        const b = Number(rgb[3]);
        const alpha = rgb[4]?.endsWith('%')
            ? Number(rgb[4].slice(0, -1)) / 100
            : rgb[4] === undefined
                ? 1
                : Number(rgb[4]);
        if (!Number.isNaN(alpha) && alpha <= 0) return null;
        if ([r, g, b].some(component => Number.isNaN(component))) return null;
        return { r, g, b };
    }

    function parseHexColor(color) {
        const hex = color.slice(1);
        const value = hex.length === 3
            ? hex.split('').map(part => part + part).join('')
            : hex.padEnd(6, '0').slice(0, 6);
        const r = parseInt(value.slice(0, 2), 16);
        const g = parseInt(value.slice(2, 4), 16);
        const b = parseInt(value.slice(4, 6), 16);
        if ([r, g, b].some(component => Number.isNaN(component))) return null;
        return { r, g, b };
    }

    function buildRenderedOntologyLinks(rawEdges, allNodesById, properties) {
        const rendered = [];
        const seen = new Set();
        rawEdges.forEach(edgeData => {
            const source = endpointId(edgeData.source);
            const target = endpointId(edgeData.target);
            if (!source || !target) {
                return;
            }
            if (isOntologyPropertyNode(allNodesById.get(source)) || isOntologyPropertyNode(allNodesById.get(target))) {
                return;
            }
            pushRenderedLink(rendered, seen, {
                ...edgeData,
                source,
                target,
                rendered_kind: 'construct'
            });
        });

        properties.forEach(propertyNode => {
            const domains = propertyEndpointTerms(propertyNode.domain);
            const ranges = propertyEndpointTerms(propertyNode.range);
            domains.forEach(domain => {
                ranges.forEach(range => {
                    if (domain.iri === range.iri) {
                        return;
                    }
                    pushRenderedLink(rendered, seen, {
                        source: domain.iri,
                        target: range.iri,
                        label: graphNodeDisplayLabel(propertyNode),
                        rendered_kind: 'property',
                        property_node_id: propertyNode.id,
                        property_kind: propertyNode.semantic_type || 'rdf-property'
                    });
                });
            });
        });
        return rendered;
    }

    function pushRenderedLink(rendered, seen, linkData) {
        const source = endpointId(linkData.source);
        const target = endpointId(linkData.target);
        if (!source || !target || !nodeExistsForRenderedGraph(source) || !nodeExistsForRenderedGraph(target)) {
            return;
        }
        const key = [
            source,
            target,
            linkData.label || '',
            linkData.rendered_kind || '',
            linkData.property_node_id || ''
        ].join('|');
        if (seen.has(key)) {
            return;
        }
        seen.add(key);
        rendered.push(linkData);
    }

    function nodeExistsForRenderedGraph(nodeId) {
        const nodeData = rawNodeById.get(nodeId);
        return Boolean(nodeData && !isOntologyPropertyNode(nodeData));
    }

    function propertyEndpointTerms(terms) {
        return (terms || [])
            .filter(term => term && term.iri && nodeExistsForRenderedGraph(term.iri));
    }

    function isOntologyPropertyNode(nodeData) {
        const type = String((nodeData && nodeData.semantic_type) || '');
        return type === 'object-property' || type === 'datatype-property' || type === 'rdf-property';
    }

    function computeRenderedNodeConnections(renderedNodes, renderedLinks) {
        const counts = new Map(renderedNodes.map(nodeData => [nodeData.id, 0]));
        renderedLinks.forEach(linkData => {
            if (!isMeaningfulConnectionLink(linkData)) {
                return;
            }
            const source = endpointId(linkData.source);
            const target = endpointId(linkData.target);
            if (counts.has(source)) {
                counts.set(source, counts.get(source) + 1);
            }
            if (counts.has(target) && target !== source) {
                counts.set(target, counts.get(target) + 1);
            }
        });
        return counts;
    }

    function isMeaningfulConnectionLink(linkData) {
        return linkData.property_kind !== 'datatype-property';
    }

    function edgeDisplayLabel(edgeData) {
        return edgeData.display_label || edgeData.label || '';
    }

    function nodeLayoutBand(nodeData) {
        const type = String(nodeData.semantic_type || '');
        if (type === 'node-shape' || type === 'property-shape') return 'shacl';
        if (type.endsWith('property')) return 'property';
        if (type === 'named-individual' || type === 'datatype' || type === 'resource') return 'value';
        if (nodeHasConstructClass(nodeData, 'restriction') || nodeHasConstructClass(nodeData, 'class-expression')) return 'construct';
        return 'concept';
    }

    function nodeShapeType(nodeData) {
        const type = String(nodeData.semantic_type || '');
        if (type === 'class' || nodeHasConstructClass(nodeData, 'restriction') || nodeHasConstructClass(nodeData, 'class-expression')) {
            return 'class-anchor';
        }
        return 'box';
    }

    function truncateLabel(value, width) {
        const text = String(value || '');
        const max = Math.max(8, Math.floor((width - 20) / 7));
        return text.length > max ? `${text.slice(0, max - 1)}...` : text;
    }

    function escapeHtml(value) {
        return String(value || '')
            .replaceAll('&', '&amp;')
            .replaceAll('<', '&lt;')
            .replaceAll('>', '&gt;')
            .replaceAll('"', '&quot;')
            .replaceAll("'", '&#39;');
    }

    function replaceTrustedOntologyHtml(target, html) {
        const parsed = new DOMParser().parseFromString(`<body>${html}</body>`, 'text/html');
        target.replaceChildren(...Array.from(parsed.body.childNodes));
    }

    function renderOntologyEmptyState(target) {
        const message = document.createElement('p');
        message.className = 'text-gray-500 italic m-0';
        message.textContent = 'Search or select a graph node to inspect URI, RDF type, comments, and SHACL constraints.';
        target.replaceChildren(message);
    }

    function escapeXml(value) {
        return escapeHtml(value);
    }

    function shortLabel(value) {
        const text = String(value || '');
        const parts = text.split(/[/#]/).filter(Boolean);
        return parts.length ? parts[parts.length - 1] : text;
    }

    function classExpressionMembersLabel(nodeData) {
        const expression = (nodeData.constructs || []).find(item => item.kind === 'class-expression');
        const members = expression ? (expression.members || []) : [];
        if (!members.length) {
            return nodeData.label || 'Class expression';
        }
        const joiner = expression.label === 'Intersection'
            ? ' ∩ '
            : expression.label === 'Complement'
            ? ' ∖ '
            : ' ∪ ';
        return members.map(shortLabel).join(joiner);
    }

    function graphNodeDisplayLabel(nodeData) {
        if (!nodeData || nodeData.semantic_type !== 'class-expression') {
            return (nodeData && (nodeData.label || nodeData.id)) || '';
        }
        const usage = (nodeData.constructs || []).find(item =>
            item.kind === 'property-domain' || item.kind === 'property-range'
        );
        const expression = classExpressionMembersLabel(nodeData);
        if (!usage) {
            return expression;
        }
        const property = shortLabel(usage.subject || usage.property || '');
        const role = usage.kind === 'property-domain' ? 'domain' : 'range';
        return property ? `${property} ${role}: ${expression}` : `${role}: ${expression}`;
    }

    function badgeAriaLabel(badge) {
        return `${badge.label} (symbol ${badge.symbol})`;
    }

    function renderBadges(badges) {
        if (!badges || !badges.length) {
            return '';
        }
        const items = badges.map(badge => {
            const aria = escapeHtml(badgeAriaLabel(badge));
            return `<span class="ontology-badge" role="img" aria-label="${aria}" title="${aria}">`
                + `<span class="ontology-badge-symbol" aria-hidden="true">${escapeHtml(badge.symbol)}</span>`
                + `<span class="ontology-badge-text">${escapeHtml(badge.label)}</span>`
                + `</span>`;
        }).join('');
        return `<div class="ontology-badge-row">${items}</div>`;
    }

    function badgeFilterValue(badge) {
        const kind = String((badge && badge.kind) || '');
        if (kind === 'subset-or-equal') return 'subclass';
        if (kind === 'member-of') return 'membership';
        if (kind === 'disjointness') return 'disjoint';
        if (kind === 'logical-equivalence') return 'equivalence';
        if (kind === 'inverse-property') return 'inverse';
        if (['functional', 'inverse-functional', 'symmetric', 'asymmetric', 'reflexive', 'irreflexive', 'transitive'].includes(kind)) {
            return 'property-characteristic';
        }
        if (kind === 'universal-restriction' || kind === 'existential-restriction') return 'restriction';
        if (kind === 'intersection' || kind === 'union' || kind === 'set-difference') return 'class-expression';
        return '';
    }

    function visibleBadges(nodeData) {
        return (nodeData.badges || []).filter(badge => {
            const value = badgeFilterValue(badge);
            return value ? filterState.construct.has(value) && constructPassesRelationFilters(value) : true;
        });
    }

    function visibleBadgeSymbols(nodeData) {
        return visibleBadges(nodeData).map(badge => badge.symbol).join(' ');
    }

    function inspectorConstructDetails(nodeData) {
        return nodeData.constructs || [];
    }

    function inspectorSlotFacets(nodeData) {
        return nodeData.slot_facets || [];
    }

    function equivalenceMembers(group) {
        const raw = String(group || '');
        const body = raw.startsWith('equivalence:') ? raw.slice('equivalence:'.length) : raw;
        return body ? body.split('|').filter(Boolean) : [];
    }

    function renderRelationPills(ids) {
        if (!ids || !ids.length) {
            return '<span class="text-gray-400 italic">None</span>';
        }
        return ids.map(id => `<span class="ontology-rel-pill" title="${escapeHtml(id)}">${escapeHtml(shortLabel(id))}</span>`).join('');
    }

    const SEMANTIC_TYPE_LABELS = {
        'object-property': 'Object property',
        'datatype-property': 'Datatype property',
        'rdf-property': 'RDF property',
        'class': 'Class',
        'named-individual': 'Named individual',
        'node-shape': 'SHACL node shape',
        'property-shape': 'SHACL property shape',
        'restriction': 'OWL restriction',
        'class-expression': 'Class expression',
        'datatype': 'Datatype',
        'skos-concept': 'SKOS concept',
        'skos-concept-scheme': 'SKOS concept scheme',
        'literal': 'Literal',
        'resource': 'Resource'
    };

    function humanizeSemanticType(value) {
        return SEMANTIC_TYPE_LABELS[value] || (value || 'Resource');
    }

    function renderTermRefs(terms, emptyLabel) {
        if (!terms || !terms.length) {
            return `<span class="text-gray-400 italic">${escapeHtml(emptyLabel)}</span>`;
        }
        return terms.map(term => {
            const kind = String(term.kind || 'class');
            const title = escapeHtml(term.iri || term.label || '');
            return `<span class="ontology-term-ref ontology-term-${escapeHtml(kind)}" title="${title}">`
                + `${escapeHtml(term.label || term.iri || '')}`
                + `<span class="ontology-term-kind">${escapeHtml(kind)}</span>`
                + `</span>`;
        }).join('');
    }

    function renderLiteralValues(values) {
        if (!values || !values.length) {
            return '';
        }
        return `<div class="ontology-meta-section">
            <div class="ontology-meta-title">Literal Values</div>
            ${values.map(item => (
                `<div class="ontology-literal-value"><span>${escapeHtml(item.predicate || 'value')}</span><strong>${escapeHtml(item.value || '')}</strong></div>`
            )).join('')}
        </div>`;
    }

    function renderSources(sources) {
        if (!sources || !sources.length) {
            return '<span class="text-gray-400 italic">No source recorded.</span>';
        }
        return sources.map(src => {
            const name = src.source_name || src.source || src.file_path || 'source';
            const loc = src.line_number ? `${src.file_path}:${src.line_number}` : (src.file_path || '');
            const kind = src.kind ? `<span class="ontology-source-kind">${escapeHtml(src.kind)}</span>` : '';
            const href = src.link ? escapeHtml(src.link) : '';
            const title = escapeHtml(src.source || loc || name);
            const nameHtml = href
                ? `<a class="ontology-source-name" href="${href}" title="${title}">${escapeHtml(name)}</a>`
                : `<span>${escapeHtml(name)}</span>`;
            const locHtml = href
                ? `<a class="ontology-source-loc" href="${href}" title="${title}">${escapeHtml(loc)}</a>`
                : `<span class="ontology-source-loc">${escapeHtml(loc)}</span>`;
            return `<div class="ontology-source"><span>${nameHtml}${kind}</span>${locHtml}</div>`;
        }).join('');
    }

    function renderPropertyChain(chain) {
        const members = (chain.members || []).map(member => escapeHtml(shortLabel(member)));
        const body = members.length
            ? members.join('<span class="ontology-chain-sep">∘</span>')
            : '<span class="text-gray-400 italic">Empty chain</span>';
        const source = chain.source
            ? `<div class="ontology-chain-source">${escapeHtml(chain.source)}</div>`
            : '';
        return `<div class="ontology-chain"><div>${body}</div>${source}</div>`;
    }

    function propertyUsagesForNode(nodeData) {
        const id = nodeData.id;
        const usages = [];
        propertyNodes.forEach(propertyNode => {
            const domains = propertyEndpointTerms(propertyNode.domain);
            const ranges = propertyEndpointTerms(propertyNode.range);
            const domainMatch = domains.some(term => term.iri === id);
            const rangeMatch = ranges.some(term => term.iri === id);
            if (!domainMatch && !rangeMatch) {
                return;
            }
            usages.push({
                isDomain: domainMatch,
                isRange: rangeMatch,
                property: propertyNode,
                domains,
                ranges,
                facets: domainMatch ? slotFacetsForPropertyOnNode(propertyNode, nodeData) : []
            });
        });
        usages.sort((a, b) => {
            const propertyCompare = graphNodeDisplayLabel(a.property).localeCompare(graphNodeDisplayLabel(b.property));
            if (propertyCompare) {
                return propertyCompare;
            }
            const aRole = propertyUsageRoleLabel(a);
            const bRole = propertyUsageRoleLabel(b);
            return aRole.localeCompare(bRole);
        });
        return usages;
    }

    function slotFacetsForPropertyOnNode(propertyNode, nodeData) {
        return (nodeData.slot_facets || []).filter(facet => facet.slot_iri === propertyNode.id);
    }

    function renderPropertyUsages(nodeData) {
        const usages = propertyUsagesForNode(nodeData);
        if (!usages.length) {
            return '';
        }
        return `<div class="ontology-meta-section">
            <div class="ontology-meta-title">Properties</div>
            ${usages.map(renderPropertyUsage).join('')}
        </div>`;
    }

    function renderPropertyUsage(usage) {
        const propertyNode = usage.property;
        const propertyLabel = graphNodeDisplayLabel(propertyNode) || propertyNode.id;
        const propertyKind = humanizeSemanticType(propertyNode.semantic_type);
        const facets = usage.facets && usage.facets.length
            ? `<div class="ontology-slot-facet-values">${usage.facets.flatMap(facet => facet.facets || []).map(item => (
                `<span class="ontology-slot-facet-pill"><span>${escapeHtml(item.name)}</span>${escapeHtml(item.value)}</span>`
            )).join('')}</div>`
            : '';
        return `<div class="ontology-property-usage">`
            + `<div class="ontology-property-usage-title"><strong title="${escapeHtml(propertyNode.full_uri || propertyNode.id)}">${escapeHtml(propertyLabel)}</strong><span class="ontology-property-kind">${escapeHtml(propertyKind)}</span></div>`
            + `<div class="ontology-property-usage-body">${renderPropertyUsageBody(usage)}</div>`
            + facets
            + `</div>`;
    }

    function propertyUsageRoleLabel(usage) {
        if (usage.isDomain && usage.isRange) {
            return 'domain and range';
        }
        return usage.isDomain ? 'domain' : 'range';
    }

    function renderPropertyUsageBody(usage) {
        if (usage.isDomain && usage.isRange) {
            return `<span>domain/range</span> property; Domain ${renderTermRefs(usage.domains, 'Any')}; Range ${renderTermRefs(usage.ranges, 'Any')}`;
        }
        if (usage.isDomain) {
            return `<span>domain</span> property; Range ${renderTermRefs(usage.ranges, 'Any')}`;
        }
        return `<span>range</span> property; Domain ${renderTermRefs(usage.domains, 'Any')}`;
    }

    function renderConstructDetail(item) {
        if (item.kind === 'class-expression') {
            return renderClassExpressionConstructDetail(item);
        }
        const badge = item.badge
            ? `<span class="ontology-construct-symbol" title="${escapeHtml(badgeAriaLabel(item.badge))}">${escapeHtml(item.badge.symbol)}</span>`
            : '';
        const fields = [];
        if (item.subject) fields.push(`subject=${item.subject}`);
        if (item.predicate) fields.push(`predicate=${item.predicate}`);
        if (item.property) fields.push(`property=${item.property}`);
        if (item.object) fields.push(`object=${item.object}`);
        if (item.members && item.members.length) fields.push(`members=${item.members.join(' -> ')}`);
        const source = item.source && (item.source.source_name || item.source.source || item.source.file_path)
            ? `source=${item.source.source_name || item.source.source || item.source.file_path}${item.source.line_number ? ':' + item.source.line_number : ''}`
            : '';
        if (source) fields.push(source);
        const meta = fields.length
            ? `<div class="ontology-construct-meta">${escapeHtml(fields.join(' | '))}</div>`
            : '';
        return `<div class="ontology-construct">`
            + `<div class="ontology-construct-title">${badge}<span>${escapeHtml(item.label || item.kind || 'Construct')}</span></div>`
            + meta
            + `</div>`;
    }

    function renderClassExpressionConstructDetail(item) {
        const badge = item.badge
            ? `<span class="ontology-construct-symbol" title="${escapeHtml(badgeAriaLabel(item.badge))}">${escapeHtml(item.badge.symbol)}</span>`
            : '';
        const members = (item.members || []).length
            ? `<div class="ontology-construct-members">${item.members.map((member, index) => (
                `<div class="ontology-construct-member"><strong>${index + 1}.</strong> ${escapeHtml(shortLabel(member))}</div>`
            )).join('')}</div>`
            : '<span class="text-gray-400 italic">No members recorded.</span>';
        const source = item.source && (item.source.source_name || item.source.source || item.source.file_path)
            ? `<div class="ontology-construct-meta">source=${escapeHtml(item.source.source_name || item.source.source || item.source.file_path)}${item.source.line_number ? ':' + escapeHtml(item.source.line_number) : ''}</div>`
            : '';
        return `<div class="ontology-construct">`
            + `<div class="ontology-construct-title">${badge}<span>${escapeHtml(item.label || 'Class expression')}</span></div>`
            + members
            + source
            + `</div>`;
    }

    function isBlankNodeIdentifier(value) {
        return String(value || '').startsWith('_:');
    }

    function classExpressionConstructs(nodeData) {
        return inspectorConstructDetails(nodeData).filter(item => item.kind === 'class-expression');
    }

    function graphUsagesForNode(nodeData) {
        const id = nodeData.id;
        return links
            .filter(link => endpointId(link.source) === id || endpointId(link.target) === id)
            .map(link => {
                const sourceId = endpointId(link.source);
                const targetId = endpointId(link.target);
                const source = nodeById.get(sourceId);
                const target = nodeById.get(targetId);
                const direction = sourceId === id ? 'to' : 'from';
                const other = sourceId === id ? target : source;
                return {
                    label: link.label || 'relation',
                    direction,
                    other: other ? (other.label || other.id) : (direction === 'to' ? targetId : sourceId),
                };
            });
    }

    function renderClassExpressionSummary(nodeData) {
        if (nodeData.semantic_type !== 'class-expression') {
            return '';
        }
        const expressionConstructs = classExpressionConstructs(nodeData);
        const memberSection = expressionConstructs.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Expression Members</div>
                ${expressionConstructs.map(renderClassExpressionConstructDetail).join('')}
            </div>`
            : '';
        const usages = graphUsagesForNode(nodeData)
            .filter(usage => !['union', 'intersection', 'complement'].includes(String(usage.label || '')));
        const usageSection = usages.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Used As</div>
                <div class="ontology-construct-usages">${usages.map(usage => (
                    `<div class="ontology-construct-usage"><strong>${escapeHtml(usage.label)}</strong> ${escapeHtml(usage.direction)} ${escapeHtml(usage.other)}</div>`
                )).join('')}</div>
            </div>`
            : '';
        return memberSection + usageSection;
    }

    function renderIdentifierSection(nodeData, title) {
        const value = nodeData.full_uri || '';
        if (isBlankNodeIdentifier(value) && nodeData.semantic_type === 'class-expression') {
            return `<details class="ontology-raw-details">
                <summary>Raw Details</summary>
                <div class="ontology-meta-title">Blank Node Identifier</div>
                <div class="ontology-uri-block">${escapeHtml(value)}</div>
            </details>`;
        }
        return `<div class="ontology-meta-section">
            <div class="ontology-meta-title">${escapeHtml(title)}</div>
            <div class="ontology-uri-block">${escapeHtml(value)}</div>
        </div>`;
    }

    function renderSlotFacet(facet, contextKind) {
        const isPropertyContext = contextKind === 'property';
        const values = (facet.facets || []).length
            ? `<div class="ontology-slot-facet-values">${facet.facets.map(item => (
                `<span class="ontology-slot-facet-pill"><span>${escapeHtml(item.name)}</span>${escapeHtml(item.value)}</span>`
            )).join('')}</div>`
            : '<span class="text-gray-400 italic">No explicit facets.</span>';
        const slotLabel = facet.slot_label || facet.slot_iri || 'slot';
        const targetLabel = facet.target_class_label || facet.target_class_iri || 'target class';
        const titleLabel = isPropertyContext ? `Class ${targetLabel}` : slotLabel;
        const titleIri = isPropertyContext
            ? (facet.target_class_iri || facet.target_class_label || '')
            : (facet.slot_iri || facet.slot_label || '');
        const target = isPropertyContext
            ? `<div class="ontology-slot-facet-target">Path ${escapeHtml(slotLabel)}</div>`
            : facet.target_class_label
            ? `<div class="ontology-slot-facet-target">Class ${escapeHtml(facet.target_class_label)}</div>`
            : '';
        return `<div class="ontology-slot-facet">`
            + `<div class="ontology-slot-facet-title"><strong title="${escapeHtml(titleIri)}">${escapeHtml(titleLabel)}</strong><span class="ontology-slot-facet-source">${escapeHtml(facet.source_shape_label || 'shape')}</span></div>`
            + target
            + values
            + `</div>`;
    }

    function ontologyNodeHaystack(node) {
        const badges = (node.badges || [])
            .map(badge => `${badge.label} ${badge.symbol} ${badge.code_point}`)
            .join(' ');
        const chains = (node.property_chains || [])
            .map(chain => `${(chain.members || []).join(' ')} ${chain.source || ''}`)
            .join(' ');
        const terms = []
            .concat(node.domain || [], node.range || [])
            .map(term => `${term.label || ''} ${term.iri || ''} ${term.kind || ''}`)
            .join(' ');
        const constraints = (node.constraints || [])
            .map(item => `${item.property || ''} ${item.value || ''}`)
            .join(' ');
        const sources = (node.sources || [])
            .map(src => `${src.source || ''} ${src.source_name || ''} ${src.file_path || ''} ${src.kind || ''}`)
            .join(' ');
        const constructs = (node.constructs || [])
            .map(item => [
                item.id,
                item.family,
                item.kind,
                item.label,
                item.subject,
                item.predicate,
                item.object,
                item.property,
                (item.members || []).join(' '),
                item.source ? `${item.source.source || ''} ${item.source.source_name || ''} ${item.source.file_path || ''}` : '',
                item.badge ? `${item.badge.label || ''} ${item.badge.symbol || ''} ${item.badge.code_point || ''}` : ''
            ].join(' '))
            .join(' ');
        const slots = (node.slot_facets || [])
            .map(slot => [
                slot.slot_label,
                slot.slot_iri,
                slot.target_class_label,
                slot.target_class_iri,
                slot.source_shape_label,
                slot.source_shape_iri,
                (slot.facets || []).map(facet => `${facet.name} ${facet.value}`).join(' ')
            ].join(' '))
            .join(' ');
        const literalValues = (node.literal_values || [])
            .map(item => `${item.predicate || ''} ${item.value || ''} ${item.source ? (item.source.source_name || item.source.source || item.source.file_path || '') : ''}`)
            .join(' ');
        return [
            graphNodeDisplayLabel(node),
            node.label,
            node.full_uri,
            node.ontology_document || '',
            node.type || '',
            node.semantic_type || '',
            (node.rdf_types || []).join(' '),
            badges,
            node.equivalence_group || '',
            (node.inverse_properties || []).join(' '),
            chains,
            terms,
            constraints,
            literalValues,
            slots,
            sources,
            constructs
        ].join(' ').toLowerCase();
    }

    function renderInspector(nodeData) {
        onSelect?.(nodeData);
        const title = document.getElementById('ontology-inspector-title');
        const body = document.getElementById('ontology-inspector-body');
        const clear = document.getElementById('ontology-inspector-clear');
        if (!title || !body || !clear) {
            return;
        }
        title.textContent = graphNodeDisplayLabel(nodeData) || nodeData.id;
        clear.style.display = 'block';
        const identifierTitle = String(nodeData.full_uri || '').startsWith('_:')
            ? 'Blank Node Identifier'
            : 'Full URI';

        const types = nodeData.rdf_types && nodeData.rdf_types.length
            ? nodeData.rdf_types.map(type => `<span class="ontology-type-pill">${escapeHtml(type)}</span>`).join('')
            : `<span class="text-gray-400 italic">Implicit ${escapeHtml(nodeData.type)} entity</span>`;
        const rawShaclEvidence = nodeData.constraints && nodeData.constraints.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Raw SHACL Evidence</div>
                ${nodeData.constraints.map(item => `<div class="ontology-constraint"><span>${escapeHtml(item.property)}</span><strong>${escapeHtml(item.value)}</strong></div>`).join('')}
            </div>`
            : '';

        const badges = renderBadges(nodeData.badges || []);

        const equivalence = nodeData.equivalence_group
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Equivalence Group</div>
                <div>${renderRelationPills(equivalenceMembers(nodeData.equivalence_group))}</div>
            </div>`
            : '';

        const inverses = nodeData.inverse_properties && nodeData.inverse_properties.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Inverse Properties</div>
                <div>${renderRelationPills(nodeData.inverse_properties)}</div>
            </div>`
            : '';

        const chains = nodeData.property_chains && nodeData.property_chains.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Property Chains</div>
                ${nodeData.property_chains.map(renderPropertyChain).join('')}
            </div>`
            : '';

        const visibleConstructs = inspectorConstructDetails(nodeData).filter(item => {
            if (nodeData.semantic_type !== 'class-expression') {
                return true;
            }
            return !['class-expression', 'property-domain', 'property-range'].includes(item.kind);
        });
        const constructs = visibleConstructs.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Projection Constructs</div>
                ${visibleConstructs.map(renderConstructDetail).join('')}
            </div>`
            : '';
        const classExpressionSummary = renderClassExpressionSummary(nodeData);
        const literalValues = renderLiteralValues(nodeData.literal_values);
        const identifier = renderIdentifierSection(nodeData, identifierTitle);
        const ontologyDocument = nodeData.ontology_document
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">OWL Document</div>
                <div class="ontology-uri-block">${escapeHtml(nodeData.ontology_document)}</div>
            </div>`
            : '';
        const properties = renderPropertyUsages(nodeData);

        const isProperty = String(nodeData.semantic_type || '').endsWith('property')
            || (nodeData.domain && nodeData.domain.length)
            || (nodeData.range && nodeData.range.length);

        const visibleSlots = inspectorSlotFacets(nodeData);
        const slots = visibleSlots.length
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">${isProperty ? 'Used As Slot / Facets' : 'Slots / Facets'}</div>
                ${visibleSlots.map(facet => renderSlotFacet(facet, isProperty ? 'property' : 'class')).join('')}
            </div>`
            : '';

        const domain = isProperty
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Domain</div>
                <div>${renderTermRefs(nodeData.domain, 'Any (unconstrained)')}</div>
            </div>`
            : '';

        const range = isProperty
            ? `<div class="ontology-meta-section">
                <div class="ontology-meta-title">Range</div>
                <div>${renderTermRefs(nodeData.range, 'Any (unconstrained)')}</div>
            </div>`
            : '';

        replaceTrustedOntologyHtml(body, `
            ${badges}
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">Kind</div>
                <div><span class="ontology-kind-pill">${escapeHtml(humanizeSemanticType(nodeData.semantic_type))}</span></div>
            </div>
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">RDF Type</div>
                <div>${types}</div>
            </div>
            ${identifier}
            ${ontologyDocument}
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">Description</div>
                <p class="m-0">${escapeHtml(nodeData.comment || 'None specified.')}</p>
            </div>
            ${literalValues}
            ${classExpressionSummary}
            ${properties}
            ${domain}
            ${range}
            ${slots}
            ${equivalence}
            ${inverses}
            ${chains}
            ${constructs}
            <div class="ontology-meta-section">
                <div class="ontology-meta-title">Sources</div>
                ${renderSources(nodeData.sources)}
            </div>
            ${rawShaclEvidence}
        `);
    }

    function endpointId(value) {
        return typeof value === 'string' ? value : (value && value.id ? value.id : '');
    }

    function hasAny(values, activeValues) {
        for (const value of values || []) {
            if (activeValues.has(value)) {
                return true;
            }
        }
        return false;
    }

    function nodeRoleValues(nodeData) {
        return new Set([nodeRoleFilterValue(nodeData)]);
    }

    function nodeOriginValues(nodeData) {
        const origins = new Set();
        if (isConstructOnlyNode(nodeData)) {
            origins.add('construct');
        }
        if (hasAuthoredSource(nodeData)) {
            origins.add('authored');
        }
        if (!origins.size) {
            origins.add('registry');
        }
        return origins;
    }

    function nodeLayerValues(nodeData) {
        return new Set([layerFilterValue(nodeData && nodeData.layer)]);
    }

    function edgeLayerValues(edgeData) {
        return new Set([layerFilterValue(edgeData && edgeData.layer)]);
    }

    function layerFilterValue(layer) {
        if (layer === 'concepts') return 'layer-concepts';
        if (layer === 'reqvire-context') return 'layer-reqvire-context';
        if (layer === 'external-source') return 'layer-external-source';
        return 'layer-authored';
    }

    function nodeRoleFilterValue(nodeData) {
        if (isExternalReferenceNode(nodeData)) {
            return 'external-reference';
        }
        const semanticType = nodeData.semantic_type || 'resource';
        if (['object-property', 'datatype-property', 'rdf-property'].includes(semanticType)) {
            return 'property';
        }
        if (['node-shape', 'property-shape'].includes(semanticType)) {
            return 'shacl-shape';
        }
        if (semanticType === 'resource') {
            return 'resource';
        }
        return 'ontology-term';
    }

    function isExternalReferenceNode(nodeData) {
        const iri = String(nodeData.full_uri || nodeData.id || '');
        return iri.startsWith('http://www.w3.org/2001/XMLSchema#')
            || iri.startsWith('http://www.w3.org/1999/02/22-rdf-syntax-ns#')
            || iri.startsWith('http://www.w3.org/2000/01/rdf-schema#')
            || iri.startsWith('http://www.w3.org/2002/07/owl#')
            || iri.startsWith('http://www.w3.org/ns/shacl#');
    }

    function nodeConstructValues(nodeData) {
        const constructs = new Set();
        const semanticType = String(nodeData.semantic_type || '');
        if (semanticType === 'restriction') {
            constructs.add('restriction');
        }
        if (semanticType === 'class-expression') {
            constructs.add('class-expression');
        }
        (nodeData.constructs || []).forEach(item => {
            constructFilterValues(item).forEach(value => constructs.add(value));
        });
        if ((nodeData.inverse_properties || []).length) {
            constructs.add('inverse');
        }
        if ((nodeData.property_chains || []).length) {
            constructs.add('property-chain');
        }
        if ((nodeData.domain || []).length || (nodeData.range || []).length) {
            constructs.add('domain-range');
        }
        if ((nodeData.slot_facets || []).length) {
            constructs.add('shape-overlay');
        }
        return constructs;
    }

    function edgeConstructValues(edgeData) {
        const constructs = new Set();
        const label = String(edgeData.label || '');
        if (edgeData.rendered_kind === 'property') constructs.add('domain-range');
        if (label === 'domain' || label === 'range') constructs.add('domain-range');
        if (label === 'subclass') constructs.add('subclass');
        if (label === 'member') constructs.add('membership');
        if (label === 'disjoint') constructs.add('disjoint');
        if (label === 'inverse') constructs.add('inverse');
        if (label === 'on property' || label.endsWith('restriction')) constructs.add('restriction');
        if (['intersection', 'union', 'complement', 'class-expression'].includes(label)) {
            constructs.add('class-expression');
        }
        if (label.includes('shape overlay')) constructs.add('shape-overlay');
        return constructs;
    }

    function constructFilterValues(construct) {
        const kind = String((construct && construct.kind) || '');
        const family = String((construct && construct.family) || '');
        const values = new Set();
        if (kind === 'property-domain' || kind === 'property-range' || family === 'property-domain-range') values.add('domain-range');
        if (kind === 'subclass-inclusion') values.add('subclass');
        if (kind === 'membership') values.add('membership');
        if (kind === 'disjointness') values.add('disjoint');
        if (kind === 'equivalence-group') values.add('equivalence');
        if (kind === 'inverse-property') values.add('inverse');
        if (kind === 'property-chain' || family === 'property-chain') values.add('property-chain');
        if (kind === 'property-characteristic' || family === 'property-characteristic') values.add('property-characteristic');
        if (kind === 'restriction' || family === 'restriction') values.add('restriction');
        if (kind === 'class-expression' || family === 'class-expression') values.add('class-expression');
        if (kind === 'shape-overlay' || family === 'shape-overlay') values.add('shape-overlay');
        return values;
    }

    function isConstructOnlyNode(nodeData) {
        const id = String(nodeData.id || nodeData.full_uri || '');
        if (nodeHasConstructClass(nodeData, 'restriction') || nodeHasConstructClass(nodeData, 'class-expression')) {
            return true;
        }
        return id.startsWith('urn:reqvire:ontology-construct')
            || id.startsWith('urn:reqvire:ontology-member')
            || id.startsWith('urn:reqvire:ontology-symbol');
    }

    function hasAuthoredSource(nodeData) {
        return (nodeData.sources || []).some(source => source.kind === 'ontology' || source.kind === 'shapes');
    }

    function nodePassesOwnFilters(nodeData) {
        if (!hasAny(nodeData._ontologyLayers, filterState.layer)) {
            return false;
        }
        if (!hasAny(nodeData._ontologyRoles, filterState.role)) {
            return false;
        }
        if (!hasAny(nodeData._ontologyOrigins, filterState.origin)) {
            return false;
        }
        if (!nodePassesRelationFilters(nodeData)) {
            return false;
        }
        if (isConstructOnlyNode(nodeData)) {
            return hasAny(nodeData._ontologyConstructs, filterState.construct);
        }
        if ((nodeData.layer || 'authored') !== 'authored') {
            return true;
        }
        return hasAuthoredSource(nodeData);
    }

    function edgePassesFilters(edgeData) {
        const hasConstructRole = edgeData._ontologyConstructs && edgeData._ontologyConstructs.size > 0;
        if (!hasAny(edgeData._ontologyLayers, filterState.layer)) {
            return false;
        }
        if (!edgePassesRelationFilters(edgeData)) {
            return false;
        }
        if ((edgeData.layer || 'authored') !== 'authored') {
            return true;
        }
        return hasConstructRole
            && hasAny(edgeData._ontologyConstructs, filterState.construct);
    }

    function nodePassesRelationFilters(nodeData) {
        if (!relationFilterState.has('class-expressions') && nodeHasConstructClass(nodeData, 'class-expression')) {
            return false;
        }
        if (!relationFilterState.has('restrictions') && nodeHasConstructClass(nodeData, 'restriction')) {
            return false;
        }
        return true;
    }

    function edgePassesRelationFilters(edgeData) {
        if (!relationFilterState.has('class-disjointness') && edgeHasConstruct(edgeData, 'disjoint')) {
            return false;
        }
        if (!relationFilterState.has('class-membership') && edgeHasConstruct(edgeData, 'membership')) {
            return false;
        }
        if (!relationFilterState.has('restrictions') && edgeHasConstruct(edgeData, 'restriction')) {
            return false;
        }
        if (!relationFilterState.has('class-expressions') && edgeHasConstruct(edgeData, 'class-expression')) {
            return false;
        }
        if (!filterState.role.has('shacl-shape') && edgeHasConstruct(edgeData, 'shape-overlay')) {
            return false;
        }
        return true;
    }

    function edgeHasConstruct(edgeData, construct) {
        return Boolean(edgeData._ontologyConstructs && edgeData._ontologyConstructs.has(construct));
    }

    function constructPassesRelationFilters(construct) {
        if (construct === 'membership' && !relationFilterState.has('class-membership')) {
            return false;
        }
        if (construct === 'disjoint' && !relationFilterState.has('class-disjointness')) {
            return false;
        }
        if (construct === 'restriction' && !relationFilterState.has('restrictions')) {
            return false;
        }
        if (construct === 'class-expression' && !relationFilterState.has('class-expressions')) {
            return false;
        }
        if (construct === 'shape-overlay' && !filterState.role.has('shacl-shape')) {
            return false;
        }
        return true;
    }

    function computeVisibleNodeIds() {
        const visible = new Set();
        nodes.forEach(nodeData => {
            if (nodePassesOwnFilters(nodeData)) {
                visible.add(nodeData.id);
            }
        });
        return visible;
    }

    function isEdgeVisible(edgeData) {
        const sourceId = endpointId(edgeData.source);
        const targetId = endpointId(edgeData.target);
        return edgePassesFilters(edgeData)
            && visibleNodeIds.has(sourceId)
            && visibleNodeIds.has(targetId);
    }

    function applyGraphFilters() {
        visibleNodeIds = computeVisibleNodeIds();
        graphFilterRevision += 1;
        focusNeighborhoodCacheKey = '';
        if (graph) {
            nodes.forEach(nodeData => {
                if (graph.hasNode(nodeData.id)) {
                    graph.setNodeAttribute(nodeData.id, 'hidden', !visibleNodeIds.has(nodeData.id));
                    graph.setNodeAttribute(nodeData.id, 'label', sigmaNodeLabel(nodeData));
                    graph.setNodeAttribute(nodeData.id, 'fullLabel', fullSigmaNodeLabel(nodeData));
                }
            });
            links.forEach((_linkData, index) => {
                const key = `o${index}`;
                if (graph.hasEdge(key)) {
                    graph.setEdgeAttribute(key, 'hidden', !isEdgeVisible(_linkData));
                }
            });
        }

        document.querySelectorAll('.ontology-filter-toggle').forEach(button => {
            const category = button.dataset.filterCategory;
            const value = button.dataset.filterValue;
            const active = category === 'relation'
                ? relationFilterState.has(value)
                : Boolean(filterState[category] && filterState[category].has(value));
            button.classList.toggle('is-active', active);
            button.setAttribute('aria-pressed', active ? 'true' : 'false');
        });

        const search = document.getElementById('ontology-graph-search');
        if (search && search.value.trim()) {
            window.filterOntologyGraph(search.value);
        }
        if (selectedNodeId && nodeById.has(selectedNodeId)) {
            renderInspector(nodeById.get(selectedNodeId));
        }
        refreshOntologyRenderer();
    }

    window.filterOntologyGraph = function (query) {
        const results = document.getElementById('ontology-graph-results');
        if (!results) {
            return;
        }
        const normalized = query.trim().toLowerCase();
        if (!normalized) {
            results.style.display = 'none';
            results.replaceChildren();
            return;
        }
        results.style.listStyle = 'none';
        results.style.overflowY = 'auto';
        const matches = nodes
            .filter(node => visibleNodeIds.has(node.id) && ontologyNodeHaystack(node).includes(normalized))
            .slice(0, 40);

        if (!matches.length) {
            const empty = document.createElement('li');
            empty.className = 'ontology-graph-result text-gray-400';
            empty.style.listStyle = 'none';
            empty.textContent = 'No matching nodes found';
            results.replaceChildren(empty);
            results.style.display = 'block';
            return;
        }

        results.replaceChildren(...matches.map(node => {
            const item = document.createElement('li');
            item.style.listStyle = 'none';
            item.setAttribute('role', 'presentation');

            const button = document.createElement('button');
            button.type = 'button';
            button.className = 'ontology-graph-result';
            item.style.listStyle = 'none';
            button.setAttribute('aria-label', `Focus ${graphNodeDisplayLabel(node)} (${humanizeSemanticType(node.semantic_type)})`);
            button.addEventListener('click', () => window.focusOntologyNode(node.id));

            const label = document.createElement('span');
            label.className = 'ontology-graph-result-label';
            label.textContent = graphNodeDisplayLabel(node);
            label.title = graphNodeDisplayLabel(node);

            const glyph = document.createElement('span');
            glyph.className = 'ontology-graph-result-glyph';
            glyph.setAttribute('data-semantic-type', String(node.semantic_type || node.node_type || 'resource'));
            glyph.title = humanizeSemanticType(node.semantic_type);

            const notation = visibleBadgeSymbols(node);
            if (notation) {
                const badge = document.createElement('span');
                badge.className = 'ontology-graph-badge';
                badge.textContent = notation;
                badge.title = notation;
                button.append(glyph, label, badge);
            } else {
                button.append(glyph, label);
            }
            item.append(button);
            return item;
        }));
        results.style.display = 'block';
    };

    window.focusOntologyNode = function (nodeId) {
        const selected = nodeById.get(nodeId);
        if (!selected) return;
        if (!visibleNodeIds.has(nodeId) || !graph || !graph.hasNode(nodeId)) return;
        selectedNodeId = nodeId;
        onSelect?.(selected);
        const results = document.getElementById('ontology-graph-results');
        const search = document.getElementById('ontology-graph-search');
        if (results) results.style.display = 'none';
        if (search) search.value = '';
        renderInspector(selected);
        centerOnOntologyNode(nodeId);
        refreshOntologyRenderer();
    };

    window.clearOntologySelection = function () {
        selectedNodeId = null;
        hoveredNodeId = null;
        onSelect?.(null);
        const clear = document.getElementById('ontology-inspector-clear');
        const title = document.getElementById('ontology-inspector-title');
        const body = document.getElementById('ontology-inspector-body');
        if (clear) clear.style.display = 'none';
        if (title) title.textContent = 'Node Inspector';
        if (body) renderOntologyEmptyState(body);
        refreshOntologyRenderer();
    };

    window.fitOntologyGraph = function () {
        if (!renderer || !graph) return;
        renderer.getCamera().animatedReset({ duration: 250 });
    };

    window.resetOntologyGraphLayout = function () {
        if (!graph) return;
        assignInitialSigmaPositions(nodes);
        nodes.forEach(nodeData => {
            if (graph.hasNode(nodeData.id)) {
                graph.mergeNodeAttributes(nodeData.id, { x: nodeData.x, y: nodeData.y });
            }
        });
        applyOntologyLayout();
        refreshOntologyRenderer();
        window.fitOntologyGraph();
    };

    function refreshOntologyRenderer() {
        if (renderer) {
            renderer.refresh();
        }
    }

    function activeOntologyFocusIds() {
        const ids = [];
        if (selectedNodeId) ids.push(selectedNodeId);
        return ids;
    }

    function activeOntologyHoverIds() {
        const ids = [];
        if (hoveredNodeId && hoveredNodeId !== selectedNodeId) ids.push(hoveredNodeId);
        return ids;
    }

    function activeOntologyFocusNeighborhoodIds() {
        const focusIds = activeOntologyFocusIds().filter(nodeId => visibleNodeIds.has(nodeId));
        if (!focusIds.length) {
            return new Set();
        }
        const cacheKey = `${graphFilterRevision}|${focusIds.join('\u001f')}`;
        if (focusNeighborhoodCacheKey === cacheKey) {
            return focusNeighborhoodCache;
        }
        focusNeighborhoodCacheKey = cacheKey;
        focusNeighborhoodCache = computeFocusNeighborhoodIds(focusIds);
        return focusNeighborhoodCache;
    }

    function activeOntologyHoverNeighborhoodIds() {
        const hoverIds = activeOntologyHoverIds().filter(nodeId => visibleNodeIds.has(nodeId));
        if (!hoverIds.length) {
            return new Set();
        }
        return computeFocusNeighborhoodIds(hoverIds);
    }

    function computeFocusNeighborhoodIds(focusIds) {
        const focusSet = new Set(focusIds);
        const neighborhood = new Set();
        const expansionQueue = [];
        const expandedFrom = new Set();

        focusIds.forEach(nodeId => {
            if (visibleNodeIds.has(nodeId)) {
                neighborhood.add(nodeId);
                expansionQueue.push(nodeId);
            }
        });

        while (expansionQueue.length) {
            const currentId = expansionQueue.shift();
            if (expandedFrom.has(currentId)) {
                continue;
            }
            const currentNode = nodeById.get(currentId);
            const canExpand = focusSet.has(currentId) || (currentNode && isConstructOnlyNode(currentNode));
            if (!canExpand) {
                continue;
            }
            expandedFrom.add(currentId);

            (linkAdjacency.get(currentId) || []).forEach(linkData => {
                if (!isEdgeVisible(linkData)) {
                    return;
                }
                const sourceId = endpointId(linkData.source);
                const targetId = endpointId(linkData.target);
                if (sourceId !== currentId && targetId !== currentId) {
                    return;
                }
                const otherId = sourceId === currentId ? targetId : sourceId;
                if (!visibleNodeIds.has(otherId)) {
                    return;
                }
                neighborhood.add(otherId);
                const otherNode = nodeById.get(otherId);
                if (otherNode && isConstructOnlyNode(otherNode) && !expandedFrom.has(otherId)) {
                    expansionQueue.push(otherId);
                }
            });
        }

        return neighborhood;
    }

    function isEdgeInFocusNeighborhood(edgeData, focusIds, focusNeighborhoodIds) {
        const sourceId = endpointId(edgeData.source);
        const targetId = endpointId(edgeData.target);
        if (!focusNeighborhoodIds.has(sourceId) || !focusNeighborhoodIds.has(targetId)) {
            return false;
        }
        if (focusIds.some(focusId => sourceId === focusId || targetId === focusId)) {
            return true;
        }
        const sourceNode = nodeById.get(sourceId);
        const targetNode = nodeById.get(targetId);
        return Boolean((sourceNode && isConstructOnlyNode(sourceNode)) || (targetNode && isConstructOnlyNode(targetNode)));
    }

    function centerOnOntologyNode(nodeId) {
        if (!renderer || !graph || !graph.hasNode(nodeId)) return;
        const display = renderer.getNodeDisplayData(nodeId);
        if (!display) return;
        const camera = renderer.getCamera();
        const state = camera.getState();
        camera.animate(
            { x: display.x, y: display.y, ratio: Math.min(state.ratio, 0.9) },
            { duration: 280 }
        );
    }

    window.setOntologyGraphFilter = function (category, value, active) {
        if (!value) {
            return;
        }
        const targetSet = category === 'relation'
            ? relationFilterState
            : filterState[category];
        if (!targetSet) {
            return;
        }
        if (active) {
            targetSet.add(value);
        } else {
            targetSet.delete(value);
        }
        applyGraphFilters();
    };

    window.syncOntologyGraphFilters = function (activeValues) {
        const activeSet = new Set(activeValues || []);
        filterState.role.clear();
        filterState.construct.clear();
        filterState.origin.clear();
        filterState.layer.clear();
        relationFilterState.clear();
        filterState.role.add('ontology-term');
        filterState.role.add('shacl-shape');
        filterState.role.add('resource');
        filterState.role.add('external-reference');
        relationFilterState.add('class-membership');
        ['shacl-shape', 'resource', 'external-reference'].forEach(value => {
            if (activeSet.has(value)) filterState.role.add(value);
        });
        [
            'domain-range',
            'subclass',
            'membership',
            'disjoint',
            'equivalence',
            'inverse',
            'property-chain',
            'property-characteristic',
            'class-expression',
            'shape-overlay'
        ].forEach(value => {
            if (activeSet.has(value)) filterState.construct.add(value);
        });
        ['authored', 'registry', 'construct'].forEach(value => {
            if (activeSet.has(value)) filterState.origin.add(value);
        });
        ['layer-authored', 'layer-concepts', 'layer-reqvire-context', 'layer-external-source'].forEach(value => {
            if (activeSet.has(value)) filterState.layer.add(value);
        });
        [
            'class-disjointness',
            'class-expressions'
        ].forEach(value => {
            if (activeSet.has(value)) relationFilterState.add(value);
        });
        applyGraphFilters();
    };

    applyGraphFilters();
    const fitTimer = window.setTimeout(window.fitOntologyGraph, 550);
    const resizeHandler = () => refreshOntologyRenderer();
    window.addEventListener('resize', resizeHandler);

    return {
        destroy() {
            window.clearTimeout(fitTimer);
            if (suppressStageClearTimer) {
                window.clearTimeout(suppressStageClearTimer);
                suppressStageClearTimer = null;
            }
            window.removeEventListener('resize', resizeHandler);
            if (renderer) {
                setGraphCursor('');
                renderer.kill();
            }
            delete window.filterOntologyGraph;
            delete window.focusOntologyNode;
            delete window.clearOntologySelection;
            delete window.fitOntologyGraph;
            delete window.resetOntologyGraphLayout;
            delete window.setOntologyGraphFilter;
            delete window.syncOntologyGraphFilters;
        },
        filter(query) {
            window.filterOntologyGraph?.(query);
        },
        focusNode(nodeId) {
            window.focusOntologyNode?.(nodeId);
        },
        clearSelection() {
            window.clearOntologySelection?.();
        },
        resetLayout() {
            window.resetOntologyGraphLayout?.();
        },
        setFilter(category, value, active) {
            window.setOntologyGraphFilter?.(category, value, active);
        }
    };
}
