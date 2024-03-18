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
pub enum LevelDetectionType {
    LowPassFilter,
    Rms,
}
pub struct Compressor {
    pub gain: f32,
    attack_time: f32,
    release_time: f32,
    threshold: f32,
    ratio: f32,
    knee_width: f32,
    rms: RmsLevelDetector,
    level_detection_type: LevelDetectionType,
}

impl Compressor {
    pub fn process(&mut self, sample: f32) -> (f32, f32) {
        self.update_gain(sample);
        let c = self.calculate_gain_reduction();
        (sample * c, 0.0)
    }
    pub fn update_gain(&mut self, sample: f32) {
        let gain = match self.level_detection_type {
            LevelDetectionType::LowPassFilter => sample,
            LevelDetectionType::Rms => self.rms.calculate_rms(sample),
        };
        let theta = if gain > self.gain {
            self.attack_time
        } else {
            self.release_time
        };
        self.gain = (1.0 - theta) * gain + theta * self.gain;
    }
    pub fn calculate_gain_reduction(&mut self) -> f32 {
        let input_db = gain_to_db(self.gain);
        // GAIN COMPUTER
        let reduced_db = {
            let difference = input_db - self.threshold;
            if 2.0 * (difference).abs() <= self.knee_width {
                //println!("AB");
                let gain_reduction =
                    (difference + (self.knee_width / 2.0)).powi(2) / (2.0 * self.knee_width);
                input_db + (1.0 / self.ratio - 1.0) * gain_reduction
            } else if 2.0 * (difference) > self.knee_width {
                //println!("A");
                self.threshold + (difference / self.ratio)
            } else {
                //println!("ABC");
                input_db
            }
        };
        // APPLY
        let final_db = reduced_db - input_db;
        db_to_gain(final_db)
    }
    /// Construct a new `Compressor`.
    /// ### Parameters
    /// - `attack_time`: the attack time of the compressor **in seconds**.
    /// - `release_time`: the release time of the compressor **in seconds**.
    /// - `threshold`: the threshold at which the compressor begins working **in decibels**.
    /// - `ratio`: the input/output ratio **in decibels**. The actual ratio would be `ratio`:1.
    /// - `knee_width`: knee width **in decibels**.
    /// - `level_detection_type`: the type of level detection that will be used to calculate the average gain of inputs.
    pub fn new(
        attack_time: f32,
        release_time: f32,
        threshold: f32,
        ratio: f32,
        knee_width: f32,
        level_detection_type: LevelDetectionType,
    ) -> Self {
        let attack_time = (-1.0 / (SAMPLE_RATE * attack_time)).exp();
        let release_time = (-1.0 / (SAMPLE_RATE * release_time)).exp();
        let default_gain = 0.0;
        Compressor {
            gain: default_gain,
            attack_time,
            release_time,
            threshold,
            ratio,
            knee_width,
            rms: RmsLevelDetector::default(),
            level_detection_type,
        }
    }
    pub fn set_threshold() {
        todo!()
    }
    pub fn set_ratio() {
        todo!()
    }
    pub fn set_knee_width() {
        todo!()
    }
    pub fn set_level_detection_type() {
        todo!()
    }
    pub fn set_attack_time() {
        todo!()
    }
    pub fn set_release_time() {
        todo!()
    }
}
impl Default for Compressor {
    fn default() -> Self {
        let attack = (-1.0 / (SAMPLE_RATE * 0.01)).exp();
        let release = (-1.0 / (SAMPLE_RATE * 0.3)).exp();
        Self {
            gain: 0.0,
            rms: RmsLevelDetector::default(),
            attack_time: attack,
            release_time: release,
            threshold: 0.0,
            ratio: 1.0,
            knee_width: 5.0,
            level_detection_type: LevelDetectionType::LowPassFilter,
        }
    }
}
// TODO:
// make tests half decent
