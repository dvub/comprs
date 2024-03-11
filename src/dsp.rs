use circular_buffer::CircularBuffer;
const SAMPLE_RATE: f32 = 44_100.0;

pub const BUFFER_SIZE: usize = (SAMPLE_RATE / 100.0) as usize;
pub struct Compressor {
    pub average_gain: f32,
    pub squared_sum: f32,
    pub buf: CircularBuffer<BUFFER_SIZE, f32>,
}
impl Compressor {
    pub fn process(
        &mut self,
        sample: f32,
        attack_time: f32,
        release_time: f32,
        threshold: f32,
        slope: f32,
    ) -> f32 {
        let attack = (-1.0 / (SAMPLE_RATE * attack_time)).exp();
        let release = (-1.0 / (SAMPLE_RATE * release_time)).exp();
        // 1. first we need to know, essentially the average gain of the signal
        // we'll use an RMS for this
        // we'll keep track of this in a circular buffer
        self.buf.push_front(sample);
        let old_sample = self.buf.pop_back().unwrap();

        self.squared_sum += sample.powi(2);
        self.squared_sum -= old_sample.powi(2);
        // finish calculating RMS
        let rms = (self.squared_sum / BUFFER_SIZE as f32).sqrt();

        //
        let theta = if rms > self.average_gain {
            attack
        } else {
            release
        };

        self.average_gain = (1.0 - theta) * rms + theta * self.average_gain;

        let mut gain = 1.0f32;
        if self.average_gain > threshold {
            gain -= (self.average_gain - threshold) * slope;
        }

        println!(
            "sample: {}, env: {}, theta: {}, rms: {}, squared_sum: {}, gain: {}, FINAL: {}",
            sample,
            self.average_gain,
            theta,
            rms,
            self.squared_sum,
            gain,
            sample * gain
        );
        sample * gain
    }
}
