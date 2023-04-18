use nih_plug::prelude::*;
use nih_plug_iced::IcedState;

use crate::process::TransferFunctionResults;

use std::sync::Arc;

mod editor;
mod process;

pub struct AT {
    params: Arc<ATparams>,
    buffers: process::ATbuffers,
    sample_counter: Arc<u32>,
    results: Arc<Vec<TransferFunctionResults>>,
}

#[derive(Params)]
struct ATparams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
    #[id = "measure-type"]
    measure_type: ATtype,
    #[id = "measure-status"]
    measure_status: BoolParam,
}

#[derive(Params, Enum)]
enum ATtype {
    Verify,
    Align,
    Continuous,
}

impl Default for AT {
    fn default() -> Self {
        Self {
            params: Arc::new(ATparams::default()),
            buffers: process::ATbuffers::new(),
            sample_counter: 0.into(),
            results: Vec::new().into(),
        }
    }
}

impl Default for ATparams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            measure_type: ATtype::Verify,
            measure_status: BoolParam::new("measure-status", false),
        }
    }
}

impl Plugin for AT {
    const NAME: &'static str = "Alignment Tool v.1";
    const VENDOR: &'static str = "Wirebender Audio";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = "0.1.0";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
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
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    //fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {}

    fn initialize(
        &mut self,
        audio_config: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        const MAX_BUFF: i32 = 7; // seconds
        let size = (buffer_config.sample_rate as i32 * MAX_BUFF) as usize;
        let n_chan: u32 = audio_config.main_input_channels.unwrap().into();
        self.buffers.init(size, n_chan);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        match self.params.measure_status.value() {
            false => (),
            true => match self.sample_counter {
                None => {
                    process::run_analysis(&self);
                    self.params.measure_status = false;
                }
            },
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for AT {
    const CLAP_ID: &'static str = "discrete.symbol.continuous.syntax.alignment.tool.0.0";
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
