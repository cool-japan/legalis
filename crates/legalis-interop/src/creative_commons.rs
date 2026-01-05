//! Creative Commons license format import/export.
//!
//! Supports parsing and generating Creative Commons licenses in both
//! human-readable and RDF/XML formats (CC REL).

use crate::{
    ConversionReport, FormatExporter, FormatImporter, InteropError, InteropResult, LegalFormat,
};
use legalis_core::{Effect, EffectType, Statute};
use serde::{Deserialize, Serialize};

/// Creative Commons license types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum CcLicenseType {
    /// CC0 - Public Domain
    CC0,
    /// CC BY - Attribution
    BY,
    /// CC BY-SA - Attribution-ShareAlike
    BYSA,
    /// CC BY-ND - Attribution-NoDerivs
    BYND,
    /// CC BY-NC - Attribution-NonCommercial
    BYNC,
    /// CC BY-NC-SA - Attribution-NonCommercial-ShareAlike
    BYNCSA,
    /// CC BY-NC-ND - Attribution-NonCommercial-NoDerivs
    BYNCND,
}

impl CcLicenseType {
    fn from_url(url: &str) -> Option<Self> {
        if url.contains("publicdomain/zero") {
            Some(CcLicenseType::CC0)
        } else if url.contains("by-nc-nd") {
            Some(CcLicenseType::BYNCND)
        } else if url.contains("by-nc-sa") {
            Some(CcLicenseType::BYNCSA)
        } else if url.contains("by-nc") {
            Some(CcLicenseType::BYNC)
        } else if url.contains("by-nd") {
            Some(CcLicenseType::BYND)
        } else if url.contains("by-sa") {
            Some(CcLicenseType::BYSA)
        } else if url.contains("by/") {
            Some(CcLicenseType::BY)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    fn to_url(self) -> &'static str {
        match self {
            CcLicenseType::CC0 => "https://creativecommons.org/publicdomain/zero/1.0/",
            CcLicenseType::BY => "https://creativecommons.org/licenses/by/4.0/",
            CcLicenseType::BYSA => "https://creativecommons.org/licenses/by-sa/4.0/",
            CcLicenseType::BYND => "https://creativecommons.org/licenses/by-nd/4.0/",
            CcLicenseType::BYNC => "https://creativecommons.org/licenses/by-nc/4.0/",
            CcLicenseType::BYNCSA => "https://creativecommons.org/licenses/by-nc-sa/4.0/",
            CcLicenseType::BYNCND => "https://creativecommons.org/licenses/by-nc-nd/4.0/",
        }
    }

    fn to_name(self) -> &'static str {
        match self {
            CcLicenseType::CC0 => "CC0 1.0 Universal",
            CcLicenseType::BY => "CC BY 4.0",
            CcLicenseType::BYSA => "CC BY-SA 4.0",
            CcLicenseType::BYND => "CC BY-ND 4.0",
            CcLicenseType::BYNC => "CC BY-NC 4.0",
            CcLicenseType::BYNCSA => "CC BY-NC-SA 4.0",
            CcLicenseType::BYNCND => "CC BY-NC-ND 4.0",
        }
    }
}

/// Creative Commons RDF document (simplified).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct CcRdfDocument {
    #[serde(rename = "RDF")]
    rdf: CcRdfContent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CcRdfContent {
    #[serde(rename = "License")]
    license: CcLicense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
struct CcLicense {
    #[serde(rename = "@about")]
    about: String,
    #[serde(rename = "permits", default)]
    permits: Vec<String>,
    #[serde(rename = "requires", default)]
    requires: Vec<String>,
    #[serde(rename = "prohibits", default)]
    prohibits: Vec<String>,
}

/// Importer for Creative Commons licenses.
pub struct CreativeCommonsImporter;

impl CreativeCommonsImporter {
    /// Creates a new Creative Commons importer.
    pub fn new() -> Self {
        Self
    }

    fn parse_license(&self, license: &CcLicense) -> InteropResult<Vec<Statute>> {
        let mut statutes = Vec::new();

        // Parse license type from URL
        let license_type = CcLicenseType::from_url(&license.about)
            .ok_or_else(|| InteropError::ParseError("Unknown CC license type".to_string()))?;

        // Create statutes for each permission
        for (i, permit) in license.permits.iter().enumerate() {
            let id = format!("cc_permit_{}", i);
            let title = format!("{} - Permits: {}", license_type.to_name(), permit);
            let effect = Effect::new(EffectType::Grant, permit);
            statutes.push(Statute::new(&id, &title, effect));
        }

        // Create statutes for requirements
        for (i, require) in license.requires.iter().enumerate() {
            let id = format!("cc_require_{}", i);
            let title = format!("{} - Requires: {}", license_type.to_name(), require);
            let effect = Effect::new(EffectType::Obligation, require);
            statutes.push(Statute::new(&id, &title, effect));
        }

        // Create statutes for prohibitions
        for (i, prohibit) in license.prohibits.iter().enumerate() {
            let id = format!("cc_prohibit_{}", i);
            let title = format!("{} - Prohibits: {}", license_type.to_name(), prohibit);
            let effect = Effect::new(EffectType::Prohibition, prohibit);
            statutes.push(Statute::new(&id, &title, effect));
        }

        Ok(statutes)
    }
}

impl Default for CreativeCommonsImporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatImporter for CreativeCommonsImporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::CreativeCommons
    }

    fn import(&self, source: &str) -> InteropResult<(Vec<Statute>, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::CreativeCommons, LegalFormat::Legalis);

        // Try to parse as RDF/XML first
        if source.contains("rdf:RDF") || source.contains("cc:License") {
            let doc: CcRdfDocument = quick_xml::de::from_str(source)
                .map_err(|e| InteropError::ParseError(format!("Failed to parse CC RDF: {}", e)))?;

            let statutes = self.parse_license(&doc.rdf.license)?;
            report.statutes_converted = statutes.len();
            return Ok((statutes, report));
        }

        // Fallback: Try to detect license from URL
        let mut statutes = Vec::new();
        if let Some(license_type) = CcLicenseType::from_url(source) {
            let effect = Effect::new(EffectType::Grant, "use under CC license");
            let statute = Statute::new("cc_license", license_type.to_name(), effect);
            statutes.push(statute);
            report.statutes_converted = 1;
            return Ok((statutes, report));
        }

        Err(InteropError::ParseError(
            "Could not parse Creative Commons license".to_string(),
        ))
    }

    fn validate(&self, source: &str) -> bool {
        source.contains("creativecommons.org")
            || source.contains("cc:License")
            || source.contains("rdf:RDF")
            || (source.contains("<License>") && source.contains("<RDF>"))
    }
}

/// Exporter for Creative Commons licenses.
pub struct CreativeCommonsExporter;

impl CreativeCommonsExporter {
    /// Creates a new Creative Commons exporter.
    pub fn new() -> Self {
        Self
    }
}

impl Default for CreativeCommonsExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl FormatExporter for CreativeCommonsExporter {
    fn format(&self) -> LegalFormat {
        LegalFormat::CreativeCommons
    }

    fn export(&self, statutes: &[Statute]) -> InteropResult<(String, ConversionReport)> {
        let mut report = ConversionReport::new(LegalFormat::Legalis, LegalFormat::CreativeCommons);

        let mut permits = Vec::new();
        let mut requires = Vec::new();
        let mut prohibits = Vec::new();

        for statute in statutes {
            match statute.effect.effect_type {
                EffectType::Grant => permits.push(statute.effect.description.clone()),
                EffectType::Obligation => requires.push(statute.effect.description.clone()),
                EffectType::Prohibition => prohibits.push(statute.effect.description.clone()),
                _ => {}
            }
        }

        // Default to CC BY 4.0
        let license_url = "https://creativecommons.org/licenses/by/4.0/";

        let doc = CcRdfDocument {
            rdf: CcRdfContent {
                license: CcLicense {
                    about: license_url.to_string(),
                    permits,
                    requires,
                    prohibits,
                },
            },
        };

        let output = quick_xml::se::to_string(&doc).map_err(|e| {
            InteropError::SerializationError(format!("Failed to serialize CC RDF: {}", e))
        })?;

        report.statutes_converted = statutes.len();
        report.add_warning(
            "Creative Commons licenses are simplified, complex legal rules may be lost",
        );

        Ok((output, report))
    }

    fn can_represent(&self, _statute: &Statute) -> Vec<String> {
        vec![
            "Only basic permissions, requirements, and prohibitions".to_string(),
            "Complex preconditions not supported".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cc_export() {
        let exporter = CreativeCommonsExporter::new();
        let statute = Statute::new(
            "cc-permit",
            "Permit Reproduction",
            Effect::new(EffectType::Grant, "Reproduction"),
        );

        let (output, report) = exporter.export(&[statute]).unwrap();

        assert!(output.contains("rdf:RDF") || output.contains("License"));
        assert!(output.contains("Reproduction"));
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_cc_import_url() {
        let importer = CreativeCommonsImporter::new();
        let source = "https://creativecommons.org/licenses/by/4.0/";

        let (statutes, report) = importer.import(source).unwrap();

        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].title, "CC BY 4.0");
        assert_eq!(report.statutes_converted, 1);
    }

    #[test]
    fn test_cc_validate() {
        let importer = CreativeCommonsImporter::new();
        assert!(importer.validate("https://creativecommons.org/licenses/by/4.0/"));
        assert!(importer.validate("<cc:License></cc:License>"));
        assert!(!importer.validate("not creative commons"));
    }

    #[test]
    fn test_cc_license_type_detection() {
        assert_eq!(
            CcLicenseType::from_url("https://creativecommons.org/licenses/by/4.0/"),
            Some(CcLicenseType::BY)
        );
        assert_eq!(
            CcLicenseType::from_url("https://creativecommons.org/licenses/by-sa/4.0/"),
            Some(CcLicenseType::BYSA)
        );
        assert_eq!(
            CcLicenseType::from_url("https://creativecommons.org/publicdomain/zero/1.0/"),
            Some(CcLicenseType::CC0)
        );
    }
}
