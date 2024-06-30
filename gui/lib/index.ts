import { Action } from "@/bindings/Action";
import { PluginMessage } from "@/bindings/PluginMessage";
// source:
// https://stackoverflow.com/questions/12709074/how-do-you-explicitly-set-a-new-property-on-window-in-typescript

// TODO:
// maybe there's a better way to do this?
declare global {
  interface Window {
    ipc: { postMessage: (message: string) => void };
    onPluginMessage: (message: PluginMessage) => void;
  }
}
/*
window.ipc = window.ipc || {};
window.onPluginMessage = window.onPluginMessage || {};
*/
export function sendToPlugin(msg: Action) {
  window.ipc.postMessage(JSON.stringify(msg));
}
