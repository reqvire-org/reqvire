import { useEffect, useState } from "preact/hooks";
import { Toaster, toast, type ToastPosition } from "@ds";
import { ToastDemo } from "./toast/Toast.ads.helpers";
import { ToastDemoSettingsContext } from "./toast/settings.ads.helpers";
import { fixture } from "./Toast.ads.fixture";
export const scenario = { title: "Toast notifications", covers: ["Toast", "Toaster"] };
export default function Scenario() {
 const [position,setPosition] = useState<ToastPosition>(fixture.position);
 const [expand,setExpand] = useState<boolean>(fixture.expand);
 useEffect(() => () => { toast.dismiss(); }, []);
 return <ToastDemoSettingsContext.Provider value={{position,setPosition,expand,setExpand}}><ToastDemo /><Toaster position={position} expand={expand} /></ToastDemoSettingsContext.Provider>;
}
