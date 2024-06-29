pub mod editor;
mod enums;
mod params;

use dsp::{Compressor, LevelDetectionType};
use editor::create_editor;
use nih_plug::prelude::*;
use params::{
    CompressorParams, DEFAULT_ATTACK_TIME, DEFAULT_KNEE, DEFAULT_RATIO, DEFAULT_RELEASE_TIME,
    DEFAULT_THRESHOLD,
};

use std::sync::Arc;

pub struct CompressorPlugin {
    compressor: Compressor,
    params: Arc<CompressorParams>,
}

impl Default for CompressorPlugin {
    fn default() -> Self {
        // TODO:
        // unify with params
        let threshold = DEFAULT_THRESHOLD;
        let ratio = DEFAULT_RATIO;
        let knee = DEFAULT_KNEE;
        let attack_time = DEFAULT_ATTACK_TIME;
        let release_time = DEFAULT_RELEASE_TIME;

        Self {
            params: Arc::new(CompressorParams::default()),
            compressor: Compressor::new(
                attack_time,
                release_time,
                threshold,
                ratio,
                knee,
                LevelDetectionType::Rms,
            ),
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
        self.params.clone()
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
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for mut channel_samples in buffer.iter_samples() {
            for sample in channel_samples.iter_mut() {
                *sample = self.compressor.process(*sample).0;
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
