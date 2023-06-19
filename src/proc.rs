use crate::buffers::*;
use crate::AT;
use rustfft::*;
use std::sync::mpsc::*;
use std::thread;

pub struct ProcessObject {
    s_r: usize,
    size: usize,
    t_c: f32,
    ta_buff: ResultsBuffer,
}

impl ProcessObject {
    pub fn default() -> Self {
        Self {
            s_r: 0,
            size: 0,
            t_c: 0.0,
            ta_buff: ResultsBuffer::default(),
        }
    }
    pub fn init(
        &mut self,
        s_r: usize,
        size: usize,
        context: &AT,
        rx: Receiver<UndelayedBuffer>,
        tx: Sender<TFresults>,
    ) {
        self.s_r = s_r;
        self.size = size;
        self.t_c = size as f32 / s_r as f32;

        thread::spawn(move || loop {
            let input = rx.recv().unwrap();
            tx.send(self.proc(input));
        });
    }

    fn proc(&self, input: UndelayedBuffer) -> TFresults {
        return self
            .fft(input)
            .ift()
            .find_delay()
            .find_difference()
            .time_avg();
    }

    fn fft(&self, input: UndelayedBuffer) -> SpectralResults {}

    fn ift(&self, input: SpectralResults) -> IRresults {}

    fn find_delay(&self, input: IRresults) -> TFinput {}

    fn find_difference(&self, input: TFinput) -> TFresults {}

    fn time_avg(&mut self, input: TFresults) -> TFresults {}
}
