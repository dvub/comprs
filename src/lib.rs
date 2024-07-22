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
/// The time it takes for the peak meter to decay by 12 dB after switching to complete silence.
const PEAK_METER_DECAY_MS: f64 = 100.0;

pub struct CompressorPlugin {
    sample_rate: f32,
    params: Arc<CompressorParams>,
    compressors: [Compressor; 2],
    shared_rms: RmsLevelDetector,
    pre_amplitude: Arc<AtomicF32>,
    post_amplitude: Arc<AtomicF32>,
    peak_meter_decay_weight: f32,
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
            peak_meter_decay_weight: 1.0,
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
    fn calculate_amplitude(&self, current_amplitude: f32, new_amplitude: f32) -> f32 {
        if new_amplitude > current_amplitude {
            new_amplitude
        } else {
            current_amplitude * self.peak_meter_decay_weight
                + new_amplitude * (1.0 - self.peak_meter_decay_weight)
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
        // After `PEAK_METER_DECAY_MS` milliseconds of pure silence, the peak meter's value should
        // have dropped by 12 dB
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;

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
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let num_channels = buffer.channels();

        for mut channel_samples in buffer.iter_samples() {
            let mut pre_amplitude = 0.0;
            let mut post_amplitude = 0.0;
            let num_samples = channel_samples.len();

            // this loops twice, once for L/R channels
            for i in 0..num_channels {
                let sample = channel_samples.get_mut(i).unwrap();
                pre_amplitude += *sample;
                self.compressors[i].process(
                    sample,
                    &self.params,
                    &mut self.shared_rms,
                    self.sample_rate,
                );
                post_amplitude += *sample;
            }
            pre_amplitude = (pre_amplitude / num_samples as f32).abs();
            post_amplitude = (post_amplitude / num_channels as f32).abs();

            let current_pre_amplitude = self
                .pre_amplitude
                .load(std::sync::atomic::Ordering::Relaxed);

            let current_post_amplitude = self
                .post_amplitude
                .load(std::sync::atomic::Ordering::Relaxed);

            self.pre_amplitude.store(
                self.calculate_amplitude(current_pre_amplitude, pre_amplitude),
                Ordering::Relaxed,
            );
            self.post_amplitude.store(
                self.calculate_amplitude(current_post_amplitude, post_amplitude),
                Ordering::Relaxed,
            );
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
