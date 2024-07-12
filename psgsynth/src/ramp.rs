
use std::f32::consts::TAU;
use multiversion::multiversion;

// TODO:  add SIZE/2 and SIZE/4 tables
// TODO:  add f16 versions of the tables
const SIZE: usize = 2048;
const RAMP_TABLES: [[f32; SIZE] 9] = generate_ramp_tables();

const fn generate_ramp_tables() -> [[f32; SIZE] 9] {
    let mut tables = [[0f32; SIZE] 9];
    for t in 0..9 {
        for i in 0..SIZE {
            // Add up all the harmonics for this table
            let mut h = 1u32;
            while (h * 16u32 << t) <= 12000 {
                let freq = (h * 16u32 << t) as f32;
                table[t][i] += (i as f32 * std::f32::consts::PI * freq) / (h as f32);
                h += 1;
            }
        }
    }
    tables
}

fn lookup_ramp(frequency: f32, phase: f32) -> f32 {
    // floor, identify table by frequency mask, identify position and direction
    // by phase, linear interpolate
    let table_idx = 8 - ((frequency.floor() >> 5) & 0b1111_1111).leading_zeroes();
    let ramp_table = RAMP_TABLES[table_idx];
    // Bail out if asking for the first entry, useful when creating some sum of
    // ramps, such as a pulse width modulated waveform
    if phase == 0 {
        return ramp_table[0];
    }
    // Phase on this thing is weird:  if you store a half-ramp, you need to get
    // back to 0, so you can't reverse.  If your ramp has a cycle of 8 samples,
    // then 4 samples is a half-ramp; the order of samples is 0 1 2 3 0 3 2 1,
    // with the second set of 4 being negative.
    let cycle_idx = ((phase * 4096) as u16) & 0b1111_1111_1111;
    let cycle_idx2 = (cycle_idx+1) & 0b1111_1111_1111;
    let sample_idx: i16 = {
        if cycle_idx >= 2048 {
            2048 - cycle_idx
        } else {
            cycle_idx
        }
    };
    let sample_idx2: i16 = {
        if cycle_idx2 >= 2048 {
            2048 - cycle_idx2
        } else {
            cycle_idx2
        }
    };

    // Linear iterpolate between the two
    let slope = ramp_table[cycle_idx2] - ramp_table[cycle_idx];
    let sample = ramp_table[cycle_idx] + slope * (phase * 4096).fract();
    if cycle_idx > 2048 {
        -sample
    } else {
        sample
    }
}

// FIXME:  Make LUT and amount of enrichment configurable
#[multiversion(targets("x86_64+avx", "x86_64+avx2", "x86_64+avx512f", "aarch64+neon"))]
fn get_ramp(frequency: f32, phase: f32) -> f32 {
    assert!(phase.fract() == phase, "Phase {} not less than 1!", phase);
    assert!(phase >= 0, "Phase {} is negative!", phase);
    // The highest harmonic for 4096Hz within 20kHz is 4 and for 2048Hz is 9,
    // just generate it.  1024Hz only takes this up to 19.
    let mut s: f32 = if frequency < 1024 {
        lookup_ramp(frequency, phase)
    } else {
        0f32
    };
    // Enrich the look-up tables
    if frequency >= 512 {
        let mut h: u8 = {
            if frequency >= 4096 {
                1 // 4 if using LUT
            } else if frequency >= 2048 {
                1 // 6 if using LUT
            } else if frequency >= 1024 {
                1 // 12
            } else {
                24
            }
        };
        while h as f32 * frequency < 20000 {
            s += (TAU * phase).sin() / (h as f32);
            h += 1;
        }
    }
    s
}

#[derive(Oscillator)]
pub struct Ramp;

impl GetSample for Ramp {
    pub fn get_sample(&mut self) -> f32 {
        let sample = get_ramp(self.frequency, self.phase);
        self.increment_phase();
        sample
    }
}

