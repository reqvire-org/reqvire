var ReqvireExplorerDesignSystem_48409e = (function(exports, require$$0, reactDom) {
  "use strict";
  var jsxRuntime = { exports: {} };
  var reactJsxRuntime_production_min = {};
  /**
   * @license React
   * react-jsx-runtime.production.min.js
   *
   * Copyright (c) Facebook, Inc. and its affiliates.
   *
   * This source code is licensed under the MIT license found in the
   * LICENSE file in the root directory of this source tree.
   */
  var hasRequiredReactJsxRuntime_production_min;
  function requireReactJsxRuntime_production_min() {
    if (hasRequiredReactJsxRuntime_production_min) return reactJsxRuntime_production_min;
    hasRequiredReactJsxRuntime_production_min = 1;
    var f = require$$0, k = Symbol.for("react.element"), l = Symbol.for("react.fragment"), m = Object.prototype.hasOwnProperty, n = f.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED.ReactCurrentOwner, p = { key: true, ref: true, __self: true, __source: true };
    function q(c, a, g) {
      var b, d = {}, e = null, h = null;
      void 0 !== g && (e = "" + g);
      void 0 !== a.key && (e = "" + a.key);
      void 0 !== a.ref && (h = a.ref);
      for (b in a) m.call(a, b) && !p.hasOwnProperty(b) && (d[b] = a[b]);
      if (c && c.defaultProps) for (b in a = c.defaultProps, a) void 0 === d[b] && (d[b] = a[b]);
      return { $$typeof: k, type: c, key: e, ref: h, props: d, _owner: n.current };
    }
    reactJsxRuntime_production_min.Fragment = l;
    reactJsxRuntime_production_min.jsx = q;
    reactJsxRuntime_production_min.jsxs = q;
    return reactJsxRuntime_production_min;
  }
  var reactJsxRuntime_development = {};
  /**
   * @license React
   * react-jsx-runtime.development.js
   *
   * Copyright (c) Facebook, Inc. and its affiliates.
   *
   * This source code is licensed under the MIT license found in the
   * LICENSE file in the root directory of this source tree.
   */
  var hasRequiredReactJsxRuntime_development;
  function requireReactJsxRuntime_development() {
    if (hasRequiredReactJsxRuntime_development) return reactJsxRuntime_development;
    hasRequiredReactJsxRuntime_development = 1;
    if (process.env.NODE_ENV !== "production") {
      (function() {
        var React2 = require$$0;
        var REACT_ELEMENT_TYPE = Symbol.for("react.element");
        var REACT_PORTAL_TYPE = Symbol.for("react.portal");
        var REACT_FRAGMENT_TYPE = Symbol.for("react.fragment");
        var REACT_STRICT_MODE_TYPE = Symbol.for("react.strict_mode");
        var REACT_PROFILER_TYPE = Symbol.for("react.profiler");
        var REACT_PROVIDER_TYPE = Symbol.for("react.provider");
        var REACT_CONTEXT_TYPE = Symbol.for("react.context");
        var REACT_FORWARD_REF_TYPE = Symbol.for("react.forward_ref");
        var REACT_SUSPENSE_TYPE = Symbol.for("react.suspense");
        var REACT_SUSPENSE_LIST_TYPE = Symbol.for("react.suspense_list");
        var REACT_MEMO_TYPE = Symbol.for("react.memo");
        var REACT_LAZY_TYPE = Symbol.for("react.lazy");
        var REACT_OFFSCREEN_TYPE = Symbol.for("react.offscreen");
        var MAYBE_ITERATOR_SYMBOL = Symbol.iterator;
        var FAUX_ITERATOR_SYMBOL = "@@iterator";
        function getIteratorFn(maybeIterable) {
          if (maybeIterable === null || typeof maybeIterable !== "object") {
            return null;
          }
          var maybeIterator = MAYBE_ITERATOR_SYMBOL && maybeIterable[MAYBE_ITERATOR_SYMBOL] || maybeIterable[FAUX_ITERATOR_SYMBOL];
          if (typeof maybeIterator === "function") {
            return maybeIterator;
          }
          return null;
        }
        var ReactSharedInternals = React2.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED;
        function error(format) {
          {
            {
              for (var _len2 = arguments.length, args = new Array(_len2 > 1 ? _len2 - 1 : 0), _key2 = 1; _key2 < _len2; _key2++) {
                args[_key2 - 1] = arguments[_key2];
              }
              printWarning("error", format, args);
            }
          }
        }
        function printWarning(level, format, args) {
          {
            var ReactDebugCurrentFrame2 = ReactSharedInternals.ReactDebugCurrentFrame;
            var stack = ReactDebugCurrentFrame2.getStackAddendum();
            if (stack !== "") {
              format += "%s";
              args = args.concat([stack]);
            }
            var argsWithFormat = args.map(function(item) {
              return String(item);
            });
            argsWithFormat.unshift("Warning: " + format);
            Function.prototype.apply.call(console[level], console, argsWithFormat);
          }
        }
        var enableScopeAPI = false;
        var enableCacheElement = false;
        var enableTransitionTracing = false;
        var enableLegacyHidden = false;
        var enableDebugTracing = false;
        var REACT_MODULE_REFERENCE;
        {
          REACT_MODULE_REFERENCE = Symbol.for("react.module.reference");
        }
        function isValidElementType(type) {
          if (typeof type === "string" || typeof type === "function") {
            return true;
          }
          if (type === REACT_FRAGMENT_TYPE || type === REACT_PROFILER_TYPE || enableDebugTracing || type === REACT_STRICT_MODE_TYPE || type === REACT_SUSPENSE_TYPE || type === REACT_SUSPENSE_LIST_TYPE || enableLegacyHidden || type === REACT_OFFSCREEN_TYPE || enableScopeAPI || enableCacheElement || enableTransitionTracing) {
            return true;
          }
          if (typeof type === "object" && type !== null) {
            if (type.$$typeof === REACT_LAZY_TYPE || type.$$typeof === REACT_MEMO_TYPE || type.$$typeof === REACT_PROVIDER_TYPE || type.$$typeof === REACT_CONTEXT_TYPE || type.$$typeof === REACT_FORWARD_REF_TYPE || // This needs to include all possible module reference object
            // types supported by any Flight configuration anywhere since
            // we don't know which Flight build this will end up being used
            // with.
            type.$$typeof === REACT_MODULE_REFERENCE || type.getModuleId !== void 0) {
              return true;
            }
          }
          return false;
        }
        function getWrappedName(outerType, innerType, wrapperName) {
          var displayName = outerType.displayName;
          if (displayName) {
            return displayName;
          }
          var functionName = innerType.displayName || innerType.name || "";
          return functionName !== "" ? wrapperName + "(" + functionName + ")" : wrapperName;
        }
        function getContextName(type) {
          return type.displayName || "Context";
        }
        function getComponentNameFromType(type) {
          if (type == null) {
            return null;
          }
          {
            if (typeof type.tag === "number") {
              error("Received an unexpected object in getComponentNameFromType(). This is likely a bug in React. Please file an issue.");
            }
          }
          if (typeof type === "function") {
            return type.displayName || type.name || null;
          }
          if (typeof type === "string") {
            return type;
          }
          switch (type) {
            case REACT_FRAGMENT_TYPE:
              return "Fragment";
            case REACT_PORTAL_TYPE:
              return "Portal";
            case REACT_PROFILER_TYPE:
              return "Profiler";
            case REACT_STRICT_MODE_TYPE:
              return "StrictMode";
            case REACT_SUSPENSE_TYPE:
              return "Suspense";
            case REACT_SUSPENSE_LIST_TYPE:
              return "SuspenseList";
          }
          if (typeof type === "object") {
            switch (type.$$typeof) {
              case REACT_CONTEXT_TYPE:
                var context = type;
                return getContextName(context) + ".Consumer";
              case REACT_PROVIDER_TYPE:
                var provider = type;
                return getContextName(provider._context) + ".Provider";
              case REACT_FORWARD_REF_TYPE:
                return getWrappedName(type, type.render, "ForwardRef");
              case REACT_MEMO_TYPE:
                var outerName = type.displayName || null;
                if (outerName !== null) {
                  return outerName;
                }
                return getComponentNameFromType(type.type) || "Memo";
              case REACT_LAZY_TYPE: {
                var lazyComponent = type;
                var payload = lazyComponent._payload;
                var init = lazyComponent._init;
                try {
                  return getComponentNameFromType(init(payload));
                } catch (x) {
                  return null;
                }
              }
            }
          }
          return null;
        }
        var assign = Object.assign;
        var disabledDepth = 0;
        var prevLog;
        var prevInfo;
        var prevWarn;
        var prevError;
        var prevGroup;
        var prevGroupCollapsed;
        var prevGroupEnd;
        function disabledLog() {
        }
        disabledLog.__reactDisabledLog = true;
        function disableLogs() {
          {
            if (disabledDepth === 0) {
              prevLog = console.log;
              prevInfo = console.info;
              prevWarn = console.warn;
              prevError = console.error;
              prevGroup = console.group;
              prevGroupCollapsed = console.groupCollapsed;
              prevGroupEnd = console.groupEnd;
              var props = {
                configurable: true,
                enumerable: true,
                value: disabledLog,
                writable: true
              };
              Object.defineProperties(console, {
                info: props,
                log: props,
                warn: props,
                error: props,
                group: props,
                groupCollapsed: props,
                groupEnd: props
              });
            }
            disabledDepth++;
          }
        }
        function reenableLogs() {
          {
            disabledDepth--;
            if (disabledDepth === 0) {
              var props = {
                configurable: true,
                enumerable: true,
                writable: true
              };
              Object.defineProperties(console, {
                log: assign({}, props, {
                  value: prevLog
                }),
                info: assign({}, props, {
                  value: prevInfo
                }),
                warn: assign({}, props, {
                  value: prevWarn
                }),
                error: assign({}, props, {
                  value: prevError
                }),
                group: assign({}, props, {
                  value: prevGroup
                }),
                groupCollapsed: assign({}, props, {
                  value: prevGroupCollapsed
                }),
                groupEnd: assign({}, props, {
                  value: prevGroupEnd
                })
              });
            }
            if (disabledDepth < 0) {
              error("disabledDepth fell below zero. This is a bug in React. Please file an issue.");
            }
          }
        }
        var ReactCurrentDispatcher = ReactSharedInternals.ReactCurrentDispatcher;
        var prefix;
        function describeBuiltInComponentFrame(name, source, ownerFn) {
          {
            if (prefix === void 0) {
              try {
                throw Error();
              } catch (x) {
                var match = x.stack.trim().match(/\n( *(at )?)/);
                prefix = match && match[1] || "";
              }
            }
            return "\n" + prefix + name;
          }
        }
        var reentry = false;
        var componentFrameCache;
        {
          var PossiblyWeakMap = typeof WeakMap === "function" ? WeakMap : Map;
          componentFrameCache = new PossiblyWeakMap();
        }
        function describeNativeComponentFrame(fn, construct) {
          if (!fn || reentry) {
            return "";
          }
          {
            var frame = componentFrameCache.get(fn);
            if (frame !== void 0) {
              return frame;
            }
          }
          var control;
          reentry = true;
          var previousPrepareStackTrace = Error.prepareStackTrace;
          Error.prepareStackTrace = void 0;
          var previousDispatcher;
          {
            previousDispatcher = ReactCurrentDispatcher.current;
            ReactCurrentDispatcher.current = null;
            disableLogs();
          }
          try {
            if (construct) {
              var Fake = function() {
                throw Error();
              };
              Object.defineProperty(Fake.prototype, "props", {
                set: function() {
                  throw Error();
                }
              });
              if (typeof Reflect === "object" && Reflect.construct) {
                try {
                  Reflect.construct(Fake, []);
                } catch (x) {
                  control = x;
                }
                Reflect.construct(fn, [], Fake);
              } else {
                try {
                  Fake.call();
                } catch (x) {
                  control = x;
                }
                fn.call(Fake.prototype);
              }
            } else {
              try {
                throw Error();
              } catch (x) {
                control = x;
              }
              fn();
            }
          } catch (sample) {
            if (sample && control && typeof sample.stack === "string") {
              var sampleLines = sample.stack.split("\n");
              var controlLines = control.stack.split("\n");
              var s = sampleLines.length - 1;
              var c = controlLines.length - 1;
              while (s >= 1 && c >= 0 && sampleLines[s] !== controlLines[c]) {
                c--;
              }
              for (; s >= 1 && c >= 0; s--, c--) {
                if (sampleLines[s] !== controlLines[c]) {
                  if (s !== 1 || c !== 1) {
                    do {
                      s--;
                      c--;
                      if (c < 0 || sampleLines[s] !== controlLines[c]) {
                        var _frame = "\n" + sampleLines[s].replace(" at new ", " at ");
                        if (fn.displayName && _frame.includes("<anonymous>")) {
                          _frame = _frame.replace("<anonymous>", fn.displayName);
                        }
                        {
                          if (typeof fn === "function") {
                            componentFrameCache.set(fn, _frame);
                          }
                        }
                        return _frame;
                      }
                    } while (s >= 1 && c >= 0);
                  }
                  break;
                }
              }
            }
          } finally {
            reentry = false;
            {
              ReactCurrentDispatcher.current = previousDispatcher;
              reenableLogs();
            }
            Error.prepareStackTrace = previousPrepareStackTrace;
          }
          var name = fn ? fn.displayName || fn.name : "";
          var syntheticFrame = name ? describeBuiltInComponentFrame(name) : "";
          {
            if (typeof fn === "function") {
              componentFrameCache.set(fn, syntheticFrame);
            }
          }
          return syntheticFrame;
        }
        function describeFunctionComponentFrame(fn, source, ownerFn) {
          {
            return describeNativeComponentFrame(fn, false);
          }
        }
        function shouldConstruct(Component) {
          var prototype = Component.prototype;
          return !!(prototype && prototype.isReactComponent);
        }
        function describeUnknownElementTypeFrameInDEV(type, source, ownerFn) {
          if (type == null) {
            return "";
          }
          if (typeof type === "function") {
            {
              return describeNativeComponentFrame(type, shouldConstruct(type));
            }
          }
          if (typeof type === "string") {
            return describeBuiltInComponentFrame(type);
          }
          switch (type) {
            case REACT_SUSPENSE_TYPE:
              return describeBuiltInComponentFrame("Suspense");
            case REACT_SUSPENSE_LIST_TYPE:
              return describeBuiltInComponentFrame("SuspenseList");
          }
          if (typeof type === "object") {
            switch (type.$$typeof) {
              case REACT_FORWARD_REF_TYPE:
                return describeFunctionComponentFrame(type.render);
              case REACT_MEMO_TYPE:
                return describeUnknownElementTypeFrameInDEV(type.type, source, ownerFn);
              case REACT_LAZY_TYPE: {
                var lazyComponent = type;
                var payload = lazyComponent._payload;
                var init = lazyComponent._init;
                try {
                  return describeUnknownElementTypeFrameInDEV(init(payload), source, ownerFn);
                } catch (x) {
                }
              }
            }
          }
          return "";
        }
        var hasOwnProperty = Object.prototype.hasOwnProperty;
        var loggedTypeFailures = {};
        var ReactDebugCurrentFrame = ReactSharedInternals.ReactDebugCurrentFrame;
        function setCurrentlyValidatingElement(element) {
          {
            if (element) {
              var owner = element._owner;
              var stack = describeUnknownElementTypeFrameInDEV(element.type, element._source, owner ? owner.type : null);
              ReactDebugCurrentFrame.setExtraStackFrame(stack);
            } else {
              ReactDebugCurrentFrame.setExtraStackFrame(null);
            }
          }
        }
        function checkPropTypes(typeSpecs, values, location, componentName, element) {
          {
            var has = Function.call.bind(hasOwnProperty);
            for (var typeSpecName in typeSpecs) {
              if (has(typeSpecs, typeSpecName)) {
                var error$1 = void 0;
                try {
                  if (typeof typeSpecs[typeSpecName] !== "function") {
                    var err = Error((componentName || "React class") + ": " + location + " type `" + typeSpecName + "` is invalid; it must be a function, usually from the `prop-types` package, but received `" + typeof typeSpecs[typeSpecName] + "`.This often happens because of typos such as `PropTypes.function` instead of `PropTypes.func`.");
                    err.name = "Invariant Violation";
                    throw err;
                  }
                  error$1 = typeSpecs[typeSpecName](values, typeSpecName, componentName, location, null, "SECRET_DO_NOT_PASS_THIS_OR_YOU_WILL_BE_FIRED");
                } catch (ex) {
                  error$1 = ex;
                }
                if (error$1 && !(error$1 instanceof Error)) {
                  setCurrentlyValidatingElement(element);
                  error("%s: type specification of %s `%s` is invalid; the type checker function must return `null` or an `Error` but returned a %s. You may have forgotten to pass an argument to the type checker creator (arrayOf, instanceOf, objectOf, oneOf, oneOfType, and shape all require an argument).", componentName || "React class", location, typeSpecName, typeof error$1);
                  setCurrentlyValidatingElement(null);
                }
                if (error$1 instanceof Error && !(error$1.message in loggedTypeFailures)) {
                  loggedTypeFailures[error$1.message] = true;
                  setCurrentlyValidatingElement(element);
                  error("Failed %s type: %s", location, error$1.message);
                  setCurrentlyValidatingElement(null);
                }
              }
            }
          }
        }
        var isArrayImpl = Array.isArray;
        function isArray(a) {
          return isArrayImpl(a);
        }
        function typeName(value) {
          {
            var hasToStringTag = typeof Symbol === "function" && Symbol.toStringTag;
            var type = hasToStringTag && value[Symbol.toStringTag] || value.constructor.name || "Object";
            return type;
          }
        }
        function willCoercionThrow(value) {
          {
            try {
              testStringCoercion(value);
              return false;
            } catch (e) {
              return true;
            }
          }
        }
        function testStringCoercion(value) {
          return "" + value;
        }
        function checkKeyStringCoercion(value) {
          {
            if (willCoercionThrow(value)) {
              error("The provided key is an unsupported type %s. This value must be coerced to a string before before using it here.", typeName(value));
              return testStringCoercion(value);
            }
          }
        }
        var ReactCurrentOwner = ReactSharedInternals.ReactCurrentOwner;
        var RESERVED_PROPS = {
          key: true,
          ref: true,
          __self: true,
          __source: true
        };
        var specialPropKeyWarningShown;
        var specialPropRefWarningShown;
        function hasValidRef(config) {
          {
            if (hasOwnProperty.call(config, "ref")) {
              var getter = Object.getOwnPropertyDescriptor(config, "ref").get;
              if (getter && getter.isReactWarning) {
                return false;
              }
            }
          }
          return config.ref !== void 0;
        }
        function hasValidKey(config) {
          {
            if (hasOwnProperty.call(config, "key")) {
              var getter = Object.getOwnPropertyDescriptor(config, "key").get;
              if (getter && getter.isReactWarning) {
                return false;
              }
            }
          }
          return config.key !== void 0;
        }
        function warnIfStringRefCannotBeAutoConverted(config, self) {
          {
            if (typeof config.ref === "string" && ReactCurrentOwner.current && self) ;
          }
        }
        function defineKeyPropWarningGetter(props, displayName) {
          {
            var warnAboutAccessingKey = function() {
              if (!specialPropKeyWarningShown) {
                specialPropKeyWarningShown = true;
                error("%s: `key` is not a prop. Trying to access it will result in `undefined` being returned. If you need to access the same value within the child component, you should pass it as a different prop. (https://reactjs.org/link/special-props)", displayName);
              }
            };
            warnAboutAccessingKey.isReactWarning = true;
            Object.defineProperty(props, "key", {
              get: warnAboutAccessingKey,
              configurable: true
            });
          }
        }
        function defineRefPropWarningGetter(props, displayName) {
          {
            var warnAboutAccessingRef = function() {
              if (!specialPropRefWarningShown) {
                specialPropRefWarningShown = true;
                error("%s: `ref` is not a prop. Trying to access it will result in `undefined` being returned. If you need to access the same value within the child component, you should pass it as a different prop. (https://reactjs.org/link/special-props)", displayName);
              }
            };
            warnAboutAccessingRef.isReactWarning = true;
            Object.defineProperty(props, "ref", {
              get: warnAboutAccessingRef,
              configurable: true
            });
          }
        }
        var ReactElement = function(type, key, ref, self, source, owner, props) {
          var element = {
            // This tag allows us to uniquely identify this as a React Element
            $$typeof: REACT_ELEMENT_TYPE,
            // Built-in properties that belong on the element
            type,
            key,
            ref,
            props,
            // Record the component responsible for creating this element.
            _owner: owner
          };
          {
            element._store = {};
            Object.defineProperty(element._store, "validated", {
              configurable: false,
              enumerable: false,
              writable: true,
              value: false
            });
            Object.defineProperty(element, "_self", {
              configurable: false,
              enumerable: false,
              writable: false,
              value: self
            });
            Object.defineProperty(element, "_source", {
              configurable: false,
              enumerable: false,
              writable: false,
              value: source
            });
            if (Object.freeze) {
              Object.freeze(element.props);
              Object.freeze(element);
            }
          }
          return element;
        };
        function jsxDEV(type, config, maybeKey, source, self) {
          {
            var propName;
            var props = {};
            var key = null;
            var ref = null;
            if (maybeKey !== void 0) {
              {
                checkKeyStringCoercion(maybeKey);
              }
              key = "" + maybeKey;
            }
            if (hasValidKey(config)) {
              {
                checkKeyStringCoercion(config.key);
              }
              key = "" + config.key;
            }
            if (hasValidRef(config)) {
              ref = config.ref;
              warnIfStringRefCannotBeAutoConverted(config, self);
            }
            for (propName in config) {
              if (hasOwnProperty.call(config, propName) && !RESERVED_PROPS.hasOwnProperty(propName)) {
                props[propName] = config[propName];
              }
            }
            if (type && type.defaultProps) {
              var defaultProps = type.defaultProps;
              for (propName in defaultProps) {
                if (props[propName] === void 0) {
                  props[propName] = defaultProps[propName];
                }
              }
            }
            if (key || ref) {
              var displayName = typeof type === "function" ? type.displayName || type.name || "Unknown" : type;
              if (key) {
                defineKeyPropWarningGetter(props, displayName);
              }
              if (ref) {
                defineRefPropWarningGetter(props, displayName);
              }
            }
            return ReactElement(type, key, ref, self, source, ReactCurrentOwner.current, props);
          }
        }
        var ReactCurrentOwner$1 = ReactSharedInternals.ReactCurrentOwner;
        var ReactDebugCurrentFrame$1 = ReactSharedInternals.ReactDebugCurrentFrame;
        function setCurrentlyValidatingElement$1(element) {
          {
            if (element) {
              var owner = element._owner;
              var stack = describeUnknownElementTypeFrameInDEV(element.type, element._source, owner ? owner.type : null);
              ReactDebugCurrentFrame$1.setExtraStackFrame(stack);
            } else {
              ReactDebugCurrentFrame$1.setExtraStackFrame(null);
            }
          }
        }
        var propTypesMisspellWarningShown;
        {
          propTypesMisspellWarningShown = false;
        }
        function isValidElement(object) {
          {
            return typeof object === "object" && object !== null && object.$$typeof === REACT_ELEMENT_TYPE;
          }
        }
        function getDeclarationErrorAddendum() {
          {
            if (ReactCurrentOwner$1.current) {
              var name = getComponentNameFromType(ReactCurrentOwner$1.current.type);
              if (name) {
                return "\n\nCheck the render method of `" + name + "`.";
              }
            }
            return "";
          }
        }
        function getSourceInfoErrorAddendum(source) {
          {
            return "";
          }
        }
        var ownerHasKeyUseWarning = {};
        function getCurrentComponentErrorInfo(parentType) {
          {
            var info = getDeclarationErrorAddendum();
            if (!info) {
              var parentName = typeof parentType === "string" ? parentType : parentType.displayName || parentType.name;
              if (parentName) {
                info = "\n\nCheck the top-level render call using <" + parentName + ">.";
              }
            }
            return info;
          }
        }
        function validateExplicitKey(element, parentType) {
          {
            if (!element._store || element._store.validated || element.key != null) {
              return;
            }
            element._store.validated = true;
            var currentComponentErrorInfo = getCurrentComponentErrorInfo(parentType);
            if (ownerHasKeyUseWarning[currentComponentErrorInfo]) {
              return;
            }
            ownerHasKeyUseWarning[currentComponentErrorInfo] = true;
            var childOwner = "";
            if (element && element._owner && element._owner !== ReactCurrentOwner$1.current) {
              childOwner = " It was passed a child from " + getComponentNameFromType(element._owner.type) + ".";
            }
            setCurrentlyValidatingElement$1(element);
            error('Each child in a list should have a unique "key" prop.%s%s See https://reactjs.org/link/warning-keys for more information.', currentComponentErrorInfo, childOwner);
            setCurrentlyValidatingElement$1(null);
          }
        }
        function validateChildKeys(node, parentType) {
          {
            if (typeof node !== "object") {
              return;
            }
            if (isArray(node)) {
              for (var i = 0; i < node.length; i++) {
                var child = node[i];
                if (isValidElement(child)) {
                  validateExplicitKey(child, parentType);
                }
              }
            } else if (isValidElement(node)) {
              if (node._store) {
                node._store.validated = true;
              }
            } else if (node) {
              var iteratorFn = getIteratorFn(node);
              if (typeof iteratorFn === "function") {
                if (iteratorFn !== node.entries) {
                  var iterator = iteratorFn.call(node);
                  var step;
                  while (!(step = iterator.next()).done) {
                    if (isValidElement(step.value)) {
                      validateExplicitKey(step.value, parentType);
                    }
                  }
                }
              }
            }
          }
        }
        function validatePropTypes(element) {
          {
            var type = element.type;
            if (type === null || type === void 0 || typeof type === "string") {
              return;
            }
            var propTypes;
            if (typeof type === "function") {
              propTypes = type.propTypes;
            } else if (typeof type === "object" && (type.$$typeof === REACT_FORWARD_REF_TYPE || // Note: Memo only checks outer props here.
            // Inner props are checked in the reconciler.
            type.$$typeof === REACT_MEMO_TYPE)) {
              propTypes = type.propTypes;
            } else {
              return;
            }
            if (propTypes) {
              var name = getComponentNameFromType(type);
              checkPropTypes(propTypes, element.props, "prop", name, element);
            } else if (type.PropTypes !== void 0 && !propTypesMisspellWarningShown) {
              propTypesMisspellWarningShown = true;
              var _name = getComponentNameFromType(type);
              error("Component %s declared `PropTypes` instead of `propTypes`. Did you misspell the property assignment?", _name || "Unknown");
            }
            if (typeof type.getDefaultProps === "function" && !type.getDefaultProps.isReactClassApproved) {
              error("getDefaultProps is only used on classic React.createClass definitions. Use a static property named `defaultProps` instead.");
            }
          }
        }
        function validateFragmentProps(fragment) {
          {
            var keys = Object.keys(fragment.props);
            for (var i = 0; i < keys.length; i++) {
              var key = keys[i];
              if (key !== "children" && key !== "key") {
                setCurrentlyValidatingElement$1(fragment);
                error("Invalid prop `%s` supplied to `React.Fragment`. React.Fragment can only have `key` and `children` props.", key);
                setCurrentlyValidatingElement$1(null);
                break;
              }
            }
            if (fragment.ref !== null) {
              setCurrentlyValidatingElement$1(fragment);
              error("Invalid attribute `ref` supplied to `React.Fragment`.");
              setCurrentlyValidatingElement$1(null);
            }
          }
        }
        var didWarnAboutKeySpread = {};
        function jsxWithValidation(type, props, key, isStaticChildren, source, self) {
          {
            var validType = isValidElementType(type);
            if (!validType) {
              var info = "";
              if (type === void 0 || typeof type === "object" && type !== null && Object.keys(type).length === 0) {
                info += " You likely forgot to export your component from the file it's defined in, or you might have mixed up default and named imports.";
              }
              var sourceInfo = getSourceInfoErrorAddendum();
              if (sourceInfo) {
                info += sourceInfo;
              } else {
                info += getDeclarationErrorAddendum();
              }
              var typeString;
              if (type === null) {
                typeString = "null";
              } else if (isArray(type)) {
                typeString = "array";
              } else if (type !== void 0 && type.$$typeof === REACT_ELEMENT_TYPE) {
                typeString = "<" + (getComponentNameFromType(type.type) || "Unknown") + " />";
                info = " Did you accidentally export a JSX literal instead of a component?";
              } else {
                typeString = typeof type;
              }
              error("React.jsx: type is invalid -- expected a string (for built-in components) or a class/function (for composite components) but got: %s.%s", typeString, info);
            }
            var element = jsxDEV(type, props, key, source, self);
            if (element == null) {
              return element;
            }
            if (validType) {
              var children = props.children;
              if (children !== void 0) {
                if (isStaticChildren) {
                  if (isArray(children)) {
                    for (var i = 0; i < children.length; i++) {
                      validateChildKeys(children[i], type);
                    }
                    if (Object.freeze) {
                      Object.freeze(children);
                    }
                  } else {
                    error("React.jsx: Static children should always be an array. You are likely explicitly calling React.jsxs or React.jsxDEV. Use the Babel transform instead.");
                  }
                } else {
                  validateChildKeys(children, type);
                }
              }
            }
            {
              if (hasOwnProperty.call(props, "key")) {
                var componentName = getComponentNameFromType(type);
                var keys = Object.keys(props).filter(function(k) {
                  return k !== "key";
                });
                var beforeExample = keys.length > 0 ? "{key: someKey, " + keys.join(": ..., ") + ": ...}" : "{key: someKey}";
                if (!didWarnAboutKeySpread[componentName + beforeExample]) {
                  var afterExample = keys.length > 0 ? "{" + keys.join(": ..., ") + ": ...}" : "{}";
                  error('A props object containing a "key" prop is being spread into JSX:\n  let props = %s;\n  <%s {...props} />\nReact keys must be passed directly to JSX without using spread:\n  let props = %s;\n  <%s key={someKey} {...props} />', beforeExample, componentName, afterExample, componentName);
                  didWarnAboutKeySpread[componentName + beforeExample] = true;
                }
              }
            }
            if (type === REACT_FRAGMENT_TYPE) {
              validateFragmentProps(element);
            } else {
              validatePropTypes(element);
            }
            return element;
          }
        }
        function jsxWithValidationStatic(type, props, key) {
          {
            return jsxWithValidation(type, props, key, true);
          }
        }
        function jsxWithValidationDynamic(type, props, key) {
          {
            return jsxWithValidation(type, props, key, false);
          }
        }
        var jsx = jsxWithValidationDynamic;
        var jsxs = jsxWithValidationStatic;
        reactJsxRuntime_development.Fragment = REACT_FRAGMENT_TYPE;
        reactJsxRuntime_development.jsx = jsx;
        reactJsxRuntime_development.jsxs = jsxs;
      })();
    }
    return reactJsxRuntime_development;
  }
  var hasRequiredJsxRuntime;
  function requireJsxRuntime() {
    if (hasRequiredJsxRuntime) return jsxRuntime.exports;
    hasRequiredJsxRuntime = 1;
    if (process.env.NODE_ENV === "production") {
      jsxRuntime.exports = requireReactJsxRuntime_production_min();
    } else {
      jsxRuntime.exports = requireReactJsxRuntime_development();
    }
    return jsxRuntime.exports;
  }
  var jsxRuntimeExports = requireJsxRuntime();
  function Alert({ children, variant = "default", className = "", role = "alert", ...props }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx(
      "div",
      {
        role,
        className: ["rq-alert", variant !== "default" ? `rq-alert--${variant}` : "", className].filter(Boolean).join(" "),
        ...props,
        children
      }
    );
  }
  function Badge({ children, variant = "default", className = "", ...props }) {
    const cls = [
      "rq-badge",
      variant !== "default" ? `rq-badge--${variant}` : "",
      className
    ].filter(Boolean).join(" ");
    return /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: cls, ...props, children: variant === "dot" ? null : children });
  }
  const BUTTON_TONE_CLASSES = {
    primary: "rq-btn--primary",
    accent: "rq-btn--accent",
    secondary: "rq-btn--secondary",
    ghost: "rq-btn--ghost",
    danger: "rq-btn--danger",
    link: "rq-btn--link"
  };
  const BUTTON_SIZE_CLASSES = {
    sm: "rq-btn--sm",
    md: "",
    lg: "rq-btn--lg"
  };
  function Button({
    children,
    tone = "secondary",
    size = "md",
    iconLeft,
    iconRight,
    block = false,
    className = "",
    type = "button",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsxs(
      "button",
      {
        type,
        className: [
          "rq-btn",
          BUTTON_TONE_CLASSES[tone],
          BUTTON_SIZE_CLASSES[size],
          block ? "rq-btn--block" : "",
          className
        ].filter(Boolean).join(" "),
        ...props,
        children: [
          iconLeft ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-btn__icon", children: iconLeft }) : null,
          children ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-btn__label", children }) : null,
          iconRight ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-btn__icon", children: iconRight }) : null
        ]
      }
    );
  }
  function Card({
    children,
    interactive = false,
    selected = false,
    padded = true,
    accentColor,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsxs(
      "div",
      {
        className: [
          "rq-card",
          padded ? "rq-card--pad" : "",
          interactive ? "rq-card--interactive" : "",
          selected ? "rq-card--selected" : "",
          className
        ].filter(Boolean).join(" "),
        ...props,
        children: [
          accentColor ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-card__accent", style: { background: accentColor } }) : null,
          children
        ]
      }
    );
  }
  const PATHS = {
    search: '<circle cx="11" cy="11" r="8"/><path d="m21 21-4.3-4.3"/>',
    x: '<path d="M18 6 6 18"/><path d="m6 6 12 12"/>',
    plus: '<path d="M5 12h14"/><path d="M12 5v14"/>',
    minus: '<path d="M5 12h14"/>',
    check: '<path d="M20 6 9 17l-5-5"/>',
    "chevron-right": '<path d="m9 18 6-6-6-6"/>',
    "chevron-left": '<path d="m15 18-6-6 6-6"/>',
    "chevron-down": '<path d="m6 9 6 6 6-6"/>',
    "chevron-up": '<path d="m18 15-6-6-6 6"/>',
    "arrow-up-right": '<path d="M7 7h10v10"/><path d="M7 17 17 7"/>',
    "alert-triangle": '<path d="m21.73 18-8-14a2 2 0 0 0-3.46 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"/><path d="M12 9v4"/><path d="M12 17h.01"/>',
    archive: '<rect width="20" height="5" x="2" y="3" rx="1"/><path d="M4 8v11a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8"/><path d="M10 12h4"/>',
    folder: '<path d="M20 20a2 2 0 0 0 2-2V8a2 2 0 0 0-2-2h-7.9a2 2 0 0 1-1.69-.9L9.6 3.9A2 2 0 0 0 7.93 3H4a2 2 0 0 0-2 2v13a2 2 0 0 0 2 2Z"/>',
    "folder-open": '<path d="m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2"/>',
    file: '<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/>',
    "file-text": '<path d="M15 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7Z"/><path d="M14 2v4a2 2 0 0 0 2 2h4"/><path d="M16 13H8"/><path d="M16 17H8"/><path d="M10 9H8"/>',
    "external-link": '<path d="M15 3h6v6"/><path d="M10 14 21 3"/><path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"/>',
    box: '<path d="M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z"/><path d="m3.3 7 8.7 5 8.7-5"/><path d="M12 22V12"/>',
    database: '<ellipse cx="12" cy="5" rx="9" ry="3"/><path d="M3 5v14a9 3 0 0 0 18 0V5"/><path d="M3 12a9 3 0 0 0 18 0"/>',
    tags: '<path d="m15 5 6.3 6.3a2.4 2.4 0 0 1 0 3.4l-5.6 5.6a2.4 2.4 0 0 1-3.4 0L6 14V5z"/><path d="M9.5 9.5h.01"/><path d="m3 7 8.6 8.6"/>',
    network: '<circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><path d="m8.6 13.5 6.8 3.9"/><path d="m15.4 6.5-6.8 3.9"/>',
    globe: '<circle cx="12" cy="12" r="10"/><path d="M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20"/><path d="M2 12h20"/>',
    activity: '<path d="M22 12h-4l-3 9L9 3l-3 9H2"/>',
    grid: '<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/>',
    "layout-grid": '<rect width="7" height="7" x="3" y="3" rx="1"/><rect width="7" height="7" x="14" y="3" rx="1"/><rect width="7" height="7" x="14" y="14" rx="1"/><rect width="7" height="7" x="3" y="14" rx="1"/>',
    list: '<path d="M8 6h13"/><path d="M8 12h13"/><path d="M8 18h13"/><path d="M3 6h.01"/><path d="M3 12h.01"/><path d="M3 18h.01"/>',
    "layout-list": '<rect width="7" height="7" x="3" y="3" rx="1"/><path d="M14 4h7"/><path d="M14 9h7"/><rect width="7" height="7" x="3" y="14" rx="1"/><path d="M14 15h7"/><path d="M14 20h7"/>',
    table: '<path d="M12 3v18"/><rect width="18" height="18" x="3" y="3" rx="2"/><path d="M3 9h18"/><path d="M3 15h18"/>',
    "pie-chart": '<path d="M21 12c.6 0 1-.4.9-1A10 10 0 0 0 13 2.1c-.6-.1-1 .4-1 .9v8a1 1 0 0 0 1 1z"/><path d="M21.2 15a10 10 0 1 1-12.2-12"/>',
    "rotate-ccw": '<path d="M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.7 2.7L3 8"/><path d="M3 3v5h5"/>',
    settings: '<path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/>',
    sliders: '<path d="M4 21v-7"/><path d="M4 10V3"/><path d="M12 21v-9"/><path d="M12 8V3"/><path d="M20 21v-5"/><path d="M20 12V3"/><path d="M2 14h4"/><path d="M10 8h4"/><path d="M18 16h4"/>',
    download: '<path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><path d="m7 10 5 5 5-5"/><path d="M12 15V3"/>',
    "git-branch": '<path d="M6 3v12"/><circle cx="18" cy="6" r="3"/><circle cx="6" cy="18" r="3"/><path d="M18 9a9 9 0 0 1-9 9"/>',
    target: '<circle cx="12" cy="12" r="10"/><circle cx="12" cy="12" r="6"/><circle cx="12" cy="12" r="2"/>',
    sun: '<circle cx="12" cy="12" r="4"/><path d="M12 2v2"/><path d="M12 20v2"/><path d="m4.93 4.93 1.41 1.41"/><path d="m17.66 17.66 1.41 1.41"/><path d="M2 12h2"/><path d="M20 12h2"/><path d="m6.34 17.66-1.41 1.41"/><path d="m19.07 4.93-1.41 1.41"/>',
    moon: '<path d="M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z"/>',
    "help-circle": '<circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><path d="M12 17h.01"/>',
    filter: '<polygon points="22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3"/>',
    diamond: '<path d="M2.7 10.3a2.41 2.41 0 0 0 0 3.41l7.59 7.59a2.41 2.41 0 0 0 3.41 0l7.59-7.59a2.41 2.41 0 0 0 0-3.41L13.7 2.71a2.41 2.41 0 0 0-3.41 0Z"/>',
    layers: '<path d="m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z"/><path d="m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65"/><path d="m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65"/>',
    link: '<path d="M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71"/><path d="M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71"/>',
    copy: '<rect width="14" height="14" x="8" y="8" rx="2"/><path d="M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2"/>',
    "wrap-text": '<path d="M3 6h18"/><path d="M3 12h15a3 3 0 1 1 0 6h-4"/><path d="m16 16-2 2 2 2"/><path d="M3 18h7"/>',
    circle: '<circle cx="12" cy="12" r="10"/>'
  };
  const ICON_NAMES = Object.keys(PATHS);
  function Icon({ name, size = 18, strokeWidth = 2, className = "", ...props }) {
    const inner = PATHS[name];
    return /* @__PURE__ */ jsxRuntimeExports.jsx(
      "svg",
      {
        xmlns: "http://www.w3.org/2000/svg",
        width: size,
        height: size,
        viewBox: "0 0 24 24",
        fill: "none",
        stroke: "currentColor",
        strokeWidth,
        strokeLinecap: "round",
        strokeLinejoin: "round",
        className,
        "aria-hidden": "true",
        dangerouslySetInnerHTML: { __html: inner },
        ...props
      }
    );
  }
  function IconButton({
    children,
    size = "md",
    tone = "secondary",
    active = false,
    className = "",
    type = "button",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx(
      "button",
      {
        type,
        className: [
          "rq-iconbtn",
          size === "sm" ? "rq-iconbtn--sm" : "",
          tone === "ghost" ? "rq-iconbtn--ghost" : "",
          active ? "is-active" : "",
          className
        ].filter(Boolean).join(" "),
        "aria-pressed": active || void 0,
        ...props,
        children
      }
    );
  }
  const ModalContext = require$$0.createContext(null);
  const ModalContentContext = require$$0.createContext(null);
  function Modal({
    open,
    onOpenChange,
    children
  }) {
    require$$0.useEffect(() => {
      if (!open) return void 0;
      const onKeyDown = (event) => {
        if (event.key === "Escape") onOpenChange == null ? void 0 : onOpenChange(false);
      };
      document.addEventListener("keydown", onKeyDown);
      return () => document.removeEventListener("keydown", onKeyDown);
    }, [onOpenChange, open]);
    return /* @__PURE__ */ jsxRuntimeExports.jsx(ModalContext.Provider, { value: { open, onOpenChange }, children });
  }
  function ModalContent({
    className = "",
    children,
    showCloseButton: _showCloseButton,
    "aria-describedby": ariaDescribedBy,
    ...props
  }) {
    const context = require$$0.useContext(ModalContext);
    if (!(context == null ? void 0 : context.open)) return null;
    return reactDom.createPortal(
      /* @__PURE__ */ jsxRuntimeExports.jsx(ModalContentContext.Provider, { value: context, children: /* @__PURE__ */ jsxRuntimeExports.jsx(
        "div",
        {
          className: "ex-modal-scrim",
          role: "presentation",
          onMouseDown: (event) => {
            var _a;
            if (event.target === event.currentTarget) (_a = context.onOpenChange) == null ? void 0 : _a.call(context, false);
          },
          children: /* @__PURE__ */ jsxRuntimeExports.jsx(
            "div",
            {
              className: ["ex-modal", className].filter(Boolean).join(" "),
              role: "dialog",
              "aria-modal": "true",
              "aria-describedby": ariaDescribedBy,
              tabIndex: -1,
              ...props,
              children
            }
          )
        }
      ) }),
      document.body
    );
  }
  function ModalHeader({
    className = "",
    children,
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("div", { className: ["ex-modal__head", className].filter(Boolean).join(" "), ...props, children });
  }
  function ModalBody({
    className = "",
    children,
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("div", { className: ["ex-modal__body", className].filter(Boolean).join(" "), ...props, children });
  }
  function ModalFooter({
    className = "",
    children,
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("div", { className: ["ex-modal__foot", className].filter(Boolean).join(" "), ...props, children });
  }
  function ModalTitle({
    className = "",
    children,
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("h2", { className, ...props, children });
  }
  function ModalDescription({
    className = "",
    children,
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("p", { className, ...props, children });
  }
  function ModalClose({
    asChild = false,
    children,
    onClick,
    ...props
  }) {
    const context = require$$0.useContext(ModalContentContext);
    const close = () => {
      var _a;
      return (_a = context == null ? void 0 : context.onOpenChange) == null ? void 0 : _a.call(context, false);
    };
    if (asChild && require$$0.isValidElement(children)) {
      const child = children;
      return require$$0.cloneElement(child, {
        ...props,
        onClick: (event) => {
          var _a, _b;
          (_b = (_a = child.props).onClick) == null ? void 0 : _b.call(_a, event);
          onClick == null ? void 0 : onClick(event);
          if (!event.defaultPrevented) close();
        }
      });
    }
    return /* @__PURE__ */ jsxRuntimeExports.jsx(
      "button",
      {
        type: "button",
        onClick: (event) => {
          onClick == null ? void 0 : onClick(event);
          if (!event.defaultPrevented) close();
        },
        ...props,
        children
      }
    );
  }
  function SearchInput({
    size = "md",
    icon,
    kbd,
    className = "",
    type = "text",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsxs("div", { className: ["rq-search", size === "lg" ? "rq-search--lg" : "", className].filter(Boolean).join(" "), children: [
      /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-search__icon", "aria-hidden": "true", children: icon ?? /* @__PURE__ */ jsxRuntimeExports.jsxs("svg", { viewBox: "0 0 24 24", fill: "none", stroke: "currentColor", strokeWidth: "2", strokeLinecap: "round", strokeLinejoin: "round", children: [
        /* @__PURE__ */ jsxRuntimeExports.jsx("circle", { cx: "11", cy: "11", r: "8" }),
        /* @__PURE__ */ jsxRuntimeExports.jsx("path", { d: "m21 21-4.3-4.3" })
      ] }) }),
      /* @__PURE__ */ jsxRuntimeExports.jsx("input", { className: "rq-search__input", type, ...props }),
      kbd ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-search__kbd", children: kbd }) : null
    ] });
  }
  function SegmentedControl({
    items,
    value,
    onChange,
    className = "",
    ariaLabel
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("div", { className: ["rq-segmented", className].filter(Boolean).join(" "), role: "group", "aria-label": ariaLabel, children: items.map((item) => {
      const active = item.value === value;
      return /* @__PURE__ */ jsxRuntimeExports.jsxs(
        "button",
        {
          type: "button",
          "aria-pressed": active,
          className: ["rq-segmented__item", active ? "is-active" : ""].filter(Boolean).join(" "),
          onClick: () => onChange(item.value),
          children: [
            item.icon ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-segmented__icon", children: item.icon }) : null,
            item.label ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { children: item.label }) : null
          ]
        },
        item.value
      );
    }) });
  }
  function Tabs({
    items = [],
    value,
    onChange,
    variant = "underline",
    className = "",
    ...props
  }) {
    const cls = ["rq-tabs", `rq-tabs--${variant}`, className].filter(Boolean).join(" ");
    return /* @__PURE__ */ jsxRuntimeExports.jsx("div", { className: cls, role: "tablist", ...props, children: items.map((it) => {
      const active = it.value === value;
      return /* @__PURE__ */ jsxRuntimeExports.jsxs(
        "button",
        {
          type: "button",
          role: "tab",
          "aria-selected": active,
          className: ["rq-tab", active ? "is-active" : ""].filter(Boolean).join(" "),
          onClick: () => onChange == null ? void 0 : onChange(it.value),
          children: [
            it.icon ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-tab__icon", children: it.icon }) : null,
            /* @__PURE__ */ jsxRuntimeExports.jsx("span", { children: it.label ?? it.value }),
            it.badge != null ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-tab__badge", children: it.badge }) : null
          ]
        },
        it.value
      );
    }) });
  }
  function ToggleRow({
    label,
    color,
    icon,
    meta,
    on = true,
    line = false,
    className = "",
    onToggle,
    ...props
  }) {
    const swatchStyle = color ? line ? { borderColor: color } : { background: color, borderColor: color } : void 0;
    return /* @__PURE__ */ jsxRuntimeExports.jsxs(
      "button",
      {
        type: "button",
        className: [
          "rq-togglerow",
          line ? "rq-togglerow--line" : "",
          on ? "" : "is-off",
          className
        ].filter(Boolean).join(" "),
        "aria-pressed": on,
        onClick: onToggle,
        ...props,
        children: [
          icon ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-togglerow__icon", children: icon }) : /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-togglerow__swatch", style: swatchStyle }),
          /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-togglerow__label", children: label }),
          meta != null ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-togglerow__meta", children: meta }) : null
        ]
      }
    );
  }
  function Chip({
    active = false,
    icon,
    count,
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsxs(
      "button",
      {
        type: "button",
        "aria-pressed": active,
        className: ["rq-chip", active ? "is-active" : "", className].filter(Boolean).join(" "),
        ...props,
        children: [
          icon,
          /* @__PURE__ */ jsxRuntimeExports.jsx("span", { children }),
          count != null ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-chip__count", children: count }) : null
        ]
      }
    );
  }
  const ELEMENT_ROLE_TOKENS = {
    capability: { fill: "--capability", ink: "--capability-ink", tint: "--capability-tint" },
    requirement: { fill: "--requirement", ink: "--requirement-ink", tint: "--requirement-tint" },
    refinement: { fill: "--refinement", ink: "--refinement-ink", tint: "--refinement-tint" },
    source: { fill: "--refinement", ink: "--refinement-ink", tint: "--refinement-tint" },
    constraint: { fill: "--refinement", ink: "--refinement-ink", tint: "--refinement-tint" },
    behavior: { fill: "--refinement", ink: "--refinement-ink", tint: "--refinement-tint" },
    state: { fill: "--refinement", ink: "--refinement-ink", tint: "--refinement-tint" },
    "input-output": { fill: "--refinement", ink: "--refinement-ink", tint: "--refinement-tint" },
    verification: { fill: "--verification", ink: "--verification-ink", tint: "--verification-tint" },
    specification: { fill: "--refinement", ink: "--refinement-ink", tint: "--refinement-tint" },
    "semantic-contract": { fill: "--refinement", ink: "--refinement-ink", tint: "--refinement-tint" },
    ontology: { fill: "--ontology", ink: "--ontology-ink", tint: "--ontology-tint" },
    resource: { fill: "--resource", ink: "--resource-ink", tint: "--resource-tint" },
    other: { fill: "--other", ink: "--other-ink", tint: "--other-tint" }
  };
  const ELEMENT_TYPES = {
    capability: { color: "var(--capability)", shape: "hub", role: "capability" },
    requirement: { color: "var(--requirement)", shape: "square", role: "requirement" },
    refinement: { color: "var(--refinement)", shape: "diamond", role: "refinement", glyph: "R" },
    source: { color: "var(--refinement)", shape: "diamond", role: "source", glyph: "↗" },
    constraint: { color: "var(--refinement)", shape: "diamond", role: "constraint", glyph: "!" },
    behavior: { color: "var(--refinement)", shape: "diamond", role: "behavior", glyph: "→" },
    state: { color: "var(--refinement)", shape: "diamond", role: "state", glyph: "●" },
    "input-output": { color: "var(--refinement)", shape: "diamond", role: "input-output", glyph: "↔" },
    verification: { color: "var(--verification)", shape: "square", role: "verification" },
    specification: { color: "var(--refinement)", shape: "diamond", role: "specification", glyph: "≡" },
    "semantic-contract": { color: "var(--refinement)", shape: "diamond", role: "semantic-contract", glyph: "SH" },
    ontology: { color: "var(--ontology)", shape: "square", role: "ontology" },
    resource: { color: "var(--resource)", shape: "square", role: "resource" }
  };
  const DESIGN_SYSTEM_COLOR_TOKENS = [
    "--accent",
    "--accent-ring",
    "--bg-canvas",
    "--bg-sunken",
    "--bg-surface",
    "--border-default",
    "--edge-attach",
    "--edge-default",
    "--edge-derive",
    "--edge-satisfy",
    "--edge-trace",
    "--node-generic-fill",
    "--ontology-ink",
    "--rdf-class",
    "--rdf-classexpr",
    "--rdf-datatype",
    "--rdf-dtprop",
    "--rdf-individual",
    "--rdf-nodeshape",
    "--rdf-objprop",
    "--rdf-propshape",
    "--rdf-rdfprop",
    "--rdf-resource",
    "--rdf-restriction",
    "--rdf-shacl",
    "--requirement-ink",
    "--slate-0",
    "--slate-950",
    "--success",
    "--text-body",
    "--text-faint",
    "--text-muted",
    "--text-strong"
  ];
  const CSS_TOKEN_FALLBACKS = {
    "--capability": "#bbdefb",
    "--capability-ink": "#1565c0",
    "--capability-tint": "#e3f2fd",
    "--requirement": "#673ab7",
    "--requirement-ink": "#512da8",
    "--requirement-tint": "#ede7f6",
    "--refinement": "#ff9800",
    "--refinement-ink": "#e65100",
    "--refinement-tint": "#fff3e0",
    "--verification": "#4caf50",
    "--verification-ink": "#2e7d32",
    "--verification-tint": "#e8f5e9",
    "--ontology": "#b08a00",
    "--ontology-ink": "#6f5600",
    "--ontology-tint": "#f4e3a1",
    "--resource": "#ffca28",
    "--resource-ink": "#8d6e00",
    "--resource-tint": "#fff3cf",
    "--other": "#9e9e9e",
    "--other-ink": "#616161",
    "--other-tint": "#ececec",
    "--node-generic-fill": "#eceff1",
    "--edge-default": "#c0c8d4",
    "--edge-derive": "#673ab7",
    "--edge-satisfy": "#4caf50",
    "--edge-trace": "#97a2b4",
    "--edge-attach": "#2196f3",
    "--bg-canvas": "#fbfaf7",
    "--bg-sunken": "#f3f1eb",
    "--bg-surface": "#ffffff",
    "--border-default": "#d8d2c6",
    "--accent": "#e11d48",
    "--accent-ring": "rgba(225,29,72,0.32)",
    "--rdf-class": "#94a3b8",
    "--rdf-objprop": "#64748b",
    "--rdf-dtprop": "#0f766e",
    "--rdf-rdfprop": "#115e59",
    "--rdf-individual": "#7c3aed",
    "--rdf-datatype": "#d6a43f",
    "--rdf-restriction": "#cbd5e1",
    "--rdf-classexpr": "#e2e8f0",
    "--rdf-nodeshape": "#dc2626",
    "--rdf-propshape": "#be123c",
    "--rdf-resource": "#14b8a6",
    "--rdf-shacl": "#ef4444",
    "--success": "#1f9d57",
    "--slate-0": "#ffffff",
    "--slate-950": "#0d1119",
    "--text-body": "#232b37",
    "--text-faint": "#97a2b4",
    "--text-muted": "#6b7688",
    "--text-strong": "#161d27"
  };
  const REFINEMENT_TYPES = /* @__PURE__ */ new Set([
    "source",
    "specification",
    "constraint",
    "behavior",
    "state",
    "input-output",
    "semantic-contract",
    "semantic-query-contract"
  ]);
  function elementRole(type, family) {
    const normalizedType = (type ?? "").toLowerCase();
    const normalizedFamily = (family ?? "").toLowerCase();
    if (normalizedType in ELEMENT_TYPES) {
      return ELEMENT_TYPES[normalizedType].role;
    }
    if (normalizedType.includes("capability") || normalizedFamily === "capability") return "capability";
    if (normalizedType.includes("verification") || normalizedFamily === "verification") return "verification";
    if (normalizedType.includes("ontology") || normalizedFamily === "ontology") return "ontology";
    if (normalizedType.includes("resource") || normalizedType === "file" || normalizedFamily === "resource") {
      return "resource";
    }
    if (REFINEMENT_TYPES.has(normalizedType) && normalizedType in ELEMENT_TYPES) {
      return ELEMENT_TYPES[normalizedType].role;
    }
    if (normalizedType.includes("refinement") || normalizedFamily === "refinement") return "refinement";
    if (normalizedType.includes("requirement") || normalizedFamily === "requirement") return "requirement";
    if (normalizedType.includes("contract")) return "semantic-contract";
    if (normalizedType.includes("specification")) return "specification";
    return "other";
  }
  function roleColorToken(role, channel = "fill") {
    return ELEMENT_ROLE_TOKENS[elementRole(role)][channel];
  }
  function roleColorValue(role, channel = "fill") {
    return cssVar(roleColorToken(role, channel));
  }
  function cssVar(token) {
    if (typeof window === "undefined") return `var(${token})`;
    const resolved = resolveCssToken(token);
    return normalizeCssColor(resolved) ?? resolved;
  }
  function resolveCssToken(token, seen = /* @__PURE__ */ new Set()) {
    if (seen.has(token)) return `var(${token})`;
    seen.add(token);
    const value = window.getComputedStyle(document.documentElement).getPropertyValue(token).trim();
    if (!value) return CSS_TOKEN_FALLBACKS[token] ?? `var(${token})`;
    return resolveCssValue(value, seen);
  }
  function resolveCssValue(value, seen = /* @__PURE__ */ new Set()) {
    const normalized = value.trim();
    const nested = normalized.match(/^var\((--[a-z0-9-]+)(?:,\s*([^)]+))?\)$/i);
    if (!nested) return normalized;
    return resolveCssToken(nested[1], seen);
  }
  function replaceCssVarsForMermaid(source) {
    return source.replace(/var\((--[a-z0-9-]+)\)/gi, (match, token) => {
      const colorToken = token;
      const value = normalizeCssColor(resolveCssToken(token)) ?? CSS_TOKEN_FALLBACKS[colorToken];
      return value ?? match;
    });
  }
  function getMermaidClassDefs() {
    const classDef = (className, role, strokeWidth = "2px") => {
      const tokens = ELEMENT_ROLE_TOKENS[role];
      return `  classDef ${className} fill:${mermaidTokenColor(tokens.tint)},stroke:${mermaidTokenColor(tokens.fill)},stroke-width:${strokeWidth},color:${mermaidTokenColor("--text-body")};`;
    };
    return [
      classDef("capability", "capability", "2.5px"),
      classDef("systemRequirement", "requirement", "2px"),
      classDef("requirement", "requirement", "2px"),
      classDef("refinement", "refinement", "2px"),
      classDef("source", "source", "2px"),
      classDef("constraint", "constraint", "2px"),
      classDef("behavior", "behavior", "2px"),
      classDef("state", "state", "2px"),
      classDef("inputOutput", "input-output", "2px"),
      classDef("specification", "specification", "2px"),
      classDef("semanticContract", "semantic-contract", "2px"),
      classDef("semanticQueryContract", "semantic-contract", "2px"),
      classDef("verification", "verification", "2px"),
      classDef("ontology", "ontology", "2px"),
      classDef("resource", "resource", "2px"),
      classDef("file", "resource", "2px"),
      classDef("folder", "resource", "2px"),
      classDef("default", "other", "1.5px")
    ];
  }
  function mermaidTokenColor(token) {
    const value = normalizeCssColor(cssVar(token));
    if (value) return value;
    const fallback = CSS_TOKEN_FALLBACKS[token];
    if (fallback) return fallback;
    throw new Error(`Missing Mermaid-safe CSS token value for ${token}`);
  }
  function normalizeCssColor(value) {
    var _a;
    if (!value || typeof document === "undefined") return null;
    const probe = document.createElement("span");
    probe.style.color = "";
    probe.style.color = value.trim();
    if (!probe.style.color) return null;
    const parent = document.body ?? document.documentElement;
    parent.appendChild(probe);
    const computed = window.getComputedStyle(probe).color;
    probe.remove();
    const hex = colorToHex(computed);
    if (hex) return hex;
    if (/\bjsdom\b/i.test(((_a = window.navigator) == null ? void 0 : _a.userAgent) ?? "")) return null;
    try {
      const canvas = document.createElement("canvas");
      canvas.width = canvas.height = 1;
      const ctx = canvas.getContext("2d");
      if (!ctx) return null;
      ctx.fillStyle = computed;
      ctx.fillRect(0, 0, 1, 1);
      const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
      return `#${r.toString(16).padStart(2, "0")}${g.toString(16).padStart(2, "0")}${b.toString(16).padStart(2, "0")}`;
    } catch {
      return null;
    }
  }
  function colorToHex(color) {
    const match = color.match(/^rgba?\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)(?:\s*,\s*([0-9.]+))?\s*\)$/i);
    if (!match) return null;
    const [r, g, b] = match.slice(1, 4).map((part) => {
      const value = Number.parseFloat(part);
      if (!Number.isFinite(value)) return null;
      return Math.max(0, Math.min(255, Math.round(value)));
    });
    if (r === null || g === null || b === null) return null;
    const alpha = match[4] === void 0 ? 1 : Number.parseFloat(match[4]);
    const hex = [r, g, b].map((component) => component.toString(16).padStart(2, "0")).join("");
    if (!Number.isFinite(alpha) || alpha >= 1) return `#${hex}`;
    const alphaHex = Math.max(0, Math.min(255, Math.round(alpha * 255))).toString(16).padStart(2, "0");
    return `#${hex}${alphaHex}`;
  }
  const DIAMOND_TYPES = /* @__PURE__ */ new Set([
    "source",
    "specification",
    "constraint",
    "behavior",
    "state",
    "input-output",
    "semantic-contract",
    "semantic-query-contract"
  ]);
  function ElementIcon({
    type,
    family,
    size = "md",
    className = "",
    title,
    style,
    shape,
    glyph,
    ...props
  }) {
    const role = elementRole(type, family);
    const normalizedType = (type ?? "").toLowerCase();
    const explicitType = normalizedType in ELEMENT_TYPES ? ELEMENT_TYPES[normalizedType] : null;
    const resolvedShape = shape ?? (explicitType == null ? void 0 : explicitType.shape) ?? (DIAMOND_TYPES.has(normalizedType) ? "diamond" : role === "capability" ? "hub" : "square");
    const resolvedGlyph = glyph ?? (explicitType == null ? void 0 : explicitType.glyph) ?? null;
    const isDiamond = resolvedShape === "diamond";
    const isCapability = resolvedShape === "hub";
    const classes = [
      "rq-elemicon",
      size !== "md" ? `rq-elemicon--${size}` : "",
      isDiamond ? "rq-elemicon--diamond" : "",
      isCapability ? "rq-elemicon--hub" : "",
      className
    ].filter(Boolean).join(" ");
    const iconStyle = { "--_c": `var(${roleColorToken(role)})`, ...style };
    return /* @__PURE__ */ jsxRuntimeExports.jsxs("span", { className: classes, style: iconStyle, title: title ?? type ?? role, "aria-label": type ?? role, ...props, children: [
      isCapability ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-elemicon__pip" }) : null,
      !isCapability && resolvedGlyph ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-elemicon__glyph", children: resolvedGlyph }) : null
    ] });
  }
  function RelationPill({
    kind,
    label,
    className = "",
    pipColor,
    ...props
  }) {
    const content = /* @__PURE__ */ jsxRuntimeExports.jsxs(jsxRuntimeExports.Fragment, { children: [
      pipColor ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-relation__pip", style: { background: pipColor } }) : null,
      /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-relation__txt", children: label })
    ] });
    if ("href" in props && props.href) {
      return /* @__PURE__ */ jsxRuntimeExports.jsxs("span", { className: ["rq-relation", className].filter(Boolean).join(" "), children: [
        kind ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-relation__kind", children: kind }) : null,
        /* @__PURE__ */ jsxRuntimeExports.jsx("a", { className: "rq-relation__target", ...props, children: content })
      ] });
    }
    const buttonProps = props;
    return /* @__PURE__ */ jsxRuntimeExports.jsxs("span", { className: ["rq-relation", className].filter(Boolean).join(" "), children: [
      kind ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-relation__kind", children: kind }) : null,
      /* @__PURE__ */ jsxRuntimeExports.jsx("button", { className: "rq-relation__target", ...buttonProps, type: buttonProps.type ?? "button", children: content })
    ] });
  }
  function Stat({
    label,
    value,
    stacked = false,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: ["rq-stat", stacked ? "rq-stat--stacked" : "", className].filter(Boolean).join(" "), ...props, children: stacked ? /* @__PURE__ */ jsxRuntimeExports.jsxs(jsxRuntimeExports.Fragment, { children: [
      /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-stat__value", children: value }),
      /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-stat__label", children: label })
    ] }) : /* @__PURE__ */ jsxRuntimeExports.jsxs(jsxRuntimeExports.Fragment, { children: [
      /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-stat__label", children: label }),
      /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-stat__value", children: value })
    ] }) });
  }
  function StatRow({
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("div", { className: ["rq-statrow", className].filter(Boolean).join(" "), ...props, children });
  }
  function TableViewport({
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("div", { className: ["rq-tablewrap", className].filter(Boolean).join(" "), ...props, children });
  }
  function Table({
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("table", { className: ["rq-table", className].filter(Boolean).join(" "), ...props, children });
  }
  function TableHeader({
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("thead", { className, ...props, children });
  }
  function TableBody({
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("tbody", { className, ...props, children });
  }
  function TableRow({
    children,
    selected = false,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("tr", { className: [selected ? "is-selected" : "", className].filter(Boolean).join(" "), ...props, children });
  }
  function TableHead({
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("th", { className, ...props, children });
  }
  function TableCell({
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("td", { className, ...props, children });
  }
  function TableSortButton({
    children,
    direction,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsxs("button", { type: "button", className: ["rq-table__sort", className].filter(Boolean).join(" "), ...props, children: [
      /* @__PURE__ */ jsxRuntimeExports.jsx("span", { children }),
      direction ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-table__sortdir", children: direction }) : null
    ] });
  }
  function TypeBadge({
    type,
    family,
    children,
    dot = true,
    tinted = false,
    className = "",
    style,
    ...props
  }) {
    const role = elementRole(type, family);
    const color = `var(${roleColorToken(role)})`;
    const badgeStyle = tinted ? {
      "--_tint": `color-mix(in srgb, ${color} 16%, transparent)`,
      "--_ink": `color-mix(in srgb, ${color} 78%, var(--text-strong))`,
      ...style
    } : style;
    return /* @__PURE__ */ jsxRuntimeExports.jsxs(
      "span",
      {
        className: ["rq-typebadge", tinted ? "rq-typebadge--tinted" : "", className].filter(Boolean).join(" "),
        style: badgeStyle,
        ...props,
        children: [
          dot ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-typebadge__dot", style: { background: color } }) : null,
          children ?? type
        ]
      }
    );
  }
  function Breadcrumb({ items = [], className = "", ...props }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsx("nav", { className: ["rq-crumbs", className].filter(Boolean).join(" "), "aria-label": "Breadcrumb", ...props, children: items.map((it, i) => {
      const last = i === items.length - 1;
      return /* @__PURE__ */ jsxRuntimeExports.jsxs("span", { className: "rq-crumbs__segment", children: [
        /* @__PURE__ */ jsxRuntimeExports.jsx(
          "span",
          {
            className: ["rq-crumbs__item", last ? "is-current" : ""].filter(Boolean).join(" "),
            onClick: !last ? it.onClick : void 0,
            children: it.label
          }
        ),
        !last ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-crumbs__sep", "aria-hidden": "true", children: /* @__PURE__ */ jsxRuntimeExports.jsx(Icon, { name: "chevron-right", size: 14 }) }) : null
      ] }, i);
    }) });
  }
  function SidebarSection({
    title,
    action,
    children,
    className = "",
    ...props
  }) {
    return /* @__PURE__ */ jsxRuntimeExports.jsxs("section", { className: ["rq-section", className].filter(Boolean).join(" "), ...props, children: [
      title || action ? /* @__PURE__ */ jsxRuntimeExports.jsxs("div", { className: "rq-section__head", children: [
        title ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-section__title", children: title }) : /* @__PURE__ */ jsxRuntimeExports.jsx("span", {}),
        action ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-section__action", children: action }) : null
      ] }) : null,
      /* @__PURE__ */ jsxRuntimeExports.jsx("div", { className: "rq-section__body", children })
    ] });
  }
  function TreeItem({
    label,
    icon,
    count,
    depth = 0,
    open = false,
    selected = false,
    expandable = false,
    kind = "folder",
    onToggle,
    onSelect,
    className = "",
    ...props
  }) {
    const indent = Math.max(0, depth) * 24;
    return /* @__PURE__ */ jsxRuntimeExports.jsxs(
      "div",
      {
        className: [
          "rq-treeitem",
          `rq-treeitem--${kind}`,
          open ? "is-open" : "",
          selected ? "is-selected" : "",
          className
        ].filter(Boolean).join(" "),
        style: {
          "--tree-depth": depth,
          "--tree-indent": `${indent}px`,
          paddingLeft: `calc(var(--space-5) + ${indent}px)`
        },
        onClick: onSelect,
        ...props,
        children: [
          /* @__PURE__ */ jsxRuntimeExports.jsx(
            "span",
            {
              className: "rq-treeitem__twist",
              onClick: (event) => {
                event.stopPropagation();
                onToggle == null ? void 0 : onToggle();
              },
              children: expandable ? /* @__PURE__ */ jsxRuntimeExports.jsx("svg", { viewBox: "0 0 24 24", fill: "none", stroke: "currentColor", strokeWidth: "2.5", strokeLinecap: "round", strokeLinejoin: "round", "aria-hidden": "true", children: /* @__PURE__ */ jsxRuntimeExports.jsx("path", { d: "m9 18 6-6-6-6" }) }) : null
            }
          ),
          icon ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-treeitem__icon", children: icon }) : null,
          /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-treeitem__label", children: label }),
          count != null ? /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-treeitem__count", children: /* @__PURE__ */ jsxRuntimeExports.jsx("span", { className: "rq-badge", children: count }) }) : null
        ]
      }
    );
  }
  exports.Alert = Alert;
  exports.BUTTON_SIZE_CLASSES = BUTTON_SIZE_CLASSES;
  exports.BUTTON_TONE_CLASSES = BUTTON_TONE_CLASSES;
  exports.Badge = Badge;
  exports.Breadcrumb = Breadcrumb;
  exports.Button = Button;
  exports.Card = Card;
  exports.Chip = Chip;
  exports.DESIGN_SYSTEM_COLOR_TOKENS = DESIGN_SYSTEM_COLOR_TOKENS;
  exports.ELEMENT_ROLE_TOKENS = ELEMENT_ROLE_TOKENS;
  exports.ELEMENT_TYPES = ELEMENT_TYPES;
  exports.ElementIcon = ElementIcon;
  exports.ICON_NAMES = ICON_NAMES;
  exports.Icon = Icon;
  exports.IconButton = IconButton;
  exports.Modal = Modal;
  exports.ModalBody = ModalBody;
  exports.ModalClose = ModalClose;
  exports.ModalContent = ModalContent;
  exports.ModalDescription = ModalDescription;
  exports.ModalFooter = ModalFooter;
  exports.ModalHeader = ModalHeader;
  exports.ModalTitle = ModalTitle;
  exports.RelationPill = RelationPill;
  exports.SearchInput = SearchInput;
  exports.SegmentedControl = SegmentedControl;
  exports.SidebarSection = SidebarSection;
  exports.Stat = Stat;
  exports.StatRow = StatRow;
  exports.Table = Table;
  exports.TableBody = TableBody;
  exports.TableCell = TableCell;
  exports.TableHead = TableHead;
  exports.TableHeader = TableHeader;
  exports.TableRow = TableRow;
  exports.TableSortButton = TableSortButton;
  exports.TableViewport = TableViewport;
  exports.Tabs = Tabs;
  exports.ToggleRow = ToggleRow;
  exports.TreeItem = TreeItem;
  exports.TypeBadge = TypeBadge;
  exports.cssVar = cssVar;
  exports.elementRole = elementRole;
  exports.getMermaidClassDefs = getMermaidClassDefs;
  exports.replaceCssVarsForMermaid = replaceCssVarsForMermaid;
  exports.roleColorToken = roleColorToken;
  exports.roleColorValue = roleColorValue;
  Object.defineProperty(exports, Symbol.toStringTag, { value: "Module" });
  return exports;
})({}, React, ReactDOM);
