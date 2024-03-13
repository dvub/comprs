use circular_buffer::CircularBuffer;
const SAMPLE_RATE: f32 = 44_100.0;

pub const BUFFER_SIZE: usize = (SAMPLE_RATE * 0.01) as usize;
pub struct Compressor {
    pub average_gain: f32,
    pub squared_sum: f32,
    pub buf: CircularBuffer<BUFFER_SIZE, f32>,
}
// https://www.musicdsp.org/en/latest/Effects/169-compressor.html

impl Compressor {
    /// Processes an input sample. Returns the processed sample
    ///
    /// # Parameters
    /// - `sample`: a single sample to process.
    /// - `attack_time`: the attack of the compressor *in seconds*
    /// - `release_time`: the release of the compressor *in seconds*.
    /// - `threshold`: the level at which compression occurs. should be between 0 and 1.
    /// - `ratio`: the ratio of compression. Should be between 0 and 1.
    /// `ratio == 0` would be equivalent to a compression ratio of `1:1`, meaning no compression is applied at all.
    /// Thus, `ratio == 1` would be equivalent to a ratio of inf:1, in which everything above the threshold is totally compressed.
    /// A ratio of 0.5 would be 2:1.
    pub fn process(
        &mut self,
        sample: f32,
        attack_time: f32,
        release_time: f32,
        threshold: f32,
        ratio: f32,
        knee_width: f32,
    ) -> (f32, f32) {
        // a compressor is just a way to automatically lower volume of loud signals

        // so the first thing we need is the "loudness" or intensity of the signal
        // we're going to use RMS to appromixate the energy of the signal. there are a lot of reasons:
        // Human perception of loudness is related to the average power of a signal - so RMS is a good way to do this.
        // also RMS helps smooth out outlying samples or transients

        // RMS works with a window of samples - one way to implement this effectively is with a circular buffer.
        // circular buffers are good for continuous data - like this! (they also are an example of FIFO)
        // the size of the buffer in terms of samples depends on how large (in ms) the window is.
        // for example, 10 ms = 0.01s * 44100samples /s = 441 samples long
        // we can write the current sample to the front and fill the buffer,
        // and then when samples leave the window, we can pop those samples from the buffer (and update our RMS)

        // we should pop the last element before pushing to the front.
        // if we did this the other way around, we would push to a full buffer!
        // the buffer would then just overwrite the last element and we would lose a sample
        let old_sample = self.buf.pop_back().unwrap();
        self.buf.push_front(sample);
        // now, let's do calculations
        // this is squaring and updating our running sum
        self.squared_sum += sample * sample;
        self.squared_sum -= old_sample * old_sample;
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

        // now, the last and most important step
        // actually calculating the amount of gain to apply
        // if the loudness of the input exceeds our threshold, we'll compress
        let mut factor = 1.0;

        /*
                if (2.0 * (self.average_gain - threshold)) < -knee_width {
                    return (sample, 0.0);
                }*
                if (2.0 * (self.average_gain - threshold).abs()) <= knee_width {
                    let top = (self.average_gain - threshold + (knee_width / 2.0)).powi(2);
                    let out = self.average_gain + ((1.0 / ratio - 1.0) * (top / (2.0 * knee_width)));
                    factor = out;
                }
        */
        if (2.0 * (self.average_gain - threshold)) > knee_width {
            // here, we'll take into account our compression ratio
            let out = (threshold) + ((self.average_gain - threshold) / ratio);
            factor = out;
        }
        (sample * factor, sample)
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
