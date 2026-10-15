/** Toast usage policy is owned here; the central lint runner registers these rules. */
const DS_IMPORTS = new Set(["@ds", "design-system-template"]);
const EFFECTS = new Set(["useEffect", "useLayoutEffect"]);
const FUNCTIONS = new Set(["FunctionDeclaration", "FunctionExpression", "ArrowFunctionExpression"]);

function imported(context, node, name, sources = DS_IMPORTS) {
  if (node?.type !== "Identifier" && node?.type !== "JSXIdentifier") return false;
  let scope = context.sourceCode.getScope(node);
  while (scope) {
    const variable = scope.set.get(node.name);
    if (variable) return variable.defs.some((definition) => definition.type === "ImportBinding"
      && definition.node.type === "ImportSpecifier"
      && definition.node.imported.name === name
      && sources.has(definition.parent.source.value));
    scope = scope.upper;
  }
  return false;
}

function property(node) {
  return node?.computed ? node.property?.value : node?.property?.name;
}

function isNamespaceMember(context, node, name, sources = DS_IMPORTS) {
  if (!["MemberExpression", "JSXMemberExpression"].includes(node?.type) || property(node) !== name || !["Identifier", "JSXIdentifier"].includes(node.object.type)) return false;
  let scope = context.sourceCode.getScope(node.object);
  while (scope) {
    const variable = scope.set.get(node.object.name);
    if (variable) return variable.defs.some((definition) => definition.type === "ImportBinding"
      && definition.node.type === "ImportNamespaceSpecifier" && sources.has(definition.parent.source.value));
    scope = scope.upper;
  }
  return false;
}

function toastMethod(context, callee) {
  if (imported(context, callee, "toast") || isNamespaceMember(context, callee, "toast")) return "show";
  if (callee?.type === "MemberExpression" && (imported(context, callee.object, "toast") || isNamespaceMember(context, callee.object, "toast"))) return property(callee);
  return null;
}

function closestFunction(node) {
  for (let parent = node.parent; parent; parent = parent.parent) if (FUNCTIONS.has(parent.type)) return parent;
  return null;
}

function functionName(node) {
  return node.id?.name ?? (node.parent?.type === "VariableDeclarator" ? node.parent.id.name : "");
}

const clientUsage = {
  meta: {
    type: "problem",
    schema: [],
    messages: {
      server: "Trigger toast notifications from the client that receives the server result, not in a 'use server' module.",
      render: "Call toast() in an event handler or effect, not at module scope or during component rendering.",
    },
  },
  create(context) {
    let server = false;
    return {
      Program(node) { server = node.body.some((statement) => statement.directive === "use server"); },
      CallExpression(node) {
        if (!toastMethod(context, node.callee)) return;
        if (server) { context.report({ node, messageId: "server" }); return; }
        const nearest = closestFunction(node);
        if (!nearest || /^[A-Z]/.test(functionName(nearest)) || (nearest.parent?.type === "ExportDefaultDeclaration" && !functionName(nearest))) {
          context.report({ node, messageId: "render" });
        }
      },
    };
  },
};

const effectId = {
  meta: {
    type: "problem",
    schema: [],
    messages: { id: "Toasts created directly in an effect need an explicit stable id to avoid duplicates on reruns and remounts. Prefer an event handler for user actions." },
  },
  create(context) {
    return {
      CallExpression(node) {
        const method = toastMethod(context, node.callee);
        if (!method || method === "update" || method === "dismiss") return;
        const nearest = closestFunction(node);
        const effect = nearest?.parent;
        if (effect?.type !== "CallExpression" || effect.arguments[0] !== nearest) return;
        if (![...EFFECTS].some((name) => imported(context, effect.callee, name, new Set(["react", "preact/hooks", "preact/compat"])) || isNamespaceMember(context, effect.callee, name, new Set(["react", "preact/hooks", "preact/compat"])))) return;
        const options = node.arguments[1];
        // Spread/dynamic options need review; don't invent a conclusion about their contents.
        if (options && (options.type !== "ObjectExpression" || options.properties.some((item) => item.type === "SpreadElement"))) return;
        if (!options?.properties.some((item) => (item.key.name ?? item.key.value) === "id")) context.report({ node, messageId: "id" });
      },
    };
  },
};

const rootMount = {
  meta: {
    type: "problem",
    schema: [{ type: "object", properties: { rootFiles: { type: "array", items: { type: "string" } } }, additionalProperties: false }],
    messages: {
      root: "Mount Toaster once in an approved application root, not in a page or conditional surface. Configure rootFiles for your app's entry/layout.",
      conditional: "Keep Toaster mounted unconditionally so navigation and state changes do not lose notifications.",
      duplicate: "This file mounts more than one Toaster with the same static id. Use one instance per scope.",
    },
  },
  create(context) {
    const roots = context.options[0]?.rootFiles ?? ["src/App.tsx", "src/main.tsx", "src/app/layout.tsx"];
    const filename = context.filename.replaceAll("\\", "/");
    const ids = new Set();
    return {
      JSXOpeningElement(node) {
        if (!imported(context, node.name, "Toaster") && !isNamespaceMember(context, node.name, "Toaster")) return;
        if (!roots.some((path) => filename === path || filename.endsWith(`/${path}`))) context.report({ node, messageId: "root" });
        for (let parent = node.parent; parent && !FUNCTIONS.has(parent.type); parent = parent.parent) {
          if (["ConditionalExpression", "LogicalExpression", "IfStatement"].includes(parent.type)) {
            context.report({ node, messageId: "conditional" });
            break;
          }
        }
        const id = node.attributes.find((attribute) => attribute.type === "JSXAttribute" && attribute.name.name === "id");
        const value = !id ? "default" : id.value?.type === "Literal" ? id.value.value : id.value?.expression?.type === "Literal" ? id.value.expression.value : null;
        if (value !== null) {
          if (ids.has(value)) context.report({ node, messageId: "duplicate" });
          ids.add(value);
        }
      },
    };
  },
};

export const toastRules = {
  "toast-client-usage": clientUsage,
  "toast-effect-id": effectId,
  "toaster-root-mount": rootMount,
};
