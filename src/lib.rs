use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use twang::noise::Pink;

use crate::process::TransferFunctionResults;

use std::sync::Arc;

mod editor;
mod process;

pub struct AT {
    params: Arc<ATparams>,
    pink: Pink,
    buffers: process::ATbuffers,
    sample_counter: Arc<u32>,
    results: Arc<Vec<TransferFunctionResults>>,
}

#[derive(Params)]
struct ATparams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
    #[id = "length"]
    measure_length: IntParam, // 1 - 10 sec
    #[id = "status"]
    measure_status: IntParam, // 0 = stopped, 1 = timed measurement, -1 = live measurement
                              //using "dumb" integers because nih_plug::params::EnumParam is confusing to implement, too.
}

impl Default for AT {
    fn default() -> Self {
        Self {
            params: Arc::new(ATparams::default()),
            pink: Pink::new(),
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
            measure_length: IntParam::new("Length", 3, IntRange::Linear { min: 1, max: 10 })
                .with_unit("Seconds"),
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
        // allocate big buffer
        for chan in 1..audio_config.main_input_channels.unwrap().into() {
            self.buffers.input.push(Vec::with_capacity(
                (buffer_config.sample_rate as i32 * 11) as usize,
            ));
        }
        self.buffers
            .reference
            .reserve((buffer_config.sample_rate as i32 * 11) as usize);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // when measure_status == 0 do nothing
        // when measure_status == 1 && samples_remaining != 0, add input to buffer and output pink
        // when measure_status == 1 && samples_remaining == 0, run the analysis process and reset status to 0.
        // when measure_status == -1, run live analysis process
        match (&self.params.measure_status.value(), *self.sample_counter) {
            (0, 0) => (),
            (1, 0) => {
                process::run_analysis(&self);
            }
            (1, _) => {
                process::collect_data(buffer);
            }
            (-1, _) => (),       // todo: live feature
            _ => unreachable!(), // because measure_status will be set manually, not with math
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
