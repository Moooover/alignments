use crate::AT;
use nih_plug::prelude::*;
use twang::noise::Pink;

impl TFresults {
    pub fn new() -> Self {
        Self {
            spectrum: Vec::new(),
            freq_response: Vec::new(),
            phase_response: Vec::new(),
            impulse_response: Vec::new(),
            coherence: Vec::new(),
            delay: 0,
        }
    }
}

pub struct ATbuffers {
    pub input: Vec<Vec<f32>>,
    pub freq: Vec<Vec<(f32, f32)>>,
    pub impulse: Vec<Vec<f32>>,
    pub reference: Vec<f32>,
}

impl ATbuffers {
    pub fn add(&mut self, buffer: &Buffer) {
        for chan in buffer.iter_samples()
    }
    pub fn new() -> ATbuffers {
        Self {
            input: Vec::new(),
            freq: Vec::new(),
            impulse: Vec::new(),
            reference: Vec::new(),
        }
    }
    pub fn init(&mut self, size: usize, n_chan: u32) {
        for n in 1..=n_chan {
            self.input.push(Vec::with_capacity(size));
            self.freq.push(Vec::with_capacity(size));
            self.impulse.push(Vec::with_capacity(size));
        }
        self.reference.reserve(size);
    }
}

pub struct TFresults {
    spectrum: Vec<f32>,
    freq_response: Vec<f32>,
    phase_response: Vec<f32>,
    impulse_response: Vec<f32>,
    coherence: Vec<f32>,
    delay: u32,
}
