use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::{Arc, Mutex};
use std::thread;

pub struct Audio {
    sample_rate: u32,
    channels: u16,
}

impl Audio {
    pub fn new(sample_rate: u32, channels: u16) -> Self {
        Self { sample_rate, channels }
    }

    pub fn play_audio(&self, shared_buffer: Arc<Mutex<Vec<i16>>>) {
        let host = cpal::default_host();
        let device = host.default_output_device().expect("No output device available");
        let config = device.default_output_config().unwrap();

        let sample_format = config.sample_format();
        let config: cpal::StreamConfig = config.into();

        let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

        let stream = match sample_format {
            cpal::SampleFormat::I16 => device.build_output_stream(
                &config,
                move |data: &mut [i16], _| {
                    let buffer = shared_buffer.lock().unwrap();
                    for (i, sample) in data.iter_mut().enumerate() {
                        *sample = buffer[i % buffer.len()];
                    }
                },
                err_fn,
            ),
            _ => panic!("Unsupported sample format"),
        }
        .expect("Unable to create audio stream");

        stream.play().expect("Failed to play stream");

        thread::spawn(move || loop {
            // Continuous playback, fills audio buffer in real code
            thread::sleep(std::time::Duration::from_millis(100));
        });
    }
}
