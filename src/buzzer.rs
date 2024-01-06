use sdl2::{audio::{AudioCallback, AudioSpecDesired, AudioDevice}, AudioSubsystem};
use std::f64::consts::{PI, TAU};

const DEFAULT_FREQ: i32 = 44100;
const DEFAULT_SAMPLES: u16 = 512;

#[derive(Debug)]
struct SquareWave {
    time: f64,
    volume: f64,
    frequency: f64,
}

fn square_wave(angle: f64) -> f64 {
    if angle % TAU < PI { 1.0 } else { -1.0 }
}

impl AudioCallback for SquareWave {
    type Channel = u8;

    fn callback(&mut self, output: &mut [Self::Channel]) {
        for out in output.iter_mut() {
            let x = 2. * PI * self.time * self.frequency;
            let half_max = Self::Channel::MAX as f64 / 2.;
            *out = ((half_max * self.volume * square_wave(x)) + half_max) as u8;
            self.time += 1.0 / DEFAULT_FREQ as f64;
        }
    }
}

pub struct Buzzer(AudioDevice<SquareWave>);

impl Buzzer {
    pub fn new(sys: AudioSubsystem) -> Result<Self, String> {
        let desired_spec = AudioSpecDesired {
            freq: Some(DEFAULT_FREQ),
            samples: Some(DEFAULT_SAMPLES),
            channels: Some(1),
        };
        let device = sys.open_playback(None, &desired_spec, |spec| {
            SquareWave { time: 0., volume: 0.1, frequency: 200. }
        })?;
        Ok(Self(device))
    }

    pub fn play(&self) { self.0.resume() }
    pub fn pause(&self) { self.0.pause() }
}
