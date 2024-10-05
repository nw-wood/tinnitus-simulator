use rand::Rng;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;
use hound;

const AMPLITUDE: f64 = 0.9;
const AMPLITUDE_RANDOM_OFFSET_MAX: f64 = 0.1;
const FREQUENCY_MIN: u32 = 12000;
const FREQUENCY_MAX: u32 = 15500;
const SAMPLE_RATE: u32 = 44100;
const SAMPLE_RATE_RANDOM_SUB_MAX: u32 = 1; //don't do this
const BITS_PER_SAMPLE: u16 = 16;
const CHANNELS: u16 = 1;
const TOTAL_SAMPLES: u32 = 250;
const MAX_SUSTAIN_PER_SAMPLE: u64 = 500;
const MIN_SUSTAIN_PER_SAMPLE: u64 = 100;
const WAV_NAME: &str = "great-wav";
const FADE_MS: f32 = 50.0; //higher will result in clipping

#[derive(Clone)]
struct SquareWave {
    freq: u32,
    sample_rate: u32,
    amplitude: f32,
    sample_clock: u32,
    duration_samples: u32,
    fade_samples: u32,
}

impl SquareWave {
    fn new(freq: u32, sample_rate: u32, amplitude: f32, duration: Duration) -> Self {
        let fade_samples = (FADE_MS / 1000.0 * sample_rate as f32) as u32;
        SquareWave {
            freq,
            sample_rate,
            amplitude,
            sample_clock: 0,
            duration_samples: (duration.as_secs_f32() * sample_rate as f32) as u32,
            fade_samples,
        }
    }

    fn get_fade_multiplier(&self) -> f32 {
        if self.sample_clock < self.fade_samples {
            // Fade in
            self.sample_clock as f32 / self.fade_samples as f32
        } else if self.sample_clock > self.duration_samples - self.fade_samples {
            // Fade out
            (self.duration_samples - self.sample_clock) as f32 / self.fade_samples as f32
        } else {
            // No fade
            1.0
        }
    }
}

impl Iterator for SquareWave {
    type Item = f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.sample_clock >= self.duration_samples {
            return None;
        }

        let period_samples = self.sample_rate / self.freq;
        let value = if (self.sample_clock % period_samples) < (period_samples / 2) {
            self.amplitude
        } else {
            -self.amplitude
        };

        // Apply fade
        let faded_value = value * self.get_fade_multiplier();
        
        self.sample_clock += 1;
        Some(faded_value)
    }
}

impl Source for SquareWave {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { 
        Some(Duration::from_secs_f32(self.duration_samples as f32 / self.sample_rate as f32))
    }
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
        let amplitude: f64 = AMPLITUDE + AMPLITUDE_RANDOM_OFFSET_MAX 
            - rng_generator.gen_range(0.0..AMPLITUDE_RANDOM_OFFSET_MAX) * 2.0;
        let millis = rng_generator.gen_range(MIN_SUSTAIN_PER_SAMPLE..MAX_SUSTAIN_PER_SAMPLE);
        let duration = Duration::from_millis(millis);
        let frequency: u32 = rng_generator.gen_range(FREQUENCY_MIN..FREQUENCY_MAX);
        let sample_rate = SAMPLE_RATE - rng_generator.gen_range(0..SAMPLE_RATE_RANDOM_SUB_MAX);
        
        let wave = SquareWave::new(frequency, sample_rate, amplitude as f32, duration);
        
        println!("Generated sample #{}: duration: {}ms, amplitude: {:.3}, sample_rate: {}, frequency: {}Hz", 
                current_iter, millis, amplitude, sample_rate, frequency);

        for sample in wave.clone().convert_samples::<i16>() {
            writer.write_sample(sample).unwrap();
        }
        
        sink.append(wave);
        sink.sleep_until_end();
    }
    
    writer.finalize().unwrap();
    println!("Generation finished, check working directory for output file");
}