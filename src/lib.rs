mod dsp;
pub mod editor;

mod params;

use dsp::{Compressor, RmsLevelDetector};
use editor::create_editor;
use nih_plug::prelude::*;
use params::CompressorParams;

use std::{
    collections::VecDeque,
    sync::{atomic::Ordering, Arc},
};

pub const MAX_BUFFER_SIZE: f32 = 0.03;

pub struct CompressorPlugin {
    sample_rate: f32,
    params: Arc<CompressorParams>,
    compressors: [Compressor; 2],
    shared_rms: RmsLevelDetector,
    pre_amplitude: Arc<AtomicF32>,
    post_amplitude: Arc<AtomicF32>,
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
            pre_amplitude: Arc::new(AtomicF32::new(0.0)),
            post_amplitude: Arc::new(AtomicF32::new(0.0)),
        }
    }
}

impl CompressorPlugin {
    fn initialize_rms_buffers(&mut self) {
        let max_buffer_length = (self.sample_rate * MAX_BUFFER_SIZE) as usize;

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
        self.sample_rate = sample_rate;
        let actual_size = (sample_rate * self.params.rms_buffer_size.value()) as usize;
        self.initialize_rms_buffers();
        self.resize_rms_buffers(actual_size);

        true
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_channels = buffer.channels();

        let input_gain = self.params.input_gain.value();
        let dry_wet = self.params.dry_wet.value();
        let output_gain = self.params.output_gain.value();

        for mut channel_samples in buffer.iter_samples() {
            let mut pre_amplitude = 0.0;
            let mut post_amplitude = 0.0;
            let num_samples = channel_samples.len();

            // this loops twice, once for L/R channels
            for i in 0..num_channels {
                let sample = channel_samples.get_mut(i).unwrap();

                *sample *= input_gain;
                pre_amplitude += *sample;

                let (pre_processed, processed) = self.compressors[i].process(
                    *sample,
                    &self.params,
                    &mut self.shared_rms,
                    self.sample_rate,
                );
                post_amplitude += processed;
                // blend based on dry_wet
                let mut blended_output = (1.0 - dry_wet) * pre_processed + dry_wet * processed;

                // finally, modify with output gain
                blended_output *= output_gain;
                // and we're done!
                *sample = blended_output;
            }

            pre_amplitude = (pre_amplitude / num_samples as f32).abs();
            post_amplitude = (post_amplitude / num_channels as f32).abs();

            self.pre_amplitude.store(pre_amplitude, Ordering::Relaxed);
            self.post_amplitude.store(post_amplitude, Ordering::Relaxed);
        }

        if self.params.rms_update.swap(false, Ordering::Relaxed) {
            let new_buffer_size = self.params.rms_buffer_size.smoothed.next();
            let new_size = (self.sample_rate * new_buffer_size) as usize;

            context.set_latency_samples(new_size as u32);
            self.resize_rms_buffers(new_size);
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
