use circular_buffer::CircularBuffer;
const SAMPLE_RATE: f32 = 44_100.0;

pub const BUFFER_SIZE: usize = (SAMPLE_RATE / 100.0) as usize;
pub struct Compressor {
    pub rms: f32,
    pub envelope: f32,
    pub squared_sum: f32,
    pub gain: f32,
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

        self.buf.push_front(sample);
        self.squared_sum += sample.powi(2);
        // println!("{}", self.squared_sum);

        let old_sample = self.buf.pop_back().unwrap();
        self.squared_sum -= old_sample.powi(2);
        // println!("{}", self.squared_sum);

        self.rms = (self.squared_sum / BUFFER_SIZE as f32).sqrt();

        let theta = {
            if self.rms > self.envelope {
                attack
            } else {
                release
            }
        };
        self.envelope = (1.0 - theta) * self.rms + theta * self.envelope;
        println!(
            "sample: {}, env: {}, theta: {}, rms: {}, squared_sum: {}, gain: {}",
            sample, self.envelope, theta, self.rms, self.squared_sum, self.gain
        );
        if self.envelope > threshold {
            self.gain -= (self.envelope - threshold) * slope;
        }

        sample * self.gain
    }
}
