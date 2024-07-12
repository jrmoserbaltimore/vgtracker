
use std::f32::const::TAU;
use multiversion::multiversion;

#[derive(Oscillator)]
struct Triangle {
    duty: f32 = 0.5,
};

impl GetSample for Triangle {

    #[multiversion(targets("x86_64+avx", "x86_64+avx2", "x86_64+avx512f", "aarch64+neon"))]
    pub fn get_sample(&mut self) -> f32 {
        let sample = {
            // This computes up to 20 sines per sample
            if self.frequency > 512.0 {
                let mut s: f32 = 0.0;
                let mut h = 1u8;
                while h as f32 * self.frequency < 20000.0 {
                    // self.phase = xf, 5/TAU * -cos((2n+1)TAU(xf+0.25))/h**2
                    // Yes this is the integral of a square wave
                    s += (5.0/TAU) * -(h as f32 * TAU * (self.phase + 0.25)).cos() / h.pow(2) as f32;
                    h += 2;
                }
                s
            } else if self.phase < 0.25 {
                self.phase * 4.0
            } else if self.phase >= 0.25 && self.phase < 0.75 {
                (0.5 - self.phase)*4.0
            } else {
                (self.phase - 1)*4.0
            }
        };
        self.increment_phase();
        sample
    }
}
