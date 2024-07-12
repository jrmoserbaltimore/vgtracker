
#[derive(Oscillator)]
struct PWM {
    duty: f32 = 0.5,
};

impl GetSample for PWM {

    pub fn get_sample(&mut self) -> f32 {
        // Add the ramp at phase=duty to align it properly vertically
        let sample = get_ramp(self.frequency, self.phase)
            - get_ramp(self.frequency, (self.phase + 1.0 - self.duty))
            + get_ramp(self.frequency, 1.0 - self.duty);
        self.increment_phase();
        sample
    }
}

impl PWM {
    pub fn set_duty_cycle(&mut self, duty: f32) {
        assert!(duty.fract() == duty, "Duty cycle {} not less than 1!", duty);
        assert!(duty >= 0, "Duty cycle {} is negative!", duty);

        self.duty = duty;
    }
}
