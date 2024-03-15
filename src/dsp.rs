use circular_buffer::CircularBuffer;
use nih_plug::util::{db_to_gain, gain_to_db};
const SAMPLE_RATE: f32 = 44_100.0;

pub const BUFFER_SIZE: usize = (SAMPLE_RATE * 0.01) as usize;
pub struct Compressor {
    pub average_gain: f32,
    pub squared_sum: f32,
    pub buf: CircularBuffer<BUFFER_SIZE, f32>,
}
// https://www.musicdsp.org/en/latest/Effects/169-compressor.html

impl Compressor {
    pub fn process(
        &mut self,
        sample: f32,
        attack_time: f32,
        release_time: f32,
        threshold: f32,
        ratio: f32,
        knee_width: f32,
    ) -> (f32, f32) {
        // first, we need to be in decibel land
        let sample_db = gain_to_db(sample);
        // gain computer
        let y_g = {
            if 2.0 * (sample_db - threshold) < -knee_width {
                sample_db
            } else if 2.0 * (sample_db - threshold).abs() <= knee_width {
                sample_db
                    + (1.0 / ratio - 1.0) * (sample_db - threshold + knee_width / 2.0).powi(2)
                        / (2.0 * knee_width)
            } else if 2.0 * (sample_db - threshold) > knee_width {
                threshold + (sample_db - threshold) / ratio
            } else {
                sample_db
            }
        };
        let x_l = sample_db - y_g;

        let old_sample = self.buf.pop_back().unwrap();
        self.buf.push_front(x_l);
        // now, let's do calculations
        // this is squaring and updating our running sum
        self.squared_sum += x_l.powi(2);
        self.squared_sum -= old_sample.powi(2);
        // finish calculating RMS

        let rms = (self.squared_sum / BUFFER_SIZE as f32).sqrt();

        let attack = (-1.0 / (SAMPLE_RATE * attack_time)).exp();
        let release = (-1.0 / (SAMPLE_RATE * release_time)).exp();
        // choose - do we want to use attack or release coefficient?
        let theta = if rms > self.average_gain {
            attack
        } else {
            release
        };
        // combines the current rms value and the previous average gain to get a smooth result
        self.average_gain = (1.0 - theta) * rms + theta * self.average_gain;
        let y_l = self.average_gain;
        let c_db = -y_l;
        let c = db_to_gain(c_db);
        let y_db = sample_db + c_db + 0.0;
        let y = db_to_gain(y_db) * sample.signum();

        (sample * c, 0.0)
    }
}
impl Default for Compressor {
    fn default() -> Self {
        Self {
            average_gain: 0.0,
            squared_sum: 0.0,
            buf: CircularBuffer::<BUFFER_SIZE, f32>::from([0.0; BUFFER_SIZE]),
        }
    }
}
