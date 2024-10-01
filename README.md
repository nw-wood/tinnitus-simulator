### Tinnitus Sound Simulator

This program is designed to mask high-frequency pulsing sounds associated with tinnitus by generating randomized sound waves across multiple frequency ranges.
The random nature of this sound effectively blends with these sharp pulses and makes the intrusive tones indiscernible.

UHF, VHF, Low freq powered RFID crap, LRAD's, tinnitus, whatever - this will obscure it. CRT's too f*** those things. Mosquito cell phone apps. Screw em'.

Take great-wave.wav into Audacity or some other program to work with it further. There's lots of audio popping which is intended (for me personally) in some cases.

Additionally, I find it most effective to layer two of these generations on top of each other with one played in reverse.

### Note: Adjust volume carefully, as the output is intense by design and may require lower levels for comfort. Use at your own risk.

```rust
const AMPLITUDE: f64                    = 1.0;      //default volume, essentially, set to less than 1.0 for lower vol
const AMPLITUDE_RANDOM_OFFSET_MAX: f64  = 0.25;     //volume modulation
const FREQUENCY_MIN: u32                = 12000;    //lowest possible frequency
const FREQUENCY_MAX: u32                = 15000;    //highest possible frequency
const SAMPLE_RATE: u32                  = 44100;    //for most cases 44100 would be correct
const SAMPLE_RATE_RANDOM_SUB_MAX: u32   = 1000;     //modulate the sample rate for fun
const BITS_PER_SAMPLE: u16              = 16;       //can be set to 32 but untested
const CHANNELS: u16                     = 1;        //single channel tested, multichannel untested

const TOTAL_SAMPLES: u32 = 500;                     //total samples * average(min and max sustain) = total wav duration
const MAX_SUSTAIN_PER_SAMPLE: u64       = 500;
const MIN_SUSTAIN_PER_SAMPLE: u64       = 100;      //sustain means hold for longer _right?_

const WAV_NAME: &str = "great-wav";                 //like the art piece
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
