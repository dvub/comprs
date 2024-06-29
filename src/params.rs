use std::sync::Arc;

use nih_plug::{
    formatters::{self, v2s_f32_rounded},
    params::{FloatParam, Params},
    prelude::{FloatRange, SmoothingStyle},
};

pub const DEFAULT_THRESHOLD: f32 = 0.0;
pub const DEFAULT_RATIO: f32 = 4.0;
pub const DEFAULT_KNEE: f32 = 5.0;
pub const DEFAULT_ATTACK_TIME: f32 = 0.001;
pub const DEFAULT_RELEASE_TIME: f32 = 0.05;

#[derive(Params)]
pub struct CompressorParams {
    #[id = "threshold"]
    pub threshold: FloatParam,
    #[id = "ratio"]
    pub ratio: FloatParam,
    #[id = "attack"]
    pub attack_time: FloatParam,

    #[id = "release"]
    pub release_time: FloatParam,

    #[id = "knee"]
    pub knee_width: FloatParam,

    #[id = "ingain"]
    pub input_gain: FloatParam,
    #[id = "outgain"]
    pub output_gain: FloatParam,
    #[id = "drywet"]
    pub dry_wet: FloatParam,
}

impl Default for CompressorParams {
    fn default() -> Self {
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
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
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
            .with_value_to_string(formatters::v2s_compression_ratio(2)),

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
            .with_value_to_string(v2s_rounded_multiplied(3, 1000.0)),
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
            .with_value_to_string(v2s_rounded_multiplied(3, 1000.0)),
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
            .with_value_to_string(v2s_f32_rounded(1)),
            // INPUT GAIN
            input_gain: FloatParam::new(
                "Input Gain",
                0.0,
                FloatRange::Skewed {
                    min: -100.0,
                    max: 10.0,
                    factor: FloatRange::skew_factor(2.25),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            // OUTPUT GAIN
            output_gain: FloatParam::new(
                "Output Gain",
                0.0,
                FloatRange::Skewed {
                    min: -100.0,
                    max: 10.0,
                    factor: FloatRange::skew_factor(2.25),
                },
            )
            .with_smoother(SmoothingStyle::Linear(10.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            dry_wet: FloatParam::new("Dry/Wet", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 }) // 1.0 default for full compressor effect
                .with_smoother(SmoothingStyle::Linear(10.0))
                .with_unit("%")
                .with_value_to_string(v2s_rounded_multiplied(1, 100.0)),
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
