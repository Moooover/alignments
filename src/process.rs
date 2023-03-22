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
