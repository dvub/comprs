use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Deserialize, TS, Serialize)]
#[ts(export)]
#[ts(export_to = "../gui/bindings/PluginMessage.ts")]
#[serde(tag = "type")]
pub enum PluginMessage {
    ParamChange { param: String, value: f32 },
    PeakMeterData { value: f32 },
}
