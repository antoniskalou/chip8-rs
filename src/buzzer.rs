use sdl2::{
    audio::{AudioCallback, AudioDevice, AudioSpecDesired},
    AudioSubsystem,
};

struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = u8;

    fn callback(&mut self, output: &mut [u8]) {
        for out in output.iter_mut() {
            let x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            let half_max = u8::MAX as f32 / 2.;
            *out = (half_max * x + half_max) as u8;
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

const DEFAULT_FREQ: i32 = 44100;
const DEFAULT_SAMPLES: u16 = 512;

pub struct Buzzer(AudioDevice<SquareWave>);

impl Buzzer {
    pub fn new(sys: AudioSubsystem) -> Result<Self, String> {
        let desired_spec = AudioSpecDesired {
            freq: Some(DEFAULT_FREQ),
            samples: Some(DEFAULT_SAMPLES),
            channels: Some(1),
        };
        let device = sys.open_playback(None, &desired_spec, |spec| SquareWave {
            phase_inc: 150.0 / spec.freq as f32,
            phase: 0.0,
            volume: 0.25,
        })?;
        Ok(Self(device))
    }

    pub fn play(&self) {
        self.0.resume()
    }
    pub fn pause(&self) {
        self.0.pause()
    }
}
