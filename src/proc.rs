use crate::buffers::*;
use crate::AT;
use rustfft::*;
use std::sync::mpsc::*;
use std::thread;

    pub fn measure(&self, input: UndelayedBuffer) -> TFresults {
        return spectral_output(input);
   //         .fft(input)
   //         .ift()
   //         .find_delay()
   //         .find_difference()
   //         .time_avg();
    }

    fn spectral_output(input: UndelayedBuffer) -> TFresults {}

   // fn fft(&self, input: UndelayedBuffer) -> SpectralResults {}

   // fn ift(&self, input: SpectralResults) -> IRresults {}

   // fn find_delay(&self, input: IRresults) -> TFinput {}

   // fn find_difference(&self, input: TFinput) -> TFresults {}

   // fn time_avg(&mut self, input: TFresults) -> TFresults {}
}
