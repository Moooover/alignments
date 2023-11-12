use crate::buffers::*;
use crate::SDE;
use rustfft::*;
use std::sync::mpsc::*;
use std::thread;

    pub fn measure(input: UndelayedBuffer) -> UndelayedBuffer {
        input
   //         .fft(input)
   //         .ift()
   //         .find_delay()
   //         .find_difference()
   //         .time_avg();
    }


   // fn fft(&self, input: UndelayedBuffer) -> SpectralResults {}

   // fn ift(&self, input: SpectralResults) -> IRresults {}

   // fn find_delay(&self, input: IRresults) -> TFinput {}

   // fn find_difference(&self, input: TFinput) -> TFresults {}

   // fn time_avg(&mut self, input: TFresults) -> TFresults {}
