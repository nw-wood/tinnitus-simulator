use rand::Rng;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;
use hound;

const AMPLITUDE: f64 = 1.0;                     //default volume, essentially, set to less than 1.0 for lower vol
const AMPLITUDE_RANDOM_OFFSET_MAX: f64 = 0.25;  //volume modulation
const FREQUENCY_MIN: u32 = 12000;               //lowest possible frequency
const FREQUENCY_MAX: u32 = 15000;               //highest possible frequency
const SAMPLE_RATE: u32 = 44100;                 //for most cases 44100 would be correct
const SAMPLE_RATE_RANDOM_SUB_MAX: u32 = 1000;   //modulate the sample rate for fun
const BITS_PER_SAMPLE: u16 = 16;                //can be set to 32 but untested
const CHANNELS: u16 = 1;                        //single channel tested, multichannel untested

const TOTAL_SAMPLES: u32 = 500;                 //total samples * average(min and max sustain) = total wav duration
const MAX_SUSTAIN_PER_SAMPLE: u64 = 500;
const MIN_SUSTAIN_PER_SAMPLE: u64 = 100;        //sustain means hold for longer _right?_

const WAV_NAME: &str = "great-wav";             //like the art piece

#[derive(Clone)]
struct SquareWave {
    freq: u32,
    sample_rate: u32,
    amplitude: f32,
    sample_clock: u32,
}

impl SquareWave {
    fn new(freq: u32, sample_rate: u32, amplitude: f32) -> Self {
        SquareWave {
            freq,
            sample_rate,
            amplitude,
            sample_clock: 0,
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let value = if (self.sample_clock % (self.sample_rate / self.freq)) < (self.sample_rate / self.freq / 2) { self.amplitude
        } else { -self.amplitude };
        self.sample_clock += 1;
        Some(value)
    }
}

impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { None }
}

fn main() {
    
    let spec = hound::WavSpec {
        channels: CHANNELS,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: BITS_PER_SAMPLE,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create(format!("{WAV_NAME}.wav"), spec).unwrap();
    let mut rng_generator = rand::thread_rng();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    for current_iter in 0..TOTAL_SAMPLES {
        
        let amplitude: f64 = AMPLITUDE + AMPLITUDE_RANDOM_OFFSET_MAX - rng_generator.gen_range(0.0..AMPLITUDE_RANDOM_OFFSET_MAX) * 2.0;
        let millis = rng_generator.gen_range(MIN_SUSTAIN_PER_SAMPLE..MAX_SUSTAIN_PER_SAMPLE);
        let duration= Duration::from_millis(millis);
        let frequency: u32 = rng_generator.gen_range(FREQUENCY_MIN..FREQUENCY_MAX);
        let sample_rate = SAMPLE_RATE - rng_generator.gen_range(0..SAMPLE_RATE_RANDOM_SUB_MAX);

        let wave = SquareWave::new(frequency, sample_rate, amplitude as f32).take_duration(duration);

        println!("{}: generated sample #: {current_iter} -> dir: {}, amp: {}, sr: {}, freq: {}", name, millis, amplitude.to_string().split_at(5).0, sample_rate, frequency);

        writer.write_sample(0 as i16).unwrap();

        for sample in wave.clone().convert_samples::<i16>() {
            writer.write_sample(sample).unwrap();
        }

        sink.append(wave);

        sink.sleep_until_end();

    }

    writer.finalize().unwrap();

    println!("generation finished, check working dir");

}
