use nih_plug::util::{db_to_gain_fast, gain_to_db_fast};
use std::collections::VecDeque;

use crate::params::CompressorParams;
// https://www.musicdsp.org/en/latest/Effects/169-compressor.html (not the best source)
// recommended:
// https://www.eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf

/// Struct to represent an RMS level detector. Uses a running sum and circular buffer.
pub struct RmsLevelDetector {
    squared_sum: f32,
    pub buffer: VecDeque<f32>,
}
impl Default for RmsLevelDetector {
    fn default() -> Self {
        Self {
            squared_sum: 0.0,
            buffer: VecDeque::new(),
        }
    }
}
impl RmsLevelDetector {
    pub fn calculate_rms(&mut self, input: f32) -> f32 {
        let old_sample = self.buffer.pop_front().unwrap();
        self.buffer.push_back(input);
        self.squared_sum += input.powi(2);
        self.squared_sum -= old_sample.powi(2);
        //
        // really weird workaround lol
        // sometimes (for some reason), squared_sum can be negative, and square rooting a negative leads to NaN
        //
        // when the compressor's gain state is NaN, the compressor will just never compress
        // TODO:
        // panic on nan
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
    /// Average input gain *in linear space*.
    /// The method of calculating this average gain is controlled by the `level_detection_type` field.
    average_gain: f32,
    /// RMS state.
    pub rms: RmsLevelDetector,
}

impl Compressor {
    /// Processes a single input sample and returns the processed sample.
    pub fn process(
        &mut self,
        sample: f32,
        params: &CompressorParams,
        shared_rms: &mut RmsLevelDetector,
        sample_rate: f32,
    ) -> (f32, f32, f32) {
        let threshold = params.threshold.value();
        let ratio = params.ratio.value();
        let knee_width = params.knee_width.value();

        // TODO:
        // there might be a way to optimize this..
        let attack_coeff =
            calculate_filter_coefficient(params.attack_time.smoothed.next(), sample_rate);
        let release_coeff =
            calculate_filter_coefficient(params.release_time.smoothed.next(), sample_rate);

        // blends the shared/independent RMS
        let rms_mix = params.rms_mix.smoothed.next();
        self.update_gain(sample, shared_rms, rms_mix, attack_coeff, release_coeff);

        // we can implement lookahead by using/processing an older sample while updating our gain state with the current sample
        // thus, we'll effectively have our internal gain state being updated *ahead* of the samples we're processing
        // this does introduce some latency, of course
        let lookahead_s = params.lookahead.smoothed.next();
        // TODO:
        // is there a better way than casting like this?
        let lookahead_index = (sample_rate * lookahead_s) as i32;
        let buffer_length = self.rms.buffer.len() as i32 - 1;
        let buffer_index = (buffer_length - lookahead_index).max(0) as usize;
        // if this is None, we have bigger problems
        let target_sample = *self.rms.buffer.get(buffer_index).unwrap();

        let c = self.calculate_gain_reduction(threshold, ratio, knee_width);
        (target_sample, target_sample * c, c)
    }

    /// Updates the internal gain of the compressor given an input sample.
    fn update_gain(
        &mut self,
        sample: f32,
        shared_rms: &mut RmsLevelDetector,
        rms_mix: f32,
        attack_coeff: f32,
        release_coeff: f32,
    ) {
        let avg_gain = self.average_gain;

        let shared_gain = shared_rms.calculate_rms(sample);
        let ind_gain = self.rms.calculate_rms(sample);

        let new_gain = (1.0 - rms_mix) * ind_gain + rms_mix * shared_gain;

        // based on if our incoming signal is increasing or decreasing, choose the filter coefficent to use.
        let theta = if new_gain > avg_gain {
            attack_coeff
        } else {
            release_coeff
        };

        let n = (1.0 - theta) * new_gain + theta * avg_gain;
        self.average_gain = n;
    }

    /// This function converts the internal average gain of the compressor to decibels, then uses a soft-knee equation to calculate the gain reduction.
    /// Returns a factor to multiply the input signal by.
    fn calculate_gain_reduction(&mut self, threshold: f32, ratio: f32, knee_width: f32) -> f32 {
        // first, we need to convert our gain to decibels.
        let input_db = gain_to_db_fast(self.average_gain);

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
        db_to_gain_fast(final_db)
    }

    /// Construct a new `Compressor`. For more information on what each field actually does, see the `Compressor` docs.
    pub fn new() -> Self {
        let default_gain = 0.0;
        Compressor {
            average_gain: default_gain,
            rms: RmsLevelDetector::default(),
        }
    }
}
pub fn calculate_filter_coefficient(input: f32, sample_rate: f32) -> f32 {
    (-1.0 / (sample_rate * input)).exp()
}
