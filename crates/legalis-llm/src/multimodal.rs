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

/// Represents a parsed multi-modal response from an LLM.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiModalResponse {
    /// Text content in the response.
    pub text: Option<String>,
    /// Generated or returned images.
    pub images: Vec<GeneratedImage>,
    /// Generated or returned audio.
    pub audio: Vec<GeneratedAudio>,
    /// Additional metadata from the response.
    pub metadata: serde_json::Value,
}

/// Represents a generated or returned image in a response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedImage {
    /// Image data as base64.
    pub data: Option<String>,
    /// URL to the image (if hosted).
    pub url: Option<String>,
    /// MIME type of the image.
    pub mime_type: String,
    /// Optional caption or description.
    pub caption: Option<String>,
}

/// Represents generated or returned audio in a response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedAudio {
    /// Audio data as base64.
    pub data: Option<String>,
    /// URL to the audio (if hosted).
    pub url: Option<String>,
    /// MIME type of the audio.
    pub mime_type: String,
    /// Optional transcript or description.
    pub transcript: Option<String>,
}

impl MultiModalResponse {
    /// Creates a new empty multi-modal response.
    pub fn new() -> Self {
        Self {
            text: None,
            images: Vec::new(),
            audio: Vec::new(),
            metadata: serde_json::Value::Null,
        }
    }

    /// Creates a response with only text content.
    pub fn text_only(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            images: Vec::new(),
            audio: Vec::new(),
            metadata: serde_json::Value::Null,
        }
    }

    /// Adds text content to the response.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Adds an image to the response.
    pub fn add_image(mut self, image: GeneratedImage) -> Self {
        self.images.push(image);
        self
    }

    /// Adds audio to the response.
    pub fn add_audio(mut self, audio: GeneratedAudio) -> Self {
        self.audio.push(audio);
        self
    }

    /// Adds metadata to the response.
    pub fn with_metadata(mut self, metadata: serde_json::Value) -> Self {
        self.metadata = metadata;
        self
    }

    /// Returns true if the response contains any images.
    pub fn has_images(&self) -> bool {
        !self.images.is_empty()
    }

    /// Returns true if the response contains any audio.
    pub fn has_audio(&self) -> bool {
        !self.audio.is_empty()
    }

    /// Returns true if the response contains text.
    pub fn has_text(&self) -> bool {
        self.text.is_some()
    }

    /// Returns the text content, or an empty string if none.
    pub fn text_or_empty(&self) -> &str {
        self.text.as_deref().unwrap_or("")
    }

    /// Parses a JSON response into a MultiModalResponse.
    ///
    /// This handles various provider-specific formats.
    pub fn from_json(json: &serde_json::Value) -> Result<Self> {
        let mut response = Self::new();

        // Extract text content
        if let Some(text) = json.get("text").and_then(|v| v.as_str()) {
            response.text = Some(text.to_string());
        } else if let Some(content) = json.get("content").and_then(|v| v.as_str()) {
            response.text = Some(content.to_string());
        } else if let Some(message) = json.get("message").and_then(|v| v.as_str()) {
            response.text = Some(message.to_string());
        }

        // Extract images
        if let Some(images) = json.get("images").and_then(|v| v.as_array()) {
            for img in images {
                if let Ok(generated_img) = Self::parse_image(img) {
                    response.images.push(generated_img);
                }
            }
        }

        // Extract audio
        if let Some(audio_arr) = json.get("audio").and_then(|v| v.as_array()) {
            for audio in audio_arr {
                if let Ok(generated_audio) = Self::parse_audio(audio) {
                    response.audio.push(generated_audio);
                }
            }
        }

        // Store full JSON as metadata
        response.metadata = json.clone();

        Ok(response)
    }

    /// Parses an image from JSON.
    fn parse_image(json: &serde_json::Value) -> Result<GeneratedImage> {
        let data = json.get("data").and_then(|v| v.as_str()).map(String::from);
        let url = json.get("url").and_then(|v| v.as_str()).map(String::from);
        let mime_type = json
            .get("mime_type")
            .or_else(|| json.get("mimeType"))
            .and_then(|v| v.as_str())
            .unwrap_or("image/png")
            .to_string();
        let caption = json
            .get("caption")
            .and_then(|v| v.as_str())
            .map(String::from);

        Ok(GeneratedImage {
            data,
            url,
            mime_type,
            caption,
        })
    }

    /// Parses audio from JSON.
    fn parse_audio(json: &serde_json::Value) -> Result<GeneratedAudio> {
        let data = json.get("data").and_then(|v| v.as_str()).map(String::from);
        let url = json.get("url").and_then(|v| v.as_str()).map(String::from);
        let mime_type = json
            .get("mime_type")
            .or_else(|| json.get("mimeType"))
            .and_then(|v| v.as_str())
            .unwrap_or("audio/mpeg")
            .to_string();
        let transcript = json
            .get("transcript")
            .and_then(|v| v.as_str())
            .map(String::from);

        Ok(GeneratedAudio {
            data,
            url,
            mime_type,
            transcript,
        })
    }
}

impl Default for MultiModalResponse {
    fn default() -> Self {
        Self::new()
    }
}

impl GeneratedImage {
    /// Creates a new generated image from base64 data.
    pub fn from_data(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            data: Some(data.into()),
            url: None,
            mime_type: mime_type.into(),
            caption: None,
        }
    }

    /// Creates a new generated image from a URL.
    pub fn from_url(mime_type: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            data: None,
            url: Some(url.into()),
            mime_type: mime_type.into(),
            caption: None,
        }
    }

    /// Adds a caption to the image.
    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }
}

impl GeneratedAudio {
    /// Creates a new generated audio from base64 data.
    pub fn from_data(mime_type: impl Into<String>, data: impl Into<String>) -> Self {
        Self {
            data: Some(data.into()),
            url: None,
            mime_type: mime_type.into(),
            transcript: None,
        }
    }

    /// Creates a new generated audio from a URL.
    pub fn from_url(mime_type: impl Into<String>, url: impl Into<String>) -> Self {
        Self {
            data: None,
            url: Some(url.into()),
            mime_type: mime_type.into(),
            transcript: None,
        }
    }

    /// Adds a transcript to the audio.
    pub fn with_transcript(mut self, transcript: impl Into<String>) -> Self {
        self.transcript = Some(transcript.into());
        self
    }
}

/// Legal document analysis module for multi-modal inputs.
pub mod legal_document_analysis {
    use serde::{Deserialize, Serialize};

    /// Types of legal documents that can be analyzed.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LegalDocumentType {
        /// Contract or agreement.
        Contract,
        /// Court filing or pleading.
        CourtFiling,
        /// Legal brief or memorandum.
        Brief,
        /// Certificate or official document.
        Certificate,
        /// Historical legal document.
        Historical,
        /// Evidence document.
        Evidence,
        /// Signature page.
        Signature,
        /// General legal document.
        General,
    }

    /// Configuration for legal document analysis.
    #[derive(Debug, Clone)]
    pub struct LegalDocAnalysisConfig {
        /// Type of legal document being analyzed.
        pub document_type: LegalDocumentType,
        /// Whether to extract signatures.
        pub extract_signatures: bool,
        /// Whether to extract seals/stamps.
        pub extract_seals: bool,
        /// Whether to perform OCR on handwriting.
        pub ocr_handwriting: bool,
        /// Whether to analyze document layout.
        pub analyze_layout: bool,
        /// Whether to extract metadata (dates, parties, etc.).
        pub extract_metadata: bool,
    }

    impl Default for LegalDocAnalysisConfig {
        fn default() -> Self {
            Self {
                document_type: LegalDocumentType::General,
                extract_signatures: true,
                extract_seals: true,
                ocr_handwriting: false,
                analyze_layout: true,
                extract_metadata: true,
            }
        }
    }

    impl LegalDocAnalysisConfig {
        /// Creates a new config with default values.
        pub fn new(document_type: LegalDocumentType) -> Self {
            Self {
                document_type,
                ..Default::default()
            }
        }

        /// Enables signature extraction.
        pub fn with_signature_extraction(mut self, enable: bool) -> Self {
            self.extract_signatures = enable;
            self
        }

        /// Enables seal/stamp extraction.
        pub fn with_seal_extraction(mut self, enable: bool) -> Self {
            self.extract_seals = enable;
            self
        }

        /// Enables handwriting OCR.
        pub fn with_handwriting_ocr(mut self, enable: bool) -> Self {
            self.ocr_handwriting = enable;
            self
        }

        /// Enables layout analysis.
        pub fn with_layout_analysis(mut self, enable: bool) -> Self {
            self.analyze_layout = enable;
            self
        }

        /// Enables metadata extraction.
        pub fn with_metadata_extraction(mut self, enable: bool) -> Self {
            self.extract_metadata = enable;
            self
        }
    }

    /// Result of legal document analysis.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LegalDocAnalysisResult {
        /// Extracted text content.
        pub text: String,
        /// Detected signatures.
        pub signatures: Vec<SignatureInfo>,
        /// Detected seals/stamps.
        pub seals: Vec<SealInfo>,
        /// Handwritten text (if OCR was performed).
        pub handwritten_text: Vec<HandwritingInfo>,
        /// Document layout information.
        pub layout: Option<DocumentLayout>,
        /// Extracted metadata.
        pub metadata: LegalDocMetadata,
        /// Confidence score (0.0-1.0).
        pub confidence: f32,
    }

    /// Information about a detected signature.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SignatureInfo {
        /// Bounding box (x, y, width, height) in normalized coordinates.
        pub bbox: (f32, f32, f32, f32),
        /// Type of signature (handwritten, digital, stamp).
        pub signature_type: SignatureType,
        /// Extracted signer name (if available).
        pub signer_name: Option<String>,
        /// Date of signature (if available).
        pub date: Option<String>,
        /// Confidence score (0.0-1.0).
        pub confidence: f32,
    }

    /// Type of signature.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SignatureType {
        /// Handwritten signature.
        Handwritten,
        /// Digital signature.
        Digital,
        /// Signature stamp.
        Stamp,
        /// Unknown type.
        Unknown,
    }

    /// Information about a detected seal/stamp.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SealInfo {
        /// Bounding box (x, y, width, height) in normalized coordinates.
        pub bbox: (f32, f32, f32, f32),
        /// Type of seal (notary, corporate, government).
        pub seal_type: SealType,
        /// Extracted text from the seal.
        pub text: Option<String>,
        /// Confidence score (0.0-1.0).
        pub confidence: f32,
    }

    /// Type of seal/stamp.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SealType {
        /// Notary seal.
        Notary,
        /// Corporate seal.
        Corporate,
        /// Government seal.
        Government,
        /// Court seal.
        Court,
        /// Unknown type.
        Unknown,
    }

    /// Information about handwritten text.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct HandwritingInfo {
        /// Bounding box (x, y, width, height) in normalized coordinates.
        pub bbox: (f32, f32, f32, f32),
        /// Transcribed text.
        pub text: String,
        /// Confidence score (0.0-1.0).
        pub confidence: f32,
    }

    /// Document layout information.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DocumentLayout {
        /// Number of pages.
        pub page_count: usize,
        /// Sections detected in the document.
        pub sections: Vec<LayoutSection>,
        /// Headers detected.
        pub headers: Vec<String>,
        /// Footers detected.
        pub footers: Vec<String>,
        /// Table of contents (if detected).
        pub toc: Vec<TocEntry>,
    }

    /// A section in the document layout.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LayoutSection {
        /// Section title.
        pub title: String,
        /// Page number where section starts.
        pub page: usize,
        /// Section type (heading, paragraph, list, table).
        pub section_type: SectionType,
        /// Nested subsections.
        pub subsections: Vec<LayoutSection>,
    }

    /// Type of layout section.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum SectionType {
        /// Heading.
        Heading,
        /// Paragraph.
        Paragraph,
        /// List (bulleted or numbered).
        List,
        /// Table.
        Table,
        /// Block quote.
        BlockQuote,
        /// Code block.
        CodeBlock,
    }

    /// Table of contents entry.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TocEntry {
        /// Entry title.
        pub title: String,
        /// Page number.
        pub page: usize,
        /// Level (1 for top-level, 2 for sub-level, etc.).
        pub level: usize,
    }

    /// Metadata extracted from a legal document.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct LegalDocMetadata {
        /// Document title.
        pub title: Option<String>,
        /// Document date.
        pub date: Option<String>,
        /// Parties involved.
        pub parties: Vec<String>,
        /// Jurisdiction.
        pub jurisdiction: Option<String>,
        /// Court (if applicable).
        pub court: Option<String>,
        /// Case number (if applicable).
        pub case_number: Option<String>,
        /// Document type description.
        pub doc_type: Option<String>,
        /// Language.
        pub language: Option<String>,
    }

    impl LegalDocAnalysisResult {
        /// Creates a new empty analysis result.
        pub fn new(text: impl Into<String>) -> Self {
            Self {
                text: text.into(),
                signatures: Vec::new(),
                seals: Vec::new(),
                handwritten_text: Vec::new(),
                layout: None,
                metadata: LegalDocMetadata::default(),
                confidence: 0.0,
            }
        }

        /// Returns true if signatures were detected.
        pub fn has_signatures(&self) -> bool {
            !self.signatures.is_empty()
        }

        /// Returns true if seals were detected.
        pub fn has_seals(&self) -> bool {
            !self.seals.is_empty()
        }

        /// Returns true if handwritten text was detected.
        pub fn has_handwriting(&self) -> bool {
            !self.handwritten_text.is_empty()
        }

        /// Returns the number of pages in the document.
        pub fn page_count(&self) -> usize {
            self.layout.as_ref().map_or(1, |l| l.page_count)
        }
    }

    impl Default for LegalDocMetadata {
        fn default() -> Self {
            Self {
                title: None,
                date: None,
                parties: Vec::new(),
                jurisdiction: None,
                court: None,
                case_number: None,
                doc_type: None,
                language: None,
            }
        }
    }

    impl LegalDocMetadata {
        /// Creates a new empty metadata object.
        pub fn new() -> Self {
            Self::default()
        }

        /// Adds a party to the metadata.
        pub fn add_party(mut self, party: impl Into<String>) -> Self {
            self.parties.push(party.into());
            self
        }

        /// Sets the document title.
        pub fn with_title(mut self, title: impl Into<String>) -> Self {
            self.title = Some(title.into());
            self
        }

        /// Sets the document date.
        pub fn with_date(mut self, date: impl Into<String>) -> Self {
            self.date = Some(date.into());
            self
        }

        /// Sets the jurisdiction.
        pub fn with_jurisdiction(mut self, jurisdiction: impl Into<String>) -> Self {
            self.jurisdiction = Some(jurisdiction.into());
            self
        }
    }
}

/// PDF parsing utilities for legal documents.
pub mod pdf_parsing {
    use super::legal_document_analysis::*;
    use super::*;

    /// Configuration for PDF parsing.
    #[derive(Debug, Clone)]
    pub struct PdfParseConfig {
        /// Whether to extract text.
        pub extract_text: bool,
        /// Whether to extract images.
        pub extract_images: bool,
        /// Whether to preserve layout.
        pub preserve_layout: bool,
        /// Whether to extract tables.
        pub extract_tables: bool,
        /// Whether to perform OCR on images.
        pub ocr_images: bool,
        /// Analysis configuration for legal documents.
        pub legal_analysis: Option<LegalDocAnalysisConfig>,
    }

    impl Default for PdfParseConfig {
        fn default() -> Self {
            Self {
                extract_text: true,
                extract_images: false,
                preserve_layout: true,
                extract_tables: false,
                ocr_images: false,
                legal_analysis: None,
            }
        }
    }

    impl PdfParseConfig {
        /// Creates a new PDF parse config with default values.
        pub fn new() -> Self {
            Self::default()
        }

        /// Enables text extraction.
        pub fn with_text_extraction(mut self, enable: bool) -> Self {
            self.extract_text = enable;
            self
        }

        /// Enables image extraction.
        pub fn with_image_extraction(mut self, enable: bool) -> Self {
            self.extract_images = enable;
            self
        }

        /// Enables layout preservation.
        pub fn with_layout_preservation(mut self, enable: bool) -> Self {
            self.preserve_layout = enable;
            self
        }

        /// Enables table extraction.
        pub fn with_table_extraction(mut self, enable: bool) -> Self {
            self.extract_tables = enable;
            self
        }

        /// Enables OCR on images.
        pub fn with_ocr(mut self, enable: bool) -> Self {
            self.ocr_images = enable;
            self
        }

        /// Sets legal document analysis configuration.
        pub fn with_legal_analysis(mut self, config: LegalDocAnalysisConfig) -> Self {
            self.legal_analysis = Some(config);
            self
        }
    }

    /// Result of PDF parsing.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PdfParseResult {
        /// Extracted text content.
        pub text: String,
        /// Extracted images.
        pub images: Vec<ExtractedImage>,
        /// Extracted tables.
        pub tables: Vec<ExtractedTable>,
        /// Page information.
        pub pages: Vec<PageInfo>,
        /// Legal document analysis result (if requested).
        pub legal_analysis: Option<LegalDocAnalysisResult>,
    }

    /// Information about an extracted image from a PDF.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExtractedImage {
        /// Page number where the image was found.
        pub page: usize,
        /// Image data as base64.
        pub data: String,
        /// MIME type.
        pub mime_type: String,
        /// Bounding box (x, y, width, height) in normalized coordinates.
        pub bbox: (f32, f32, f32, f32),
        /// OCR text from the image (if OCR was performed).
        pub ocr_text: Option<String>,
    }

    /// Information about an extracted table from a PDF.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct ExtractedTable {
        /// Page number where the table was found.
        pub page: usize,
        /// Table caption (if available).
        pub caption: Option<String>,
        /// Table headers.
        pub headers: Vec<String>,
        /// Table rows.
        pub rows: Vec<Vec<String>>,
        /// Bounding box (x, y, width, height) in normalized coordinates.
        pub bbox: (f32, f32, f32, f32),
    }

    /// Information about a PDF page.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct PageInfo {
        /// Page number (1-indexed).
        pub page_number: usize,
        /// Page width in points.
        pub width: f32,
        /// Page height in points.
        pub height: f32,
        /// Text content on this page.
        pub text: String,
        /// Number of images on this page.
        pub image_count: usize,
        /// Number of tables on this page.
        pub table_count: usize,
    }

    impl PdfParseResult {
        /// Creates a new empty PDF parse result.
        pub fn new() -> Self {
            Self {
                text: String::new(),
                images: Vec::new(),
                tables: Vec::new(),
                pages: Vec::new(),
                legal_analysis: None,
            }
        }

        /// Returns the number of pages in the PDF.
        pub fn page_count(&self) -> usize {
            self.pages.len()
        }

        /// Returns true if the PDF contains images.
        pub fn has_images(&self) -> bool {
            !self.images.is_empty()
        }

        /// Returns true if the PDF contains tables.
        pub fn has_tables(&self) -> bool {
            !self.tables.is_empty()
        }

        /// Gets text from a specific page.
        pub fn page_text(&self, page: usize) -> Option<&str> {
            self.pages
                .iter()
                .find(|p| p.page_number == page)
                .map(|p| p.text.as_str())
        }
    }

    impl Default for PdfParseResult {
        fn default() -> Self {
            Self::new()
        }
    }
}

/// Audio transcription utilities for court recordings and legal audio.
pub mod audio_transcription {
    use super::*;

    /// Configuration for audio transcription.
    #[derive(Debug, Clone)]
    pub struct TranscriptionConfig {
        /// Language code (e.g., "en", "es", "fr").
        pub language: Option<String>,
        /// Whether to include timestamps.
        pub timestamps: bool,
        /// Whether to identify speakers (diarization).
        pub speaker_diarization: bool,
        /// Whether to add punctuation.
        pub add_punctuation: bool,
        /// Acoustic model to use (if applicable).
        pub acoustic_model: Option<String>,
        /// Whether to filter profanity.
        pub filter_profanity: bool,
    }

    impl Default for TranscriptionConfig {
        fn default() -> Self {
            Self {
                language: None,
                timestamps: true,
                speaker_diarization: false,
                add_punctuation: true,
                acoustic_model: None,
                filter_profanity: false,
            }
        }
    }

    impl TranscriptionConfig {
        /// Creates a new transcription config with default values.
        pub fn new() -> Self {
            Self::default()
        }

        /// Sets the language code.
        pub fn with_language(mut self, language: impl Into<String>) -> Self {
            self.language = Some(language.into());
            self
        }

        /// Enables or disables timestamps.
        pub fn with_timestamps(mut self, enable: bool) -> Self {
            self.timestamps = enable;
            self
        }

        /// Enables or disables speaker diarization.
        pub fn with_speaker_diarization(mut self, enable: bool) -> Self {
            self.speaker_diarization = enable;
            self
        }

        /// Enables or disables punctuation.
        pub fn with_punctuation(mut self, enable: bool) -> Self {
            self.add_punctuation = enable;
            self
        }
    }

    /// Result of audio transcription.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TranscriptionResult {
        /// Full transcript text.
        pub text: String,
        /// Segments with timestamps.
        pub segments: Vec<TranscriptSegment>,
        /// Identified speakers (if diarization was performed).
        pub speakers: Vec<SpeakerInfo>,
        /// Language detected (if not specified).
        pub detected_language: Option<String>,
        /// Confidence score (0.0-1.0).
        pub confidence: f32,
    }

    /// A segment of transcribed audio with timing information.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct TranscriptSegment {
        /// Start time in seconds.
        pub start: f32,
        /// End time in seconds.
        pub end: f32,
        /// Transcribed text for this segment.
        pub text: String,
        /// Speaker ID (if diarization was performed).
        pub speaker: Option<usize>,
        /// Confidence score (0.0-1.0).
        pub confidence: f32,
    }

    /// Information about an identified speaker.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SpeakerInfo {
        /// Speaker ID.
        pub id: usize,
        /// Speaker label (if known).
        pub label: Option<String>,
        /// Total speaking time in seconds.
        pub total_time: f32,
        /// Number of segments spoken.
        pub segment_count: usize,
    }

    impl TranscriptionResult {
        /// Creates a new transcription result.
        pub fn new(text: impl Into<String>) -> Self {
            Self {
                text: text.into(),
                segments: Vec::new(),
                speakers: Vec::new(),
                detected_language: None,
                confidence: 0.0,
            }
        }

        /// Returns the total duration of the transcription in seconds.
        pub fn duration(&self) -> f32 {
            self.segments.last().map_or(0.0, |s| s.end)
        }

        /// Returns the number of speakers identified.
        pub fn speaker_count(&self) -> usize {
            self.speakers.len()
        }

        /// Gets all text spoken by a specific speaker.
        pub fn text_by_speaker(&self, speaker_id: usize) -> String {
            self.segments
                .iter()
                .filter(|s| s.speaker == Some(speaker_id))
                .map(|s| s.text.as_str())
                .collect::<Vec<_>>()
                .join(" ")
        }
    }
}

/// Video analysis utilities for evidence review.
pub mod video_analysis {
    use super::*;

    /// Configuration for video analysis.
    #[derive(Debug, Clone)]
    pub struct VideoAnalysisConfig {
        /// Whether to extract frames at intervals.
        pub extract_frames: bool,
        /// Frame extraction interval in seconds (if extract_frames is true).
        pub frame_interval: f32,
        /// Whether to perform object detection.
        pub object_detection: bool,
        /// Whether to perform face detection.
        pub face_detection: bool,
        /// Whether to extract audio and transcribe.
        pub transcribe_audio: bool,
        /// Whether to perform scene detection.
        pub scene_detection: bool,
    }

    impl Default for VideoAnalysisConfig {
        fn default() -> Self {
            Self {
                extract_frames: true,
                frame_interval: 1.0,
                object_detection: false,
                face_detection: false,
                transcribe_audio: false,
                scene_detection: false,
            }
        }
    }

    impl VideoAnalysisConfig {
        /// Creates a new video analysis config with default values.
        pub fn new() -> Self {
            Self::default()
        }

        /// Sets frame extraction settings.
        pub fn with_frame_extraction(mut self, enable: bool, interval: f32) -> Self {
            self.extract_frames = enable;
            self.frame_interval = interval;
            self
        }

        /// Enables object detection.
        pub fn with_object_detection(mut self, enable: bool) -> Self {
            self.object_detection = enable;
            self
        }

        /// Enables face detection.
        pub fn with_face_detection(mut self, enable: bool) -> Self {
            self.face_detection = enable;
            self
        }

        /// Enables audio transcription.
        pub fn with_audio_transcription(mut self, enable: bool) -> Self {
            self.transcribe_audio = enable;
            self
        }
    }

    /// Result of video analysis.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VideoAnalysisResult {
        /// Video metadata.
        pub metadata: VideoMetadata,
        /// Extracted frames.
        pub frames: Vec<VideoFrame>,
        /// Detected objects (if object detection was performed).
        pub objects: Vec<DetectedObject>,
        /// Detected faces (if face detection was performed).
        pub faces: Vec<DetectedFace>,
        /// Audio transcription (if performed).
        pub transcription: Option<super::audio_transcription::TranscriptionResult>,
        /// Detected scenes.
        pub scenes: Vec<SceneInfo>,
    }

    /// Video metadata.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VideoMetadata {
        /// Duration in seconds.
        pub duration: f32,
        /// Frame rate (FPS).
        pub fps: f32,
        /// Width in pixels.
        pub width: usize,
        /// Height in pixels.
        pub height: usize,
        /// Video codec.
        pub codec: Option<String>,
        /// Audio codec.
        pub audio_codec: Option<String>,
    }

    /// A frame extracted from a video.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct VideoFrame {
        /// Timestamp in seconds.
        pub timestamp: f32,
        /// Frame number.
        pub frame_number: usize,
        /// Image data as base64.
        pub data: String,
        /// MIME type (usually "image/jpeg" or "image/png").
        pub mime_type: String,
        /// Optional description/caption.
        pub description: Option<String>,
    }

    /// Information about a detected object in a video.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DetectedObject {
        /// Object class/label.
        pub label: String,
        /// Confidence score (0.0-1.0).
        pub confidence: f32,
        /// Bounding box (x, y, width, height) in normalized coordinates.
        pub bbox: (f32, f32, f32, f32),
        /// Frame number where the object was detected.
        pub frame: usize,
        /// Timestamp in seconds.
        pub timestamp: f32,
    }

    /// Information about a detected face in a video.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct DetectedFace {
        /// Confidence score (0.0-1.0).
        pub confidence: f32,
        /// Bounding box (x, y, width, height) in normalized coordinates.
        pub bbox: (f32, f32, f32, f32),
        /// Frame number where the face was detected.
        pub frame: usize,
        /// Timestamp in seconds.
        pub timestamp: f32,
        /// Optional face ID (for tracking across frames).
        pub face_id: Option<usize>,
    }

    /// Information about a detected scene in a video.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct SceneInfo {
        /// Start time in seconds.
        pub start: f32,
        /// End time in seconds.
        pub end: f32,
        /// Scene description/summary.
        pub description: Option<String>,
        /// Key frame from this scene.
        pub key_frame: Option<usize>,
    }

    impl VideoAnalysisResult {
        /// Creates a new empty video analysis result.
        pub fn new(metadata: VideoMetadata) -> Self {
            Self {
                metadata,
                frames: Vec::new(),
                objects: Vec::new(),
                faces: Vec::new(),
                transcription: None,
                scenes: Vec::new(),
            }
        }

        /// Returns the video duration in seconds.
        pub fn duration(&self) -> f32 {
            self.metadata.duration
        }

        /// Returns the number of extracted frames.
        pub fn frame_count(&self) -> usize {
            self.frames.len()
        }

        /// Returns true if objects were detected.
        pub fn has_objects(&self) -> bool {
            !self.objects.is_empty()
        }

        /// Returns true if faces were detected.
        pub fn has_faces(&self) -> bool {
            !self.faces.is_empty()
        }
    }
}

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

    #[test]
    fn test_multimodal_response_text_only() {
        let response = MultiModalResponse::text_only("Hello, world!");
        assert_eq!(response.text_or_empty(), "Hello, world!");
        assert!(!response.has_images());
        assert!(!response.has_audio());
    }

    #[test]
    fn test_multimodal_response_with_image() {
        let image = GeneratedImage::from_url("image/png", "https://example.com/generated.png");
        let response = MultiModalResponse::new()
            .with_text("Generated image:")
            .add_image(image);

        assert!(response.has_text());
        assert!(response.has_images());
        assert_eq!(response.images.len(), 1);
    }

    #[test]
    fn test_multimodal_response_from_json() {
        let json = serde_json::json!({
            "text": "This is a response",
            "images": [
                {
                    "url": "https://example.com/image1.png",
                    "mime_type": "image/png",
                    "caption": "A generated image"
                }
            ],
            "audio": [
                {
                    "url": "https://example.com/audio1.mp3",
                    "mime_type": "audio/mpeg",
                    "transcript": "Hello world"
                }
            ]
        });

        let response = MultiModalResponse::from_json(&json).unwrap();
        assert_eq!(response.text_or_empty(), "This is a response");
        assert_eq!(response.images.len(), 1);
        assert_eq!(response.audio.len(), 1);
        assert_eq!(
            response.images[0].caption.as_deref(),
            Some("A generated image")
        );
        assert_eq!(response.audio[0].transcript.as_deref(), Some("Hello world"));
    }

    #[test]
    fn test_generated_image_from_data() {
        let image =
            GeneratedImage::from_data("image/jpeg", "base64data").with_caption("Test image");

        assert_eq!(image.data.as_deref(), Some("base64data"));
        assert_eq!(image.mime_type, "image/jpeg");
        assert_eq!(image.caption.as_deref(), Some("Test image"));
    }

    #[test]
    fn test_generated_audio_from_url() {
        let audio = GeneratedAudio::from_url("audio/wav", "https://example.com/audio.wav")
            .with_transcript("Spoken text");

        assert_eq!(audio.url.as_deref(), Some("https://example.com/audio.wav"));
        assert_eq!(audio.mime_type, "audio/wav");
        assert_eq!(audio.transcript.as_deref(), Some("Spoken text"));
    }
}
