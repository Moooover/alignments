use crate::{FFT_32K, FFT_64K, TEMPORAL_AVG_DEPTH as TAD};
use nih_plug::prelude::*;

pub struct InputBuffer {
    data: UndelayedBuffer,
    size: usize,
}

impl InputBuffer {
    pub fn default() -> Self {
        Self {
            data: UndelayedBuffer::default(),
            size: FFT_32K,
        }
    }
    pub fn init(&mut self, s_r: usize, n_ch: usize) -> &usize {
        match s_r {
            96000 => self.size = FFT_64K,
            _ => (),
        }
        self.data.init(n_ch, self.size);

        &self.size
    }

    pub fn update(&mut self, input: &mut Buffer) -> Option<UndelayedBuffer> {
        for (n_ch, chan) in input.iter_samples().enumerate() {
            for sample in chan {
                self.data.push(n_ch, sample);
            }
        }

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
pub struct SDEresults {
}
impl SDEresults {
    pub fn default() -> Self {
        Self {
        }
    }
    pub fn new(s_r: usize, n_ch: usize) -> Self {
        Self {
        }
    }
}

pub struct TimeAvgBuffer {
    current: SDEresults,
    data: Vec<SDEresults>,
    cursor: usize,
    full: bool,
}

impl TimeAvgBuffer {
    pub fn default() -> Self {
        Self {
            current: SDEresults::default(),
            data: Vec::new(),
            cursor: 0,
            full: false,
        }
    }

    fn init(&mut self, size: usize, n_ch: usize) {
        self.current = SDEresults::new(size, n_ch);
        for n in 0..TAD {
            self.data.push(SDEresults::new(size, n_ch));
        }
    }

    fn update(&mut self, input: &SDEresults) {
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

pub struct OutputBuffer {
    data: UndelayedBuffer,
}

impl OutputBuffer {
    pub fn default() -> Self {
        Self {
            data: UndelayedBuffer::default(),
        }
    }
    pub fn init(&mut self, s_r: usize, n_ch: usize) {
        self.data.init(s_r, n_ch)
    }
    pub fn update(&mut self, input: UndelayedBuffer) -> UndelayedBuffer {
        input
    }
    pub fn refresh(&mut self) {
        self.data.clear();
    }
}
