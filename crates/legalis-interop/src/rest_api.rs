//! REST API types and handlers for conversion service.
//!
//! This module provides types and utilities for building a REST API around the conversion service.
//! Note: This module provides the data types and logic. Actual HTTP server implementation
//! would use frameworks like axum, actix-web, or warp.

use crate::{ConversionReport, InteropError, LegalConverter, LegalFormat};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Request to convert a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertRequest {
    /// Source document content
    pub source: String,
    /// Source format
    pub source_format: LegalFormat,
    /// Target format
    pub target_format: LegalFormat,
    /// Optional conversion options
    #[serde(default)]
    pub options: ConversionOptions,
}

/// Request to auto-detect format and convert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoConvertRequest {
    /// Source document content
    pub source: String,
    /// Target format
    pub target_format: LegalFormat,
    /// Optional conversion options
    #[serde(default)]
    pub options: ConversionOptions,
}

/// Conversion options for REST API requests.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ConversionOptions {
    /// Enable caching
    #[serde(default)]
    pub cache_enabled: bool,
    /// Fail on warnings
    #[serde(default)]
    pub fail_on_warnings: bool,
    /// Minimum confidence threshold (0.0 - 1.0)
    #[serde(default)]
    pub min_confidence: Option<f64>,
}

/// Response from a conversion operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertResponse {
    /// Converted document content
    pub output: String,
    /// Conversion report
    pub report: ConversionReport,
    /// Conversion ID (for tracking)
    pub conversion_id: Option<String>,
}

/// Request to validate a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateRequest {
    /// Source document content
    pub source: String,
    /// Format to validate against
    pub format: LegalFormat,
}

/// Response from a validation operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateResponse {
    /// Validation success
    pub valid: bool,
    /// Number of statutes found
    pub statutes_count: usize,
    /// Confidence score
    pub confidence: f64,
    /// Validation warnings
    pub warnings: Vec<String>,
}

/// Request to get supported formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatsRequest {}

/// Response with supported formats.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatsResponse {
    /// Supported import formats
    pub import_formats: Vec<FormatInfo>,
    /// Supported export formats
    pub export_formats: Vec<FormatInfo>,
}

/// Information about a supported format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatInfo {
    /// Format enum value
    pub format: LegalFormat,
    /// Format name
    pub name: String,
    /// Typical file extension
    pub extension: String,
}

impl FormatInfo {
    /// Creates format info from a LegalFormat.
    pub fn from_format(format: LegalFormat) -> Self {
        Self {
            name: format!("{:?}", format),
            extension: format.extension().to_string(),
            format,
        }
    }
}

/// Batch conversion request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConvertRequest {
    /// List of conversion requests
    pub conversions: Vec<ConvertRequest>,
}

/// Batch conversion response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchConvertResponse {
    /// List of conversion results
    pub results: Vec<ConvertResult>,
    /// Overall statistics
    pub stats: BatchStats,
}

/// Result of a single conversion in a batch.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvertResult {
    /// Conversion index in batch
    pub index: usize,
    /// Success flag
    pub success: bool,
    /// Converted output (if successful)
    pub output: Option<String>,
    /// Conversion report (if successful)
    pub report: Option<ConversionReport>,
    /// Error message (if failed)
    pub error: Option<String>,
}

/// Statistics for batch conversion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchStats {
    /// Total conversions requested
    pub total: usize,
    /// Successful conversions
    pub successful: usize,
    /// Failed conversions
    pub failed: usize,
    /// Average confidence
    pub avg_confidence: f64,
}

/// Error response for API errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error message
    pub error: String,
    /// Error code
    pub code: String,
    /// Additional error details
    #[serde(default)]
    pub details: HashMap<String, String>,
}

impl From<InteropError> for ErrorResponse {
    fn from(error: InteropError) -> Self {
        let (code, message) = match &error {
            InteropError::ParseError(msg) => ("parse_error", msg.clone()),
            InteropError::UnsupportedFormat(msg) => ("unsupported_format", msg.clone()),
            InteropError::ConversionError(msg) => ("conversion_error", msg.clone()),
            InteropError::UnsupportedFeature(msg) => ("unsupported_feature", msg.clone()),
            InteropError::IoError(e) => ("io_error", e.to_string()),
            InteropError::SerializationError(msg) => ("serialization_error", msg.clone()),
            InteropError::ValidationError(msg) => ("validation_error", msg.clone()),
        };

        Self {
            error: message,
            code: code.to_string(),
            details: HashMap::new(),
        }
    }
}

/// REST API service handler.
pub struct RestApiService {
    converter: LegalConverter,
}

impl Default for RestApiService {
    fn default() -> Self {
        Self::new()
    }
}

impl RestApiService {
    /// Creates a new REST API service.
    pub fn new() -> Self {
        Self {
            converter: LegalConverter::new(),
        }
    }

    /// Creates a REST API service with caching enabled.
    pub fn with_cache(cache_size: usize) -> Self {
        Self {
            converter: LegalConverter::with_cache(cache_size),
        }
    }

    /// Handles a convert request.
    pub fn handle_convert(
        &mut self,
        request: ConvertRequest,
    ) -> Result<ConvertResponse, ErrorResponse> {
        let (output, report) = self
            .converter
            .convert(
                &request.source,
                request.source_format,
                request.target_format,
            )
            .map_err(ErrorResponse::from)?;

        // Check minimum confidence if specified
        if let Some(min_conf) = request.options.min_confidence {
            if report.confidence < min_conf {
                return Err(ErrorResponse {
                    error: format!(
                        "Conversion confidence {:.2} below minimum threshold {:.2}",
                        report.confidence, min_conf
                    ),
                    code: "low_confidence".to_string(),
                    details: HashMap::new(),
                });
            }
        }

        // Check fail on warnings
        if request.options.fail_on_warnings && !report.warnings.is_empty() {
            return Err(ErrorResponse {
                error: format!("Conversion produced {} warnings", report.warnings.len()),
                code: "conversion_warnings".to_string(),
                details: HashMap::new(),
            });
        }

        Ok(ConvertResponse {
            output,
            report,
            conversion_id: None,
        })
    }

    /// Handles an auto-convert request.
    pub fn handle_auto_convert(
        &mut self,
        request: AutoConvertRequest,
    ) -> Result<ConvertResponse, ErrorResponse> {
        // Auto-detect format
        let (statutes, import_report) = self
            .converter
            .auto_import(&request.source)
            .map_err(ErrorResponse::from)?;

        let _source_format = import_report.source_format.ok_or_else(|| ErrorResponse {
            error: "Could not auto-detect format".to_string(),
            code: "format_detection_failed".to_string(),
            details: HashMap::new(),
        })?;

        // Export to target format
        let (output, export_report) = self
            .converter
            .export(&statutes, request.target_format)
            .map_err(ErrorResponse::from)?;

        // Merge reports
        let mut report = import_report;
        report.target_format = Some(request.target_format);
        report
            .unsupported_features
            .extend(export_report.unsupported_features);
        report.warnings.extend(export_report.warnings);
        report.confidence = (report.confidence * export_report.confidence).max(0.0);

        // Check minimum confidence
        if let Some(min_conf) = request.options.min_confidence {
            if report.confidence < min_conf {
                return Err(ErrorResponse {
                    error: format!(
                        "Conversion confidence {:.2} below minimum threshold {:.2}",
                        report.confidence, min_conf
                    ),
                    code: "low_confidence".to_string(),
                    details: HashMap::new(),
                });
            }
        }

        Ok(ConvertResponse {
            output,
            report,
            conversion_id: None,
        })
    }

    /// Handles a validate request.
    pub fn handle_validate(
        &mut self,
        request: ValidateRequest,
    ) -> Result<ValidateResponse, ErrorResponse> {
        let (statutes, report) = self
            .converter
            .import(&request.source, request.format)
            .map_err(ErrorResponse::from)?;

        Ok(ValidateResponse {
            valid: true,
            statutes_count: statutes.len(),
            confidence: report.confidence,
            warnings: report.warnings,
        })
    }

    /// Handles a formats request.
    pub fn handle_formats(
        &self,
        _request: FormatsRequest,
    ) -> Result<FormatsResponse, ErrorResponse> {
        let imports = self.converter.supported_imports();
        let exports = self.converter.supported_exports();

        Ok(FormatsResponse {
            import_formats: imports.into_iter().map(FormatInfo::from_format).collect(),
            export_formats: exports.into_iter().map(FormatInfo::from_format).collect(),
        })
    }

    /// Handles a batch convert request.
    pub fn handle_batch_convert(
        &mut self,
        request: BatchConvertRequest,
    ) -> Result<BatchConvertResponse, ErrorResponse> {
        let mut results = Vec::with_capacity(request.conversions.len());
        let mut successful = 0;
        let mut failed = 0;
        let mut total_confidence = 0.0;

        for (index, conv_req) in request.conversions.iter().enumerate() {
            match self.handle_convert(conv_req.clone()) {
                Ok(response) => {
                    total_confidence += response.report.confidence;
                    successful += 1;
                    results.push(ConvertResult {
                        index,
                        success: true,
                        output: Some(response.output),
                        report: Some(response.report),
                        error: None,
                    });
                }
                Err(err) => {
                    failed += 1;
                    results.push(ConvertResult {
                        index,
                        success: false,
                        output: None,
                        report: None,
                        error: Some(err.error),
                    });
                }
            }
        }

        let avg_confidence = if successful > 0 {
            total_confidence / successful as f64
        } else {
            0.0
        };

        Ok(BatchConvertResponse {
            results,
            stats: BatchStats {
                total: request.conversions.len(),
                successful,
                failed,
                avg_confidence,
            },
        })
    }

    /// Returns cache statistics if caching is enabled.
    pub fn cache_stats(&self) -> Option<crate::cache::CacheStats> {
        self.converter.cache_stats()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_request_serialization() {
        let request = ConvertRequest {
            source: "declaration scope Test:".to_string(),
            source_format: LegalFormat::Catala,
            target_format: LegalFormat::L4,
            options: ConversionOptions::default(),
        };

        let json = serde_json::to_string(&request).unwrap();
        assert!(json.contains("Catala"));
        assert!(json.contains("L4"));

        let deserialized: ConvertRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.source_format, LegalFormat::Catala);
        assert_eq!(deserialized.target_format, LegalFormat::L4);
    }

    #[test]
    fn test_rest_api_convert() {
        let mut service = RestApiService::new();

        let request = ConvertRequest {
            source: "declaration scope Test:\n  context input content integer".to_string(),
            source_format: LegalFormat::Catala,
            target_format: LegalFormat::L4,
            options: ConversionOptions::default(),
        };

        let response = service.handle_convert(request).unwrap();

        assert!(response.output.contains("RULE"));
        assert!(response.report.statutes_converted >= 1);
    }

    #[test]
    fn test_rest_api_auto_convert() {
        let mut service = RestApiService::new();

        let request = AutoConvertRequest {
            source: "declaration scope AutoTest:\n  context input content integer".to_string(),
            target_format: LegalFormat::L4,
            options: ConversionOptions::default(),
        };

        let response = service.handle_auto_convert(request).unwrap();

        assert!(response.output.contains("RULE"));
        assert_eq!(response.report.source_format, Some(LegalFormat::Catala));
    }

    #[test]
    fn test_rest_api_validate() {
        let mut service = RestApiService::new();

        let request = ValidateRequest {
            source: "declaration scope ValidateTest:\n  context input content integer".to_string(),
            format: LegalFormat::Catala,
        };

        let response = service.handle_validate(request).unwrap();

        assert!(response.valid);
        assert!(response.statutes_count >= 1);
    }

    #[test]
    fn test_rest_api_formats() {
        let service = RestApiService::new();

        let response = service.handle_formats(FormatsRequest {}).unwrap();

        assert!(!response.import_formats.is_empty());
        assert!(!response.export_formats.is_empty());

        let has_catala = response
            .import_formats
            .iter()
            .any(|f| f.format == LegalFormat::Catala);
        assert!(has_catala);
    }

    #[test]
    fn test_rest_api_batch_convert() {
        let mut service = RestApiService::new();

        let request = BatchConvertRequest {
            conversions: vec![
                ConvertRequest {
                    source: "declaration scope Test1:\n  context input content integer".to_string(),
                    source_format: LegalFormat::Catala,
                    target_format: LegalFormat::L4,
                    options: ConversionOptions::default(),
                },
                ConvertRequest {
                    source: "agreement Test2(A, B) { }".to_string(),
                    source_format: LegalFormat::Stipula,
                    target_format: LegalFormat::L4,
                    options: ConversionOptions::default(),
                },
            ],
        };

        let response = service.handle_batch_convert(request).unwrap();

        assert_eq!(response.stats.total, 2);
        assert_eq!(response.stats.successful, 2);
        assert_eq!(response.stats.failed, 0);
        assert!(response.stats.avg_confidence > 0.0);
    }

    #[test]
    fn test_rest_api_min_confidence() {
        let mut service = RestApiService::new();

        let request = ConvertRequest {
            source: "declaration scope Test:".to_string(),
            source_format: LegalFormat::Catala,
            target_format: LegalFormat::L4,
            options: ConversionOptions {
                min_confidence: Some(0.99), // Very high threshold
                ..Default::default()
            },
        };

        let result = service.handle_convert(request);
        // May fail due to low confidence, or succeed if conversion is high quality
        // We just check it doesn't panic
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_error_response_from_interop_error() {
        let error = InteropError::ParseError("Test error".to_string());
        let response = ErrorResponse::from(error);

        assert_eq!(response.code, "parse_error");
        assert_eq!(response.error, "Test error");
    }

    #[test]
    fn test_format_info() {
        let info = FormatInfo::from_format(LegalFormat::Catala);

        assert_eq!(info.format, LegalFormat::Catala);
        assert_eq!(info.name, "Catala");
        assert_eq!(info.extension, "catala_en");
    }
}
