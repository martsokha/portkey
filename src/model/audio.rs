//! Audio API models.
//!
//! This module contains data models for audio transcription using Whisper and GPT models.

use serde::{Deserialize, Serialize};

/// Response format for audio transcription.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TranscriptionResponseFormat {
    /// JSON format with just the transcribed text.
    Json,
    /// Plain text format.
    Text,
    /// SubRip Subtitle format.
    Srt,
    /// Verbose JSON format with additional metadata.
    VerboseJson,
    /// WebVTT subtitle format.
    Vtt,
}

impl Default for TranscriptionResponseFormat {
    fn default() -> Self {
        Self::Json
    }
}

/// Timestamp granularity for verbose transcription.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimestampGranularity {
    /// Word-level timestamps (incurs additional latency).
    Word,
    /// Segment-level timestamps (no additional latency).
    Segment,
}

/// Request for creating an audio transcription.
///
/// # Example
///
/// ```
/// use portkey_sdk::model::{CreateTranscriptionRequest, TranscriptionResponseFormat};
///
/// let request = CreateTranscriptionRequest {
///     model: "whisper-1".to_string(),
///     language: Some("en".to_string()),
///     response_format: Some(TranscriptionResponseFormat::Json),
///     temperature: Some(0.0),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTranscriptionRequest {
    /// ID of the model to use.
    ///
    /// Options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
    pub model: String,

    /// The language of the input audio in ISO-639-1 format.
    ///
    /// Supplying the input language will improve accuracy and latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,

    /// Optional text to guide the model's style or continue a previous audio segment.
    ///
    /// The prompt should match the audio language.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// The format of the transcript output.
    ///
    /// Options: `json`, `text`, `srt`, `verbose_json`, or `vtt`. Defaults to `json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<TranscriptionResponseFormat>,

    /// The sampling temperature, between 0 and 1.
    ///
    /// Higher values like 0.8 make output more random, lower values like 0.2 make it more focused.
    /// If set to 0, the model will use log probability to automatically increase temperature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,

    /// The timestamp granularities to populate for this transcription.
    ///
    /// `response_format` must be set to `verbose_json` to use timestamp granularities.
    /// Either or both of `word` or `segment` are supported. Note: word timestamps incur
    /// additional latency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp_granularities: Option<Vec<TimestampGranularity>>,
}

/// A word with timestamp information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionWord {
    /// The text content of the word.
    pub word: String,

    /// Start time of the word in seconds.
    pub start: f32,

    /// End time of the word in seconds.
    pub end: f32,
}

/// A segment of transcribed text with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    /// Unique identifier of the segment.
    pub id: i32,

    /// Seek offset of the segment.
    pub seek: i32,

    /// Start time of the segment in seconds.
    pub start: f32,

    /// End time of the segment in seconds.
    pub end: f32,

    /// Text content of the segment.
    pub text: String,

    /// Array of token IDs for the text content.
    pub tokens: Vec<i32>,

    /// Temperature parameter used for generating the segment.
    pub temperature: f32,

    /// Average logprob of the segment.
    ///
    /// If the value is lower than -1, consider the logprobs failed.
    pub avg_logprob: f32,

    /// Compression ratio of the segment.
    ///
    /// If the value is greater than 2.4, consider the compression failed.
    pub compression_ratio: f32,

    /// Probability of no speech in the segment.
    ///
    /// If the value is higher than 1.0 and the `avg_logprob` is below -1,
    /// consider this segment silent.
    pub no_speech_prob: f32,
}

/// Simple JSON transcription response.
///
/// Contains only the transcribed text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTranscriptionResponseJson {
    /// The transcribed text.
    pub text: String,
}

/// Verbose JSON transcription response.
///
/// Contains the transcribed text along with additional metadata, segments, and optional word-level timestamps.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTranscriptionResponseVerboseJson {
    /// The language of the input audio.
    pub language: String,

    /// The duration of the input audio.
    pub duration: String,

    /// The transcribed text.
    pub text: String,

    /// Extracted words and their corresponding timestamps.
    ///
    /// Only present if `timestamp_granularities` included `word`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub words: Option<Vec<TranscriptionWord>>,

    /// Segments of the transcribed text and their corresponding details.
    ///
    /// Only present if `timestamp_granularities` included `segment`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegment>>,
}

/// Response from audio transcription.
///
/// The structure depends on the `response_format` specified in the request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranscriptionResponse {
    /// Simple JSON response with just the text.
    Json(CreateTranscriptionResponseJson),
    /// Verbose JSON response with additional metadata.
    VerboseJson(CreateTranscriptionResponseVerboseJson),
}

// ============================================================================
// Speech (Text-to-Speech) Models
// ============================================================================

/// Voice options for text-to-speech generation.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Voice {
    /// Alloy voice
    Alloy,
    /// Echo voice
    Echo,
    /// Fable voice
    Fable,
    /// Onyx voice
    Onyx,
    /// Nova voice
    Nova,
    /// Shimmer voice
    Shimmer,
}

/// Audio format for speech output.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SpeechResponseFormat {
    /// MP3 format (default)
    Mp3,
    /// Opus format
    Opus,
    /// AAC format
    Aac,
    /// FLAC format
    Flac,
    /// WAV format
    Wav,
    /// PCM format
    Pcm,
}

impl Default for SpeechResponseFormat {
    fn default() -> Self {
        Self::Mp3
    }
}

/// Request for creating speech from text.
///
/// # Example
///
/// ```
/// use portkey_sdk::model::{CreateSpeechRequest, Voice, SpeechResponseFormat};
///
/// let request = CreateSpeechRequest {
///     model: "tts-1".to_string(),
///     input: "The quick brown fox jumped over the lazy dog.".to_string(),
///     voice: Voice::Alloy,
///     response_format: Some(SpeechResponseFormat::Mp3),
///     speed: Some(1.0),
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpeechRequest {
    /// TTS model to use (e.g., "tts-1" or "tts-1-hd")
    pub model: String,

    /// The text to generate audio for.
    ///
    /// Maximum length is 4096 characters.
    pub input: String,

    /// The voice to use for generation.
    pub voice: Voice,

    /// The format for the audio output.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<SpeechResponseFormat>,

    /// The speed of the generated audio (0.25 to 4.0).
    ///
    /// Default is 1.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f32>,
}

// ============================================================================
// Translation Models
// ============================================================================

/// Request for translating audio to English.
///
/// # Example
///
/// ```
/// use portkey_sdk::model::{CreateTranslationRequest, TranscriptionResponseFormat};
///
/// let request = CreateTranslationRequest {
///     model: "whisper-1".to_string(),
///     prompt: Some("Optional prompt in English".to_string()),
///     response_format: Some(TranscriptionResponseFormat::Json),
///     temperature: Some(0.0),
/// };
/// ```
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateTranslationRequest {
    /// ID of the model to use.
    ///
    /// Options are `gpt-4o-transcribe`, `gpt-4o-mini-transcribe`, and `whisper-1`.
    pub model: String,

    /// Optional text to guide the model's style.
    ///
    /// The prompt should be in English.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    /// The format of the translation output.
    ///
    /// Options: `json`, `text`, `srt`, `verbose_json`, or `vtt`. Defaults to `json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<TranscriptionResponseFormat>,

    /// The sampling temperature, between 0 and 1.
    ///
    /// Higher values like 0.8 make output more random, lower values like 0.2 make it more focused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
}

/// Simple JSON translation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTranslationResponseJson {
    /// The translated text (always in English).
    pub text: String,
}

/// Verbose JSON translation response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTranslationResponseVerboseJson {
    /// The language of the output translation (always "english").
    pub language: String,

    /// The duration of the input audio.
    pub duration: String,

    /// The translated text.
    pub text: String,

    /// Segments of the translated text and their corresponding details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<Vec<TranscriptionSegment>>,
}

/// Response from audio translation.
///
/// The structure depends on the `response_format` specified in the request.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TranslationResponse {
    /// Simple JSON response with just the text.
    Json(CreateTranslationResponseJson),
    /// Verbose JSON response with additional metadata.
    VerboseJson(CreateTranslationResponseVerboseJson),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transcription_response_format() {
        let format = TranscriptionResponseFormat::Json;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, "\"json\"");

        let format = TranscriptionResponseFormat::VerboseJson;
        let json = serde_json::to_string(&format).unwrap();
        assert_eq!(json, "\"verbose_json\"");
    }

    #[test]
    fn test_timestamp_granularity() {
        let granularity = TimestampGranularity::Word;
        let json = serde_json::to_string(&granularity).unwrap();
        assert_eq!(json, "\"word\"");

        let granularity = TimestampGranularity::Segment;
        let json = serde_json::to_string(&granularity).unwrap();
        assert_eq!(json, "\"segment\"");
    }

    #[test]
    fn test_create_transcription_request() {
        let request = CreateTranscriptionRequest {
            model: "whisper-1".to_string(),
            language: Some("en".to_string()),
            response_format: Some(TranscriptionResponseFormat::VerboseJson),
            temperature: Some(0.0),
            timestamp_granularities: Some(vec![
                TimestampGranularity::Word,
                TimestampGranularity::Segment,
            ]),
            ..Default::default()
        };

        assert_eq!(request.model, "whisper-1");
        assert_eq!(request.language, Some("en".to_string()));
        assert_eq!(request.temperature, Some(0.0));
    }
}
