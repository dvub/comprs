// considering that there are 2 SO links in this file,
// it should be clear i'm a typescript noob

import { Parameter } from "@/bindings/Parameter";
import { Messages } from "@/bindings/Messages";

// super weird, but i guess it works
// https://stackoverflow.com/questions/49401866/all-possible-keys-of-an-union-type

export type KeysOfUnion<T> = T extends T ? keyof T : never;
export type ParameterType = KeysOfUnion<Parameter>;

// source:
// https://stackoverflow.com/questions/12709074/how-do-you-explicitly-set-a-new-property-on-window-in-typescript

// TODO:
// maybe there's a better way to do this?
declare global {
  interface Window {
    ipc: { postMessage: (message: string) => void };
    onPluginMessage: (message: Parameter) => void;
  }
}
/*
window.ipc = window.ipc || {};
window.onPluginMessage = window.onPluginMessage || {};
*/
export function sendToPlugin(msg: Messages) {
  window.ipc.postMessage(JSON.stringify(msg));
}
