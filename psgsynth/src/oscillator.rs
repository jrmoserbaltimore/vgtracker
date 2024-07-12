
#[derive(default)]
struct Oscillator {
    phase: f32,
    frequency: f32,
    sample_rate: u32,
};

impl Oscillator {
    pub fn set_frequency(&mut self, frequency: f32) {
        assert!(frequency >= 0, "Frequency {}Hz is negative!", frequency);
        assert!(frequency <= 5500, "Frequencies above 5500Hz not supported!  {}Hz requested!", frequency);
        self.frequency = frequency;
    }

    pub fn set_sample_rate(&mut self, sample_rate: u32) {
        self.sample_rate = sample_rate;
    }

    fn increment_phase(&mut self) {
        self.phase += (self.frequency / self_sample_rate as f32).fract();
    }
}

pub trait GetSample {
    pub fn get_sample(&mut self) -> f32;
}
