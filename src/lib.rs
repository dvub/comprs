mod dsp;
pub mod editor;

mod params;

use dsp::{Compressor, RmsLevelDetector};
use editor::create_editor;
use nih_plug::prelude::*;
use params::CompressorParams;

use std::{collections::VecDeque, sync::Arc};

pub const MAX_BUFFER_SIZE: f32 = 0.03;

pub struct CompressorPlugin {
    sample_rate: f32,
    params: Arc<CompressorParams>,
    compressors: [Compressor; 2],
    shared_rms: RmsLevelDetector,
}

impl Default for CompressorPlugin {
    fn default() -> Self {
        Self {
            // this doesn't really matter, as long as we set everything correctly in initialize()
            sample_rate: 0.0,
            params: Arc::new(CompressorParams::default()),
            // TODO: FIX THIS LMAO
            compressors: [Compressor::new(), Compressor::new()],
            shared_rms: RmsLevelDetector::default(),
        }
    }
}

impl CompressorPlugin {
    fn initialize_rms_buffers(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        let max_buffer_length = (sample_rate * MAX_BUFFER_SIZE) as usize;

        self.shared_rms.buffer = VecDeque::with_capacity(max_buffer_length);

        for compressor in &mut self.compressors {
            compressor.rms.buffer = VecDeque::with_capacity(max_buffer_length);
        }
    }
    fn resize_rms_buffers(&mut self, new_size: usize) {
        // resize independent and shared RMS
        self.shared_rms.buffer.resize_with(new_size, || 0.0);
        for compressor in &mut self.compressors {
            compressor.rms.buffer.resize_with(new_size, || 0.0);
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
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // NOTE:
        // i don't really have a good way of knowing if this code will actually work correctly
        let sample_rate = buffer_config.sample_rate;
        let actual_size = (sample_rate * self.params.rms_buffer_size.value()) as usize;
        self.initialize_rms_buffers(sample_rate);
        self.resize_rms_buffers(actual_size);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_channels = buffer.channels();

        // this loops however many samples are in the buffer, so maybe 44100/256 or something
        for mut channel_samples in buffer.iter_samples() {
            // this loops twice, once for L/R channels
            for i in 0..num_channels {
                let sample = channel_samples.get_mut(i).unwrap();
                self.compressors[i].process(
                    sample,
                    &self.params,
                    &mut self.shared_rms,
                    self.sample_rate,
                );
            }
        }

        // TODO:
        // i think this is bugged somewhere

        // also todo
        // this might not be optimal
        let new_buffer_size = self.params.rms_buffer_size.smoothed.next();
        let new_size = (self.sample_rate * new_buffer_size) as usize;
        if self.shared_rms.buffer.len() != new_size {
            self.resize_rms_buffers(new_size);
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let editor = create_editor(self.params.clone());
        Some(Box::new(editor))
    }
}
impl Vst3Plugin for CompressorPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"COMPRSSSSSSSSSSS";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_vst3!(CompressorPlugin);
