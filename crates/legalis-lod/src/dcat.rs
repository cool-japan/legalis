//! DCAT (Data Catalog Vocabulary) support for dataset publishing.
//!
//! This module provides utilities to generate DCAT catalogs and dataset descriptions
//! for publishing legal statute data as Linked Open Data.

use crate::{RdfValue, Triple};
use chrono::{DateTime, Utc};

/// DCAT dataset catalog.
#[derive(Debug, Clone)]
pub struct Catalog {
    /// Catalog URI
    pub uri: String,
    /// Catalog title
    pub title: String,
    /// Catalog description
    pub description: Option<String>,
    /// Publisher
    pub publisher: Option<String>,
    /// Homepage
    pub homepage: Option<String>,
    /// Language
    pub language: Option<String>,
    /// License
    pub license: Option<String>,
    /// Issued date
    pub issued: Option<DateTime<Utc>>,
    /// Modified date
    pub modified: Option<DateTime<Utc>>,
    /// Datasets in this catalog
    pub datasets: Vec<Dataset>,
}

/// DCAT dataset.
#[derive(Debug, Clone)]
pub struct Dataset {
    /// Dataset URI
    pub uri: String,
    /// Title
    pub title: String,
    /// Description
    pub description: Option<String>,
    /// Keywords
    pub keywords: Vec<String>,
    /// Themes/categories
    pub themes: Vec<String>,
    /// Publisher
    pub publisher: Option<String>,
    /// Contact point
    pub contact_point: Option<ContactPoint>,
    /// Distributions (different serializations)
    pub distributions: Vec<Distribution>,
    /// Issued date
    pub issued: Option<DateTime<Utc>>,
    /// Modified date
    pub modified: Option<DateTime<Utc>>,
    /// Temporal coverage
    pub temporal_coverage: Option<TemporalCoverage>,
    /// Spatial coverage (jurisdiction)
    pub spatial_coverage: Option<String>,
    /// Landing page
    pub landing_page: Option<String>,
    /// Access rights
    pub access_rights: Option<String>,
}

/// Contact point for a dataset.
#[derive(Debug, Clone)]
pub struct ContactPoint {
    /// Name
    pub name: String,
    /// Email
    pub email: Option<String>,
}

/// Distribution (serialization format) of a dataset.
#[derive(Debug, Clone)]
pub struct Distribution {
    /// Distribution URI
    pub uri: String,
    /// Title
    pub title: Option<String>,
    /// Format (MIME type)
    pub format: String,
    /// Access URL
    pub access_url: String,
    /// Download URL
    pub download_url: Option<String>,
    /// Byte size
    pub byte_size: Option<u64>,
    /// Issued date
    pub issued: Option<DateTime<Utc>>,
    /// Modified date
    pub modified: Option<DateTime<Utc>>,
}

/// Temporal coverage period.
#[derive(Debug, Clone)]
pub struct TemporalCoverage {
    /// Start date
    pub start: Option<DateTime<Utc>>,
    /// End date
    pub end: Option<DateTime<Utc>>,
}

impl Catalog {
    /// Creates a new catalog.
    pub fn new(uri: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            title: title.into(),
            description: None,
            publisher: None,
            homepage: None,
            language: None,
            license: None,
            issued: None,
            modified: None,
            datasets: Vec::new(),
        }
    }

    /// Adds a dataset to the catalog.
    pub fn add_dataset(&mut self, dataset: Dataset) {
        self.datasets.push(dataset);
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets the publisher.
    pub fn with_publisher(mut self, publisher: impl Into<String>) -> Self {
        self.publisher = Some(publisher.into());
        self
    }

    /// Sets the homepage.
    pub fn with_homepage(mut self, homepage: impl Into<String>) -> Self {
        self.homepage = Some(homepage.into());
        self
    }

    /// Sets the language.
    pub fn with_language(mut self, language: impl Into<String>) -> Self {
        self.language = Some(language.into());
        self
    }

    /// Sets the license.
    pub fn with_license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Converts the catalog to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("dcat:Catalog".to_string()),
        });

        // Title
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "dcterms:title".to_string(),
            object: RdfValue::string(&self.title),
        });

        // Description
        if let Some(ref desc) = self.description {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:description".to_string(),
                object: RdfValue::string(desc),
            });
        }

        // Publisher
        if let Some(ref publisher) = self.publisher {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:publisher".to_string(),
                object: RdfValue::Uri(publisher.clone()),
            });
        }

        // Homepage
        if let Some(ref homepage) = self.homepage {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "foaf:homepage".to_string(),
                object: RdfValue::Uri(homepage.clone()),
            });
        }

        // Language
        if let Some(ref lang) = self.language {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:language".to_string(),
                object: RdfValue::string(lang),
            });
        }

        // License
        if let Some(ref license) = self.license {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:license".to_string(),
                object: RdfValue::Uri(license.clone()),
            });
        }

        // Dates
        if let Some(issued) = self.issued {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:issued".to_string(),
                object: RdfValue::datetime(issued),
            });
        }

        if let Some(modified) = self.modified {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:modified".to_string(),
                object: RdfValue::datetime(modified),
            });
        }

        // Datasets
        for dataset in &self.datasets {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcat:dataset".to_string(),
                object: RdfValue::Uri(dataset.uri.clone()),
            });
            triples.extend(dataset.to_triples());
        }

        triples
    }
}

impl Dataset {
    /// Creates a new dataset.
    pub fn new(uri: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            uri: uri.into(),
            title: title.into(),
            description: None,
            keywords: Vec::new(),
            themes: Vec::new(),
            publisher: None,
            contact_point: None,
            distributions: Vec::new(),
            issued: None,
            modified: None,
            temporal_coverage: None,
            spatial_coverage: None,
            landing_page: None,
            access_rights: None,
        }
    }

    /// Sets the description.
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Adds a keyword.
    pub fn with_keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keywords.push(keyword.into());
        self
    }

    /// Adds a theme.
    pub fn with_theme(mut self, theme: impl Into<String>) -> Self {
        self.themes.push(theme.into());
        self
    }

    /// Sets the publisher.
    pub fn with_publisher(mut self, publisher: impl Into<String>) -> Self {
        self.publisher = Some(publisher.into());
        self
    }

    /// Adds a distribution.
    pub fn with_distribution(mut self, distribution: Distribution) -> Self {
        self.distributions.push(distribution);
        self
    }

    /// Sets the spatial coverage.
    pub fn with_spatial_coverage(mut self, coverage: impl Into<String>) -> Self {
        self.spatial_coverage = Some(coverage.into());
        self
    }

    /// Converts the dataset to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("dcat:Dataset".to_string()),
        });

        // Title
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "dcterms:title".to_string(),
            object: RdfValue::string(&self.title),
        });

        // Description
        if let Some(ref desc) = self.description {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:description".to_string(),
                object: RdfValue::string(desc),
            });
        }

        // Keywords
        for keyword in &self.keywords {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcat:keyword".to_string(),
                object: RdfValue::string(keyword),
            });
        }

        // Themes
        for theme in &self.themes {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcat:theme".to_string(),
                object: RdfValue::Uri(theme.clone()),
            });
        }

        // Publisher
        if let Some(ref publisher) = self.publisher {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:publisher".to_string(),
                object: RdfValue::Uri(publisher.clone()),
            });
        }

        // Spatial coverage
        if let Some(ref spatial) = self.spatial_coverage {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:spatial".to_string(),
                object: RdfValue::string(spatial),
            });
        }

        // Landing page
        if let Some(ref landing) = self.landing_page {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcat:landingPage".to_string(),
                object: RdfValue::Uri(landing.clone()),
            });
        }

        // Access rights
        if let Some(ref rights) = self.access_rights {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:accessRights".to_string(),
                object: RdfValue::string(rights),
            });
        }

        // Dates
        if let Some(issued) = self.issued {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:issued".to_string(),
                object: RdfValue::datetime(issued),
            });
        }

        if let Some(modified) = self.modified {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcterms:modified".to_string(),
                object: RdfValue::datetime(modified),
            });
        }

        // Distributions
        for dist in &self.distributions {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcat:distribution".to_string(),
                object: RdfValue::Uri(dist.uri.clone()),
            });
            triples.extend(dist.to_triples());
        }

        triples
    }
}

impl Distribution {
    /// Creates a new distribution.
    pub fn new(
        uri: impl Into<String>,
        format: impl Into<String>,
        access_url: impl Into<String>,
    ) -> Self {
        Self {
            uri: uri.into(),
            title: None,
            format: format.into(),
            access_url: access_url.into(),
            download_url: None,
            byte_size: None,
            issued: None,
            modified: None,
        }
    }

    /// Sets the download URL.
    pub fn with_download_url(mut self, url: impl Into<String>) -> Self {
        self.download_url = Some(url.into());
        self
    }

    /// Sets the byte size.
    pub fn with_byte_size(mut self, size: u64) -> Self {
        self.byte_size = Some(size);
        self
    }

    /// Converts the distribution to RDF triples.
    pub fn to_triples(&self) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Type
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "rdf:type".to_string(),
            object: RdfValue::Uri("dcat:Distribution".to_string()),
        });

        // Format
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "dcat:mediaType".to_string(),
            object: RdfValue::string(&self.format),
        });

        // Access URL
        triples.push(Triple {
            subject: self.uri.clone(),
            predicate: "dcat:accessURL".to_string(),
            object: RdfValue::Uri(self.access_url.clone()),
        });

        // Download URL
        if let Some(ref download) = self.download_url {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcat:downloadURL".to_string(),
                object: RdfValue::Uri(download.clone()),
            });
        }

        // Byte size
        if let Some(size) = self.byte_size {
            triples.push(Triple {
                subject: self.uri.clone(),
                predicate: "dcat:byteSize".to_string(),
                object: RdfValue::integer(size as i64),
            });
        }

        triples
    }
}

/// Publishing workflow for datasets.
#[derive(Debug)]
pub struct PublishingWorkflow {
    /// Catalog to publish to
    catalog: Catalog,
    /// Validation rules
    validation_rules: Vec<ValidationRule>,
}

/// Validation rule for publishing.
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// Rule name
    pub name: String,
    /// Rule description
    pub description: String,
    /// Validation function
    validator: fn(&Dataset) -> bool,
}

impl PublishingWorkflow {
    /// Creates a new publishing workflow.
    pub fn new(catalog: Catalog) -> Self {
        let mut workflow = Self {
            catalog,
            validation_rules: Vec::new(),
        };

        // Add default validation rules
        workflow.add_default_rules();
        workflow
    }

    /// Adds default validation rules.
    fn add_default_rules(&mut self) {
        // Rule: Dataset must have title
        self.add_rule(ValidationRule {
            name: "has_title".to_string(),
            description: "Dataset must have a title".to_string(),
            validator: |dataset| !dataset.title.is_empty(),
        });

        // Rule: Dataset must have at least one distribution
        self.add_rule(ValidationRule {
            name: "has_distribution".to_string(),
            description: "Dataset must have at least one distribution".to_string(),
            validator: |dataset| !dataset.distributions.is_empty(),
        });

        // Rule: Dataset must have a URI
        self.add_rule(ValidationRule {
            name: "has_uri".to_string(),
            description: "Dataset must have a valid URI".to_string(),
            validator: |dataset| !dataset.uri.is_empty(),
        });
    }

    /// Adds a custom validation rule.
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.validation_rules.push(rule);
    }

    /// Validates a dataset against all rules.
    pub fn validate(&self, dataset: &Dataset) -> PublishingResult {
        let mut errors = Vec::new();
        let mut warnings = Vec::new();

        for rule in &self.validation_rules {
            if !(rule.validator)(dataset) {
                errors.push(format!("{}: {}", rule.name, rule.description));
            }
        }

        // Additional quality checks (warnings)
        if dataset.description.is_none() {
            warnings.push("Dataset description is recommended".to_string());
        }

        if dataset.keywords.is_empty() {
            warnings.push("Dataset keywords are recommended for discoverability".to_string());
        }

        if dataset.publisher.is_none() {
            warnings.push("Dataset publisher information is recommended".to_string());
        }

        PublishingResult {
            valid: errors.is_empty(),
            errors,
            warnings,
        }
    }

    /// Publishes a dataset to the catalog if it passes validation.
    pub fn publish(&mut self, dataset: Dataset) -> Result<(), PublishingError> {
        let result = self.validate(&dataset);

        if !result.valid {
            return Err(PublishingError::ValidationFailed(result.errors));
        }

        self.catalog.add_dataset(dataset);
        Ok(())
    }

    /// Returns the catalog.
    pub fn catalog(&self) -> &Catalog {
        &self.catalog
    }

    /// Exports the catalog to RDF triples.
    pub fn export_catalog(&self) -> Vec<Triple> {
        self.catalog.to_triples()
    }
}

/// Result of dataset validation.
#[derive(Debug, Clone)]
pub struct PublishingResult {
    /// Whether validation passed
    pub valid: bool,
    /// Validation errors (must be fixed)
    pub errors: Vec<String>,
    /// Warnings (should be addressed)
    pub warnings: Vec<String>,
}

/// Publishing error.
#[derive(Debug, Clone)]
pub enum PublishingError {
    /// Validation failed
    ValidationFailed(Vec<String>),
    /// Publishing failed
    PublishingFailed(String),
}

impl std::fmt::Display for PublishingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PublishingError::ValidationFailed(errors) => {
                write!(f, "Validation failed: {}", errors.join(", "))
            }
            PublishingError::PublishingFailed(msg) => write!(f, "Publishing failed: {}", msg),
        }
    }
}

impl std::error::Error for PublishingError {}

/// Data.gov integration for publishing datasets.
pub mod datagov {
    use super::*;

    /// Data.gov specific metadata requirements.
    #[derive(Debug, Clone)]
    pub struct DataGovMetadata {
        /// Bureau code (required by Data.gov)
        pub bureau_code: Vec<String>,
        /// Program code (required by Data.gov)
        pub program_code: Vec<String>,
        /// Access level (public, restricted public, non-public)
        pub access_level: AccessLevel,
        /// Data quality indicator
        pub data_quality: bool,
        /// Primary IT investment UII
        pub primary_it_investment_uii: Option<String>,
        /// Is this the primary data asset for the agency?
        pub is_primary_source: Option<bool>,
    }

    /// Access level for Data.gov datasets.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AccessLevel {
        /// Publicly accessible
        Public,
        /// Restricted public access
        RestrictedPublic,
        /// Non-public
        NonPublic,
    }

    impl AccessLevel {
        /// Returns the string representation for Data.gov.
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Public => "public",
                Self::RestrictedPublic => "restricted public",
                Self::NonPublic => "non-public",
            }
        }
    }

    impl DataGovMetadata {
        /// Creates new Data.gov metadata with required fields.
        pub fn new(bureau_code: Vec<String>, program_code: Vec<String>) -> Self {
            Self {
                bureau_code,
                program_code,
                access_level: AccessLevel::Public,
                data_quality: true,
                primary_it_investment_uii: None,
                is_primary_source: None,
            }
        }

        /// Sets the access level.
        pub fn with_access_level(mut self, access_level: AccessLevel) -> Self {
            self.access_level = access_level;
            self
        }

        /// Sets the data quality indicator.
        pub fn with_data_quality(mut self, quality: bool) -> Self {
            self.data_quality = quality;
            self
        }

        /// Converts Data.gov metadata to RDF triples.
        pub fn to_triples(&self, subject: &str) -> Vec<Triple> {
            let mut triples = Vec::new();

            // Bureau codes
            for code in &self.bureau_code {
                triples.push(Triple {
                    subject: subject.to_string(),
                    predicate: "pod:bureauCode".to_string(),
                    object: RdfValue::string(code),
                });
            }

            // Program codes
            for code in &self.program_code {
                triples.push(Triple {
                    subject: subject.to_string(),
                    predicate: "pod:programCode".to_string(),
                    object: RdfValue::string(code),
                });
            }

            // Access level
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "pod:accessLevel".to_string(),
                object: RdfValue::string(self.access_level.as_str()),
            });

            // Data quality
            triples.push(Triple {
                subject: subject.to_string(),
                predicate: "dqv:hasQualityMeasurement".to_string(),
                object: RdfValue::boolean(self.data_quality),
            });

            // Primary IT investment UII
            if let Some(ref uii) = self.primary_it_investment_uii {
                triples.push(Triple {
                    subject: subject.to_string(),
                    predicate: "pod:primaryITInvestmentUII".to_string(),
                    object: RdfValue::string(uii),
                });
            }

            triples
        }
    }

    /// Data.gov compliant dataset.
    #[derive(Debug, Clone)]
    pub struct DataGovDataset {
        /// Base DCAT dataset
        pub dataset: Dataset,
        /// Data.gov specific metadata
        pub datagov_metadata: DataGovMetadata,
        /// Contact email (required by Data.gov)
        pub contact_email: String,
    }

    impl DataGovDataset {
        /// Creates a new Data.gov compliant dataset.
        pub fn new(
            dataset: Dataset,
            datagov_metadata: DataGovMetadata,
            contact_email: impl Into<String>,
        ) -> Self {
            Self {
                dataset,
                datagov_metadata,
                contact_email: contact_email.into(),
            }
        }

        /// Converts to RDF triples with Data.gov extensions.
        pub fn to_triples(&self) -> Vec<Triple> {
            let mut triples = self.dataset.to_triples();

            // Add Data.gov specific triples
            triples.extend(self.datagov_metadata.to_triples(&self.dataset.uri));

            // Add contact email (required)
            triples.push(Triple {
                subject: self.dataset.uri.clone(),
                predicate: "dcat:contactPoint".to_string(),
                object: RdfValue::Uri(format!("mailto:{}", self.contact_email)),
            });

            triples
        }

        /// Validates Data.gov requirements.
        pub fn validate(&self) -> Result<(), Vec<String>> {
            let mut errors = Vec::new();

            // Required fields
            if self.dataset.title.is_empty() {
                errors.push("Title is required".to_string());
            }

            if self.dataset.description.is_none() {
                errors.push("Description is required by Data.gov".to_string());
            }

            if self.dataset.keywords.is_empty() {
                errors.push("At least one keyword is required by Data.gov".to_string());
            }

            if self.dataset.publisher.is_none() {
                errors.push("Publisher is required by Data.gov".to_string());
            }

            if self.contact_email.is_empty() {
                errors.push("Contact email is required by Data.gov".to_string());
            }

            if self.datagov_metadata.bureau_code.is_empty() {
                errors.push("At least one bureau code is required by Data.gov".to_string());
            }

            if self.datagov_metadata.program_code.is_empty() {
                errors.push("At least one program code is required by Data.gov".to_string());
            }

            if self.dataset.distributions.is_empty() {
                errors.push("At least one distribution is required".to_string());
            }

            if errors.is_empty() {
                Ok(())
            } else {
                Err(errors)
            }
        }
    }

    /// Data.gov catalog exporter.
    #[derive(Debug)]
    pub struct DataGovExporter {
        /// Organization name
        pub organization: String,
        /// Catalog conformance version
        pub conformance: String,
        /// Datasets
        pub datasets: Vec<DataGovDataset>,
    }

    impl DataGovExporter {
        /// Creates a new Data.gov exporter.
        pub fn new(organization: impl Into<String>) -> Self {
            Self {
                organization: organization.into(),
                conformance: "https://project-open-data.cio.gov/v1.1/schema".to_string(),
                datasets: Vec::new(),
            }
        }

        /// Adds a dataset.
        pub fn add_dataset(&mut self, dataset: DataGovDataset) -> Result<(), Vec<String>> {
            // Validate before adding
            dataset.validate()?;
            self.datasets.push(dataset);
            Ok(())
        }

        /// Exports to JSON in Data.gov (Project Open Data) format.
        pub fn export_json(&self) -> Result<String, serde_json::Error> {
            let mut catalog = serde_json::Map::new();

            // Conformance
            catalog.insert(
                "@context".to_string(),
                serde_json::json!("https://project-open-data.cio.gov/v1.1/schema/catalog.jsonld"),
            );
            catalog.insert("@type".to_string(), serde_json::json!("dcat:Catalog"));
            catalog.insert(
                "conformsTo".to_string(),
                serde_json::json!(self.conformance),
            );

            // Datasets
            let datasets_json: Vec<serde_json::Value> = self
                .datasets
                .iter()
                .map(|ds| self.dataset_to_json(ds))
                .collect();

            catalog.insert("dataset".to_string(), serde_json::json!(datasets_json));

            serde_json::to_string_pretty(&catalog)
        }

        /// Converts a dataset to JSON.
        fn dataset_to_json(&self, ds: &DataGovDataset) -> serde_json::Value {
            let mut obj = serde_json::Map::new();

            obj.insert("@type".to_string(), serde_json::json!("dcat:Dataset"));
            obj.insert("identifier".to_string(), serde_json::json!(ds.dataset.uri));
            obj.insert("title".to_string(), serde_json::json!(ds.dataset.title));

            if let Some(ref desc) = ds.dataset.description {
                obj.insert("description".to_string(), serde_json::json!(desc));
            }

            // Keywords
            if !ds.dataset.keywords.is_empty() {
                obj.insert(
                    "keyword".to_string(),
                    serde_json::json!(ds.dataset.keywords),
                );
            }

            // Modified date
            if let Some(modified) = ds.dataset.modified {
                obj.insert(
                    "modified".to_string(),
                    serde_json::json!(modified.to_rfc3339()),
                );
            }

            // Publisher
            if let Some(ref publisher) = ds.dataset.publisher {
                obj.insert(
                    "publisher".to_string(),
                    serde_json::json!({
                        "@type": "org:Organization",
                        "name": publisher
                    }),
                );
            }

            // Contact point
            obj.insert(
                "contactPoint".to_string(),
                serde_json::json!({
                    "@type": "vcard:Contact",
                    "fn": "Dataset Contact",
                    "hasEmail": format!("mailto:{}", ds.contact_email)
                }),
            );

            // Access level
            obj.insert(
                "accessLevel".to_string(),
                serde_json::json!(ds.datagov_metadata.access_level.as_str()),
            );

            // Bureau and program codes
            obj.insert(
                "bureauCode".to_string(),
                serde_json::json!(ds.datagov_metadata.bureau_code),
            );
            obj.insert(
                "programCode".to_string(),
                serde_json::json!(ds.datagov_metadata.program_code),
            );

            // Distributions
            let dists: Vec<serde_json::Value> = ds
                .dataset
                .distributions
                .iter()
                .map(|d| {
                    serde_json::json!({
                        "@type": "dcat:Distribution",
                        "mediaType": d.format,
                        "accessURL": d.access_url,
                        "downloadURL": d.download_url,
                    })
                })
                .collect();

            if !dists.is_empty() {
                obj.insert("distribution".to_string(), serde_json::json!(dists));
            }

            serde_json::Value::Object(obj)
        }

        /// Exports to RDF triples.
        pub fn export_triples(&self) -> Vec<Triple> {
            let mut triples = Vec::new();

            for ds in &self.datasets {
                triples.extend(ds.to_triples());
            }

            triples
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_create_datagov_metadata() {
            let metadata =
                DataGovMetadata::new(vec!["015:11".to_string()], vec!["015:001".to_string()])
                    .with_access_level(AccessLevel::Public)
                    .with_data_quality(true);

            assert_eq!(metadata.bureau_code.len(), 1);
            assert_eq!(metadata.program_code.len(), 1);
            assert_eq!(metadata.access_level, AccessLevel::Public);
        }

        #[test]
        fn test_datagov_metadata_to_triples() {
            let metadata =
                DataGovMetadata::new(vec!["015:11".to_string()], vec!["015:001".to_string()]);

            let triples = metadata.to_triples("https://example.org/dataset");

            assert!(!triples.is_empty());
            assert!(triples.iter().any(|t| t.predicate == "pod:bureauCode"));
            assert!(triples.iter().any(|t| t.predicate == "pod:programCode"));
            assert!(triples.iter().any(|t| t.predicate == "pod:accessLevel"));
        }

        #[test]
        fn test_create_datagov_dataset() {
            let dist = Distribution::new(
                "https://example.org/dist",
                "text/turtle",
                "https://example.org/data.ttl",
            );

            let dataset = Dataset::new("https://example.org/dataset", "Legal Statutes")
                .with_description("Collection of legal statutes")
                .with_keyword("law")
                .with_publisher("https://example.org/agency")
                .with_distribution(dist);

            let metadata =
                DataGovMetadata::new(vec!["015:11".to_string()], vec!["015:001".to_string()]);

            let datagov_ds = DataGovDataset::new(dataset, metadata, "contact@example.org");

            assert_eq!(datagov_ds.contact_email, "contact@example.org");
        }

        #[test]
        fn test_datagov_dataset_validation_valid() {
            let dist = Distribution::new(
                "https://example.org/dist",
                "text/turtle",
                "https://example.org/data.ttl",
            );

            let dataset = Dataset::new("https://example.org/dataset", "Legal Statutes")
                .with_description("Collection of legal statutes")
                .with_keyword("law")
                .with_publisher("https://example.org/agency")
                .with_distribution(dist);

            let metadata =
                DataGovMetadata::new(vec!["015:11".to_string()], vec!["015:001".to_string()]);

            let datagov_ds = DataGovDataset::new(dataset, metadata, "contact@example.org");

            assert!(datagov_ds.validate().is_ok());
        }

        #[test]
        fn test_datagov_dataset_validation_missing_fields() {
            let dataset = Dataset::new("https://example.org/dataset", "Legal Statutes");

            let metadata = DataGovMetadata::new(Vec::new(), Vec::new());

            let datagov_ds = DataGovDataset::new(dataset, metadata, "");

            let result = datagov_ds.validate();
            assert!(result.is_err());

            let errors = result.unwrap_err();
            assert!(!errors.is_empty());
        }

        #[test]
        fn test_datagov_exporter() {
            let mut exporter = DataGovExporter::new("Example Agency");

            let dist = Distribution::new(
                "https://example.org/dist",
                "text/turtle",
                "https://example.org/data.ttl",
            );

            let dataset = Dataset::new("https://example.org/dataset", "Legal Statutes")
                .with_description("Collection of legal statutes")
                .with_keyword("law")
                .with_publisher("https://example.org/agency")
                .with_distribution(dist);

            let metadata =
                DataGovMetadata::new(vec!["015:11".to_string()], vec!["015:001".to_string()]);

            let datagov_ds = DataGovDataset::new(dataset, metadata, "contact@example.org");

            assert!(exporter.add_dataset(datagov_ds).is_ok());
            assert_eq!(exporter.datasets.len(), 1);
        }

        #[test]
        fn test_datagov_export_json() {
            let mut exporter = DataGovExporter::new("Example Agency");

            let dist = Distribution::new(
                "https://example.org/dist",
                "text/turtle",
                "https://example.org/data.ttl",
            );

            let dataset = Dataset::new("https://example.org/dataset", "Legal Statutes")
                .with_description("Collection of legal statutes")
                .with_keyword("law")
                .with_publisher("https://example.org/agency")
                .with_distribution(dist);

            let metadata =
                DataGovMetadata::new(vec!["015:11".to_string()], vec!["015:001".to_string()]);

            let datagov_ds = DataGovDataset::new(dataset, metadata, "contact@example.org");

            exporter.add_dataset(datagov_ds).unwrap();

            let json = exporter.export_json().unwrap();

            assert!(json.contains("dcat:Catalog"));
            assert!(json.contains("Legal Statutes"));
            assert!(json.contains("bureauCode"));
            assert!(json.contains("programCode"));
        }

        #[test]
        fn test_access_level_string() {
            assert_eq!(AccessLevel::Public.as_str(), "public");
            assert_eq!(AccessLevel::RestrictedPublic.as_str(), "restricted public");
            assert_eq!(AccessLevel::NonPublic.as_str(), "non-public");
        }

        #[test]
        fn test_datagov_export_triples() {
            let mut exporter = DataGovExporter::new("Example Agency");

            let dist = Distribution::new(
                "https://example.org/dist",
                "text/turtle",
                "https://example.org/data.ttl",
            );

            let dataset = Dataset::new("https://example.org/dataset", "Legal Statutes")
                .with_description("Collection of legal statutes")
                .with_keyword("law")
                .with_publisher("https://example.org/agency")
                .with_distribution(dist);

            let metadata =
                DataGovMetadata::new(vec!["015:11".to_string()], vec!["015:001".to_string()]);

            let datagov_ds = DataGovDataset::new(dataset, metadata, "contact@example.org");

            exporter.add_dataset(datagov_ds).unwrap();

            let triples = exporter.export_triples();

            assert!(!triples.is_empty());
            assert!(triples.iter().any(|t| t.predicate == "pod:bureauCode"));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_catalog() {
        let catalog = Catalog::new("https://example.org/catalog", "Legal Statutes Catalog")
            .with_description("Catalog of legal statutes in RDF format")
            .with_publisher("https://example.org/publisher");

        assert_eq!(catalog.title, "Legal Statutes Catalog");
        assert!(catalog.description.is_some());
        assert!(catalog.publisher.is_some());
    }

    #[test]
    fn test_create_dataset() {
        let dataset = Dataset::new("https://example.org/dataset/statutes", "Statutes Dataset")
            .with_description("All legal statutes")
            .with_keyword("law")
            .with_keyword("legal")
            .with_spatial_coverage("EU");

        assert_eq!(dataset.title, "Statutes Dataset");
        assert_eq!(dataset.keywords.len(), 2);
        assert!(dataset.spatial_coverage.is_some());
    }

    #[test]
    fn test_create_distribution() {
        let dist = Distribution::new(
            "https://example.org/dist/turtle",
            "text/turtle",
            "https://example.org/data.ttl",
        )
        .with_download_url("https://example.org/download/data.ttl")
        .with_byte_size(1024);

        assert_eq!(dist.format, "text/turtle");
        assert!(dist.download_url.is_some());
        assert_eq!(dist.byte_size, Some(1024));
    }

    #[test]
    fn test_catalog_to_triples() {
        let catalog = Catalog::new("https://example.org/catalog", "Test Catalog");
        let triples = catalog.to_triples();

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "dcat:Catalog")));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:title"));
    }

    #[test]
    fn test_dataset_to_triples() {
        let dataset =
            Dataset::new("https://example.org/dataset", "Test Dataset").with_keyword("test");
        let triples = dataset.to_triples();

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "dcat:Dataset")));
        assert!(triples.iter().any(|t| t.predicate == "dcat:keyword"));
    }

    #[test]
    fn test_distribution_to_triples() {
        let dist = Distribution::new(
            "https://example.org/dist",
            "text/turtle",
            "https://example.org/data",
        );
        let triples = dist.to_triples();

        assert!(!triples.is_empty());
        assert!(triples.iter().any(|t| t.predicate == "rdf:type"
            && matches!(&t.object, RdfValue::Uri(u) if u == "dcat:Distribution")));
        assert!(triples.iter().any(|t| t.predicate == "dcat:mediaType"));
        assert!(triples.iter().any(|t| t.predicate == "dcat:accessURL"));
    }

    #[test]
    fn test_catalog_with_datasets() {
        let dataset = Dataset::new("https://example.org/dataset", "Test Dataset");
        let mut catalog = Catalog::new("https://example.org/catalog", "Test Catalog");
        catalog.add_dataset(dataset);

        let triples = catalog.to_triples();

        assert!(triples.iter().any(|t| t.predicate == "dcat:dataset"));
    }

    #[test]
    fn test_dataset_with_distributions() {
        let dist1 = Distribution::new(
            "https://example.org/dist/ttl",
            "text/turtle",
            "https://example.org/data.ttl",
        );
        let dist2 = Distribution::new(
            "https://example.org/dist/json",
            "application/ld+json",
            "https://example.org/data.jsonld",
        );

        let dataset = Dataset::new("https://example.org/dataset", "Test Dataset")
            .with_distribution(dist1)
            .with_distribution(dist2);

        let triples = dataset.to_triples();

        let dist_count = triples
            .iter()
            .filter(|t| t.predicate == "dcat:distribution")
            .count();
        assert_eq!(dist_count, 2);
    }

    #[test]
    fn test_publishing_workflow() {
        let catalog = Catalog::new("https://example.org/catalog", "Test Catalog");
        let workflow = PublishingWorkflow::new(catalog);

        assert!(workflow.catalog().title == "Test Catalog");
    }

    #[test]
    fn test_validate_valid_dataset() {
        let catalog = Catalog::new("https://example.org/catalog", "Test Catalog");
        let workflow = PublishingWorkflow::new(catalog);

        let dist = Distribution::new(
            "https://example.org/dist",
            "text/turtle",
            "https://example.org/data.ttl",
        );
        let dataset =
            Dataset::new("https://example.org/dataset", "Test Dataset").with_distribution(dist);

        let result = workflow.validate(&dataset);
        assert!(result.valid);
    }

    #[test]
    fn test_validate_invalid_dataset() {
        let catalog = Catalog::new("https://example.org/catalog", "Test Catalog");
        let workflow = PublishingWorkflow::new(catalog);

        // Dataset without distribution (invalid)
        let dataset = Dataset::new("https://example.org/dataset", "Test Dataset");

        let result = workflow.validate(&dataset);
        assert!(!result.valid);
        assert!(!result.errors.is_empty());
    }

    #[test]
    fn test_publish_valid_dataset() {
        let catalog = Catalog::new("https://example.org/catalog", "Test Catalog");
        let mut workflow = PublishingWorkflow::new(catalog);

        let dist = Distribution::new(
            "https://example.org/dist",
            "text/turtle",
            "https://example.org/data.ttl",
        );
        let dataset =
            Dataset::new("https://example.org/dataset", "Test Dataset").with_distribution(dist);

        assert!(workflow.publish(dataset).is_ok());
        assert_eq!(workflow.catalog().datasets.len(), 1);
    }

    #[test]
    fn test_publish_invalid_dataset() {
        let catalog = Catalog::new("https://example.org/catalog", "Test Catalog");
        let mut workflow = PublishingWorkflow::new(catalog);

        // Dataset without distribution (invalid)
        let dataset = Dataset::new("https://example.org/dataset", "Test Dataset");

        assert!(workflow.publish(dataset).is_err());
        assert_eq!(workflow.catalog().datasets.len(), 0);
    }

    #[test]
    fn test_validation_warnings() {
        let catalog = Catalog::new("https://example.org/catalog", "Test Catalog");
        let workflow = PublishingWorkflow::new(catalog);

        let dist = Distribution::new(
            "https://example.org/dist",
            "text/turtle",
            "https://example.org/data.ttl",
        );
        let dataset =
            Dataset::new("https://example.org/dataset", "Test Dataset").with_distribution(dist);

        let result = workflow.validate(&dataset);
        assert!(result.valid);
        // Should have warnings about missing description, keywords, etc.
        assert!(!result.warnings.is_empty());
    }
}
