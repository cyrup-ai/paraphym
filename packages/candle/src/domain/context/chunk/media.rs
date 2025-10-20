//! Media Chunk Types
//!
//! Chunk types for streaming media operations including images, audio, 
//! voice, and speech data.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use cyrup_sugars::prelude::MessageChunk;

/// Candle image format types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum CandleImageFormat {
    /// Portable Network Graphics format
    PNG,
    /// Joint Photographic Experts Group format
    JPEG,
    /// Graphics Interchange Format
    GIF,
    /// WebP image format by Google
    WebP,
    /// Bitmap image format
    BMP,
    /// Tagged Image File Format
    TIFF,
}

/// Candle chunk of image data for streaming image operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandleImageChunk {
    /// Raw image data
    pub data: Vec<u8>,

    /// Image format
    pub format: CandleImageFormat,

    /// Optional dimensions (width, height)
    pub dimensions: Option<(u32, u32)>,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Audio format types
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AudioFormat {
    /// MPEG-1 Audio Layer III compressed audio format
    MP3,
    /// Waveform Audio File Format (uncompressed)
    WAV,
    /// Free Lossless Audio Codec
    FLAC,
    /// Ogg Vorbis compressed audio format
    OGG,
    /// MPEG-4 Audio compressed format (AAC)
    M4A,
    /// Opus low-latency audio codec
    OPUS,
}

/// Chunk of audio/voice data for streaming audio operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceChunk {
    /// Raw audio data
    pub audio_data: Vec<u8>,

    /// Audio format
    pub format: AudioFormat,

    /// Duration in milliseconds
    pub duration_ms: Option<u64>,

    /// Sample rate in Hz
    pub sample_rate: Option<u32>,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Chunk of transcribed text from speech-to-text
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionChunk {
    /// The transcribed text for this chunk
    pub text: String,

    /// Confidence score (0.0 to 1.0)
    pub confidence: Option<f32>,

    /// Start time in milliseconds
    pub start_time_ms: Option<u64>,

    /// End time in milliseconds  
    pub end_time_ms: Option<u64>,

    /// Whether this is the final chunk
    pub is_final: bool,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}

/// Chunk of synthesized speech for text-to-speech
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechChunk {
    /// Raw audio data
    pub audio_data: Vec<u8>,

    /// Audio format
    pub format: AudioFormat,

    /// Duration in milliseconds
    pub duration_ms: Option<u64>,

    /// Sample rate in Hz
    pub sample_rate: Option<u32>,

    /// Whether this is the final chunk
    pub is_final: bool,

    /// Additional metadata
    #[serde(flatten)]
    pub metadata: HashMap<String, Value>,
}
