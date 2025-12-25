//! Multi-modal support for vision and audio models.
//!
//! This module provides abstractions for handling multi-modal inputs
//! including images and audio for models like GPT-4 Vision, Claude 3, and Gemini Pro Vision.

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Represents an image input in various formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageInput {
    /// Image from a URL.
    Url(String),
    /// Image as base64-encoded data with MIME type.
    Base64 { mime_type: String, data: String },
    /// Image from a file path (will be read and encoded).
    FilePath(String),
}

impl ImageInput {
    /// Creates an image input from a URL.
    pub fn from_url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// Creates an image input from base64 data.
    pub fn from_base64(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::Base64 {
            mime_type: mime_type.into(),
            data: data.into(),
        }
    }

    /// Creates an image input from a file path.
    pub fn from_file(path: impl Into<String>) -> Self {
        Self::FilePath(path.into())
    }

    /// Converts the image input to base64 format.
    ///
    /// If the input is already base64, returns it as-is.
    /// If it's a file path, reads and encodes the file.
    /// If it's a URL, returns an error (URLs should be handled by the provider).
    pub fn to_base64(&self) -> Result<(String, String)> {
        match self {
            Self::Base64 { mime_type, data } => Ok((mime_type.clone(), data.clone())),
            Self::FilePath(path) => {
                let bytes = std::fs::read(path)?;
                let mime_type = Self::detect_mime_type(path)?;
                let encoded = base64_encode(&bytes);
                Ok((mime_type, encoded))
            }
            Self::Url(_) => Err(anyhow!(
                "Cannot convert URL to base64; use provider's URL support"
            )),
        }
    }

    /// Detects the MIME type from a file path.
    fn detect_mime_type(path: &str) -> Result<String> {
        let path = Path::new(path);
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("Cannot determine file extension"))?
            .to_lowercase();

        Ok(match extension.as_str() {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "webp" => "image/webp",
            "bmp" => "image/bmp",
            "svg" => "image/svg+xml",
            _ => return Err(anyhow!("Unsupported image format: {}", extension)),
        }
        .to_string())
    }
}

/// Helper function to encode bytes as base64.
fn base64_encode(bytes: &[u8]) -> String {
    use std::io::Write;
    let mut buf = Vec::new();
    {
        let mut encoder =
            base64::write::EncoderWriter::new(&mut buf, &base64::engine::general_purpose::STANDARD);
        encoder.write_all(bytes).unwrap();
    }
    String::from_utf8(buf).unwrap()
}

/// Represents an audio input in various formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioInput {
    /// Audio from a URL.
    Url(String),
    /// Audio as base64-encoded data with MIME type.
    Base64 { mime_type: String, data: String },
    /// Audio from a file path (will be read and encoded).
    FilePath(String),
}

impl AudioInput {
    /// Creates an audio input from a URL.
    pub fn from_url(url: impl Into<String>) -> Self {
        Self::Url(url.into())
    }

    /// Creates an audio input from base64 data.
    pub fn from_base64(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::Base64 {
            mime_type: mime_type.into(),
            data: data.into(),
        }
    }

    /// Creates an audio input from a file path.
    pub fn from_file(path: impl Into<String>) -> Self {
        Self::FilePath(path.into())
    }

    /// Converts the audio input to base64 format.
    pub fn to_base64(&self) -> Result<(String, String)> {
        match self {
            Self::Base64 { mime_type, data } => Ok((mime_type.clone(), data.clone())),
            Self::FilePath(path) => {
                let bytes = std::fs::read(path)?;
                let mime_type = Self::detect_mime_type(path)?;
                let encoded = base64_encode(&bytes);
                Ok((mime_type, encoded))
            }
            Self::Url(_) => Err(anyhow!(
                "Cannot convert URL to base64; use provider's URL support"
            )),
        }
    }

    /// Detects the MIME type from an audio file path.
    fn detect_mime_type(path: &str) -> Result<String> {
        let path = Path::new(path);
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| anyhow!("Cannot determine file extension"))?
            .to_lowercase();

        Ok(match extension.as_str() {
            "mp3" => "audio/mpeg",
            "wav" => "audio/wav",
            "ogg" => "audio/ogg",
            "flac" => "audio/flac",
            "m4a" => "audio/mp4",
            "aac" => "audio/aac",
            "webm" => "audio/webm",
            _ => return Err(anyhow!("Unsupported audio format: {}", extension)),
        }
        .to_string())
    }
}

/// Represents different types of content in a multi-modal message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentPart {
    /// Text content.
    Text(String),
    /// Image content.
    Image(ImageInput),
    /// Audio content.
    Audio(AudioInput),
}

impl ContentPart {
    /// Creates a text content part.
    pub fn text(content: impl Into<String>) -> Self {
        Self::Text(content.into())
    }

    /// Creates an image content part from a URL.
    pub fn image_url(url: impl Into<String>) -> Self {
        Self::Image(ImageInput::from_url(url))
    }

    /// Creates an image content part from a file path.
    pub fn image_file(path: impl Into<String>) -> Self {
        Self::Image(ImageInput::from_file(path))
    }

    /// Creates an image content part from base64 data.
    pub fn image_base64(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::Image(ImageInput::from_base64(mime_type, data))
    }

    /// Creates an audio content part from a URL.
    pub fn audio_url(url: impl Into<String>) -> Self {
        Self::Audio(AudioInput::from_url(url))
    }

    /// Creates an audio content part from a file path.
    pub fn audio_file(path: impl Into<String>) -> Self {
        Self::Audio(AudioInput::from_file(path))
    }

    /// Creates an audio content part from base64 data.
    pub fn audio_base64(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self::Audio(AudioInput::from_base64(mime_type, data))
    }
}

/// A multi-modal message with text and/or images.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiModalMessage {
    /// The role of the message sender (e.g., "user", "assistant", "system").
    pub role: String,
    /// The content parts of the message.
    pub content: Vec<ContentPart>,
}

impl MultiModalMessage {
    /// Creates a new multi-modal message.
    pub fn new(role: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: Vec::new(),
        }
    }

    /// Adds a text part to the message.
    pub fn add_text(mut self, text: impl Into<String>) -> Self {
        self.content.push(ContentPart::text(text));
        self
    }

    /// Adds an image from a URL.
    pub fn add_image_url(mut self, url: impl Into<String>) -> Self {
        self.content.push(ContentPart::image_url(url));
        self
    }

    /// Adds an image from a file path.
    pub fn add_image_file(mut self, path: impl Into<String>) -> Self {
        self.content.push(ContentPart::image_file(path));
        self
    }

    /// Adds an image from base64 data.
    pub fn add_image_base64(
        mut self,
        mime_type: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        self.content
            .push(ContentPart::image_base64(mime_type, data));
        self
    }

    /// Adds an audio from a URL.
    pub fn add_audio_url(mut self, url: impl Into<String>) -> Self {
        self.content.push(ContentPart::audio_url(url));
        self
    }

    /// Adds an audio from a file path.
    pub fn add_audio_file(mut self, path: impl Into<String>) -> Self {
        self.content.push(ContentPart::audio_file(path));
        self
    }

    /// Adds an audio from base64 data.
    pub fn add_audio_base64(
        mut self,
        mime_type: impl Into<String>,
        data: impl Into<String>,
    ) -> Self {
        self.content
            .push(ContentPart::audio_base64(mime_type, data));
        self
    }

    /// Returns true if the message contains any images.
    pub fn has_images(&self) -> bool {
        self.content
            .iter()
            .any(|part| matches!(part, ContentPart::Image(_)))
    }

    /// Returns the number of images in the message.
    pub fn image_count(&self) -> usize {
        self.content
            .iter()
            .filter(|part| matches!(part, ContentPart::Image(_)))
            .count()
    }

    /// Returns true if the message contains any audio.
    pub fn has_audio(&self) -> bool {
        self.content
            .iter()
            .any(|part| matches!(part, ContentPart::Audio(_)))
    }

    /// Returns the number of audio clips in the message.
    pub fn audio_count(&self) -> usize {
        self.content
            .iter()
            .filter(|part| matches!(part, ContentPart::Audio(_)))
            .count()
    }

    /// Extracts all text content concatenated.
    pub fn text_content(&self) -> String {
        self.content
            .iter()
            .filter_map(|part| match part {
                ContentPart::Text(text) => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ")
    }
}

/// Builder for creating multi-modal prompts easily.
pub struct MultiModalPromptBuilder {
    messages: Vec<MultiModalMessage>,
}

impl MultiModalPromptBuilder {
    /// Creates a new prompt builder.
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// Adds a user message with text.
    pub fn user_text(mut self, text: impl Into<String>) -> Self {
        self.messages
            .push(MultiModalMessage::new("user").add_text(text));
        self
    }

    /// Adds a user message with text and image from URL.
    pub fn user_with_image_url(
        mut self,
        text: impl Into<String>,
        image_url: impl Into<String>,
    ) -> Self {
        self.messages.push(
            MultiModalMessage::new("user")
                .add_text(text)
                .add_image_url(image_url),
        );
        self
    }

    /// Adds a user message with text and image from file.
    pub fn user_with_image_file(
        mut self,
        text: impl Into<String>,
        image_path: impl Into<String>,
    ) -> Self {
        self.messages.push(
            MultiModalMessage::new("user")
                .add_text(text)
                .add_image_file(image_path),
        );
        self
    }

    /// Adds a system message.
    pub fn system(mut self, text: impl Into<String>) -> Self {
        self.messages
            .push(MultiModalMessage::new("system").add_text(text));
        self
    }

    /// Adds a custom message.
    pub fn message(mut self, message: MultiModalMessage) -> Self {
        self.messages.push(message);
        self
    }

    /// Builds the final list of messages.
    pub fn build(self) -> Vec<MultiModalMessage> {
        self.messages
    }
}

impl Default for MultiModalPromptBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Vision-specific configuration.
#[derive(Debug, Clone)]
pub struct VisionConfig {
    /// Maximum number of images per request.
    pub max_images: usize,
    /// Whether to include image details (e.g., high/low resolution).
    pub detail_level: ImageDetailLevel,
}

/// Image detail level for vision models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageDetailLevel {
    /// Low resolution, faster processing.
    Low,
    /// High resolution, more detailed analysis.
    High,
    /// Automatic selection based on image size.
    Auto,
}

impl Default for VisionConfig {
    fn default() -> Self {
        Self {
            max_images: 10,
            detail_level: ImageDetailLevel::Auto,
        }
    }
}

impl VisionConfig {
    /// Creates a new vision config with default values.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the maximum number of images.
    pub fn with_max_images(mut self, max_images: usize) -> Self {
        self.max_images = max_images;
        self
    }

    /// Sets the image detail level.
    pub fn with_detail_level(mut self, level: ImageDetailLevel) -> Self {
        self.detail_level = level;
        self
    }
}

// Add base64 dependency
mod base64_impl {
    pub struct Base64Engine;

    pub mod engine {
        pub mod general_purpose {
            use super::super::Base64Engine;
            pub const STANDARD: Base64Engine = Base64Engine;
        }
    }

    pub mod write {
        use super::Base64Engine;
        use std::io::{self, Write};

        pub struct EncoderWriter<W: Write> {
            writer: W,
            buf: Vec<u8>,
        }

        impl<W: Write> EncoderWriter<W> {
            pub fn new(writer: W, _engine: &Base64Engine) -> Self {
                Self {
                    writer,
                    buf: Vec::new(),
                }
            }
        }

        impl<W: Write> Write for EncoderWriter<W> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.buf.extend_from_slice(buf);
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                let encoded = simple_base64_encode(&self.buf);
                self.writer.write_all(encoded.as_bytes())?;
                self.buf.clear();
                self.writer.flush()
            }
        }

        impl<W: Write> Drop for EncoderWriter<W> {
            fn drop(&mut self) {
                let _ = self.flush();
            }
        }

        fn simple_base64_encode(input: &[u8]) -> String {
            const CHARSET: &[u8] =
                b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let mut result = String::new();
            let mut i = 0;

            while i < input.len() {
                let b1 = input[i];
                let b2 = if i + 1 < input.len() { input[i + 1] } else { 0 };
                let b3 = if i + 2 < input.len() { input[i + 2] } else { 0 };

                result.push(CHARSET[(b1 >> 2) as usize] as char);
                result.push(CHARSET[(((b1 & 0x03) << 4) | (b2 >> 4)) as usize] as char);

                if i + 1 < input.len() {
                    result.push(CHARSET[(((b2 & 0x0F) << 2) | (b3 >> 6)) as usize] as char);
                } else {
                    result.push('=');
                }

                if i + 2 < input.len() {
                    result.push(CHARSET[(b3 & 0x3F) as usize] as char);
                } else {
                    result.push('=');
                }

                i += 3;
            }

            result
        }
    }
}

use base64_impl as base64;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_image_input_from_url() {
        let input = ImageInput::from_url("https://example.com/image.jpg");
        matches!(input, ImageInput::Url(_));
    }

    #[test]
    fn test_image_input_from_base64() {
        let input = ImageInput::from_base64("image/png", "base64data");
        match input {
            ImageInput::Base64 { mime_type, data } => {
                assert_eq!(mime_type, "image/png");
                assert_eq!(data, "base64data");
            }
            _ => panic!("Expected Base64 variant"),
        }
    }

    #[test]
    fn test_multimodal_message_builder() {
        let message = MultiModalMessage::new("user")
            .add_text("What's in this image?")
            .add_image_url("https://example.com/image.jpg");

        assert_eq!(message.role, "user");
        assert_eq!(message.content.len(), 2);
        assert!(message.has_images());
        assert_eq!(message.image_count(), 1);
    }

    #[test]
    fn test_multimodal_prompt_builder() {
        let messages = MultiModalPromptBuilder::new()
            .system("You are a helpful assistant.")
            .user_with_image_url("Describe this image", "https://example.com/image.jpg")
            .build();

        assert_eq!(messages.len(), 2);
        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
        assert!(messages[1].has_images());
    }

    #[test]
    fn test_text_content_extraction() {
        let message = MultiModalMessage::new("user")
            .add_text("First part")
            .add_image_url("https://example.com/image.jpg")
            .add_text("Second part");

        assert_eq!(message.text_content(), "First part Second part");
    }

    #[test]
    fn test_vision_config_builder() {
        let config = VisionConfig::new()
            .with_max_images(5)
            .with_detail_level(ImageDetailLevel::High);

        assert_eq!(config.max_images, 5);
        assert_eq!(config.detail_level, ImageDetailLevel::High);
    }

    #[test]
    fn test_mime_type_detection() {
        assert_eq!(
            ImageInput::detect_mime_type("test.jpg").unwrap(),
            "image/jpeg"
        );
        assert_eq!(
            ImageInput::detect_mime_type("test.png").unwrap(),
            "image/png"
        );
        assert_eq!(
            ImageInput::detect_mime_type("test.gif").unwrap(),
            "image/gif"
        );
        assert_eq!(
            ImageInput::detect_mime_type("test.webp").unwrap(),
            "image/webp"
        );
    }

    #[test]
    fn test_base64_encoding() {
        let data = b"Hello, World!";
        let encoded = base64_encode(data);
        assert!(!encoded.is_empty());
        // Basic validation that it looks like base64
        assert!(
            encoded
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=')
        );
    }

    #[test]
    fn test_audio_input_from_url() {
        let input = AudioInput::from_url("https://example.com/audio.mp3");
        matches!(input, AudioInput::Url(_));
    }

    #[test]
    fn test_audio_input_from_base64() {
        let input = AudioInput::from_base64("audio/mpeg", "base64audiodata");
        match input {
            AudioInput::Base64 { mime_type, data } => {
                assert_eq!(mime_type, "audio/mpeg");
                assert_eq!(data, "base64audiodata");
            }
            _ => panic!("Expected Base64 variant"),
        }
    }

    #[test]
    fn test_audio_mime_type_detection() {
        assert_eq!(
            AudioInput::detect_mime_type("test.mp3").unwrap(),
            "audio/mpeg"
        );
        assert_eq!(
            AudioInput::detect_mime_type("test.wav").unwrap(),
            "audio/wav"
        );
        assert_eq!(
            AudioInput::detect_mime_type("test.ogg").unwrap(),
            "audio/ogg"
        );
        assert_eq!(
            AudioInput::detect_mime_type("test.flac").unwrap(),
            "audio/flac"
        );
    }

    #[test]
    fn test_multimodal_message_with_audio() {
        let message = MultiModalMessage::new("user")
            .add_text("Transcribe this audio")
            .add_audio_url("https://example.com/audio.mp3");

        assert_eq!(message.role, "user");
        assert_eq!(message.content.len(), 2);
        assert!(message.has_audio());
        assert_eq!(message.audio_count(), 1);
    }

    #[test]
    fn test_multimodal_message_mixed_content() {
        let message = MultiModalMessage::new("user")
            .add_text("Analyze this")
            .add_image_url("https://example.com/image.jpg")
            .add_audio_url("https://example.com/audio.mp3");

        assert_eq!(message.content.len(), 3);
        assert!(message.has_images());
        assert!(message.has_audio());
        assert_eq!(message.image_count(), 1);
        assert_eq!(message.audio_count(), 1);
    }
}
