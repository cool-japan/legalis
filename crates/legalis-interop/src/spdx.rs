//! SPDX (Software Package Data Exchange) license expression format import/export.
//!
//! SPDX is an open standard (ISO/IEC 5962:2021) for communicating software
//! bill of material information, including components, licenses, copyrights,
//! and security references.

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use std::fmt;

/// SPDX license identifier.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct SpdxLicense {
    /// License identifier (e.g., "MIT", "Apache-2.0")
    id: String,
    /// License exception (e.g., "Classpath-exception-2.0")
    exception: Option<String>,
}

impl SpdxLicense {
    fn parse(expr: &str) -> Result<Self, String> {
        let expr = expr.trim();

        if let Some((id, exception)) = expr.split_once(" WITH ") {
            Ok(SpdxLicense {
                id: id.trim().to_string(),
                exception: Some(exception.trim().to_string()),
            })
        } else {
            Ok(SpdxLicense {
                id: expr.to_string(),
                exception: None,
            })
        }
    }
}

impl fmt::Display for SpdxLicense {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(exception) = &self.exception {
            write!(f, "{} WITH {}", self.id, exception)
        } else {
            write!(f, "{}", self.id)
        }
    }
}

/// SPDX license expression (supporting AND, OR, WITH).
#[derive(Debug, Clone)]
pub enum SpdxExpression {
    /// Single license
    License(SpdxLicense),
    /// AND combination
    And(Box<SpdxExpression>, Box<SpdxExpression>),
    /// OR combination
    Or(Box<SpdxExpression>, Box<SpdxExpression>),
}

impl SpdxExpression {
    /// Parses an SPDX license expression.
    fn parse(expr: &str) -> Result<Self, String> {
        let expr = expr.trim();

        // Simple parsing: handle AND and OR operators
        if let Some(or_pos) = expr.find(" OR ") {
            let left = Self::parse(&expr[..or_pos])?;
            let right = Self::parse(&expr[or_pos + 4..])?;
            return Ok(SpdxExpression::Or(Box::new(left), Box::new(right)));
        }

        if let Some(and_pos) = expr.find(" AND ") {
            let left = Self::parse(&expr[..and_pos])?;
            let right = Self::parse(&expr[and_pos + 5..])?;
            return Ok(SpdxExpression::And(Box::new(left), Box::new(right)));
        }

        // Handle parentheses
        if expr.starts_with('(') && expr.ends_with(')') {
            return Self::parse(&expr[1..expr.len() - 1]);
        }

        // Single license
        let license = SpdxLicense::parse(expr)?;
        Ok(SpdxExpression::License(license))
    }

    /// Flattens expression to list of licenses.
    fn flatten(&self) -> Vec<SpdxLicense> {
        match self {
            SpdxExpression::License(lic) => vec![lic.clone()],
            SpdxExpression::And(left, right) | SpdxExpression::Or(left, right) => {
                let mut result = left.flatten();
                result.extend(right.flatten());
                result
            }
        }
    }
}

impl fmt::Display for SpdxExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpdxExpression::License(lic) => write!(f, "{}", lic),
            SpdxExpression::And(left, right) => {
                write!(f, "({} AND {})", left, right)
            }
            SpdxExpression::Or(left, right) => {
                write!(f, "({} OR {})", left, right)
            }
        }
    }
}

/// Importer for SPDX license expressions.
pub struct SpdxImporter;

impl SpdxImporter {
    /// Creates a new SPDX importer.
    pub fn new() -> Self {
        Self
    }
}

impl Default for SpdxImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for SpdxImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Spdx
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Spdx, LegalFormat::Legalis);

        // Parse SPDX expression
        let expr = SpdxExpression::parse(source).map_err(|e| {
            InteropError::ParseError(format!("Failed to parse SPDX expression: {}", e))
        })?;

        // Convert to statutes
        let licenses = expr.flatten();
        let mut statutes = Vec::new();

        for license in licenses {
            let id = license.id.to_lowercase().replace(['-', '.'], "_");
            let title = format!("License: {}", license.id);
            let mut effect = Effect::new(EffectType::Grant, "use under license");

            // Store SPDX ID in effect parameters
            effect
                .parameters
                .insert("spdx_id".to_string(), license.id.clone());

            if let Some(exception) = &license.exception {
                effect
                    .parameters
                    .insert("exception".to_string(), exception.clone());
            }

            let statute = Statute::new(&id, &title, effect);
            statutes.push(statute);
        }

        report.statutes_converted = statutes.len();
        Ok((statutes, report))
    }

    fn validate(&self, source: &str) -> bool {
        // Check for common SPDX license identifiers
        let common_licenses = [
            "MIT",
            "Apache-2.0",
            "GPL-3.0",
            "BSD-3-Clause",
            "ISC",
            "LGPL-3.0",
            "GPL-2.0",
            "BSD-2-Clause",
            "MPL-2.0",
            "AGPL-3.0",
            "Unlicense",
            "CC0-1.0",
            "Apache-1.1",
            "LGPL-2.1",
        ];

        let has_license = common_licenses.iter().any(|lic| source.contains(lic));
        let has_operator =
            source.contains(" AND ") || source.contains(" OR ") || source.contains(" WITH ");

        has_license || has_operator
    }
}

/// Exporter for SPDX license expressions.
pub struct SpdxExporter;

impl SpdxExporter {
    /// Creates a new SPDX exporter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for SpdxExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for SpdxExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::Spdx
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::Spdx);

        if statutes.is_empty() {
            return Ok(("NOASSERTION".to_string(), report));
        }

        let mut license_ids = Vec::new();

        for statute in statutes {
            if let Some(spdx_id) = statute.effect.parameters.get("spdx_id") {
                let mut expr = spdx_id.clone();
                if let Some(exception) = statute.effect.parameters.get("exception") {
                    expr = format!("{} WITH {}", expr, exception);
                }
                license_ids.push(expr);
            } else {
                // Try to infer from title
                if statute.title.contains("License:") {
                    let id = statute.title.replace("License:", "").trim().to_string();
                    license_ids.push(id);
                }
            }
        }

        if license_ids.is_empty() {
            report.add_warning("No SPDX license identifiers found, using NOASSERTION");
            return Ok(("NOASSERTION".to_string(), report));
        }

        // Combine with AND operator
        let output = if license_ids.len() == 1 {
            license_ids[0].clone()
        } else {
            license_ids.join(" AND ")
        };

        report.statutes_converted = statutes.len();
        report.add_warning(
            "SPDX expressions are license identifiers only, semantic details are lost",
        );

        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![
            "Only license identifiers supported".to_string(),
            "Preconditions and complex rules not supported".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spdx_parse_simple() {
        let expr = SpdxExpression::parse("MIT").unwrap();
        assert_eq!(expr.to_string(), "MIT");
    }

    #[test]
    fn test_spdx_parse_and() {
        let expr = SpdxExpression::parse("MIT AND Apache-2.0").unwrap();
        let licenses = expr.flatten();
        assert_eq!(licenses.len(), 2);
    }

    #[test]
    fn test_spdx_parse_or() {
        let expr = SpdxExpression::parse("GPL-2.0 OR GPL-3.0").unwrap();
        let licenses = expr.flatten();
        assert_eq!(licenses.len(), 2);
    }

    #[test]
    fn test_spdx_parse_with() {
        let license = SpdxLicense::parse("GPL-2.0 WITH Classpath-exception-2.0").unwrap();
        assert_eq!(license.id, "GPL-2.0");
        assert_eq!(
            license.exception,
            Some("Classpath-exception-2.0".to_string())
        );
    }

    #[test]
    fn test_spdx_import() {
        let importer = SpdxImporter::new();
        let (statutes, report) = importer.import("MIT AND Apache-2.0").unwrap();

        assert_eq!(statutes.len(), 2);
        assert_eq!(report.statutes_converted, 2);
    }

    #[test]
    fn test_spdx_export() {
        let exporter = SpdxExporter::new();
        let mut effect = Effect::new(EffectType::Grant, "use");
        effect
            .parameters
            .insert("spdx_id".to_string(), "MIT".to_string());
        let statute = Statute::new("mit_license", "License: MIT", effect);

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert_eq!(output, "MIT");
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_spdx_validate() {
        let importer = SpdxImporter::new();
        assert!(importer.validate("MIT"));
        assert!(importer.validate("Apache-2.0"));
        assert!(importer.validate("GPL-3.0 OR MIT"));
        assert!(!importer.validate("not spdx"));
    }

    #[test]
    fn test_spdx_roundtrip() {
        let exporter = SpdxExporter::new();
        let importer = SpdxImporter::new();

        let mut effect = Effect::new(EffectType::Grant, "use");
        effect
            .parameters
            .insert("spdx_id".to_string(), "Apache-2.0".to_string());
        let original = Statute::new("apache", "License: Apache-2.0", effect);

        let (exported, _) = exporter.export(&[original.clone()]).unwrap();
        let (imported, _) = importer.import(&exported).unwrap();

        assert_eq!(imported.len(), 1);
        assert_eq!(
            imported[0].effect.parameters.get("spdx_id"),
            Some(&"Apache-2.0".to_string())
        );
    }
}
