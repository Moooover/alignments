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

pub struct SDE {
    params: Arc<SDEparams>,
    in_buff: buffers::InputBuffer,
    out_buff: buffers::OutputBuffer,
    s_r: usize,
    size: usize,
    n_chan: usize,
    rx_plug: Receiver<TFresults>,
}

#[derive(Params)]
struct SDEparams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
    // #[id = "reset_buff"]
    // pub reset_buff: BoolParam
}

impl Default for SDE {
    fn default() -> Self {
        let (tx_plug, rx_plug) = channel();
        let (tx_gui, rx_gui) = channel();
        Self {
            params: Arc::new(SDEparams::default()),
            in_buff: InputBuffer::default(),
            s_r: 0,
            size: 0,
            n_chan: 2,
            rx_plug,
        }
    }
}

impl Default for SDEparams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
        }
    }
}

impl Plugin for SDE {

    fn initialize(
        &mut self,
        audio_config: &AudioIOLayout,
        buff_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        self.s_r = buff_config.sample_rate as usize;
        self.n_chan = audio_config.main_input_channels.unwrap() as usize;//theoretically never fails
        //since host knows we need minimum 2 channels
        self.size = self.in_buff.init(self.s_r, self.n_chan);
        self.out_buff.init(self.s_r, self.n_chan);

        true
    }

    fn process(
        &mut self,
        buff: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // todo missing some event handling here: need to watch for buff resets from user
        // if let Some(event) = self.rx_plug.try_recv() {}

        if let Some(tf_buff) = self.in_buff.update(buffer) {
                self.tx_gui.send(self.out_buff.update(proc::measure(tf_buff)));
        }
        //todo set output to zeros
        ProcessStatus::Normal
    }

    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let (tx_p, rx_p) = channel();
        self.rx_plug = rx_p;
        editor::create(self.params.clone(), tx_p, self.params.editor_state.clone())
    }

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn reset(&mut self) {
        self.in_buff.refresh();
        self.out_buff.refresh();
        //how to reset editor?
    }

    const NAME: &'static str = "Signal Difference Engine v.0";
    const VENDOR: &'static str = "Wirebender Audio";
    const URL: &'static str = "wirebender.hypertool.xyz";
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

    const SAMPLE_ACCURSDEE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

}

impl ClapPlugin for SDE {
    const CLAP_ID: &'static str = "discrete.symbol.continuous.syntax.signal.difference.engine.0.1";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("For rapid sound system deployment");
    const CLAP_MANUAL_URL: Option<&'static str> = Some("sde.hypertool.xyz/manual");
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FESDEURES: &'static [ClapFeature] = &[ClapFeature::Utility];
}

impl Vst3Plugin for SDE {
    const VST3_CLASS_ID: [u8; 16] = *b"Signal Difference Engine v.0";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Tools];
}

nih_export_clap!(SDE);
nih_export_vst3!(SDE);
