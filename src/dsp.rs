pub struct LPF {
    z: f32,
}

impl LPF {
    pub fn new() -> LPF {
        LPF { z: 0.0 }
    }
    pub fn process(&mut self, input: f32, cutoff: f32) -> f32 {
        let b = 1.0 - cutoff;
        let a = cutoff;
        // dsp
        self.z = (input * b) + (self.z * a);
        self.z
    }
}
