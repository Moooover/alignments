use crate::buffers::TFresults;
use crate::ATparams;
use nih_plug::prelude::Editor;
use nih_plug_iced::*;

use std::sync::mpsc::Receiver;
use std::sync::Arc;

const EDITOR_W: u32 = 200;
const EDITOR_H: u32 = 150;

struct ATeditor {
    params: Arc<ATparams>,
    results: Arc<Vec<TFresults>>,
}

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(EDITOR_W, EDITOR_H)
}

pub(crate) fn create(
    params: Arc<ATparams>,
    current_measurement: Receiver<TFresults>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<ATeditor>(editor_state, (params, current_measurement))
}
