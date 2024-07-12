Oscillators for several PSGs, primarily NES.

* [ ] Pulse wave (2A03 APU, MMC5, VRC6, 8910)
* [ ] Triangle wave (2A03 APU)
* [ ] Saw wave (VRC6)
* [ ] SIMD

Table of chips using these:

| Chip | Pulse | Saw | Triangle | PCM | FM | Wavetable |
| --- | :---: | :---: | :---: | :---: | :---: | :---: |
| 2A03 | X | | X | X | | |
| MMC5 | X | | | X | | |
| VRC6 | X | X | | | | |
| 8910 | X | | | | | |
| VRC7 | | | | | X |
| Namco 163 | | | | | | X |

Generating a pulse wave is relatively easy:  two saw waves at an offset period
are generated, then one is subtracted from the other. Half a saw wave can be
stored to compute a full period. This must be adjusted by adding the offset saw
wave at time=0 to keep the pulse wave properly centered.

To achieve this, a set of look-up tables of band-limited saw waves is used.
The below table shows the top harmonic at the highest frequency produced via
the given look-up table.  In general, the top harmonic is right under 12,000Hz
at the base frequency and reaches to just below 24,000Hz at the top frequency.

| Fundamental | Max of harmonic | Top frequency | Top dB drop|
| --- | --- | --- | --- |
| 16Hz | 749 | 23,968Hz | -57.5dB |
| 32Hz | 373 | 23,872Hz | -51.5dB |
| 64Hz | 187 | 23,936Hz | -45.5dB |
| 128Hz | 93 | 23,808Hz | -39.4dB |
| 256Hz | 45 | 23,040Hz | -33.1dB |
| 512Hz | 23 | 23,552Hz | -27.2dB |
| 1024Hz| 11 | 22,528Hz | -20.8dB |
| 2048Hz | 5 | 20,480Hz | -14.0dB |
| 4096Hz | 3 | 24,576Hz |  -9.5dB |

The last can reach 6,000Hz or F#8--6 higher than the highest piano key--without
aliasing at 48,000Hz sample rate.  These can also climb 16% higher than their
top frequency, or almost 3 half steps, before aliasing extends below 20,000Hz.
Higher fundamentals can be enhanced by calculating additional sines up to
20,000Hz.  The number of additional sines required is given below.

| Key | Max harmonic | Extra sines |
| --- | --- | --- |
|256Hz | 89 | 44 |
| C4 | 77 | 32 |
| C#4 | 71 | 26 |
| D4 | 67 | 22 |
| D#4 | 63 | 18 |
| E4 | 59 | 14 |
| F4 | 57 | 12 |
| F#4 | 53 | 8 |
| G4  | 51 | 6 |
| G#4 | 47 | 2 |
| A4 | 0 | 0 |
| 512Hz | 39 | 16 |
| C5 | 37 | 14 |
| C#5 | 35 | 12 |
| 1024Hz | 19 | 8 |
| 2048Hz | 9 | 4 |

Each table contains 2,048 entries of 32-bit floating point data, or 8KiB of
data, representing one half of a saw wave.  Steps between entries produce
high-frequency noise, creating aliasing; this noise is filtered by using
linear interpolation, a form of low-pass filtering.

Because these look-up tables can have an impact on cache, with L1 CPU cache
being a mere 32KiB, there is an option to compute some ranges on the fly
entirely.  Computing all of 512Hz and above requires calculating at most 39
sines per sample or 1,872,000 sines per second per channel.  With SIMD
instructions computing 8 at once, this is 234,000 batches per second.
Computing 512Hz and above leaves only 40KiB of tables.

Pulse width modulated oscillators double the number of sines computed.

The naive triangle can be used for frequencies up to 512Hz, about C5, with a
drop of -68dB for the 47th harmonic--the first aliased into the audible range.
Above that, the triangle is composed of no more than 20 sines, and is
generated as a sum of sines.
