use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use twang::noise::Pink;

use crate::process::{FourChanBuff, TransferFunctionResults};

use std::sync::Arc;

mod editor;
mod process;

struct AT {
    params: Arc<ATparams>,
    input_buffer: Vec<Vec<f32>>,
    spectrum_buffer: Vec<Vec<f32>>,
    samples_remaining: Arc<i32>,
    results: Arc<TransferFunctionResults>,
    pink: Pink,
    ref_buff: Vec<f32>,
}

#[derive(Params)]
struct ATparams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
    #[id = "length"]
    measure_length: IntParam, // 1 - 10 sec
    #[id = "status"]
    measure_status: IntParam, // 0 = stopped, 1 = timed measurement, -1 = live measurement
}

impl Default for AT {
    fn default() -> Self {
        Self {
            params: Arc::new(ATparams::default()),
            input_buffer: Vec::new(),
            samples_remaining: 0.into(),
            results: TransferFunctionResults::new().into(),
            pink: Pink::new(),
            ref_buff: Vec::new(),
        }
    }
}

impl Default for ATparams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            measure_length: IntParam::new("Length", 3, IntRange::Linear { min: 1, max: 10 }),
            measure_status: IntParam::new("Status", 0, IntRange::Linear { min: -1, max: 1 }),
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
            main_input_channels: NonZeroU32::new(4),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {}

    fn initialize(
        &mut self,
        audio_config: &AudioIOLayout,
        buffer_config: &BufferConfig,
        context: &mut impl InitContext<Self>,
    ) -> bool {
        // allocate buffers
        for chan in 1..audio_config.main_input_channels.unwrap().into() {
            self.input_buffer.push(Vec::with_capacity(
                (buffer_config.sample_rate as i32 * 11) as usize,
            ));
        }

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        ProcessStatus::Normal
    }
}

impl ClapPlugin for AT {
    const CLAP_ID: &'static str = "discrete.symbol.continuous.syntax.alignment.tool.0.1.0";
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
