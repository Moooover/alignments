use crate::process::TransferFunctionResults;
use crate::ATparams;
use nih_plug_iced::*;

use std::sync::Arc;

const EDITOR_W: u32 = 200;
const EDITOR_H: u32 = 150;

struct ATeditor {
    params: Arc<ATparams>,
    results: Arc<TransferFunctionResults>,
}

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(EDITOR_W, EDITOR_H)
}
