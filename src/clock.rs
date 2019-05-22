#[derive(Debug, Clone)]
pub struct Clock {
    tempo: Bpm,
    sample_rate: u64,
    beat_length: u64,
}

impl Clock {
    pub fn new(tempo: Bpm, sample_rate: u64) -> Self {
        Clock {
            tempo,
            sample_rate,
            beat_length: (sample_rate*60) / tempo.0
        }
    }

    pub fn set_tempo(&mut self, tempo: Bpm) {
        self.tempo = tempo;
        self.beat_length = self.sample_rate*60 / self.tempo.0;
    }

    pub fn set_sample_rate(&mut self, sample_rate: u64) {
        self.sample_rate = sample_rate;
        self.beat_length = self.sample_rate*60 / self.tempo.0;
    }

    pub fn beat_length(&self) -> u64 {
        self.beat_length
    }

    pub fn tempo(&self) -> Bpm {
        self.tempo
    }

    pub fn sample_rate(&self) -> u64 {
        self.sample_rate
    }
}

impl Default for Clock {
    fn default() -> Self {
        Clock::new(Bpm::default(), 44100)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Bpm(u64);

impl Bpm {
    pub fn set(&mut self, value: u64) {
        self.0 = value.min(200).max(27);
    }
}

impl Default for Bpm {
    fn default() -> Self {
        Bpm(117)
    }
}