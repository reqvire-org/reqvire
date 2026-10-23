# Toast notifications

`Toast` is the standalone visual primitive. `Toaster` owns the stack and lifecycle;
`toast` creates notifications from application code. This is a native DS
implementation with stacking, motion, and input behavior adapted from Sonner's
source references. The implementation was ported from the ADS tooling proof of
concept into Reqvire Explorer; credits and notices stay with this component.

## References and credits

This component draws on [Sonner](https://github.com/emilkowalski/sonner)
by [Emil Kowalski](https://emilkowal.ski/). Its compact stacking, hover expansion,
swipe resistance and dismissal, imperative API, and loading/status animation
were studied and adapted into this implementation. The browser reference was the
published
[Sonner 2.0.8 package](https://www.npmjs.com/package/sonner/v/2.0.8).

The native store, public types, DS primitives, token integration, and measured
geometry boundary are local implementations. That does not make the source-derived
stacking and motion a wholly independent rewrite. Sonner's original copyright
and full MIT terms are retained in [LICENSE.sonner.md](LICENSE.sonner.md) for
those adapted portions. Keep that notice with this component when copying or
distributing it; an attribution link alone does not replace the license notice.

Source references for that release are Sonner's
[CSS](https://github.com/emilkowalski/sonner/blob/ecce1841c55e4a72dfe139a8992b56498660125e/src/styles.css),
[lifecycle and input implementation](https://github.com/emilkowalski/sonner/blob/ecce1841c55e4a72dfe139a8992b56498660125e/src/index.tsx).
The research also used its [official documentation](https://sonner.emilkowal.ski/), the author's
[Building a toast component](https://emilkowal.ski/ui/building-a-toast-component)
article, and the upstream
[ask-sonner usage skill](https://github.com/emilkowalski/skills/blob/main/skills/ask-sonner/SKILL.md).
The source links above pin the published package's Git revision
`ecce1841c55e4a72dfe139a8992b56498660125e`; the usage skill's `main` link may evolve.
Native primitives and tokens provide the design-system
appearance; the comparison below records the current behavioral and visual differences.

## Reqvire integration

`src/App.tsx` mounts the default toaster across routes. The colocated `../Toast.ads.scenario.tsx`
mounts the same implementation with local settings; Components → Toast notifications
exercises its public API in the MCP showcase. Existing store, renderer, and worktree errors retain their persistent
inline/dialog recovery UI. A toast is for transient feedback, not a replacement
for information needed to complete or recover a task.

The port keeps source stacking, timings, gestures, queued timers, focus restoration,
and transition cleanup. It reuses Reqvire's Button, IconButton, Icon, and Spinner,
adds missing motion/geometry tokens without replacing existing theme values,
and declares notification usage in ADS-managed `usage.notifications` and
`usage.restrictedImports` policies in `ds.config.json`. Native ADS gates enforce
those policies during the normal build. Store lifecycle tests are colocated in
`store.ads.test.ts` and run through the ordinary project Vitest command.
The geometry helper may write only its exact registered properties.

## Quick start

Mount once in the app root. Defaults are bottom-right, three visible cards, and
a 4,000 ms lifetime. Import the local foundation CSS once as described in the
[Explorer DS guide](../../../readme.md).

```tsx
import { Button, Toaster, toast } from "@ds";

function App() {
  return (
    <>
      <Toaster />
      <Button onClick={() => toast.success("Changes saved")}>Save</Button>
    </>
  );
}
```

## API and lifecycle

The following examples belong inside client handlers or callbacks;
`saveChanges()` represents the application's asynchronous operation.

```tsx
toast("Event created", { description: "Friday at 10:00." });
toast.success("Changes saved");
toast.error("Could not save changes"); // maps to the DS danger variant
toast.warning("Session expires soon");
toast.info("Update available");

const id = toast.loading("Preparing export…");
toast.update(id, { message: "Export ready", variant: "success" });
toast.dismiss(id);

const job = toast.promise(saveChanges(), {
  loading: "Saving…",
  success: (result) => `Saved ${result.name}`,
  error: "Could not save changes",
});
await job.unwrap(); // preserves the task's result or rejection
```

`ToastOptions` accepts an `id` (reuse it to update one card), `description`,
`variant`, `duration`, `dismissible`, `closeButton`, `action`, `cancel`, `toasterId`, `onDismiss`,
and `onAutoClose`. Actions provide `{ label, onClick }`; call
`event.preventDefault()` to keep the card open after the callback. Loading cards
disable their controls and swipe gesture. Loading and `duration: Infinity` cards
persist until updated or dismissed. `toast.dismiss()` dismisses all cards.

`Toaster` supports all six top/bottom × left/center/right positions, `expand`,
`visibleToasts`, a default `duration`, `closeButton` (false by default), an accessible
`label`, and an optional `id`. A card's `closeButton` option overrides the default.
Mount one instance per id and target it with `toasterId` in the toast options.
An omitted `toasterId` targets only the default scope. This explicit routing
differs from Sonner's documented multiple-toaster recipe.
Unmounting a toaster clears that scope; pending promises do not recreate its cards.
Call `toast()` in event handlers or effects, not during rendering or on the server.

## Interaction and animation

The compact stack uses the front card's height, a small vertical peek, and a 5%
scale step. Hover or keyboard focus expands cards to their measured natural
heights. Transitions can retarget smoothly during rapid additions and removals.
Timers pause on hover, keyboard focus, swipe, hidden documents, and while queued.
`Alt+T` focuses the stack; `Escape` returns to the previous focus target.
Collapsed rear cards and queued cards are inert. A separate polite live region
announces additions and updates without announcing hover expansion. Reduced
motion disables stack transitions and spinner animation.

The stack follows Sonner's 356 px width, 14 px gaps/peeks, 5% scale steps,
400 ms entrance/stack easing, and 200 ms outward swipe. Touch dragging does not
expand a compact stack. When loading finishes, the native spinner fades and
shrinks over 200 ms while the status icon enters over 300 ms.

## Comparison with Sonner

The browser comparison used the actual published Sonner 2.0.8 component with
matching messages, settings, and viewport sizes. The following differences
describe the current implementation:

| Area | Native design-system behavior | Sonner 2.0.8 reference |
|------|-------------------------------|------------------------|
| Appearance | Geist, semantic colors, outline icons, native spinner and buttons; close control inside the card | System font, neutral defaults, filled icons, segmented spinner; close control at the corner |
| Card heights | Follow DS line heights and available content width | Follow Sonner line heights and content width; matching text can wrap differently |
| Queued cards | Lifetime pauses until visible; hidden controls are inert | Hidden cards continue their timers and remain focusable |
| Keyboard focus | Focus expands and pauses the stack; Escape restores previous focus | Alt+T expands the stack; Escape collapses it and retains stack focus |
| Ordinary exit cleanup | Waits for the CSS transition to complete | Removes the card after 200 ms, including exits styled with a 400 ms transition |

## Customization

Visual customization goes through tokens and documented component variables:

| Variable | Purpose | Default |
|----------|---------|---------|
| `--ds-toast-width` | Desktop stack width | `--toast-width` (356 px), shared by standalone cards and the stack |
| `--ds-toast-inset` | Viewport edge inset | `--space-12`; `--space-8` on mobile |
| `--ds-toast-gap` | Expanded gap | `--space-7` |
| `--ds-toast-peek` | Collapsed stack step | `--space-7` |
| `--ds-toast-border` | Card border | `--border-default` |
| `--ds-toast-icon-color` | Status icon color | Semantic status token |

Standalone `Toast` cards use the same `--ds-toast-width`/`--toast-width` contract
as the live stack and fit their available container, rather than stretching
to the showcase page width.

The motion policy uses `--dur-toast-stack`, `--dur-toast-exit`,
`--dur-toast-rear-exit`, `--dur-toast-icon`, `--dur-toast-loader`,
`--ease-toast-stack`, `--ease-toast-exit`, and `--toast-stack-scale-step`.
Other `--ds-toast-*` geometry properties are private. The cards inherit the global
theme tokens through a body portal, matching `Modal`; set theme tokens at the
document root. The showcase's Toast section demonstrates variants, variable
heights, stacking, actions, persistent cards, promise outcomes, and placement.

## Usage policy and lint ownership

Mount `Toaster` unconditionally in the application's entry/root layout. Keep it
mounted across route changes. Trigger notifications from client event handlers,
callbacks, or effects; for a server action, return a result and notify from the
client that receives it. Prefer `toast.promise()` for work with a promise. If you
manage `toast.loading()` yourself, retain its id and settle or dismiss it on both
success and failure. For an effect-driven notification, pass a stable id so
effect reruns and remounts update the same card:

```tsx
useEffect(() => {
  if (!connectionRestored) return;
  toast.info("Connection restored", { id: "connection-status" });
}, [connectionRestored]);
```

Use `Toast` for a standalone visual example and `toast()` for transient
notifications. A toast should supplement persistent UI feedback; field errors,
blocking decisions, and information users need to revisit belong in the relevant
form, `Alert`, or dialog. Keep titles short and action labels clear. Use a longer
lifetime or `Infinity` when an action needs time, and provide a dismissal path.
Enable `closeButton` for an explicit dismiss control. The standalone `Toast`
shows this control when `onDismiss` is provided unless `closeButton` is false.

Trigger transient app-level feedback with `toast()` from the consuming
application. Keep the toaster outside route/shell content. For feedback tied to
incoming state, use an effect with a stable id. Keep persistent inline alerts
in normal content so they participate in layout.

The native API deliberately excludes Sonner's inline `style`, `classNames`,
`unstyled`, `richColors`, `invert`, custom render factories, and separate theme
configuration. Use DS tokens and semantic variants. Titles/descriptions accept
Preact nodes directly. Callback arguments are toast ids, and `toast.promise()`
returns `{ id, unwrap }`; these are native API contracts, not a promise of full
Sonner API compatibility.

ADS owns notification usage enforcement. [ds.config.json](../../../../ds.config.json)
registers `toast` and `Toaster` under `usage.notifications` and restricts the
`sonner` package under `usage.restrictedImports`. `ads check` runs these native
guards directly; guarded acceptance and validation use the same gate. There is
no project lint plugin or additional lint runner.

| Rule | Enforced boundary |
|------|-------------------|
| `notification-client-usage` | Reject direct render/module-scope calls and calls in `use server` modules |
| `notification-effect-id` | Require an explicit id for notifications directly created in React or Preact effects with statically inspectable options |
| `notification-host-root` / `-conditional` / `-duplicate` | Restrict hosts to registered roots; reject direct conditional mounts and repeated static ids in one file |
| `usage-restricted-import` | Reject direct Sonner package/subpath imports; consume the native public API |

Configure `rootFiles` in `ds.config.json` for a consuming app's actual
entry/layout. Imported aliases and namespace imports are recognized. The rules
do not prove application-wide uniqueness across modules, inspect dynamic/spread
options, trace notifications through helper functions, or prove that async work
will settle. Review those cases and test route persistence. Existing token and
CSS ownership guards enforce visual styling, while TypeScript enforces the
native option shapes. Tests cover accepted and rejected lint cases as well as
store/promise lifecycle behavior.

## Validation

Run the component checks and normal builds from the `explorer/` workspace:

```sh
bun run test
bun run build
bun run build:showcase
```

[store.ads.test.ts](store.ads.test.ts) checks store snapshots,
callbacks, scope teardown, promise settlement, and stale updates.
ADS tests cover accepted/rejected usage policies in the native checker. The
project build runs that shared gate, type checking, application tests and
registered browser checks.

For browser verification, open the native Toast scenario in the MCP showcase. Compare with the actual Sonner
component in a separate reference page, using identical content and settings.
Check compact/expanded stacks, timed and persistent cards, hidden-document
timers, keyboard focus, close/action controls, promises, and mouse/touch swipes.
Measure animation timing and transforms; also check mobile, dark theme, and
reduced motion. Run automation in separate browser contexts so it does not
resize or navigate the user's current showcase. Wait for the managed compiler and verify its current build revision after source
changes before comparing again.

Implementation: [Toast.tsx](../Toast.tsx), [Toaster.tsx](../Toaster.tsx),
[store.ts](store.ts), and [geometry.ts](geometry.ts). Shared architecture
and token ownership are documented in the
[project guide](../../../readme.md#source-and-tooling), with project boundaries
and registered rules in [ds.config.json](../../../../ds.config.json).


## Registered browser checks

`../Toaster.ads.contract.ts` registers `toast-actions` and `toast-promise-focus`
against the existing `components/core/Toast` scenario. The colocated browser spec
checks action callbacks, close controls, promise updates in place and keyboard
focus restoration through the real MCP App. Run
`bunx --no-install ads verify . --unit components/core/Toaster` from `explorer/`.
Store lifecycle tests remain in `bun run test`; usage-rule regressions live in ADS; application browser
checks still verify the app-root mount and real server integration.
