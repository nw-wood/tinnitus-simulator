### Tinnitus Sound Simulator

This program is designed to mask high-frequency pulsing sounds associated with tinnitus by generating randomized sound waves across multiple frequency ranges.
The random nature of this sound effectively blends with these sharp pulses and makes the intrusive tones indiscernible.

Take great-wave.wav into Audacity or some other program to work with it further.

UHF, VHF, Low freq powered RFID crap, LRAD's, tinnitus, whatever - this will obscure it. CRT's too f*** those things. Mosquito cell phone apps. Screw em'.

Take great-wave.wav into Audacity or some other program to work with it further. There's lots of audio popping which is intended (for me personally) in some cases.

### Note: Adjust volume carefully, as the output is intense by design and may require lower levels for comfort. Use at your own risk.

```rust
const AMPLITUDE: f64                    = 0.9;          //wave height target
const AMPLITUDE_RANDOM_OFFSET_MAX: f64  = 0.1;          //ranomize wave height by some amount
const FREQUENCY_MIN: u32                = 12000;        //minimum speed of sample oscilation
const FREQUENCY_MAX: u32                = 15500;        //maximum speed of sample osc
const SAMPLE_RATE: u32                  = 44100;        //sample rate, 44100 over one second
const SAMPLE_RATE_RANDOM_SUB_MAX: u32   = 1;            //don't do this, but this can be set higher for sample rate shenans
const BITS_PER_SAMPLE: u16              = 16;           //power of 16, 32, whatever else hound/rodio is happy with
const CHANNELS: u16                     = 1;            //2 for stereo
const TOTAL_SAMPLES: u32                = 100;          //amount of generations to append to the wav
const MAX_SUSTAIN_PER_SAMPLE: u64       = 200;          //each pulses' duration top end
const MIN_SUSTAIN_PER_SAMPLE: u64       = 40;           //as shmall as the duration can be
const WAV_NAME: &str                    = "great-wav";  //name for the file that gets spit out
const FADE_MS: f32                      = 20.0;         //sine based easing, this should be at most half of the min sustain per sample
const SAMPLE_DELAY: f64                 = 1.50;         //random delay between each sample generated 0.0-SAMPLE_DELAY
```

```
5249 4646 34cb 8700 5741 5645 666d 7420
1000 0000 0100 0100 44ac 0000 8858 0100
0200 1000 6461 7461 10cb 8700 bc97 cf11
31ee 0080 cf11 31ee 0080 cf11 31ee 0080
cf11 31ee 0080 cf11 31ee 0080 cf11 31ee
0080 cf11 31ee 0080 cf11 31ee 0080 cf11
31ee 0080 cf11 31ee 0080 cf11 31ee 0080
cf11 31ee 0080 cf11 31ee 0080 cf11 31ee
0080 cf11 31ee 0080 cf11 31ee 0080 cf11
31ee 0080 cf11 31ee 0080 cf11 31ee 0080
cf11 31ee 0080 cf11 31ee 0080 cf11 31ee
0080 cf11 31ee 0080 cf11 31ee 0080 cf11
31ee 0080 cf11 31ee 0080 cf11 31ee 0080
cf11 31ee 0080 cf11 31ee 0080 cf11 31ee
0080 cf11 31ee 0080 cf11 31ee 0080 cf11
```
