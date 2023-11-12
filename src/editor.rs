use crate::buffers::UndelayedBuffer;
use crate::SDEparams;
use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{widgets::ResizeHandle, assets, create_vizia_editor, ViziaTheming, ViziaState};

use std::sync::mpsc::Sender;
use std::sync::Arc;

const EDITOR_W: u32 = 200;
const EDITOR_H: u32 = 150;

#[derive(Lens)]
struct State {
    params: Arc<SDEparams>,
    results: Arc<Vec<UndelayedBuffer>>,
}
impl Model for State {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (EDITOR_H, EDITOR_W))
}


pub(crate) fn create(
    params: Arc<SDEparams>,
    tx_plugin: Sender<UndelayedBuffer>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::None, move |cx, _| {
        assets::register_noto_sans_regular(cx);

        ResizeHandle::new(cx);

    })
}
