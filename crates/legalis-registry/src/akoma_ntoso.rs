//! Import/export support for Akoma Ntoso format.
//!
//! Akoma Ntoso is an XML standard for parliamentary,
//! legislative and judiciary documents.

use super::*;
use quick_xml::de::from_str;
use quick_xml::se::to_string;

/// Akoma Ntoso document wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "akomaNtoso")]
pub struct AkomaNtoso {
    #[serde(rename = "act")]
    pub act: Act,
}

/// Akoma Ntoso act element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    #[serde(rename = "meta")]
    pub meta: Meta,
    #[serde(rename = "body")]
    pub body: Body,
}

/// Akoma Ntoso metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meta {
    #[serde(rename = "identification")]
    pub identification: Identification,
    #[serde(rename = "publication")]
    pub publication: Option<Publication>,
}

/// Akoma Ntoso identification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identification {
    #[serde(rename = "FRBRWork")]
    pub work: FRBRLevel,
    #[serde(rename = "FRBRExpression")]
    pub expression: FRBRLevel,
}

/// Akoma Ntoso FRBR level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FRBRLevel {
    #[serde(rename = "FRBRthis")]
    pub this: FRBRElement,
    #[serde(rename = "FRBRuri")]
    pub uri: FRBRElement,
    #[serde(rename = "FRBRdate")]
    pub date: FRBRDate,
    #[serde(rename = "FRBRauthor")]
    pub author: FRBRElement,
    #[serde(rename = "FRBRcountry")]
    pub country: FRBRElement,
}

/// Akoma Ntoso FRBR element.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FRBRElement {
    #[serde(rename = "@value")]
    pub value: String,
}

/// Akoma Ntoso FRBR date.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FRBRDate {
    #[serde(rename = "@date")]
    pub date: String,
    #[serde(rename = "@name")]
    pub name: String,
}

/// Akoma Ntoso publication.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Publication {
    #[serde(rename = "@date")]
    pub date: String,
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@showAs")]
    pub show_as: String,
}

/// Akoma Ntoso body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Body {
    #[serde(rename = "section", default)]
    pub sections: Vec<Section>,
}

/// Akoma Ntoso section.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    #[serde(rename = "@eId")]
    pub id: String,
    #[serde(rename = "num")]
    pub num: Option<String>,
    #[serde(rename = "heading")]
    pub heading: Option<String>,
    #[serde(rename = "content")]
    pub content: Option<String>,
}

/// Exports a statute to Akoma Ntoso format.
pub fn export_statute(entry: &StatuteEntry) -> Result<String, quick_xml::SeError> {
    let akoma = statute_to_akoma(entry);
    to_string(&akoma)
}

/// Imports a statute from Akoma Ntoso format.
pub fn import_statute(xml: &str, jurisdiction: &str) -> Result<StatuteEntry, quick_xml::DeError> {
    let akoma: AkomaNtoso = from_str(xml)?;
    Ok(akoma_to_statute(akoma, jurisdiction))
}

/// Converts a statute to Akoma Ntoso format.
fn statute_to_akoma(entry: &StatuteEntry) -> AkomaNtoso {
    AkomaNtoso {
        act: Act {
            meta: Meta {
                identification: Identification {
                    work: FRBRLevel {
                        this: FRBRElement {
                            value: format!("/akn/{}/act/{}", entry.jurisdiction, entry.statute.id),
                        },
                        uri: FRBRElement {
                            value: format!("/akn/{}/act/{}", entry.jurisdiction, entry.statute.id),
                        },
                        date: FRBRDate {
                            date: entry.created_at.format("%Y-%m-%d").to_string(),
                            name: "enactment".to_string(),
                        },
                        author: FRBRElement {
                            value: format!("#{}", entry.jurisdiction),
                        },
                        country: FRBRElement {
                            value: entry.jurisdiction.clone(),
                        },
                    },
                    expression: FRBRLevel {
                        this: FRBRElement {
                            value: format!(
                                "/akn/{}/act/{}/eng@{}",
                                entry.jurisdiction,
                                entry.statute.id,
                                entry.created_at.format("%Y-%m-%d")
                            ),
                        },
                        uri: FRBRElement {
                            value: format!(
                                "/akn/{}/act/{}/eng@",
                                entry.jurisdiction, entry.statute.id
                            ),
                        },
                        date: FRBRDate {
                            date: entry.modified_at.format("%Y-%m-%d").to_string(),
                            name: "expression".to_string(),
                        },
                        author: FRBRElement {
                            value: "#author".to_string(),
                        },
                        country: FRBRElement {
                            value: entry.jurisdiction.clone(),
                        },
                    },
                },
                publication: entry.effective_date.map(|d| Publication {
                    date: d.format("%Y-%m-%d").to_string(),
                    name: "publication".to_string(),
                    show_as: "Publication Date".to_string(),
                }),
            },
            body: Body {
                sections: vec![Section {
                    id: "main".to_string(),
                    num: Some("1".to_string()),
                    heading: Some(entry.statute.title.clone()),
                    content: Some(format!("{:?}", entry.statute)),
                }],
            },
        },
    }
}

/// Converts Akoma Ntoso format to a statute.
fn akoma_to_statute(akoma: AkomaNtoso, jurisdiction: &str) -> StatuteEntry {
    let statute_id = akoma
        .act
        .meta
        .identification
        .work
        .uri
        .value
        .split('/')
        .next_back()
        .unwrap_or("unknown")
        .to_string();

    let title = akoma
        .act
        .body
        .sections
        .first()
        .and_then(|s| s.heading.clone())
        .unwrap_or_else(|| "Untitled".to_string());

    // Create a default effect for imported statutes
    let effect = legalis_core::Effect::new(
        legalis_core::EffectType::Custom,
        "Imported from Akoma Ntoso XML",
    );

    let statute = Statute::new(&statute_id, &title, effect);

    StatuteEntry::new(statute, jurisdiction)
}
