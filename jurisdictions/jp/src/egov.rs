//! e-Gov Law XML Parser.
//!
//! Parses Japanese laws from the official e-Gov XML format.
//! See: https://laws.e-gov.go.jp/

use quick_xml::Reader;
use quick_xml::events::Event;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::era::JapaneseDate;

/// Parsed e-Gov law document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EGovLaw {
    /// Law number (e.g., "昭和二十一年憲法")
    pub law_num: String,
    /// Law title
    pub title: String,
    /// Enactment date
    pub enacted_date: Option<JapaneseDate>,
    /// Articles
    pub articles: Vec<EGovArticle>,
    /// Preamble text
    pub preamble: Option<String>,
    /// Supplementary provisions
    pub supplementary: Vec<String>,
}

/// An article in an e-Gov law.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EGovArticle {
    /// Article number (e.g., "1", "2", "9条の2")
    pub num: String,
    /// Article caption/title
    pub caption: Option<String>,
    /// Paragraphs
    pub paragraphs: Vec<EGovParagraph>,
}

/// A paragraph within an article.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EGovParagraph {
    /// Paragraph number (1-indexed)
    pub num: u32,
    /// Paragraph text
    pub text: String,
    /// Items within the paragraph
    pub items: Vec<EGovItem>,
}

/// An item within a paragraph.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EGovItem {
    /// Item number (e.g., "一", "二", "イ", "ロ")
    pub num: String,
    /// Item text
    pub text: String,
}

/// e-Gov XML parser.
#[derive(Debug, Default)]
pub struct EGovLawParser {
    _private: (),
}

impl EGovLawParser {
    /// Creates a new parser.
    #[must_use]
    pub fn new() -> Self {
        Self { _private: () }
    }

    /// Parses an e-Gov XML document.
    pub fn parse(&self, xml: &str) -> Result<EGovLaw, EGovError> {
        let mut reader = Reader::from_str(xml);
        reader.config_mut().trim_text(true);

        let mut law = EGovLaw {
            law_num: String::new(),
            title: String::new(),
            enacted_date: None,
            articles: Vec::new(),
            preamble: None,
            supplementary: Vec::new(),
        };

        let mut buf = Vec::new();
        let mut current_path: Vec<String> = Vec::new();
        let mut current_text = String::new();
        let mut current_article: Option<EGovArticle> = None;
        let mut current_paragraph: Option<EGovParagraph> = None;
        let mut current_item: Option<EGovItem> = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    current_path.push(tag.clone());
                    current_text.clear();

                    match tag.as_str() {
                        "Article" => {
                            let num = Self::get_attribute(&e, "Num").unwrap_or_default();
                            current_article = Some(EGovArticle {
                                num,
                                caption: None,
                                paragraphs: Vec::new(),
                            });
                        }
                        "Paragraph" => {
                            let num: u32 = Self::get_attribute(&e, "Num")
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(1);
                            current_paragraph = Some(EGovParagraph {
                                num,
                                text: String::new(),
                                items: Vec::new(),
                            });
                        }
                        "Item" => {
                            let num = Self::get_attribute(&e, "Num").unwrap_or_default();
                            current_item = Some(EGovItem {
                                num,
                                text: String::new(),
                            });
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    let path_str = current_path.join("/");

                    match tag.as_str() {
                        "LawNum" => {
                            law.law_num = current_text.trim().to_string();
                        }
                        "LawTitle" => {
                            law.title = current_text.trim().to_string();
                        }
                        "Preamble" | "PreambleText" => {
                            if !current_text.trim().is_empty() {
                                law.preamble = Some(current_text.trim().to_string());
                            }
                        }
                        "ArticleCaption" => {
                            if let Some(ref mut article) = current_article {
                                article.caption = Some(current_text.trim().to_string());
                            }
                        }
                        "ParagraphSentence" | "Sentence" => {
                            if let Some(ref mut para) = current_paragraph {
                                if !current_text.trim().is_empty() {
                                    if para.text.is_empty() {
                                        para.text = current_text.trim().to_string();
                                    } else {
                                        para.text.push_str(current_text.trim());
                                    }
                                }
                            }
                        }
                        "ItemSentence" => {
                            if let Some(ref mut item) = current_item {
                                item.text = current_text.trim().to_string();
                            }
                        }
                        "Item" => {
                            if let Some(item) = current_item.take() {
                                if let Some(ref mut para) = current_paragraph {
                                    para.items.push(item);
                                }
                            }
                        }
                        "Paragraph" => {
                            if let Some(para) = current_paragraph.take() {
                                if let Some(ref mut article) = current_article {
                                    article.paragraphs.push(para);
                                }
                            }
                        }
                        "Article" => {
                            if let Some(article) = current_article.take() {
                                law.articles.push(article);
                            }
                        }
                        "SupplProvision" if path_str.ends_with("SupplProvision") => {
                            if !current_text.trim().is_empty() {
                                law.supplementary.push(current_text.trim().to_string());
                            }
                        }
                        _ => {}
                    }

                    current_path.pop();
                    current_text.clear();
                }
                Ok(Event::Text(e)) => {
                    current_text.push_str(&e.unescape().unwrap_or_default());
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(EGovError::XmlParse(e.to_string())),
                _ => {}
            }
            buf.clear();
        }

        if law.title.is_empty() && law.law_num.is_empty() && law.articles.is_empty() {
            return Err(EGovError::InvalidDocument(
                "Empty or invalid document".into(),
            ));
        }

        Ok(law)
    }

    fn get_attribute(e: &quick_xml::events::BytesStart<'_>, name: &str) -> Option<String> {
        e.attributes().filter_map(|a| a.ok()).find_map(|a| {
            if a.key.as_ref() == name.as_bytes() {
                Some(String::from_utf8_lossy(&a.value).to_string())
            } else {
                None
            }
        })
    }
}

/// Errors from e-Gov parsing.
#[derive(Debug, Error)]
pub enum EGovError {
    /// XML parsing error.
    #[error("XML parse error: {0}")]
    XmlParse(String),

    /// Invalid document structure.
    #[error("Invalid document: {0}")]
    InvalidDocument(String),
}

/// Converts an e-Gov law to Legalis statutes.
impl EGovLaw {
    /// Converts to Legalis core statutes.
    pub fn to_statutes(&self) -> Vec<legalis_core::Statute> {
        self.articles
            .iter()
            .map(|article| {
                let id = format!("{}-article-{}", self.law_num, article.num);
                let title = article
                    .caption
                    .clone()
                    .unwrap_or_else(|| format!("第{}条", article.num));
                let description = article
                    .paragraphs
                    .iter()
                    .map(|p| p.text.clone())
                    .collect::<Vec<_>>()
                    .join("\n");

                legalis_core::Statute::new(
                    &id,
                    &title,
                    legalis_core::Effect::new(legalis_core::EffectType::Grant, &description),
                )
                .with_jurisdiction("JP")
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_law() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<Law>
    <LawNum>令和元年法律第一号</LawNum>
    <LawBody>
        <LawTitle>テスト法</LawTitle>
        <MainProvision>
            <Article Num="1">
                <ArticleCaption>（目的）</ArticleCaption>
                <Paragraph Num="1">
                    <ParagraphSentence>
                        <Sentence>この法律は、テストを目的とする。</Sentence>
                    </ParagraphSentence>
                </Paragraph>
            </Article>
            <Article Num="2">
                <ArticleCaption>（定義）</ArticleCaption>
                <Paragraph Num="1">
                    <ParagraphSentence>
                        <Sentence>この法律において「テスト」とは、次に掲げるものをいう。</Sentence>
                    </ParagraphSentence>
                    <Item Num="一">
                        <ItemSentence>単体テスト</ItemSentence>
                    </Item>
                    <Item Num="二">
                        <ItemSentence>結合テスト</ItemSentence>
                    </Item>
                </Paragraph>
            </Article>
        </MainProvision>
    </LawBody>
</Law>"#;

        let parser = EGovLawParser::new();
        let law = parser.parse(xml).unwrap();

        assert_eq!(law.law_num, "令和元年法律第一号");
        assert_eq!(law.title, "テスト法");
        assert_eq!(law.articles.len(), 2);

        let article1 = &law.articles[0];
        assert_eq!(article1.num, "1");
        assert_eq!(article1.caption, Some("（目的）".to_string()));
        assert_eq!(
            article1.paragraphs[0].text,
            "この法律は、テストを目的とする。"
        );

        let article2 = &law.articles[1];
        assert_eq!(article2.paragraphs[0].items.len(), 2);
        assert_eq!(article2.paragraphs[0].items[0].num, "一");
    }

    #[test]
    fn test_to_statutes() {
        let law = EGovLaw {
            law_num: "テスト法".to_string(),
            title: "テスト法".to_string(),
            enacted_date: None,
            articles: vec![EGovArticle {
                num: "1".to_string(),
                caption: Some("（目的）".to_string()),
                paragraphs: vec![EGovParagraph {
                    num: 1,
                    text: "この法律は、テストを目的とする。".to_string(),
                    items: vec![],
                }],
            }],
            preamble: None,
            supplementary: vec![],
        };

        let statutes = law.to_statutes();
        assert_eq!(statutes.len(), 1);
        assert_eq!(statutes[0].jurisdiction, Some("JP".to_string()));
    }
}
