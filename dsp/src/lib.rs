use circular_buffer::CircularBuffer;
// TODO:
// consider using fast functions
use nih_plug::util::{db_to_gain, gain_to_db};
// TODO:
// make NOT CONSTANT!
const SAMPLE_RATE: f32 = 44_100.0;
const BUFFER_SIZE: usize = (SAMPLE_RATE * 1e-3) as usize;

// TODO:
// add documentation!!

// https://www.musicdsp.org/en/latest/Effects/169-compressor.html (not the best source)
// recommended:
// https://www.eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf

/// Struct to represent an RMS level detector. Uses a running sum and circular buffer.
struct RmsLevelDetector {
    squared_sum: f32,
    buffer: CircularBuffer<BUFFER_SIZE, f32>,
}
impl Default for RmsLevelDetector {
    fn default() -> Self {
        Self {
            squared_sum: 0.0,
            buffer: CircularBuffer::<BUFFER_SIZE, f32>::from([0.0; BUFFER_SIZE]),
        }
    }
}
impl RmsLevelDetector {
    pub fn calculate_rms(&mut self, input: f32) -> f32 {
        // peak detection - RMS
        let old_sample = self.buffer.pop_back().unwrap();
        self.buffer.push_front(input);
        self.squared_sum += input.powi(2);
        self.squared_sum -= old_sample.powi(2);
        (self.squared_sum / BUFFER_SIZE as f32).sqrt()
    }
}
/// Variants represent the different types of level detection that the compressor may use to update its internal gain.
pub enum LevelDetectionType {
    /// Use RMS of the signal
    Rms,
}
/// Struct to represent a dynamic range compressor. See documentation for each field to learn more.
pub struct Compressor {
    /// Average input gain *in linear space*.
    /// The method of calculating this average gain is controlled by the `level_detection_type` field.
    average_gain: f32,

    /// The coefficient to apply when the input signal is increasing. This value is computed depending on the type of attack/release (exponential/linear).
    pub attack_coefficient: f32,

    /// The filter coefficient to apply when the input signal is decreasing. This value is computed depending on the type of attack/release (exponential/linear).
    pub release_coefficient: f32,

    /// The threshold at which to begin applying compression **in decibels.**
    /// For example, a compressor with a threshold of -10db would compress signals above -10db.
    pub threshold: f32,

    /// The compression ratio as the left side of the ratio in decibels.
    /// For example, a ratio of `2.0` would be equivalent to a ratio of 2:1,
    /// which means that for every 2db that the signal is above the `threshold`, 1db will pass through.
    pub ratio: f32,

    /// The knee width **in decibels**. This smooths the transition between compression and no compression around the threshold.
    /// If you'd like a *hard-knee compressor*, set this value to `0.0`.
    pub knee_width: f32,

    /// Keep track of the RMS.
    rms: RmsLevelDetector,

    /// The type of level detection used to update the internal average gain of the compressor.
    /// It is generally suitable and computationally cheaper to use the `Simple` variant, which directly takes into account the input and smooths it.
    /// On the other hand, the `Rms` variant computes, well, the RMS of the input, and uses that to keep track of the input signal.
    ///  For more information, do some research.
    level_detection_type: LevelDetectionType,
}

impl Compressor {
    /// Processes a single input sample and returns the processed sample.
    pub fn process(&mut self, sample: f32) -> (f32, f32) {
        self.update_gain(sample);
        let c = self.calculate_gain_reduction();
        (sample * c, 0.0)
    }

    /// Updates the internal gain of the compressor given an input sample.
    fn update_gain(&mut self, sample: f32) {
        // choose the input based on the desired level detection method
        let new_gain = match self.level_detection_type {
            LevelDetectionType::Rms => self.rms.calculate_rms(sample),
        };
        // based on if our incoming signal is increasing or decreasing, choose the filter coefficent to use.
        let theta = if new_gain > self.average_gain {
            self.attack_coefficient
        } else {
            self.release_coefficient
        };
        // filter to smooth the average gain. this is also a good place to apply our attack and release.
        self.average_gain = (1.0 - theta) * new_gain + theta * self.average_gain;
    }

    pub fn get_average_gain(&self) -> f32 {
        self.average_gain
    }

    /// This function converts the internal average gain of the compressor to decibels, then uses a soft-knee equation to calculate the gain reduction.
    /// Returns a factor to multiply the input signal by.
    fn calculate_gain_reduction(&mut self) -> f32 {
        // first, we need to convert our gain to decibels.
        let input_db = gain_to_db(self.average_gain);
        // GAIN COMPUTER
        let reduced_db = {
            let difference = input_db - self.threshold;
            if 2.0 * (difference).abs() <= self.knee_width {
                // if we're within the knee range, use some special calculations!
                let gain_reduction =
                    (difference + (self.knee_width / 2.0)).powi(2) / (2.0 * self.knee_width);
                input_db + (1.0 / self.ratio - 1.0) * gain_reduction
            } else if 2.0 * (difference) > self.knee_width {
                // above the knee, apply compression
                self.threshold + (difference / self.ratio)
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
    /// ### Parameters
    /// - `attack_time`: the attack time of the compressor **in seconds**.
    /// - `release_time`: the release time of the compressor **in seconds**.
    /// - `threshold`: the threshold at which the compressor begins working **in decibels**.
    /// - `ratio`: the input/output ratio **in decibels**. The actual ratio would be `ratio`:1.
    /// - `knee_width`: knee width **in decibels**.
    /// - `level_detection_type`: the type of level detection that will be used to calculate the average gain of an input.
    pub fn new(
        attack_time: f32,
        release_time: f32,
        threshold: f32,
        ratio: f32,
        knee_width: f32,
        level_detection_type: LevelDetectionType,
    ) -> Self {
        let attack_time = Compressor::calculate_filter_coefficient(attack_time);
        let release_time = Compressor::calculate_filter_coefficient(release_time);
        let default_gain = 0.0;
        Compressor {
            average_gain: default_gain,
            attack_coefficient: attack_time,
            release_coefficient: release_time,
            threshold,
            ratio,
            knee_width,
            rms: RmsLevelDetector::default(),
            level_detection_type,
        }
    }

    /// Set the compressor threshold. (in decibels)
    pub fn set_threshold(&mut self, threshold: f32) {
        self.threshold = threshold;
    }

    /// Set the compressor ratio. (in decibels)
    pub fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio;
    }

    /// Set the compressor threshold. (in decibels)
    pub fn set_knee_width(&mut self, knee_width: f32) {
        self.knee_width = knee_width;
    }

    /// Set the compressor level detection.
    pub fn set_level_detection_type(&mut self, level_detection_type: LevelDetectionType) {
        self.level_detection_type = level_detection_type;
    }

    /// Set the compressor's attack time. (in seconds)
    pub fn set_attack_time(&mut self, attack_time: f32) {
        self.attack_coefficient = Compressor::calculate_filter_coefficient(attack_time);
    }

    /// Set the compressor's release time. (in seconds)
    pub fn set_release_time(&mut self, release_time: f32) {
        self.release_coefficient = Compressor::calculate_filter_coefficient(release_time);
    }

    fn calculate_filter_coefficient(input: f32) -> f32 {
        (-1.0 / (SAMPLE_RATE * input)).exp()
    }
}
// some default values that i chose just for fun
impl Default for Compressor {
    fn default() -> Self {
        let attack = (-1.0 / (SAMPLE_RATE * 0.01)).exp(); // 1ms attack
        let release = (-1.0 / (SAMPLE_RATE * 0.3)).exp(); // 300ms release - very slow
        Self {
            average_gain: 0.0, // unimportant
            rms: RmsLevelDetector::default(),
            attack_coefficient: attack,
            release_coefficient: release,
            threshold: 0.0,  // if we're clipping, apply
            ratio: 4.0,      // 4:1 compression
            knee_width: 5.0, // pretty big knee
            level_detection_type: LevelDetectionType::Rms,
        }
    }
}
// TODO:
// make tests... in general
