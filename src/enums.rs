use serde::{Deserialize, Serialize};
use ts_rs::TS;

// "Run Test" (at least, in vscode) will (re-) generate the TS bindings
#[derive(Deserialize, TS, Serialize)]
#[ts(export)]
#[ts(export_to = "../gui/bindings/Action.ts")]
#[serde(tag = "type")]
pub enum Action {
    SetGain { value: f32 },
}
#[derive(Deserialize, TS, Serialize)]
#[ts(export)]
#[ts(export_to = "../gui/bindings/PluginMessage.ts")]
#[serde(tag = "type")]
pub enum PluginMessage {
    ParamChange { param: String, value: f32 },
    PeakMeterData { value: f32 },
}
