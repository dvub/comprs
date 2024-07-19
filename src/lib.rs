mod dsp;
pub mod editor;

mod params;

use dsp::Compressor;
use editor::create_editor;
use nih_plug::prelude::*;
use params::CompressorParams;

use std::{collections::VecDeque, sync::Arc};

// TODO:
// refactor and remove this level of abstraction
// i.e. impl Plugin for Compressor (and not CompressorPlugin)
pub struct CompressorPlugin {
    compressor: Compressor,
}

impl Default for CompressorPlugin {
    fn default() -> Self {
        Self {
            compressor: Compressor::new(Arc::new(CompressorParams::default())),
        }
    }
}

impl Plugin for CompressorPlugin {
    const NAME: &'static str = "COMPRS";
    const VENDOR: &'static str = "DVUB";
    const URL: &'static str = "https://dvub.net";
    const EMAIL: &'static str = "dvubdevs@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

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
        self.compressor.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        // update RMS buffer size if sample rate changes
        // TODO: does this need to be optimized?
        let sample_rate = context.transport().sample_rate;
        if self.compressor.rms.sample_rate != sample_rate {
            println!("Sample rate changed..");

            let new_buffer_size = (sample_rate * 1e-3) as usize;

            self.compressor.rms.sample_rate = sample_rate;
            self.compressor.rms.buffer = VecDeque::from(vec![0.0; new_buffer_size]);
        }

        // apply compression
        for mut channel_samples in buffer.iter_samples() {
            for sample in channel_samples.iter_mut() {
                self.compressor.process(sample);
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let editor = create_editor(self);
        Some(Box::new(editor))
    }
}
impl Vst3Plugin for CompressorPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"COMPRSSSSSSSSSSS";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_vst3!(CompressorPlugin);
