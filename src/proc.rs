use crate::buffers::*;
use rustfft::*;
use std::thread;

pub struct ProcessObject {
    proc_thread: thread,
}

impl ProcessObject {
    pub fn new() -> Self {}
    pub fn init() {}

    pub fn difference(input: TFinput) -> TFresult {}

    pub fn find_delay(input: UndelayedBuffer) -> TFinput {}
}
