use std::{collections::VecDeque, sync::Arc};

// TODO:
// consider using fast functions
use nih_plug::util::{db_to_gain, gain_to_db};

use crate::params::CompressorParams;

const DEFAULT_SAMPLE_RATE: f32 = 44_100.0;
const _DEFAULT_BUFFER_SIZE: usize = (DEFAULT_SAMPLE_RATE * 1e-3) as usize;
// TODO:
// add documentation!!

// https://www.musicdsp.org/en/latest/Effects/169-compressor.html (not the best source)
// recommended:
// https://www.eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf

/// Struct to represent an RMS level detector. Uses a running sum and circular buffer.
pub struct RmsLevelDetector {
    pub sample_rate: f32,
    squared_sum: f32,
    pub buffer: VecDeque<f32>,
}
impl Default for RmsLevelDetector {
    fn default() -> Self {
        let buffer_size = 1e-3;
        let buffer_length = (DEFAULT_SAMPLE_RATE * buffer_size) as usize;
        Self {
            sample_rate: DEFAULT_SAMPLE_RATE,
            squared_sum: 0.0,
            buffer: VecDeque::from(vec![0.0; buffer_length]),
        }
    }
}
impl RmsLevelDetector {
    pub fn calculate_rms(&mut self, input: f32) -> f32 {
        // peak detection - RMS
        let old_sample = self.buffer.pop_front().unwrap();
        self.buffer.push_back(input);
        self.squared_sum += input.powi(2);
        self.squared_sum -= old_sample.powi(2);
        //
        // THIS MIGHT BE PROBLEMATIC!
        //
        self.squared_sum = self.squared_sum.max(0.0);

        (self.squared_sum / self.buffer.len() as f32).sqrt()
    }
}
/// Variants represent the different types of level detection that the compressor may use to update its internal gain.
/*
pub enum LevelDetectionType {
    /// Use RMS of the signal
    Rms,
}
*/
/// Struct to represent a dynamic range compressor. See documentation for each field to learn more.
pub struct Compressor {
    pub params: Arc<CompressorParams>,
    /// Average input gain *in linear space*.
    /// The method of calculating this average gain is controlled by the `level_detection_type` field.
    average_gain: f32,
    /// RMS state.
    pub rms: RmsLevelDetector,
    // The type of level detection used to update the internal average gain of the compressor.
    // It is generally suitable and computationally cheaper to use the `Simple` variant, which directly takes into account the input and smooths it.
    // On the other hand, the `Rms` variant computes, well, the RMS of the input, and uses that to keep track of the input signal.
    // For more information, do some research.
    // level_detection_type: LevelDetectionType,
}

impl Compressor {
    /// Processes a single input sample and returns the processed sample.
    pub fn process(&mut self, sample: &mut f32) {
        let dry_wet = self.params.dry_wet.smoothed.next();
        let input_gain = self.params.input_gain.smoothed.next();
        let output_gain = self.params.output_gain.smoothed.next();
        // modify with input gain
        *sample *= input_gain;
        // save a dry copy
        let pre_processed = *sample;
        // save a wet copy
        self.update_gain(*sample);
        let c = self.calculate_gain_reduction();
        let processed = *sample * c;
        // blend based on dry_wet
        let mut blended_output = (1.0 - dry_wet) * pre_processed + dry_wet * processed;

        // finally, modify with output gain
        blended_output *= output_gain;
        // and we're done!
        *sample = blended_output
    }

    /// Updates the internal gain of the compressor given an input sample.
    fn update_gain(&mut self, sample: f32) {
        let avg_gain = self.average_gain;
        // choose the input based on the desired level detection method
        let new_gain = self.rms.calculate_rms(sample);
        // based on if our incoming signal is increasing or decreasing, choose the filter coefficent to use.
        let theta = if new_gain > avg_gain {
            self.params.attack_time.smoothed.next()
        } else {
            self.params.release_time.smoothed.next()
        };

        let n = (1.0 - theta) * new_gain + theta * avg_gain;
        self.average_gain = n;
    }

    /// This function converts the internal average gain of the compressor to decibels, then uses a soft-knee equation to calculate the gain reduction.
    /// Returns a factor to multiply the input signal by.
    fn calculate_gain_reduction(&mut self) -> f32 {
        let threshold = self.params.threshold.smoothed.next();
        let ratio = self.params.ratio.smoothed.next();
        let knee_width = self.params.knee_width.smoothed.next();
        // first, we need to convert our gain to decibels.
        let input_db = gain_to_db(self.average_gain);
        // GAIN COMPUTER
        let reduced_db = {
            let difference = input_db - threshold;
            if 2.0 * (difference).abs() <= knee_width {
                // if we're within the knee range, use some special calculations!
                let gain_reduction = (difference + (knee_width / 2.0)).powi(2) / (2.0 * knee_width);
                input_db + (1.0 / ratio - 1.0) * gain_reduction
            } else if 2.0 * (difference) > knee_width {
                // above the knee, apply compression
                threshold + (difference / ratio)
            } else {
                // if we're below the knee/threshold
                input_db
            }
        };
        // to be totally honest, i'm not sure why this has to be done.
        let final_db = reduced_db - input_db;
        // convert back to linear space as a factor to multiply the input
        db_to_gain(final_db)
    }

    /// Construct a new `Compressor`. For more information on what each field actually does, see the `Compressor` docs.
    pub fn new(params: Arc<CompressorParams>) -> Self {
        let default_gain = 0.0;

        Compressor {
            params,
            average_gain: default_gain,
            rms: RmsLevelDetector::default(),
        }
    }
}
// TODO:
// tests for sanity check
pub fn calculate_filter_coefficient(input: f32) -> f32 {
    (-1.0 / (DEFAULT_SAMPLE_RATE * input)).exp()
    //1.0 - (-2200.0 / (SAMPLE_RATE * input)).exp()
}
pub fn calculate_time_from_coefficient(output_coeff: f32) -> f32 {
    1.0 / (DEFAULT_SAMPLE_RATE * (-output_coeff.ln()))
}
