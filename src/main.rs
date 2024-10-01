use rand::Rng;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;
use std::thread;
use hound;

const AMPLITUDE: f64 = 1.0;
const AMPLITUDE_RANDOM_OFFSET_MAX: f64 = 0.25;
const FREQUENCY_MIN: u32 = 12000;
const FREQUENCY_MAX: u32 = 15000;
const SAMPLE_RATE: u32 = 44100;
const SAMPLE_RATE_RANDOM_SUB_MAX: u32 = 1000;

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
        let value = if (self.sample_clock % (self.sample_rate / self.freq)) < (self.sample_rate / self.freq / 2) {
            self.amplitude
        } else {
            -self.amplitude
        };
        self.sample_clock += 1;
        Some(value)
    }
}

impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn run_wave_generation(name: &str) {

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    //let some_int = rand::thread_rng().gen_range(10000..99999);

    let mut writer = hound::WavWriter::create("whack-wave.wav", spec).unwrap();

    let mut rng_generator = rand::thread_rng();
    let amplitude: f64 = AMPLITUDE + AMPLITUDE_RANDOM_OFFSET_MAX - rng_generator.gen_range(0.0..AMPLITUDE_RANDOM_OFFSET_MAX) * 2.0;

    let millis = rng_generator.gen_range(100..500);
    let duration= Duration::from_millis(millis);

    let frequency: u32 = rng_generator.gen_range(FREQUENCY_MIN..FREQUENCY_MAX);
    let sample_rate = SAMPLE_RATE - rng_generator.gen_range(0..SAMPLE_RATE_RANDOM_SUB_MAX);

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let wave = SquareWave::new(frequency, sample_rate, amplitude as f32)
        .take_duration(duration);

    println!("thread:{}/dir{}/amp{}/sr{}/freq{}", name, millis, amplitude.to_string().split_at(5).0, sample_rate, frequency);

    for sample in wave.clone().convert_samples::<i16>() {
        writer.write_sample(sample).unwrap();
    }

    //wave.convert_samples()
    sink.append(wave);
    sink.sleep_until_end();

}

fn main() {

    run_wave_generation("whack-wave");
    
    /*println!("running interference!");

    let thread1 = thread::spawn(|| {
        run_wave_generation("a");
    });

    let thread2 = thread::spawn(|| {
        run_wave_generation("b");
    });

    thread1.join().unwrap();
    thread2.join().unwrap();*/
}
