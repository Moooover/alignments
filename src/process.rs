use crate::AT;
use nih_plug::prelude::*;

const FREQ_RES: f32 = 3.0;
pub struct InputBuffer {
    reference: Vec<f32>,
    channels: Vec<Vec<f32>>,
    i: usize,
    n: usize,
}

impl InputBuffer {
    pub fn new() -> Self {
        Self {
            reference: Vec::new(),
            channels: Vec::new(),
            i: 0,
            n: 0,
        }
    }

    pub fn init(&mut self, s_r: f32, n_chan: usize) {
        self.reference.reserve(((s_r / FREQ_RES) * 1.25) as usize);
    }

    pub fn add(&mut self, input: &Buffer) {
        let n_samp = input.samples();
        let n_chan = input.channels();
        let chan_iter = input.iter_samples();
        let ref_chan = chan_iter.next().unwrap();
        for i in 0..n_samp {}

        self.i += n_samp;
        if self.i > self.n {
            self.n = self.i;
        }
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
    pub input: InputBuffer,
    pub freq: Vec<Vec<(f32, f32)>>,
    pub impulse: Vec<Vec<f32>>,
    pub reference: Vec<f32>,
}

impl ATbuffers {
    pub fn push(&mut self, buffer: &Buffer) {
        // shit
    }
    pub fn new() -> ATbuffers {
        Self {
            input: InputBuffer::new(),
            freq: Vec::new(),
            impulse: Vec::new(),
            reference: Vec::new(),
        }
    }
    pub fn init(&mut self, size: usize, n_chan: u32) {
        self.input.init(s_r, n_chan)
    }
}
/*  for n in 1..=n_chan {
    self.input.push(Vec::with_capacity(size));
    self.freq.push(Vec::with_capacity(size));
    self.impulse.push(Vec::with_capacity(size));
}
self.reference.reserve(size); */
