import { ParameterEvent } from "@/bindings/ParameterEvent";
import { Messages } from "@/bindings/Messages";

// source:
// https://stackoverflow.com/questions/12709074/how-do-you-explicitly-set-a-new-property-on-window-in-typescript

// TODO:
// maybe there's a better way to do this?
declare global {
  interface Window {
    ipc: { postMessage: (message: string) => void };
    onPluginMessage: (message: ParameterEvent) => void;
  }
}
/*
window.ipc = window.ipc || {};
window.onPluginMessage = window.onPluginMessage || {};
*/
export function sendToPlugin(msg: ParameterEvent | Messages) {
  window.ipc.postMessage(JSON.stringify(msg));
}
