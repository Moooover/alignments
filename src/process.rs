use nih_plug::prelude::*;

pub struct TransferFunctionResults {
    spectrum: Vec<f32>,
    freq_response: Vec<f32>,
    phase_response: Vec<f32>,
    impulse_response: Vec<f32>,
    delay: i32,
}

impl TransferFunctionResults {
    pub fn new() -> Self {
        Self {
            spectrum: Vec::new(),
            freq_response: Vec::new(),
            phase_response: Vec::new(),
            impulse_response: Vec::new(),
            delay: 0,
        }
    }
}

pub struct FourChanBuff {
    one: Vec<f32>,
    two: Vec<f32>,
    three: Vec<f32>,
    four: Vec<f32>,
}
impl FourChanBuff {
    pub fn new() -> Self {
        Self {
            one: Vec::new(),
            two: Vec::new(),
            three: Vec::new(),
            four: Vec::new(),
        }
    }
    pub fn init(&mut self, audio_config: &AudioIOLayout, buffer_config: &BufferConfig) {
        let new_len: usize = (buffer_config.sample_rate as i32 * 11) as usize;
        self.one.resize(new_len, 0.0);
        self.two.resize(new_len, 0.0);
        self.three.resize(new_len, 0.0);
        self.four.resize(new_len, 0.0);
    }
}
