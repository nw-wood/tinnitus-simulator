### Tinnitus Sound Simulator

This program is designed to mask high-frequency pulsing sounds associated with tinnitus, CRT's, mosquito generator apps, and long/short range LRAD and microwave abuse by generating randomized sound waves across multiple frequencies in or around the ranges the listed annoyances can produce.

The randomn nature of this sound effectively blends with the hyper-pulsing 'tinnitus', making the intrusive tones indiscernible.

Take great-wave.wav into Audacity or some other program to work with it further. There's lots of audio popping which is intended for me in some cases. Additionally, I find it most effective to layer two of these generations on top of each other with one played in reverse.

### Note: Adjust volume carefully, as the output is intense by design and may require lower levels for comfort. Use at your own risk.

```rust
const AMPLITUDE: f64 = 1.0;
const AMPLITUDE_RANDOM_OFFSET_MAX: f64 = 0.25;
const FREQUENCY_MIN: u32 = 12000;
const FREQUENCY_MAX: u32 = 15000;
const SAMPLE_RATE: u32 = 44100;
const SAMPLE_RATE_RANDOM_SUB_MAX: u32 = 1000;
```

```
running interference!
interference thread 2; dur: 182, amp: 0.836, sr: 43958, freq: 14159
interference thread 1; dur: 361, amp: 0.809, sr: 43516, freq: 14322
interference thread 2; dur: 100, amp: 0.908, sr: 43133, freq: 12827
interference thread 2; dur: 390, amp: 0.987, sr: 43256, freq: 13229
interference thread 1; dur: 406, amp: 1.096, sr: 43659, freq: 12007
interference thread 2; dur: 103, amp: 0.933, sr: 43240, freq: 13768
interference thread 2; dur: 375, amp: 0.895, sr: 44066, freq: 13031
interference thread 1; dur: 197, amp: 1.005, sr: 43368, freq: 13511
interference thread 1; dur: 275, amp: 0.989, sr: 43529, freq: 12091
interference thread 2; dur: 436, amp: 0.967, sr: 43236, freq: 14747
interference thread 1; dur: 395, amp: 0.900, sr: 43628, freq: 14442
interference thread 2; dur: 392, amp: 1.131, sr: 43121, freq: 13739
interference thread 1; dur: 258, amp: 1.200, sr: 43781, freq: 13401
interference thread 2; dur: 322, amp: 1.119, sr: 43387, freq: 13575
interference thread 1; dur: 411, amp: 0.879, sr: 43239, freq: 13077
interference thread 2; dur: 460, amp: 0.779, sr: 43273, freq: 14529
interference thread 1; dur: 171, amp: 1.043, sr: 43843, freq: 12470
interference thread 1; dur: 450, amp: 0.979, sr: 43127, freq: 14915
interference thread 2; dur: 240, amp: 1.150, sr: 44038, freq: 13402
```
