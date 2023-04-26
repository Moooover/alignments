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
    pub fn new() -> ATbuffers {
        Self {
            input: Vec::new(),
            freq: Vec::new(),
            impulse: Vec::new(),
            reference: Vec::new(),
        }
    }
    pub fn init(&self, size: usize, n_chan: u32) {
        for chan in 1..=n_chan {
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

pub struct ATcounter {
    inner: i64,
}
impl ATcounter {
    pub fn get(&self) -> Option<i64> {
        if self.inner < 1 {
            return None;
        } else {
            return Some(self.inner);
        };
    }
    pub fn set(&self, m_t: &crate::ATtype, s_r: &f32) {
        match m_t {
            Verify => ,
            Align => ,
            Live => self.inner = 2,
        }
    }
    pub fn decr(&self, n: usize) {
        self.inner = self.inner - (n as i64);
    }
    pub fn new() -> ATcounter { Self { inner: 0,}}}
    
pub fn run_analysis(plugin: &AT) {
    // find the delay on each channel
    // find the difference of each channel
    // find the impulse response of each difference
    // average if multiple channels
}

pub fn collect_data(buffer: &Buffer) {
    // write input audio to buffer
    // write pink to output audio
    // profit
}

pub fn mute_output(buffer: &Buffer) {
    
}

fn find_delay() {}
