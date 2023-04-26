use nih_plug::prelude::*;
use nih_plug_iced::IcedState;

use crate::process::*;

use std::sync::Arc;

mod editor;
mod process;

pub struct AT {
    params: Arc<ATparams>,
    buffers: ATbuffers,
    sample_counter: ATcounter,
    results: Arc<Vec<TFresults>>,
    s_r: f32,
    measure_status: bool,
}

#[derive(Params)]
struct ATparams {
    #[persist = "editor-state"]
    editor_state: Arc<IcedState>,
    #[id = "measure-type"]
    measure_type: EnumParam<ATtype>,
}

#[derive(Enum, Debug, PartialEq)]
enum ATtype {
    #[id = "Verify"]
    Verify,
    #[id = "Align"]
    Align,
    #[id = "Live"]
    Live,
}

impl Default for AT {
    fn default() -> Self {
        Self {
            params: Arc::new(ATparams::default()),
            buffers: ATbuffers::new(),
            sample_counter: ATcounter::new(),
            results: Vec::new().into(),
            s_r: 0.0,
            measure_status: false,
        }
    }
}

impl Default for ATparams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),
            measure_type: EnumParam::new("Verify", ATtype::Verify),
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
        self.s_r = buffer_config.sample_rate;
        const MAX_BUFF: i32 = 7; // seconds
        let size = (self.s_r as i32 * MAX_BUFF) as usize;
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
        match self.measure_status {
            false => mute_output(buffer),
            true => match self.sample_counter.get() {
                None => {
                    mute_output(buffer);
                    run_analysis(&self);
                    self.measure_status = false;
                }
                Some(n) => {
                    collect_data(buffer);
                    match self.params.measure_type.value() {
                        ATtype::Live => (),
                        ATtype::Verify | ATtype::Align => {
                            self.sample_counter.decr(buffer.samples())
                        }
                    }
                }
            },
        }

        ProcessStatus::Normal
    }

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
