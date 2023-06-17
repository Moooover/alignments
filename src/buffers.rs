use crate::proc::*;
use crate::TEMPORAL_AVG_DEPTH as TAD;
use nih_plug::prelude::*;
pub struct ATbuffers {
    input: InputBuffer,
    results: ResultsBuffer,
    s_r: usize,
    size: usize,
}

impl ATbuffers {
    pub fn update_input(&mut self, buffer: &Buffer) -> Option<UndelayedBuffer> {
        self.input.update(buffer)
    }

    pub fn update_results(&mut self, input: TFresults) {
        self.results.update(input);
    }

    pub fn default() -> Self {
        Self {
            input: InputBuffer::default(),
            results: ResultsBuffer::default(),
            s_r: 0,
            size: 32768,
        }
    }

    pub fn init(&mut self, s_r: usize, n_chan: usize) {
        match s_r {
            96000 => self.size = 65536,
            _ => (),
        }
        self.input.init(self.size, n_chan);
        self.results.init(self.size, n_chan);
    }
}

pub struct InputBuffer {
    data: UndelayedBuffer,
    size: usize,
}

impl InputBuffer {
    fn default() -> Self {
        Self {
            data: UndelayedBuffer::default(),
            size: 0,
        }
    }

    fn init(&mut self, size: usize, n_ch: usize) {
        self.size = size;
        self.data.init(n_ch, size);
    }

    fn update(&mut self, input: &Buffer) -> Option<UndelayedBuffer> {

        for (n_ch, chan) in input.iter_samples().enumerate() {
            for sample in chan {
                self.data.push(n_ch, sample);
            }
        }

        match self.data.len() > self.size {
            false => return None,
            true => {
                while self.data.len() > self.size {
                    self.data.pop();
                }
                self.refresh();
                let output = UndelayedBuffer::spawn(self.data);
                return Some(output);
            }
        }
    }

    fn refresh(&mut self) {
        self.data.clear();
    }
}

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

    fn spawn(input: Self) -> Self {
        let output = input;
        output
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

struct TFresults {
    inner: Vec<TFresult>,
}

impl TFresults {
    fn default() -> Self {
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

struct ResultsBuffer {
    current: TFresults,
    data: Vec<TFresults>,
    cursor: usize,
    full: bool,
}

impl ResultsBuffer {
    fn default() -> Self {
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

    fn update(&mut self, input: TFresults) {
        self.data[self.cursor] = input;
        proc::time_avg()
        self.cursor += 1;
        if self.cursor >= TAD {
            self.full = true;
            self.cursor = 0;
        }
    }
}
