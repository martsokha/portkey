//! Audio API service.
//!
//! This module provides methods for audio transcription using Whisper and GPT models.

use std::future::Future;

use reqwest::multipart::{Form, Part};

use crate::client::PortkeyClient;
use crate::error::Result;
use crate::model::{
    CreateSpeechRequest, CreateTranscriptionRequest, CreateTranslationRequest,
    TranscriptionResponse, TranslationResponse,
};

/// Trait for Audio API operations.
pub trait AudioService {
    /// Creates a transcription of an audio file.
    ///
    /// # Arguments
    ///
    /// * `file_data` - The audio file data as bytes
    /// * `file_name` - The name of the audio file
    /// * `request` - The transcription request parameters
    ///
    /// # Returns
    ///
    /// Returns a `TranscriptionResponse` containing the transcribed text and optional metadata.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use portkey_sdk::{PortkeyConfig, PortkeyClient, AudioService, AuthMethod};
    /// use portkey_sdk::model::{CreateTranscriptionRequest, TranscriptionResponseFormat};
    /// use std::fs;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_auth_method(AuthMethod::VirtualKey {
    ///         virtual_key: "your-virtual-key".to_string()
    ///     })
    ///     .build()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// let audio_data = fs::read("audio.mp3")?;
    ///
    /// let request = CreateTranscriptionRequest {
    ///     model: "whisper-1".to_string(),
    ///     language: Some("en".to_string()),
    ///     response_format: Some(TranscriptionResponseFormat::Json),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.create_transcription(
    ///     audio_data,
    ///     "audio.mp3",
    ///     request
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn create_transcription(
        &self,
        file_data: Vec<u8>,
        file_name: &str,
        request: CreateTranscriptionRequest,
    ) -> impl Future<Output = Result<TranscriptionResponse>>;

    /// Creates speech audio from text input.
    ///
    /// # Arguments
    ///
    /// * `request` - The speech generation request with text, voice, and options
    ///
    /// # Returns
    ///
    /// Returns audio data as bytes in the specified format.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use portkey_sdk::{PortkeyConfig, PortkeyClient, AudioService, AuthMethod};
    /// use portkey_sdk::model::{CreateSpeechRequest, Voice, SpeechResponseFormat};
    /// use std::fs;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_auth_method(AuthMethod::VirtualKey {
    ///         virtual_key: "your-virtual-key".to_string()
    ///     })
    ///     .build()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// let request = CreateSpeechRequest {
    ///     model: "tts-1".to_string(),
    ///     input: "The quick brown fox jumped over the lazy dog.".to_string(),
    ///     voice: Voice::Alloy,
    ///     response_format: Some(SpeechResponseFormat::Mp3),
    ///     speed: Some(1.0),
    /// };
    ///
    /// let audio_data = client.create_speech(request).await?;
    /// fs::write("speech.mp3", audio_data)?;
    /// # Ok(())
    /// # }
    /// ```
    fn create_speech(&self, request: CreateSpeechRequest) -> impl Future<Output = Result<Vec<u8>>>;

    /// Translates audio to English.
    ///
    /// # Arguments
    ///
    /// * `file_data` - The audio file data as bytes
    /// * `file_name` - The name of the audio file
    /// * `request` - The translation request parameters
    ///
    /// # Returns
    ///
    /// Returns a `TranslationResponse` containing the translated text (in English).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use portkey_sdk::{PortkeyConfig, PortkeyClient, AudioService, AuthMethod};
    /// use portkey_sdk::model::{CreateTranslationRequest, TranscriptionResponseFormat};
    /// use std::fs;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let config = PortkeyConfig::builder()
    ///     .with_api_key("your-api-key")
    ///     .with_auth_method(AuthMethod::VirtualKey {
    ///         virtual_key: "your-virtual-key".to_string()
    ///     })
    ///     .build()?;
    /// let client = PortkeyClient::new(config)?;
    ///
    /// let audio_data = fs::read("german.m4a")?;
    ///
    /// let request = CreateTranslationRequest {
    ///     model: "whisper-1".to_string(),
    ///     prompt: Some("Optional prompt in English".to_string()),
    ///     response_format: Some(TranscriptionResponseFormat::Json),
    ///     ..Default::default()
    /// };
    ///
    /// let response = client.create_translation(
    ///     audio_data,
    ///     "german.m4a",
    ///     request
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    fn create_translation(
        &self,
        file_data: Vec<u8>,
        file_name: &str,
        request: CreateTranslationRequest,
    ) -> impl Future<Output = Result<TranslationResponse>>;
}

impl AudioService for PortkeyClient {
    async fn create_transcription(
        &self,
        file_data: Vec<u8>,
        file_name: &str,
        request: CreateTranscriptionRequest,
    ) -> Result<TranscriptionResponse> {
        // Build multipart form
        let file_part = Part::bytes(file_data).file_name(file_name.to_string());

        let mut form = Form::new()
            .part("file", file_part)
            .text("model", request.model.clone());

        if let Some(language) = request.language {
            form = form.text("language", language);
        }

        if let Some(prompt) = request.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(response_format) = request.response_format {
            let format_str = match response_format {
                crate::model::TranscriptionResponseFormat::Json => "json",
                crate::model::TranscriptionResponseFormat::Text => "text",
                crate::model::TranscriptionResponseFormat::Srt => "srt",
                crate::model::TranscriptionResponseFormat::VerboseJson => "verbose_json",
                crate::model::TranscriptionResponseFormat::Vtt => "vtt",
            };
            form = form.text("response_format", format_str);
        }

        if let Some(temperature) = request.temperature {
            form = form.text("temperature", temperature.to_string());
        }

        if let Some(granularities) = request.timestamp_granularities {
            for granularity in granularities {
                let granularity_str = match granularity {
                    crate::model::TimestampGranularity::Word => "word",
                    crate::model::TimestampGranularity::Segment => "segment",
                };
                form = form.text("timestamp_granularities[]", granularity_str);
            }
        }

        let response = self
            .post("/audio/transcriptions")?
            .multipart(form)
            .send()
            .await?;

        let response = response.error_for_status()?;
        let transcription_response: TranscriptionResponse = response.json().await?;
        Ok(transcription_response)
    }

    async fn create_speech(&self, request: CreateSpeechRequest) -> Result<Vec<u8>> {
        let response = self.post("/audio/speech")?.json(&request).send().await?;

        let response = response.error_for_status()?;
        let audio_bytes = response.bytes().await?;
        Ok(audio_bytes.to_vec())
    }

    async fn create_translation(
        &self,
        file_data: Vec<u8>,
        file_name: &str,
        request: CreateTranslationRequest,
    ) -> Result<TranslationResponse> {
        // Build multipart form
        let file_part = Part::bytes(file_data).file_name(file_name.to_string());

        let mut form = Form::new()
            .part("file", file_part)
            .text("model", request.model.clone());

        if let Some(prompt) = request.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(response_format) = request.response_format {
            let format_str = match response_format {
                crate::model::TranscriptionResponseFormat::Json => "json",
                crate::model::TranscriptionResponseFormat::Text => "text",
                crate::model::TranscriptionResponseFormat::Srt => "srt",
                crate::model::TranscriptionResponseFormat::VerboseJson => "verbose_json",
                crate::model::TranscriptionResponseFormat::Vtt => "vtt",
            };
            form = form.text("response_format", format_str);
        }

        if let Some(temperature) = request.temperature {
            form = form.text("temperature", temperature.to_string());
        }

        let response = self
            .post("/audio/translations")?
            .multipart(form)
            .send()
            .await?;

        let response = response.error_for_status()?;
        let translation_response: TranslationResponse = response.json().await?;
        Ok(translation_response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_transcription_request() {
        let request = CreateTranscriptionRequest {
            model: "whisper-1".to_string(),
            language: Some("en".to_string()),
            ..Default::default()
        };

        assert_eq!(request.model, "whisper-1");
        assert_eq!(request.language, Some("en".to_string()));
    }
}
