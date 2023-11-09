use crate::{FFT_32K, FFT_64K, TEMPORAL_AVG_DEPTH as TAD};
use nih_plug::prelude::*;
use std::sync::mpsc::*;

pub struct InputBuffer {
    data: UndelayedBuffer,
    size: usize,
    tx: Sender<UndelayedBuffer>,
}

impl InputBuffer {
    pub fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            data: UndelayedBuffer::default(),
            size: FFT_32K,
            tx,
        }
    }
    pub fn init(&mut self, s_r: usize, n_ch: usize, tx_p: Sender<UndelayedBuffer>) -> &usize {
        self.tx = tx_p;
        match s_r {
            96000 => self.size = FFT_64K,
            _ => (),
        }
        self.data.init(n_ch, self.size);

        &self.size
    }

    pub fn update(&mut self, input: &Buffer) -> Option<UndelayedBuffer> {
        for (n_ch, chan) in input.iter_samples().enumerate() {
            for sample in chan {
                self.data.push(n_ch, sample);
            }
        }
        // return expression
        match self.data.len() > self.size {
            false => None,
            true => {
                while self.data.len() > self.size {
                    self.data.pop();
                }
                let output = self.data.clone();
                self.refresh();
                Some(output)
            }
        }
    }

    pub fn refresh(&mut self) {
        self.data.clear();
    }
}

#[derive(Clone)]
pub struct UndelayedBuffer {
    data: Vec<Vec<f32>>,
}

impl UndelayedBuffer {
    pub fn len(&self) -> usize {
        self.data[0].len()
    }

    fn default() -> Self {
        Self { data: Vec::new() }
    }

    fn init(&mut self, n_ch: usize, size: usize) {
        for n in 0..n_ch {
            self.data.push(Vec::with_capacity(size));
        }
    }

    fn push(&mut self, ch: usize, sample: &f32) {
        self.data[ch].push(*sample);
    }
    fn pop(&mut self) {
        for chan in self.data.iter_mut() {
            chan.pop();
        }
    }

    fn clear(&mut self) {
        for chan in self.data.iter_mut() {
            chan.clear();
        }
    }
}

pub struct TFresult {
    spectrum: Vec<f32>,
    freq_response: Vec<f32>,
    phase_response: Vec<f32>,
    impulse_response: Vec<f32>,
    delay: usize,
}

impl TFresult {
    fn new(size: usize) -> Self {
        Self {
            spectrum: Vec::with_capacity(size),
            freq_response: Vec::with_capacity(size),
            phase_response: Vec::with_capacity(size),
            impulse_response: Vec::with_capacity(size),
            delay: 0,
        }
    }
}

pub struct TFresults {
    inner: Vec<TFresult>,
}

impl TFresults {
    pub fn default() -> Self {
        Self { inner: Vec::new() }
    }
    fn new(size: usize, n_ch: usize) -> Self {
        let mut new = TFresults::default();
        for n in 0..n_ch {
            new.inner.push(TFresult::new(size));
        }
        if n_ch > 2 {
            new.inner.push(TFresult::new(size));
        }
        new
    }
}

pub struct ResultsBuffer {
    current: TFresults,
    data: Vec<TFresults>,
    cursor: usize,
    full: bool,
}

impl ResultsBuffer {
    pub fn default() -> Self {
        Self {
            current: TFresults::default(),
            data: Vec::new(),
            cursor: 0,
            full: false,
        }
    }

    fn init(&mut self, size: usize, n_ch: usize) {
        self.current = TFresults::new(size, n_ch);
        for n in 0..TAD {
            self.data.push(TFresults::new(size, n_ch));
        }
    }

    fn update(&mut self, input: &TFresults) {
        self.data[self.cursor] = *input;
        self.cursor += 1;
        if self.cursor >= TAD {
            self.full = true;
            self.cursor = 0;
        }
    }

    fn refresh(&mut self) {
        self.cursor = 0;
        self.full = false;
    }
}
