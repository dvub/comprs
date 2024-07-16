use crate::params::ParameterType::*;
use nih_plug::{
    formatters::{self, v2s_f32_rounded},
    params::{FloatParam, Params},
    prelude::{FloatRange, SmoothingStyle},
    util,
};
use serde::{Deserialize, Serialize};
use std::{
    mem::{discriminant, replace},
    sync::{Arc, Mutex},
};
use ts_rs::TS;

pub const DEFAULT_THRESHOLD: f32 = -10.0;
pub const DEFAULT_RATIO: f32 = 4.0;
pub const DEFAULT_KNEE: f32 = 5.0;
pub const DEFAULT_ATTACK_TIME: f32 = 0.001;
pub const DEFAULT_RELEASE_TIME: f32 = 0.05;

// "Run Test" (at least, in vscode) will (re-) generate the TS bindings
#[derive(Deserialize, Serialize, TS, Debug, Clone, PartialEq)]
#[ts(export_to = "../gui/bindings/Action.ts")]
#[ts(export)]
#[serde(tag = "type")]
pub enum ParameterType {
    Ratio { value: f32 },
    Threshold { value: f32 },
    AttackTime { value: f32 },
    ReleaseTime { value: f32 },
    KneeWidth { value: f32 },
    InputGain { value: f32 },
    OutputGain { value: f32 },
    DryWet { value: f32 },
}

// TODO:
// implement something like
// https://stackoverflow.com/questions/54177438/how-to-programmatically-get-the-number-of-fields-of-a-struct

const NUM_PARAMETERS: usize = 8;
// note: IF I could, I would just get rid of the enum above and then simply export this struct.
// however, the CompressorParams struct uses FloatParam which doesn't derive the traits I need. :/

/// Parameters for compressor.
/// **NOTE**: In this documentation I've used the term "level" instead of "signal."
/// This is because compressors may not always use the incoming signal as the value to use in calculations.
/// An example would instead be using RMS, etc.
///
///
#[derive(Params)]
pub struct CompressorParams {
    pub changed_params: Arc<Mutex<Vec<ParameterType>>>,

    /// The threshold at which to begin applying compression **in decibels.**
    /// For example, a compressor with a threshold of -10db would (for the most part) compress when *the level* above -10db.
    #[id = "threshold"]
    pub threshold: FloatParam,
    /// The compression ratio as the left side of the ratio **in decibels**.
    /// For example, a ratio of `2.0` would be equivalent to a ratio of 2:1,
    /// which means that for every 2db that *the level* is above the `threshold`, 1db will pass through.
    #[id = "ratio"]
    pub ratio: FloatParam,
    /// The time it takes, **in seconds**, before the compressor starts compressing after *the level* is above the threshold.
    #[id = "attack"]
    pub attack_time: FloatParam,
    /// The time it takes, **in seconds**, for the compressor to stop compressing after *the level* falls below the threshold.
    #[id = "release"]
    pub release_time: FloatParam,
    /// The knee width **in decibels**. This smooths the transition between compression and no compression around the threshold.
    /// If you'd like a *hard-knee compressor*, set this value to `0.0`.
    #[id = "knee"]
    pub knee_width: FloatParam,
    /// Modify the gain of the incoming signal ***before*** compression.
    #[id = "ingain"]
    pub input_gain: FloatParam,
    /// Modify the gain of the incoming signal ***after*** compression ***AND*** after dry/wet has been applied.
    #[id = "outgain"]
    pub output_gain: FloatParam,
    /// Blends the pre-compressed signal with the processed, compressed signal.
    /// `1.0` (100%) means that only the compressed signal will be output,
    /// while `0.0` (0%) means that essentially, no compression is applied.  
    #[id = "drywet"]
    pub dry_wet: FloatParam,
}

impl CompressorParams {
    pub fn get_param(&self, action: &ParameterType) -> (&FloatParam, f32) {
        match action {
            Ratio { value } => (&self.ratio, *value),
            Threshold { value } => (&self.threshold, *value),
            AttackTime { value } => (&self.attack_time, *value),
            ReleaseTime { value } => (&self.release_time, *value),
            KneeWidth { value } => (&self.knee_width, *value),
            InputGain { value } => (&self.input_gain, *value),
            OutputGain { value } => (&self.output_gain, *value),
            DryWet { value } => (&self.dry_wet, *value),
        }
    }
}

fn generate_callback(
    t: fn(f32) -> ParameterType,
    changed_params: &Arc<Mutex<Vec<ParameterType>>>,
) -> Arc<impl Fn(f32)> {
    let changed_params_clone = changed_params.clone();
    let callback = Arc::new(move |value: f32| {
        // create an enum variant from the value
        let new_event = t(value);
        let mut lock = changed_params_clone.lock().unwrap(); // TODO: don't unwrap lol

        // now we need to find and remove old parameter events with the same enum variant
        lock.retain(|event| discriminant(event) != discriminant(&new_event));
        // now we are ready to add the new value
        lock.push(new_event);
    });
    callback
}

impl Default for CompressorParams {
    fn default() -> Self {
        let changed_params = Arc::new(Mutex::new(Vec::with_capacity(NUM_PARAMETERS)));
        // I mostly just played around with other compressors and got a feel for their paramters
        // I spent way too much time tuning these
        Self {
            // THRESHOLD
            threshold: FloatParam::new(
                "Threshold",
                DEFAULT_THRESHOLD,
                FloatRange::Skewed {
                    min: -100.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(2.25),
                },
            )
            // our threshold is already in dB land, so we don't need any conversion/formatting
            // TODO: play with smoothing style/timing
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" dB")
            // TODO:
            // create a custom formatter for -inf dB
            .with_value_to_string(formatters::v2s_f32_rounded(2))
            .with_callback(generate_callback(
                |value| ParameterType::Threshold { value },
                &changed_params,
            )),
            // TODO:
            // do we need string_to_value..?

            // RATIO
            ratio: FloatParam::new(
                "Ratio",
                DEFAULT_RATIO, // default compression ratio of 4:1 dB
                FloatRange::Skewed {
                    min: 1.0,
                    max: 100.0,
                    factor: FloatRange::skew_factor(-1.8),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            // TODO: customize formatter
            .with_value_to_string(formatters::v2s_compression_ratio(2))
            .with_callback(generate_callback(
                |value| ParameterType::Ratio { value },
                &changed_params,
            )),

            // ATTACK TIME
            attack_time: FloatParam::new(
                "Attack Time",
                DEFAULT_ATTACK_TIME,
                FloatRange::Skewed {
                    min: 0.0, // 0 seconds atk time, meaning the compressor takes effect instantly
                    max: 1.0,
                    factor: FloatRange::skew_factor(-2.15),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" ms")
            .with_value_to_string(v2s_rounded_multiplied(3, 1000.0))
            .with_callback(generate_callback(
                |value| ParameterType::AttackTime { value },
                &changed_params,
            )),

            // RELEASE
            release_time: FloatParam::new(
                "Release Time",
                DEFAULT_RELEASE_TIME,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-1.75),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" ms")
            .with_value_to_string(v2s_rounded_multiplied(3, 1000.0))
            .with_callback(generate_callback(
                |value| ParameterType::ReleaseTime { value },
                &changed_params,
            )),
            // KNEE WIDTH
            knee_width: FloatParam::new(
                "Knee Width",
                DEFAULT_KNEE,
                FloatRange::Linear {
                    min: 0.0,
                    max: 20.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" dB")
            .with_value_to_string(v2s_f32_rounded(1))
            .with_callback(generate_callback(
                |value| ParameterType::KneeWidth { value },
                &changed_params,
            )),
            // INPUT GAIN
            // basically, the exact same as this. LOL
            // https://github.com/robbert-vdh/nih-plug/blob/ffe9b61fcb0441c9d33f4413f5ebe7394637b21f/plugins/examples/gain/src/lib.rs#L67
            input_gain: FloatParam::new(
                "Input Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    // This makes the range appear as if it was linear when displaying the values as
                    // decibels
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            // Because the gain parameter is stored as linear gain instead of storing the value as
            // decibels, we need logarithmic smoothing
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            // There are many predefined formatters we can use here. If the gain was stored as
            // decibels instead of as a linear gain value, we could have also used the
            // `.with_step_size(0.1)` function to get internal rounding.
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db())
            .with_callback(generate_callback(
                |value| ParameterType::InputGain { value },
                &changed_params,
            )),
            // OUTPUT GAIN
            output_gain: FloatParam::new(
                "Output Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db())
            .with_callback(generate_callback(
                |value| ParameterType::OutputGain { value },
                &changed_params,
            )),

            dry_wet: FloatParam::new("Dry/Wet", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 }) // 1.0 default for full compressor effect
                .with_smoother(SmoothingStyle::Linear(10.0))
                .with_unit("%")
                .with_value_to_string(v2s_rounded_multiplied(1, 100.0))
                .with_callback(generate_callback(
                    |value| ParameterType::DryWet { value },
                    &changed_params,
                )),
            changed_params,
        }
    }
}
// very slightly modified NIH-plug formatter

pub fn v2s_rounded_multiplied(
    digits: usize,
    multiplier: f32,
) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    let rounding_multiplier = 10u32.pow(digits as u32) as f32;
    Arc::new(move |value| {
        let v = value * multiplier;
        // See above
        if (v * rounding_multiplier).round() / rounding_multiplier == 0.0 {
            format!("{:.digits$}", 0.0)
        } else {
            format!("{v:.digits$}")
        }
    })
}
