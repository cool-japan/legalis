//! External data source integrations.
//!
//! This module provides integration with external legal data sources:
//! - EUR-Lex (European Union law)
//! - legislation.gov.uk (UK legislation)
//! - Wikidata (structured knowledge base)
//! - DBpedia (structured Wikipedia data)
//! - GovTrack.us (US federal legislation)

use crate::{RdfValue, Triple};

/// EUR-Lex integration for European Union legislation.
pub mod eurlex {
    use super::*;

    /// EUR-Lex base URI.
    pub const BASE_URI: &str = "http://data.europa.eu/eli/";

    /// EUR-Lex SPARQL endpoint.
    pub const SPARQL_ENDPOINT: &str = "http://publications.europa.eu/webapi/rdf/sparql";

    /// Creates a link to EUR-Lex resource.
    pub fn create_eurlex_link(subject_uri: &str, celex_number: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let eurlex_uri = format!("{}{}", BASE_URI, celex_number);

        // owl:sameAs link
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "owl:sameAs".to_string(),
            object: RdfValue::Uri(eurlex_uri.clone()),
        });

        // rdfs:seeAlso link
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "rdfs:seeAlso".to_string(),
            object: RdfValue::Uri(eurlex_uri),
        });

        triples
    }

    /// Creates triples for EUR-Lex metadata.
    pub fn add_eurlex_metadata(
        subject_uri: &str,
        celex_number: &str,
        title: Option<&str>,
        date: Option<&str>,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();
        let eurlex_uri = format!("{}{}", BASE_URI, celex_number);

        // Basic link
        triples.extend(create_eurlex_link(subject_uri, celex_number));

        // Additional metadata
        if let Some(t) = title {
            triples.push(Triple {
                subject: eurlex_uri.clone(),
                predicate: "dcterms:title".to_string(),
                object: RdfValue::string(t),
            });
        }

        if let Some(d) = date {
            triples.push(Triple {
                subject: eurlex_uri,
                predicate: "dcterms:date".to_string(),
                object: RdfValue::string(d),
            });
        }

        triples
    }
}

/// Legislation.gov.uk integration for UK legislation.
pub mod legislation_gov_uk {
    use super::*;

    /// Legislation.gov.uk base URI.
    pub const BASE_URI: &str = "http://www.legislation.gov.uk/";

    /// Legislation.gov.uk linked data URI.
    pub const LINKED_DATA_URI: &str = "http://www.legislation.gov.uk/id/";

    /// UK legislation types.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum LegislationType {
        /// UK Public General Act
        UkPublicGeneralAct,
        /// UK Statutory Instrument
        UkStatutoryInstrument,
        /// UK Local Act
        UkLocalAct,
        /// Scottish Act
        ScottishAct,
        /// Welsh Act
        WelshAct,
        /// Northern Ireland Act
        NorthernIrelandAct,
    }

    impl LegislationType {
        /// Returns the URI path segment for this type.
        pub fn path_segment(&self) -> &'static str {
            match self {
                Self::UkPublicGeneralAct => "ukpga",
                Self::UkStatutoryInstrument => "uksi",
                Self::UkLocalAct => "ukla",
                Self::ScottishAct => "asp",
                Self::WelshAct => "anaw",
                Self::NorthernIrelandAct => "nia",
            }
        }

        /// Returns the label for this type.
        pub fn label(&self) -> &'static str {
            match self {
                Self::UkPublicGeneralAct => "UK Public General Act",
                Self::UkStatutoryInstrument => "UK Statutory Instrument",
                Self::UkLocalAct => "UK Local Act",
                Self::ScottishAct => "Scottish Act",
                Self::WelshAct => "Welsh Act",
                Self::NorthernIrelandAct => "Northern Ireland Act",
            }
        }
    }

    /// Creates a link to legislation.gov.uk resource.
    pub fn create_legislation_link(
        subject_uri: &str,
        leg_type: LegislationType,
        year: u16,
        number: u32,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();
        let leg_uri = format!(
            "{}{}/{}/{}",
            LINKED_DATA_URI,
            leg_type.path_segment(),
            year,
            number
        );

        // owl:sameAs link
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "owl:sameAs".to_string(),
            object: RdfValue::Uri(leg_uri.clone()),
        });

        // rdfs:seeAlso link
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "rdfs:seeAlso".to_string(),
            object: RdfValue::Uri(leg_uri),
        });

        triples
    }
}

/// Wikidata integration for structured knowledge linking.
pub mod wikidata {
    use super::*;

    /// Wikidata entity base URI.
    pub const BASE_URI: &str = "http://www.wikidata.org/entity/";

    /// Wikidata SPARQL endpoint.
    pub const SPARQL_ENDPOINT: &str = "https://query.wikidata.org/sparql";

    /// Creates a link to Wikidata entity.
    pub fn create_wikidata_link(subject_uri: &str, wikidata_id: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let wikidata_uri = format!("{}{}", BASE_URI, wikidata_id);

        // owl:sameAs link (if it's the same concept)
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "owl:sameAs".to_string(),
            object: RdfValue::Uri(wikidata_uri.clone()),
        });

        // rdfs:seeAlso link
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "rdfs:seeAlso".to_string(),
            object: RdfValue::Uri(wikidata_uri),
        });

        triples
    }

    /// Creates triples for Wikidata concept mapping.
    pub fn create_concept_mapping(
        subject_uri: &str,
        wikidata_id: &str,
        relation: ConceptRelation,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();
        let wikidata_uri = format!("{}{}", BASE_URI, wikidata_id);

        let predicate = match relation {
            ConceptRelation::ExactMatch => "skos:exactMatch",
            ConceptRelation::CloseMatch => "skos:closeMatch",
            ConceptRelation::BroadMatch => "skos:broadMatch",
            ConceptRelation::NarrowMatch => "skos:narrowMatch",
            ConceptRelation::RelatedMatch => "skos:relatedMatch",
        };

        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: predicate.to_string(),
            object: RdfValue::Uri(wikidata_uri),
        });

        triples
    }

    /// Concept relation types for SKOS mapping.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ConceptRelation {
        /// Exact match
        ExactMatch,
        /// Close match
        CloseMatch,
        /// Broader match
        BroadMatch,
        /// Narrower match
        NarrowMatch,
        /// Related match
        RelatedMatch,
    }
}

/// DBpedia integration for structured Wikipedia data.
pub mod dbpedia {
    use super::*;

    /// DBpedia resource base URI.
    pub const BASE_URI: &str = "http://dbpedia.org/resource/";

    /// DBpedia SPARQL endpoint.
    pub const SPARQL_ENDPOINT: &str = "http://dbpedia.org/sparql";

    /// Creates a link to DBpedia resource.
    pub fn create_dbpedia_link(subject_uri: &str, resource_name: &str) -> Vec<Triple> {
        let mut triples = Vec::new();
        let dbpedia_uri = format!("{}{}", BASE_URI, resource_name);

        // rdfs:seeAlso link (typically not exact match)
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "rdfs:seeAlso".to_string(),
            object: RdfValue::Uri(dbpedia_uri.clone()),
        });

        // dcterms:references
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "dcterms:references".to_string(),
            object: RdfValue::Uri(dbpedia_uri),
        });

        triples
    }

    /// Creates concept mapping to DBpedia.
    pub fn create_concept_mapping(
        subject_uri: &str,
        resource_name: &str,
        relation: wikidata::ConceptRelation,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();
        let dbpedia_uri = format!("{}{}", BASE_URI, resource_name);

        let predicate = match relation {
            wikidata::ConceptRelation::ExactMatch => "skos:exactMatch",
            wikidata::ConceptRelation::CloseMatch => "skos:closeMatch",
            wikidata::ConceptRelation::BroadMatch => "skos:broadMatch",
            wikidata::ConceptRelation::NarrowMatch => "skos:narrowMatch",
            wikidata::ConceptRelation::RelatedMatch => "skos:relatedMatch",
        };

        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: predicate.to_string(),
            object: RdfValue::Uri(dbpedia_uri),
        });

        triples
    }
}

/// GovTrack.us integration for US federal legislation.
pub mod govtrack {
    use super::*;

    /// GovTrack.us base URI.
    pub const BASE_URI: &str = "https://www.govtrack.us/";

    /// GovTrack.us API base.
    pub const API_BASE: &str = "https://www.govtrack.us/api/v2/";

    /// US legislation types.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BillType {
        /// House bill
        HouseBill,
        /// Senate bill
        SenateBill,
        /// House resolution
        HouseResolution,
        /// Senate resolution
        SenateResolution,
        /// House joint resolution
        HouseJointResolution,
        /// Senate joint resolution
        SenateJointResolution,
        /// House concurrent resolution
        HouseConcurrentResolution,
        /// Senate concurrent resolution
        SenateConcurrentResolution,
    }

    impl BillType {
        /// Returns the abbreviation for this bill type.
        pub fn abbreviation(&self) -> &'static str {
            match self {
                Self::HouseBill => "hr",
                Self::SenateBill => "s",
                Self::HouseResolution => "hres",
                Self::SenateResolution => "sres",
                Self::HouseJointResolution => "hjres",
                Self::SenateJointResolution => "sjres",
                Self::HouseConcurrentResolution => "hconres",
                Self::SenateConcurrentResolution => "sconres",
            }
        }

        /// Returns the label for this bill type.
        pub fn label(&self) -> &'static str {
            match self {
                Self::HouseBill => "House Bill",
                Self::SenateBill => "Senate Bill",
                Self::HouseResolution => "House Resolution",
                Self::SenateResolution => "Senate Resolution",
                Self::HouseJointResolution => "House Joint Resolution",
                Self::SenateJointResolution => "Senate Joint Resolution",
                Self::HouseConcurrentResolution => "House Concurrent Resolution",
                Self::SenateConcurrentResolution => "Senate Concurrent Resolution",
            }
        }
    }

    /// Creates a link to GovTrack.us resource.
    pub fn create_govtrack_link(
        subject_uri: &str,
        congress: u16,
        bill_type: BillType,
        number: u32,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();
        let govtrack_uri = format!(
            "{}congress/bills/{}/{}{}",
            BASE_URI,
            congress,
            bill_type.abbreviation(),
            number
        );

        // rdfs:seeAlso link
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "rdfs:seeAlso".to_string(),
            object: RdfValue::Uri(govtrack_uri.clone()),
        });

        // dcterms:references
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "dcterms:references".to_string(),
            object: RdfValue::Uri(govtrack_uri),
        });

        triples
    }

    /// Creates triples with GovTrack metadata.
    pub fn add_govtrack_metadata(
        subject_uri: &str,
        congress: u16,
        bill_type: BillType,
        number: u32,
        title: Option<&str>,
    ) -> Vec<Triple> {
        let mut triples = Vec::new();

        // Basic link
        triples.extend(create_govtrack_link(
            subject_uri,
            congress,
            bill_type,
            number,
        ));

        // Bill type annotation
        triples.push(Triple {
            subject: subject_uri.to_string(),
            predicate: "dcterms:type".to_string(),
            object: RdfValue::string(bill_type.label()),
        });

        // Title if provided
        if let Some(t) = title {
            triples.push(Triple {
                subject: subject_uri.to_string(),
                predicate: "dcterms:title".to_string(),
                object: RdfValue::string(t),
            });
        }

        triples
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eurlex_link() {
        let triples = eurlex::create_eurlex_link("http://example.org/statute/test", "32016R0679");

        assert_eq!(triples.len(), 2);
        assert!(triples.iter().any(|t| t.predicate == "owl:sameAs"));
        assert!(triples.iter().any(|t| t.predicate == "rdfs:seeAlso"));
    }

    #[test]
    fn test_eurlex_metadata() {
        let triples = eurlex::add_eurlex_metadata(
            "http://example.org/statute/test",
            "32016R0679",
            Some("GDPR"),
            Some("2016-04-27"),
        );

        assert!(triples.len() >= 2);
        assert!(triples.iter().any(|t| t.predicate == "dcterms:title"));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:date"));
    }

    #[test]
    fn test_legislation_gov_uk_link() {
        let triples = legislation_gov_uk::create_legislation_link(
            "http://example.org/statute/test",
            legislation_gov_uk::LegislationType::UkPublicGeneralAct,
            1998,
            29,
        );

        assert_eq!(triples.len(), 2);
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("ukpga/1998/29")))
        );
    }

    #[test]
    fn test_legislation_types() {
        assert_eq!(
            legislation_gov_uk::LegislationType::UkPublicGeneralAct.path_segment(),
            "ukpga"
        );
        assert_eq!(
            legislation_gov_uk::LegislationType::ScottishAct.label(),
            "Scottish Act"
        );
    }

    #[test]
    fn test_wikidata_link() {
        let triples = wikidata::create_wikidata_link("http://example.org/concept/test", "Q123456");

        assert_eq!(triples.len(), 2);
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("Q123456")))
        );
    }

    #[test]
    fn test_wikidata_concept_mapping() {
        let triples = wikidata::create_concept_mapping(
            "http://example.org/concept/test",
            "Q123456",
            wikidata::ConceptRelation::ExactMatch,
        );

        assert_eq!(triples.len(), 1);
        assert!(triples.iter().any(|t| t.predicate == "skos:exactMatch"));
    }

    #[test]
    fn test_dbpedia_link() {
        let triples =
            dbpedia::create_dbpedia_link("http://example.org/concept/test", "European_Union");

        assert_eq!(triples.len(), 2);
        assert!(triples.iter().any(|t| t.predicate == "rdfs:seeAlso"));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:references"));
    }

    #[test]
    fn test_dbpedia_concept_mapping() {
        let triples = dbpedia::create_concept_mapping(
            "http://example.org/concept/test",
            "Contract_law",
            wikidata::ConceptRelation::CloseMatch,
        );

        assert_eq!(triples.len(), 1);
        assert!(triples.iter().any(|t| t.predicate == "skos:closeMatch"));
    }

    #[test]
    fn test_govtrack_link() {
        let triples = govtrack::create_govtrack_link(
            "http://example.org/statute/test",
            117,
            govtrack::BillType::SenateBill,
            1234,
        );

        assert_eq!(triples.len(), 2);
        assert!(
            triples
                .iter()
                .any(|t| matches!(&t.object, RdfValue::Uri(u) if u.contains("117/s1234")))
        );
    }

    #[test]
    fn test_govtrack_metadata() {
        let triples = govtrack::add_govtrack_metadata(
            "http://example.org/statute/test",
            117,
            govtrack::BillType::HouseBill,
            5678,
            Some("Test Bill"),
        );

        assert!(triples.len() >= 3);
        assert!(triples.iter().any(|t| t.predicate == "dcterms:type"));
        assert!(triples.iter().any(|t| t.predicate == "dcterms:title"));
    }

    #[test]
    fn test_govtrack_bill_types() {
        assert_eq!(govtrack::BillType::HouseBill.abbreviation(), "hr");
        assert_eq!(govtrack::BillType::SenateBill.label(), "Senate Bill");
    }
}
