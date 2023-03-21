use nih_plug::prelude::*;
use nih_plug_iced::IcedState;
use twang::noise::Pink;

use std::sync::Arc;

struct AT {
    params: Arc<ATparams>,
    measure_status: MeasureStatus,
    measure_buffer: Vec<f32>,
    pink: Pink,
}

#[derive(Params)]
struct ATparams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
    #[id = "active_inputs"]
    active_inputs: IntParam,
    #[id = "measure_time"]
    measure_time: IntParam,
}

enum MeasureStatus {
    Stopped,
    Ongoing(i32),
}

impl Default for AT {
    fn default() -> Self {
        Self {
            params: Arc::new(ATparams::default()),
            measure_status: MeasureStatus::Stopped,
            measure_buffer: Vec::new(),
            pink: Pink::new(),
        }
    }
}

impl Default for ATparams {
    fn default() -> Self {
        Self {
            editor_state:   //todo: editor::default_state(),
            active_inputs:  //todo
            measure_time:   //todo
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
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {

    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {

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
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for AT {
    const VST3_CLASS_ID: [u8; 16] = *b"AlignmentToolv.1";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Tools];
}

nih_export_clap!(AT);
nih_export_vst3!(AT);
