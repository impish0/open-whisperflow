use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, Stream, StreamConfig};
use hound::{WavSpec, WavWriter};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::error::{AppError, Result};

pub struct AudioRecorder {
    device: Device,
    config: StreamConfig,
    sample_format: SampleFormat,
    stream: Option<Stream>,
    buffer: Arc<Mutex<Vec<f32>>>,
    is_recording: Arc<Mutex<bool>>,
}

impl AudioRecorder {
    /// Create a new audio recorder with default input device
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .ok_or_else(|| AppError::AudioRecording("No input device available".to_string()))?;

        log::info!("Using audio device: {}", device.name().unwrap_or_default());

        let supported_config = device
            .default_input_config()
            .map_err(|e| AppError::AudioRecording(format!("Failed to get default config: {}", e)))?;

        let sample_format = supported_config.sample_format();
        let config = supported_config.into();

        Ok(Self {
            device,
            config,
            sample_format,
            stream: None,
            buffer: Arc::new(Mutex::new(Vec::new())),
            is_recording: Arc::new(Mutex::new(false)),
        })
    }

    /// Start recording audio
    pub fn start_recording(&mut self) -> Result<()> {
        *self.is_recording.lock().unwrap() = true;
        self.buffer.lock().unwrap().clear();

        let buffer = Arc::clone(&self.buffer);
        let is_recording = Arc::clone(&self.is_recording);

        let err_fn = |err| {
            log::error!("Audio stream error: {}", err);
        };

        let stream = match self.sample_format {
            SampleFormat::F32 => self.build_stream_f32(buffer, is_recording, err_fn)?,
            SampleFormat::I16 => self.build_stream_i16(buffer, is_recording, err_fn)?,
            SampleFormat::U16 => self.build_stream_u16(buffer, is_recording, err_fn)?,
            _ => {
                return Err(AppError::AudioRecording(
                    "Unsupported sample format".to_string(),
                ))
            }
        };

        stream
            .play()
            .map_err(|e| AppError::AudioRecording(format!("Failed to play stream: {}", e)))?;

        self.stream = Some(stream);
        log::info!("Started recording");

        Ok(())
    }

    /// Stop recording and save to WAV file
    pub fn stop_recording(&mut self) -> Result<PathBuf> {
        *self.is_recording.lock().unwrap() = false;

        if let Some(stream) = self.stream.take() {
            drop(stream);
        }

        let buffer = self.buffer.lock().unwrap();

        if buffer.is_empty() {
            return Err(AppError::AudioRecording(
                "No audio data recorded".to_string(),
            ));
        }

        // Save to temporary file
        let temp_path = std::env::temp_dir().join(format!(
            "openwhisperflow_{}.wav",
            chrono::Utc::now().timestamp()
        ));

        self.save_wav(&temp_path, &buffer)?;

        log::info!("Stopped recording, saved to: {}", temp_path.display());
        log::info!(
            "Recorded {} samples ({:.2} seconds)",
            buffer.len(),
            buffer.len() as f32 / self.config.sample_rate.0 as f32
        );

        Ok(temp_path)
    }

    /// Save audio buffer to WAV file
    fn save_wav(&self, path: &Path, samples: &[f32]) -> Result<()> {
        let spec = WavSpec {
            channels: self.config.channels,
            sample_rate: self.config.sample_rate.0,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = WavWriter::create(path, spec)
            .map_err(|e| AppError::AudioRecording(format!("Failed to create WAV file: {}", e)))?;

        for &sample in samples {
            // Convert f32 [-1.0, 1.0] to i16
            let sample_i16 = (sample * i16::MAX as f32) as i16;
            writer
                .write_sample(sample_i16)
                .map_err(|e| AppError::AudioRecording(format!("Failed to write sample: {}", e)))?;
        }

        writer
            .finalize()
            .map_err(|e| AppError::AudioRecording(format!("Failed to finalize WAV file: {}", e)))?;

        Ok(())
    }

    /// Build audio input stream for f32 samples
    fn build_stream_f32(
        &self,
        buffer: Arc<Mutex<Vec<f32>>>,
        is_recording: Arc<Mutex<bool>>,
        err_fn: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<Stream> {
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[f32], _: &_| {
                    if *is_recording.lock().unwrap() {
                        let mut buf = buffer.lock().unwrap();
                        buf.extend_from_slice(data);
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| AppError::AudioRecording(format!("Failed to build stream: {}", e)))?;

        Ok(stream)
    }

    /// Build audio input stream for i16 samples
    fn build_stream_i16(
        &self,
        buffer: Arc<Mutex<Vec<f32>>>,
        is_recording: Arc<Mutex<bool>>,
        err_fn: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<Stream> {
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[i16], _: &_| {
                    if *is_recording.lock().unwrap() {
                        let mut buf = buffer.lock().unwrap();
                        // Convert i16 to f32 by normalizing
                        buf.extend(data.iter().map(|&s| s as f32 / i16::MAX as f32));
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| AppError::AudioRecording(format!("Failed to build stream: {}", e)))?;

        Ok(stream)
    }

    /// Build audio input stream for u16 samples
    fn build_stream_u16(
        &self,
        buffer: Arc<Mutex<Vec<f32>>>,
        is_recording: Arc<Mutex<bool>>,
        err_fn: impl FnMut(cpal::StreamError) + Send + 'static,
    ) -> Result<Stream> {
        let stream = self
            .device
            .build_input_stream(
                &self.config,
                move |data: &[u16], _: &_| {
                    if *is_recording.lock().unwrap() {
                        let mut buf = buffer.lock().unwrap();
                        // Convert u16 to f32 by normalizing (u16 is 0-65535, convert to -1.0 to 1.0)
                        buf.extend(data.iter().map(|&s| (s as f32 / u16::MAX as f32) * 2.0 - 1.0));
                    }
                },
                err_fn,
                None,
            )
            .map_err(|e| AppError::AudioRecording(format!("Failed to build stream: {}", e)))?;

        Ok(stream)
    }
}

impl Drop for AudioRecorder {
    fn drop(&mut self) {
        *self.is_recording.lock().unwrap() = false;
        if let Some(stream) = self.stream.take() {
            drop(stream);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_recorder_creation() {
        let recorder = AudioRecorder::new();
        assert!(recorder.is_ok());
    }
}
