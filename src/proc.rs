use crate::buffers::*;
use rustfft::*;
use std::thread::*;

pub struct ProcessObject {
    proc_thread: Thread,
}

impl ProcessObject {
    pub fn default() -> Self {
        
    }
    pub fn init(&mut self) {}

    pub fn proc(&self, input: UndelayedBuffer) -> TFresults {}

    fn difference(input: TFinput) -> TFresult {}

    fn find_delay(input: UndelayedBuffer) -> TFinput {}
}
