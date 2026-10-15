import { useEffect, useState } from "preact/hooks";
import { Toaster, toast, type ToastPosition } from "@ds";
import { ToastDemo } from "./toast/Toast.scenario-adapter";
import { ToastDemoSettingsContext } from "./toast/settings.scenario-adapter";
import { fixture } from "./Toast.fixture";
export const scenario = { title: "Toast notifications", covers: ["Toast", "Toaster"] };
export default function Scenario() {
 const [position,setPosition] = useState<ToastPosition>(fixture.position);
 const [expand,setExpand] = useState<boolean>(fixture.expand);
 useEffect(() => () => { toast.dismiss(); }, []);
 return <ToastDemoSettingsContext.Provider value={{position,setPosition,expand,setExpand}}><ToastDemo /><Toaster position={position} expand={expand} /></ToastDemoSettingsContext.Provider>;
}
