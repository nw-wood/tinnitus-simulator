extern crate rodio;

use rodio::{OutputStream, Source};
use std::time::{Duration, Instant};
use rand::Rng;

struct SquareWave {
    frequency: u32,
    sample_rate: u32,
    amplitude: f32,
    current_sample: u32,
}

impl SquareWave {
    fn new(frequency: u32, sample_rate: u32, amplitude: f32) -> SquareWave {
        SquareWave {
            frequency,
            sample_rate,
            amplitude,
            current_sample: 0,
        }
    }
}

fn main() {
    loop {
        let mut rng = rand::thread_rng();
        let random_duration = rng.gen_range(1..=10);



        let random_duration = random_duration as f64 / 10.0;
        let target_duration = Duration::from_millis((random_duration * 1000.0) as u64);
        let start_time = Instant::now(); //<- the moment in time this was executed
        loop {
            let elapsed = start_time.elapsed(); //<- get amount of time since called instant::now()
            if elapsed >= target_duration {
                play_wave();
                break;
            }
        }
    }
}

fn play_wave() {
    // Parameters for the square wave
    let amplitude = 0.05;     // Amplitude of the square wave
    let duration = 1.0;      // Duration in seconds

    let mut rng = rand::thread_rng();
    let duration: f32 = duration / rng.gen_range(1..10) as f32;
    let frequency = rng.gen_range(8000..14000);
    let sample_rate = rng.gen_range(30000..35000);
    println!("beep: {frequency}, {duration}, {amplitude}, {sample_rate}");

    // Set up the audio stream
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let source = SquareWave::new(frequency, sample_rate, amplitude)
        .take_duration(Duration::from_secs_f32(duration));

    // Play the square wave
    stream_handle.play_raw(source.convert_samples()).unwrap();

    // Keep the main thread alive until the sound finishes playing
    std::thread::sleep(Duration::from_secs_f32(duration));
}

impl Iterator for SquareWave {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let period = self.sample_rate / self.frequency;
        let value = if self.current_sample < period / 2 {
            self.amplitude
        } else {
            -self.amplitude
        };

        self.current_sample = (self.current_sample + 1) % period;
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