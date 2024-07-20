use crate::{params::Parameter::*, MAX_BUFFER_SIZE};
use nih_plug::{
    formatters::{self, v2s_f32_rounded},
    params::{FloatParam, Params},
    prelude::{FloatRange, SmoothingStyle},
    util,
};
use serde::{Deserialize, Serialize};
use std::{
    mem::discriminant,
    sync::{Arc, Mutex},
};
use ts_rs::TS;

pub const DEFAULT_THRESHOLD: f32 = -10.0;
pub const DEFAULT_RATIO: f32 = 4.0;
pub const DEFAULT_KNEE: f32 = 5.0;
pub const DEFAULT_ATTACK_TIME: f32 = 0.001;
pub const DEFAULT_RELEASE_TIME: f32 = 0.05;
pub const DEFAULT_BUFFER_SIZE: f32 = 0.01;

// TODO:
// parameterize buffer size for RMS

// "Run Test" (at least, in vscode) will (re-) generate the TS bindings
#[derive(Deserialize, Serialize, TS, Debug, Clone, PartialEq)]
#[ts(export_to = "../gui/bindings/Parameter.ts")]
#[ts(export)]
// TODO:
// document this
pub enum Parameter {
    Ratio(f32),
    Threshold(f32),
    AttackTime(f32),
    ReleaseTime(f32),
    KneeWidth(f32),
    InputGain(f32),
    OutputGain(f32),
    DryWet(f32),
    RmsBufferSize(f32),
    Lookahead(f32),
    RmsMix(f32),
}

#[derive(Deserialize, Serialize, TS)]
#[ts(export_to = "../gui/bindings/Messages.ts")]
#[ts(export)]
pub enum Messages {
    Init,
    ParameterUpdate(Parameter),
}

// TODO:
// dont use a const like this lol

const NUM_PARAMETERS: usize = 11;

/// Parameters for compressor.
/// **NOTE**: In this documentation I've used the term "level" instead of "signal."
/// This is because compressors may not always use the incoming signal as the value to use in calculations.
/// An example would instead be using RMS, etc.
///
#[derive(Params)]
pub struct CompressorParams {
    pub event_buffer: Arc<Mutex<Vec<Parameter>>>,

    /// The threshold at which to begin applying compression **in decibels.**
    /// For example, a compressor with a threshold of -10db would (for the most part) compress when *the level* above -10db.
    ///
    /// Because the threshold is used in dB-based calculations in the DSP, the underlying value is dB and not voltage.
    /// Therefore, there are no display conversions, etc.
    #[id = "threshold"]
    pub threshold: FloatParam,
    /// The compression ratio as the left side of the ratio **in decibels**.
    /// For example, a ratio of `2.0` would be equivalent to a ratio of 2:1,
    /// which means that for every 2db that *the level* is above the `threshold`, 1db will pass through.
    #[id = "ratio"]
    pub ratio: FloatParam,
    /// The time it takes before the compressor starts compressing after *the level* is above the threshold.
    ///
    /// **NOTE**: The actual underlying value is the filter coefficient for the compressor, however the value is converted and displayed in (milli)seconds.
    #[id = "attack"]
    pub attack_time: FloatParam,
    /// The time it takes for the compressor to stop compressing after *the level* falls below the threshold.
    ///
    /// **NOTE**: The actual underlying value is the release filter coefficient for the compressor, however the value is converted and displayed in (milli)seconds.
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

    /// The length of time (in seconds) of input that the RMS will use to calculate its gain.
    /// For example, 30 milliseconds means that the RMS will capture the last 30 milliseconds of input as its gain.
    #[id = "bufsize"]
    pub rms_buffer_size: FloatParam,

    /// The amount of time in seconds that the output is delayed.
    /// I know there's a better definition, I just can't think of it lol
    #[id = "lookahead"]
    pub lookahead: FloatParam,

    /// Blends the gain of the independent (L/R) and shared RMS.
    /// A value of 0.0 means the L/R compressors calculate their compression *only* based on their own RMS state,
    /// while a value of 1.0 means the L/R compressors will use the gain state of the RMS shared between them ONLY.
    #[id = "rmsmix"]
    pub rms_mix: FloatParam,
}

impl CompressorParams {
    /// Returns a tuple of the corresponding FloatParam and value based on a `ParameterEvent` input
    pub fn get_param(&self, action: &Parameter) -> (&FloatParam, f32) {
        match action {
            Ratio(value) => (&self.ratio, *value),
            Threshold(value) => (&self.threshold, *value),
            AttackTime(value) => (&self.attack_time, *value),
            ReleaseTime(value) => (&self.release_time, *value),
            KneeWidth(value) => (&self.knee_width, *value),
            InputGain(value) => (&self.input_gain, *value),
            OutputGain(value) => (&self.output_gain, *value),
            DryWet(value) => (&self.dry_wet, *value),
            RmsBufferSize(value) => (&self.rms_buffer_size, *value),
            Lookahead(value) => (&self.lookahead, *value),
            RmsMix(value) => (&self.rms_mix, *value),
        }
    }
}

/// Creates a callback which pushes the given `ParameterEvent` to the `event_buffer`.
/// The callback should be attached to a `FloatParam`
fn generate_callback(
    variant: fn(f32) -> Parameter,
    event_buffer: &Arc<Mutex<Vec<Parameter>>>,
) -> Arc<impl Fn(f32)> {
    let event_buffer_clone = event_buffer.clone();

    // this is the callback that each parameter will fire when it updates
    // either from the GUI or from the DAW
    Arc::new(move |value: f32| {
        // create an enum variant from the value
        let new_event = variant(value);
        let mut event_buffer_lock = event_buffer_clone
            .lock()
            .expect("Error locking parameter buffer");
        // TODO: maybe do this elsewhere
        // now we need to find and remove old parameter events with the same enum variant
        event_buffer_lock.retain(|event| discriminant(event) != discriminant(&new_event));
        // now we are ready to add the new value
        event_buffer_lock.push(new_event);
    })
}

impl Default for CompressorParams {
    fn default() -> Self {
        let event_buffer = Arc::new(Mutex::new(Vec::with_capacity(NUM_PARAMETERS)));

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
            .with_callback(generate_callback(Threshold, &event_buffer)),
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
            .with_unit(" dB")
            .with_callback(generate_callback(Ratio, &event_buffer)),

            // ATTACK TIME
            attack_time: FloatParam::new(
                "Attack Time",
                DEFAULT_ATTACK_TIME,
                FloatRange::Skewed {
                    min: 0.0, // 0 seconds atk time, meaning the compressor takes effect instantly
                    max: 1.0,
                    factor: FloatRange::skew_factor(-2.0), // just happened to be right in the middle
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_value_to_string(v2s_time_formatter())
            .with_callback(generate_callback(AttackTime, &event_buffer)),

            // RELEASE
            release_time: FloatParam::new(
                "Release Time",
                DEFAULT_RELEASE_TIME,
                FloatRange::Skewed {
                    min: 0.0,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-2.25), // kinda funky but i tried
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_value_to_string(v2s_time_formatter())
            .with_callback(generate_callback(ReleaseTime, &event_buffer)),
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
            .with_callback(generate_callback(KneeWidth, &event_buffer)),
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
            .with_callback(generate_callback(InputGain, &event_buffer)),
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
            .with_callback(generate_callback(OutputGain, &event_buffer)),

            // DRY/WET
            dry_wet: FloatParam::new("Dry/Wet", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 }) // 1.0 default for full compressor effect
                .with_smoother(SmoothingStyle::Linear(10.0))
                .with_value_to_string(v2s_rounded_multiplied(1))
                .with_callback(generate_callback(DryWet, &event_buffer)),
            rms_buffer_size: FloatParam::new(
                "RMS Buffer Length",
                DEFAULT_BUFFER_SIZE,
                FloatRange::Linear {
                    min: 0.001,
                    max: MAX_BUFFER_SIZE,
                },
            )
            .with_value_to_string(v2s_buffer_size_formatter())
            .with_smoother(SmoothingStyle::Linear(10.0)),

            // LOOKEAHEAD
            lookahead: FloatParam::new(
                "Lookahead",
                0.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: MAX_BUFFER_SIZE,
                },
            )
            .with_value_to_string(v2s_buffer_size_formatter())
            .with_smoother(SmoothingStyle::Linear(10.0)),

            // RMS MIX
            rms_mix: FloatParam::new("RMS Mix", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_value_to_string(v2s_rounded_multiplied(1))
                .with_smoother(SmoothingStyle::Linear(10.0)),

            event_buffer,
        }
    }
}
// very slightly modified NIH-plug formatter

pub fn v2s_rounded_multiplied(digits: usize) -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    let rounding_multiplier = 10u32.pow(digits as u32) as f32;
    Arc::new(move |value| {
        let v = value * 100.0;
        // See above
        if (v * rounding_multiplier).round() / rounding_multiplier == 0.0 {
            format!("{:.digits$}%", 0.0)
        } else {
            format!("{v:.digits$}%")
        }
    })
}

pub fn v2s_time_formatter() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(move |value| {
        // time in MS
        let t = value * 1000.0;
        let mut unit = "ms";
        let mut output = t;
        if t >= 1000.0 {
            unit = "S";
            output /= 1000.0;
        }

        format!("{output:.2} {unit}")
    })
}

pub fn v2s_buffer_size_formatter() -> Arc<dyn Fn(f32) -> String + Send + Sync> {
    Arc::new(move |value| {
        // from S to MS
        let t = value * 1000.0;
        let mut unit = "ms";
        let mut output = t;
        if t >= 1000.0 {
            unit = "S";
            output /= 1000.0;
        }

        format!("{output:.2} {unit}")
    })
}
