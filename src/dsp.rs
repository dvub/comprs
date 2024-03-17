use circular_buffer::CircularBuffer;
use nih_plug::util::{db_to_gain, gain_to_db};
// TODO:
// make NOT CONSTANT!
const SAMPLE_RATE: f32 = 44_100.0;
const BUFFER_SIZE: usize = (SAMPLE_RATE * 1e-3) as usize;

// TODO:
// add documentation!!

struct RmsPeakDetector {
    squared_sum: f32,
    buffer: CircularBuffer<BUFFER_SIZE, f32>,
}
impl Default for RmsPeakDetector {
    fn default() -> Self {
        Self {
            squared_sum: 0.0,
            buffer: CircularBuffer::<BUFFER_SIZE, f32>::from([0.0; BUFFER_SIZE]),
        }
    }
}
impl RmsPeakDetector {
    pub fn new(squared_sum: f32) -> Self {
        RmsPeakDetector {
            squared_sum,
            buffer: CircularBuffer::<BUFFER_SIZE, f32>::from([0.0; BUFFER_SIZE]),
        }
    }
    pub fn calculate_rms(&mut self, input: f32) -> f32 {
        // peak detection - RMS
        let old_sample = self.buffer.pop_back().unwrap();
        self.buffer.push_front(input);
        self.squared_sum += input.powi(2);
        self.squared_sum -= old_sample.powi(2);
        (self.squared_sum / BUFFER_SIZE as f32).sqrt() * 2.0f32.sqrt()
    }
}

pub struct Compressor {
    rms: RmsPeakDetector,
    pub average_gain: f32,
}
// https://www.musicdsp.org/en/latest/Effects/169-compressor.html (not the best source)
// recommended:
// https://www.eecs.qmul.ac.uk/~josh/documents/2012/GiannoulisMassbergReiss-dynamicrangecompression-JAES2012.pdf
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
        if ratio <= 1.0 {
            return (sample, 0.0);
        }

        let rms = self.rms.calculate_rms(sample);
        let attack = (-1.0 / (SAMPLE_RATE * attack_time)).exp();
        let release = (-1.0 / (SAMPLE_RATE * release_time)).exp();

        let theta = if rms > self.average_gain {
            attack
        } else {
            release
        };
        self.average_gain = (1.0 - theta) * rms + theta * self.average_gain;

        let avg_db = gain_to_db(self.average_gain);
        // GAIN COMPUTER
        let o_db = {
            let difference = avg_db - threshold;
            if 2.0 * (difference).abs() <= knee_width {
                //println!("AB");
                let gain_reduction = (difference + (knee_width / 2.0)).powi(2) / (2.0 * knee_width);
                avg_db + (1.0 / ratio - 1.0) * gain_reduction
            } else if 2.0 * (difference) > knee_width {
                //println!("A");
                threshold + (difference / ratio)
            } else {
                //println!("ABC");
                avg_db
            }
        };
        // APPLY
        let c_db = o_db - avg_db;
        let c = db_to_gain(c_db);
        (sample * c, o_db)
    }
}
impl Default for Compressor {
    fn default() -> Self {
        Self {
            average_gain: 0.0,
            rms: RmsPeakDetector::default(),
        }
    }
}
// TODO:
// make tests half decent
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
        let (compression_results, _): ((Vec<f32>, Vec<f32>), Vec<f32>) =
            compressed_data.into_iter().unzip();

        assert_eq!(compression_results.0, data);
    }
}
