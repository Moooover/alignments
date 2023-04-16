use crate::AT;
use nih_plug::prelude::*;
use twang::noise::Pink;

pub fn run_analysis(plugin: &AT) {
    // find the delay on each channel
    // find the difference of each channel
    // average however desired
}

pub fn collect_data(buffer: &Buffer) {
    // write input audio to buffer
    // write pink to output audio
    // profit
}

pub struct TransferFunctionResults {
    spectrum: Vec<f32>,
    freq_response: Vec<f32>,
    phase_response: Vec<f32>,
    impulse_response: Vec<f32>,
    delay: i32,
}

impl TransferFunctionResults {
    pub fn new() -> Self {
        Self {
            spectrum: Vec::new(),
            freq_response: Vec::new(),
            phase_response: Vec::new(),
            impulse_response: Vec::new(),
            delay: 0,
        }
    }
}

pub struct ATbuffers {
    pub input_buff: Vec<Vec<f32>>,
    pub freq_buff: Vec<Vec<(f32, f32)>>,
    pub impulse_buff: Vec<Vec<f32>>,
    pub reference_buff: Vec<f32>,
}

impl ATbuffers {
    pub fn new() -> ATbuffers {
        Self {
            input_buff: Vec::new(),
            freq_buff: Vec::new(),
            impulse_buff: Vec::new(),
            reference_buff: Vec::new(),
        }
    }
}

fn find_delay() {}
