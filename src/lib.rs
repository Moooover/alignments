use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;

use crate::buffers::*;
use crate::editor::*;
use crate::proc::*;

use std::sync::mpsc::*;
use std::sync::Arc;

mod buffers;
mod editor;
mod proc;

static FREQ_RESOLUTION: f32 = 3.0;
static TEMPORAL_AVG_DEPTH: usize = 8;
static FFT_32K: usize = 32768;
static FFT_64K: usize = 65536;

pub struct AT {
    params: Arc<ATparams>,
    buffer: buffers::InputBuffer,
    s_r: usize,
    size: usize,
    tx_plug: Sender<TFresults>,
    rx_plug: Receiver<TFresults>,
    tx_gui: Sender<TFresults>,
    rx_gui: Receiver<TFresults>,
}

#[derive(Params)]
struct ATparams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
}

impl Default for AT {
    fn default() -> Self {
        let (tx_plug, rx_plug) = channel();
        let (tx_gui, rx_gui) = channel();
        Self {
            params: Arc::new(ATparams::default()),
            buffer: InputBuffer::default(),
            s_r: 0,
            size: 0,
            tx_plug,
            rx_plug,
            tx_gui,
            rx_gui,
        }
    }
}

impl Default for ATparams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for AT {
    //fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {}

    fn initialize(
        &mut self,
        audio_config: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        self.s_r = buffer_config.sample_rate as usize;
        let n_chan: std::num::NonZeroU32 = audio_config.main_input_channels.unwrap();//theoretically never fails
        //since host knows we need minimum 2 channels

        let (tx_proc, rx_proc) = channel();
        self.size = *self.buffer.init(self.s_r, n_chan.get() as usize, tx_proc);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // todo missing some event handling here: need to watch for buffer resets from user
        if let Some(tf_buff) = self.buffer.update(buffer) {
            thread::spawn(move || loop {
                let new_measurement = proc::measure(tf_buff);
                self.tx_gui.send(new_measurement);
            })
        }

        ProcessStatus::Normal
    }

    fn editor(&self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let (tx, rx) = channel();
        self.tx_gui = tx.clone();
        editor::create(self.params.clone(), rx, self.params.editor_state.clone())
    }

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn reset(&mut self) {
        self.buffer.refresh();
        //how to reset editor?
    }

    const NAME: &'static str = "Alignment Tool v.1";
    const VENDOR: &'static str = "Wirebender Audio";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = "0.1.0";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(3),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(3),
            main_output_channels: NonZeroU32::new(3),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(4),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(4),
            main_output_channels: NonZeroU32::new(4),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(5),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(5),
            main_output_channels: NonZeroU32::new(5),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(6),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(6),
            main_output_channels: NonZeroU32::new(6),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(7),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(7),
            main_output_channels: NonZeroU32::new(7),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(8),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(8),
            main_output_channels: NonZeroU32::new(8),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

}

impl ClapPlugin for AT {
    const CLAP_ID: &'static str = "discrete.symbol.continuous.syntax.alignment.tool.0.1";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("For rapid sound system deployment");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Utility];
}

impl Vst3Plugin for AT {
    const VST3_CLASS_ID: [u8; 16] = *b"AlignmentToolv.1";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}

nih_export_clap!(AT);
nih_export_vst3!(AT);
