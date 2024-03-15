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
            let difference = sample_db - threshold;
            if 2.0 * (difference) > knee_width {
                //println!("A");
                threshold + (difference / ratio)
            } else if 2.0 * (difference).abs() <= knee_width {
                //println!("AB");
                let gain_reduction = (difference + (knee_width / 2.0)).powi(2) / (2.0 * knee_width);
                sample_db + (1.0 / ratio - 1.0) * gain_reduction
            } else {
                //println!("ABC");
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
        self.average_gain = (1.0 - theta) * rms + theta * self.average_gain;
        let c_db = -self.average_gain;
        let c = db_to_gain(c_db);
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

#[cfg(test)]
mod tests {
    use nih_plug::util::db_to_gain;

    use super::Compressor;

    #[test]
    fn a() {
        let len = 44_100;

        let mut data: Vec<f32> = vec![0.0; len];
        for (index, value) in data.iter_mut().enumerate() {
            let q = len / 4;
            let factor = {
                if index >= (q * 3) {
                    -5.0
                } else if index >= (q * 2) {
                    0.0
                } else if index >= (q) {
                    -9.0
                } else {
                    -12.0
                }
            };
            *value = (index as f32 * 0.1).sin() * db_to_gain(factor);
        }
        let mut comp = Compressor::default();
        let threshold = -10.0;
        let ratio = 1.0;
        let knee = 0.01;
        let attack_time = 0.0;
        let release_time = 0.0;
        let compressed_data: Vec<((f32, f32), f32)> = data
            .iter()
            .enumerate()
            .map(|(_i, sample)| {
                let result =
                    comp.process(*sample, attack_time, release_time, threshold, ratio, knee);

                (result, comp.average_gain)
            })
            .collect();
        let (compression_results, envelopes): ((Vec<f32>, Vec<f32>), Vec<f32>) =
            compressed_data.into_iter().unzip();

        assert_eq!(compression_results.0, data);
    }
}
