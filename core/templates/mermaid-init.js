import mermaid from 'https://cdn.jsdelivr.net/npm/mermaid@11/dist/mermaid.esm.min.mjs';
import elkLayouts from 'https://cdn.jsdelivr.net/npm/@mermaid-js/layout-elk/dist/mermaid-layout-elk.esm.min.mjs';

// Register ELK layout for large diagrams
mermaid.registerLayoutLoaders(elkLayouts);

// Capture Mermaid errors
mermaid.parseError = function(err, hash) {
    console.error('Mermaid Parse Error:', err);
    console.error('Error details:', {
        str: err.str,
        hash: err.hash,
        message: err.message,
        stack: err.stack
    });
};

mermaid.initialize({
    startOnLoad: false,
    theme: 'neutral',
    maxTextSize: 5000000,
    maxEdges: 50000,
    flowchart: {
        useMaxWidth: true,
        htmlLabels: true,
        curve: 'basis',
        defaultRenderer: 'elk'
    },
    layout: 'elk',
    elk: {
        mergeEdges: true,
        nodePlacementStrategy: 'SIMPLE'
    },
    securityLevel: 'loose',
    logLevel: 'error'
});

// Store original mermaid content before rendering (for resize refreshes)
document.querySelectorAll('.mermaid').forEach(el => {
    el.setAttribute('data-original-content', el.textContent);
});

await mermaid.run();
window.mermaid = mermaid;

// Wait for SVGs to be fully rendered, then initialize
function waitForSvgs(callback, startedAt) {
    startedAt = startedAt || performance.now();
    var containers = document.querySelectorAll('.mermaid');
    var allReady = true;
    containers.forEach(function(c) {
        var svg = c.querySelector('svg');
        if (!svg || !svg.getBBox || svg.getBBox().width === 0) {
            allReady = false;
        }
    });
    if ((allReady && containers.length > 0) || performance.now() - startedAt > 5000) {
        if (!allReady) {
            console.warn('Timed out waiting for one or more Mermaid diagrams; initializing rendered diagrams.');
        }
        callback();
    } else {
        requestAnimationFrame(function() { waitForSvgs(callback, startedAt); });
    }
}

// Function to initialize pan/zoom on all Mermaid diagrams
function initializePanZoom() {
    document.querySelectorAll('.mermaid').forEach(function(mermaidContainer) {
            var svg = mermaidContainer.querySelector('svg');
            if (!svg) return;

            // Get the natural size of the diagram
            var bbox = svg.getBBox();
            var containerWidth = mermaidContainer.clientWidth;
            var containerHeight = Math.max(mermaidContainer.clientHeight, 280);

            // Check if diagram fits within container (with some padding)
            var padding = 40;
            var fitsInContainer = (bbox.width <= containerWidth - padding) && (bbox.height <= containerHeight - padding);

            if (fitsInContainer) {
                // Diagram fits - use natural size, shrink container to fit content
                svg.style.width = 'auto';
                svg.style.height = 'auto';
                mermaidContainer.style.height = 'auto';
                mermaidContainer.style.minHeight = (bbox.height + padding) + 'px';
                mermaidContainer.style.maxHeight = 'calc(100vh - 150px)';
                // No pan/zoom needed for small diagrams
                return;
            }

            // Large diagram - enable pan/zoom
            var viewportHeight = Math.max(360, window.innerHeight - 150);
            var desiredHeight = Math.min(Math.max(420, bbox.height + padding), viewportHeight);
            mermaidContainer.style.height = desiredHeight + 'px';
            mermaidContainer.style.minHeight = '0';
            mermaidContainer.style.maxHeight = 'none';
            svg.style.maxWidth = 'none';
            svg.style.maxHeight = 'none';

            var eventsHandler = {
                haltEventListeners: ['touchstart', 'touchend', 'touchmove', 'touchleave', 'touchcancel'],
                init: function(options) {
                    var instance = options.instance;
                    var initialScale = 1;
                    var pannedX = 0;
                    var pannedY = 0;

                    this.hammer = Hammer(options.svgElement, {
                        inputClass: Hammer.SUPPORT_POINTER_EVENTS ? Hammer.PointerEventInput : Hammer.TouchInput
                    });

                    this.hammer.get('pinch').set({enable: true});

                    this.hammer.on('doubletap', function(ev){
                        instance.zoomIn();
                    });

                    this.hammer.on('panstart panmove', function(ev){
                        if (ev.type === 'panstart') {
                            pannedX = 0;
                            pannedY = 0;
                        }
                        instance.panBy({x: ev.deltaX - pannedX, y: ev.deltaY - pannedY});
                        pannedX = ev.deltaX;
                        pannedY = ev.deltaY;
                    });

                    this.hammer.on('pinchstart pinchmove', function(ev){
                        if (ev.type === 'pinchstart') {
                            initialScale = instance.getZoom();
                        }
                        instance.zoomAtPoint(initialScale * ev.scale, {x: ev.center.x, y: ev.center.y});
                    });

                    options.svgElement.addEventListener('touchmove', function(e){ e.preventDefault(); });
                },
                destroy: function(){
                    this.hammer.destroy();
                }
            };

            // Diagrams with default zoom (no auto-fit)
            var panZoomInstance = svgPanZoom(svg, {
                zoomEnabled: true,
                controlIconsEnabled: true,
                fit: false,
                center: false,
                contain: false,
                minZoom: 0.5,
                maxZoom: 10,
                zoomScaleSensitivity: 0.3,
                customEventsHandler: eventsHandler
            });

            // Add navigation buttons
            var navButtons = document.createElement('div');
            navButtons.className = 'diagram-nav-buttons';
            navButtons.innerHTML = `
                <div class="diagram-nav-row">
                    <button class="diagram-nav-btn" data-action="up">▲</button>
                </div>
                <div class="diagram-nav-row">
                    <button class="diagram-nav-btn" data-action="left">◀</button>
                    <button class="diagram-nav-btn" data-action="down">▼</button>
                    <button class="diagram-nav-btn" data-action="right">▶</button>
                </div>
            `;
            mermaidContainer.appendChild(navButtons);

            // Wire up navigation buttons
            var panStep = 100;
            navButtons.querySelectorAll('.diagram-nav-btn').forEach(function(btn) {
                btn.addEventListener('click', function(e) {
                    e.preventDefault();
                    var action = this.getAttribute('data-action');
                    switch(action) {
                        case 'up':
                            panZoomInstance.panBy({x: 0, y: panStep});
                            break;
                        case 'down':
                            panZoomInstance.panBy({x: 0, y: -panStep});
                            break;
                        case 'left':
                            panZoomInstance.panBy({x: panStep, y: 0});
                            break;
                        case 'right':
                            panZoomInstance.panBy({x: -panStep, y: 0});
                            break;
                    }
                });
            });
    });

    // Add CSS for edge highlighting (high specificity for SVG)
    var style = document.createElement('style');
    style.textContent = `
        path.edge-highlighted,
        .edge-highlighted path,
        svg path.edge-highlighted {
            stroke: #FFAB91 !important;
            stroke-width: 3px !important;
        }
    `;
    document.head.appendChild(style);

    // Add edge highlighting on node hover
    document.querySelectorAll('.mermaid svg').forEach(function(svg) {
        var nodes = svg.querySelectorAll('.node');

        // Find flowchart-link paths (actual edges)
        var edgePaths = svg.querySelectorAll('path.flowchart-link');
        // Fallback: find paths in .edges group
        if (edgePaths.length === 0) {
            var edgesGroup = svg.querySelector('.edges');
            if (edgesGroup) {
                edgePaths = edgesGroup.querySelectorAll('path');
            }
        }

        // Get SVG point for coordinate transforms
        var svgPoint = svg.createSVGPoint();

        // Helper: transform point through element's CTM
        function getTransformedBBox(element) {
            var bbox = element.getBBox();
            var ctm = element.getCTM();
            if (!ctm) return bbox;

            // Transform all four corners
            var corners = [
                {x: bbox.x, y: bbox.y},
                {x: bbox.x + bbox.width, y: bbox.y},
                {x: bbox.x, y: bbox.y + bbox.height},
                {x: bbox.x + bbox.width, y: bbox.y + bbox.height}
            ];

            var transformed = corners.map(function(c) {
                svgPoint.x = c.x;
                svgPoint.y = c.y;
                var tp = svgPoint.matrixTransform(ctm);
                return {x: tp.x, y: tp.y};
            });

            var minX = Math.min.apply(null, transformed.map(function(p) { return p.x; }));
            var maxX = Math.max.apply(null, transformed.map(function(p) { return p.x; }));
            var minY = Math.min.apply(null, transformed.map(function(p) { return p.y; }));
            var maxY = Math.max.apply(null, transformed.map(function(p) { return p.y; }));

            return {x: minX, y: minY, width: maxX - minX, height: maxY - minY};
        }

        // Helper: get all points from path
        function getPathPoints(path) {
            var d = path.getAttribute('d') || '';
            var points = [];
            var ctm = path.getCTM();

            // Match all coordinate pairs in the path
            var regex = /([ML])\s*([\d.-]+)[,\s]+([\d.-]+)|([CQ])[^MLCQZ]*/gi;
            var match;

            // Get M (start) point
            var startMatch = d.match(/M\s*([\d.-]+)[,\s]+([\d.-]+)/i);
            if (startMatch) {
                var sx = parseFloat(startMatch[1]);
                var sy = parseFloat(startMatch[2]);
                if (ctm) {
                    svgPoint.x = sx; svgPoint.y = sy;
                    var tp = svgPoint.matrixTransform(ctm);
                    points.push({x: tp.x, y: tp.y});
                } else {
                    points.push({x: sx, y: sy});
                }
            }

            // Get last point - find last number pair before Z or end
            var allNums = d.match(/[\d.-]+/g);
            if (allNums && allNums.length >= 2) {
                var ex = parseFloat(allNums[allNums.length - 2]);
                var ey = parseFloat(allNums[allNums.length - 1]);
                if (ctm) {
                    svgPoint.x = ex; svgPoint.y = ey;
                    var tp = svgPoint.matrixTransform(ctm);
                    points.push({x: tp.x, y: tp.y});
                } else {
                    points.push({x: ex, y: ey});
                }
            }

            return points;
        }

        // Build node bounding boxes with transforms
        var nodeData = {};
        nodes.forEach(function(node) {
            var nodeId = node.id || '';
            var parts = nodeId.split('-');
            var hashId = parts.length >= 2 ? parts[1] : nodeId;

            nodeData[hashId] = {
                node: node,
                bbox: getTransformedBBox(node)
            };
        });

        // Calculate distance from point to bbox center
        function distToBboxCenter(pt, bbox) {
            var centerX = bbox.x + bbox.width / 2;
            var centerY = bbox.y + bbox.height / 2;
            return Math.sqrt(Math.pow(pt.x - centerX, 2) + Math.pow(pt.y - centerY, 2));
        }

        // Check if point is near bbox
        function isNearBbox(pt, bbox, margin) {
            margin = margin || 10;
            return pt.x >= bbox.x - margin && pt.x <= bbox.x + bbox.width + margin &&
                   pt.y >= bbox.y - margin && pt.y <= bbox.y + bbox.height + margin;
        }

        // Build edge connections with direction tracking
        var edgeConnections = []; // [{path, source, target}]
        var upstreamOf = {}; // node -> [parent nodes that point TO this node]
        var downstreamOf = {}; // node -> [child nodes this node points TO]

        Object.keys(nodeData).forEach(function(hashId) {
            upstreamOf[hashId] = [];
            downstreamOf[hashId] = [];
        });

        edgePaths.forEach(function(path) {
            var points = getPathPoints(path);
            if (points.length < 2) return;

            var startPt = points[0];
            var endPt = points[points.length - 1];

            var sourceNode = null;
            var targetNode = null;
            var sourceMinDist = Infinity;
            var targetMinDist = Infinity;

            // Find closest source and target nodes for this edge
            Object.keys(nodeData).forEach(function(hashId) {
                var bbox = nodeData[hashId].bbox;

                // Find closest node to start point (source)
                if (isNearBbox(startPt, bbox, 30)) {
                    var dist = distToBboxCenter(startPt, bbox);
                    if (dist < sourceMinDist) {
                        sourceMinDist = dist;
                        sourceNode = hashId;
                    }
                }

                // Find closest node to end point (target)
                if (isNearBbox(endPt, bbox, 30)) {
                    var dist = distToBboxCenter(endPt, bbox);
                    if (dist < targetMinDist) {
                        targetMinDist = dist;
                        targetNode = hashId;
                    }
                }
            });

            if (sourceNode && targetNode && sourceNode !== targetNode) {
                edgeConnections.push({path: path, source: sourceNode, target: targetNode});
                // source -> target means: target's upstream includes source
                if (!upstreamOf[targetNode].includes(sourceNode)) {
                    upstreamOf[targetNode].push(sourceNode);
                }
                // source -> target means: source's downstream includes target
                if (!downstreamOf[sourceNode].includes(targetNode)) {
                    downstreamOf[sourceNode].push(targetNode);
                }
            }
        });

        // Traverse only upstream (to parents/ancestors)
        function findUpstream(startNodeId) {
            var visited = {};
            var queue = [startNodeId];
            visited[startNodeId] = true;

            while (queue.length > 0) {
                var current = queue.shift();
                var parents = upstreamOf[current] || [];
                parents.forEach(function(parent) {
                    if (!visited[parent]) {
                        visited[parent] = true;
                        queue.push(parent);
                    }
                });
            }
            return visited;
        }

        // Traverse only downstream (to children/descendants)
        function findDownstream(startNodeId) {
            var visited = {};
            var queue = [startNodeId];
            visited[startNodeId] = true;

            while (queue.length > 0) {
                var current = queue.shift();
                var children = downstreamOf[current] || [];
                children.forEach(function(child) {
                    if (!visited[child]) {
                        visited[child] = true;
                        queue.push(child);
                    }
                });
            }
            return visited;
        }

        // Get edges for connected nodes
        function getChainEdges(nodeSet) {
            var edges = [];
            edgeConnections.forEach(function(conn) {
                if (nodeSet[conn.source] && nodeSet[conn.target]) {
                    edges.push(conn.path);
                }
            });
            return edges;
        }

        // Attach hover handlers
        nodes.forEach(function(node) {
            var nodeId = node.id || '';
            var parts = nodeId.split('-');
            var hashId = parts.length >= 2 ? parts[1] : nodeId;

            node.style.cursor = 'pointer';

            node.addEventListener('mouseenter', function(e) {
                e.stopPropagation();
                nodes.forEach(function(n) { n.style.filter = ''; });
                node.style.filter = 'drop-shadow(0 0 8px rgba(255, 171, 145, 0.7))';

                // Find upstream (ancestors) and downstream (descendants) separately
                var upstream = findUpstream(hashId);
                var downstream = findDownstream(hashId);
                // Merge both sets
                var allConnected = {};
                Object.keys(upstream).forEach(function(k) { allConnected[k] = true; });
                Object.keys(downstream).forEach(function(k) { allConnected[k] = true; });

                var chainEdges = getChainEdges(allConnected);
                chainEdges.forEach(function(path) {
                    path.classList.add('edge-highlighted');
                });
            });

            node.addEventListener('mouseleave', function() {
                node.style.filter = '';
                // Remove highlight from all edges
                edgePaths.forEach(function(path) {
                    path.classList.remove('edge-highlighted');
                });
            });
        });
    });
}

// Initialize pan/zoom after page load
waitForSvgs(initializePanZoom);

// Refresh diagrams on window resize (debounced to avoid performance issues)
let resizeTimer;
window.addEventListener('resize', function() {
    clearTimeout(resizeTimer);
    resizeTimer = setTimeout(async function() {
        // Re-run mermaid on all diagrams
        const mermaidElements = document.querySelectorAll('.mermaid');
        mermaidElements.forEach(el => {
            // Restore original markdown content
            const originalContent = el.getAttribute('data-original-content');
            if (originalContent) {
                el.innerHTML = originalContent;
                el.removeAttribute('data-processed');
            }
        });
        // Re-render diagrams
        await mermaid.run();
        // Re-initialize pan-zoom after re-render
        waitForSvgs(initializePanZoom);
    }, 250); // Wait 250ms after resize stops before re-rendering
});
