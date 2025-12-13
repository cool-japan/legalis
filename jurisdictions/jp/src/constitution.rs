//! Japanese Constitution (日本国憲法) support.
//!
//! Provides structures and parsing for the Japanese Constitution,
//! including both Japanese and English text.

use serde::{Deserialize, Serialize};

use crate::era::{Era, JapaneseDate};

/// The Japanese Constitution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constitution {
    /// Title in Japanese
    pub title_ja: String,
    /// Title in English
    pub title_en: String,
    /// Promulgation date (公布日): November 3, Showa 21 (1946)
    pub promulgation_date: JapaneseDate,
    /// Effective date (施行日): May 3, Showa 22 (1947)
    pub effective_date: JapaneseDate,
    /// Preamble text (前文)
    pub preamble: BilingualText,
    /// Chapters (章)
    pub chapters: Vec<ConstitutionChapter>,
}

/// Bilingual text (Japanese/English).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BilingualText {
    /// Japanese text
    pub ja: String,
    /// English translation
    pub en: String,
}

/// A chapter of the constitution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionChapter {
    /// Chapter number (1-11)
    pub number: u32,
    /// Chapter title in Japanese
    pub title_ja: String,
    /// Chapter title in English
    pub title_en: String,
    /// Articles in this chapter
    pub articles: Vec<ConstitutionArticle>,
}

/// An article of the constitution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstitutionArticle {
    /// Article number (1-103)
    pub number: u32,
    /// Article title/caption (if any)
    pub caption: Option<BilingualText>,
    /// Article paragraphs
    pub paragraphs: Vec<ArticleParagraph>,
    /// Whether this article is a key provision for fundamental rights
    pub fundamental_right: bool,
    /// Related keywords for categorization
    pub keywords: Vec<String>,
}

/// A paragraph within a constitution article.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleParagraph {
    /// Paragraph number (1-indexed)
    pub number: u32,
    /// Paragraph text
    pub text: BilingualText,
}

impl Constitution {
    /// Creates the standard Japanese Constitution structure.
    #[must_use]
    pub fn new() -> Self {
        Self {
            title_ja: "日本国憲法".to_string(),
            title_en: "The Constitution of Japan".to_string(),
            promulgation_date: JapaneseDate::new(Era::Showa, 21, 11, 3).unwrap(),
            effective_date: JapaneseDate::new(Era::Showa, 22, 5, 3).unwrap(),
            preamble: BilingualText {
                ja: PREAMBLE_JA.to_string(),
                en: PREAMBLE_EN.to_string(),
            },
            chapters: Self::default_chapters(),
        }
    }

    /// Returns the chapter structure with titles.
    fn default_chapters() -> Vec<ConstitutionChapter> {
        vec![
            ConstitutionChapter {
                number: 1,
                title_ja: "天皇".to_string(),
                title_en: "The Emperor".to_string(),
                articles: Vec::new(), // Articles 1-8
            },
            ConstitutionChapter {
                number: 2,
                title_ja: "戦争の放棄".to_string(),
                title_en: "Renunciation of War".to_string(),
                articles: Vec::new(), // Article 9
            },
            ConstitutionChapter {
                number: 3,
                title_ja: "国民の権利及び義務".to_string(),
                title_en: "Rights and Duties of the People".to_string(),
                articles: Vec::new(), // Articles 10-40
            },
            ConstitutionChapter {
                number: 4,
                title_ja: "国会".to_string(),
                title_en: "The Diet".to_string(),
                articles: Vec::new(), // Articles 41-64
            },
            ConstitutionChapter {
                number: 5,
                title_ja: "内閣".to_string(),
                title_en: "The Cabinet".to_string(),
                articles: Vec::new(), // Articles 65-75
            },
            ConstitutionChapter {
                number: 6,
                title_ja: "司法".to_string(),
                title_en: "Judiciary".to_string(),
                articles: Vec::new(), // Articles 76-82
            },
            ConstitutionChapter {
                number: 7,
                title_ja: "財政".to_string(),
                title_en: "Finance".to_string(),
                articles: Vec::new(), // Articles 83-91
            },
            ConstitutionChapter {
                number: 8,
                title_ja: "地方自治".to_string(),
                title_en: "Local Self-Government".to_string(),
                articles: Vec::new(), // Articles 92-95
            },
            ConstitutionChapter {
                number: 9,
                title_ja: "改正".to_string(),
                title_en: "Amendments".to_string(),
                articles: Vec::new(), // Article 96
            },
            ConstitutionChapter {
                number: 10,
                title_ja: "最高法規".to_string(),
                title_en: "Supreme Law".to_string(),
                articles: Vec::new(), // Articles 97-99
            },
            ConstitutionChapter {
                number: 11,
                title_ja: "補則".to_string(),
                title_en: "Supplementary Provisions".to_string(),
                articles: Vec::new(), // Articles 100-103
            },
        ]
    }

    /// Adds an article to the appropriate chapter.
    pub fn add_article(&mut self, article: ConstitutionArticle) {
        let chapter_idx = match article.number {
            1..=8 => 0,
            9 => 1,
            10..=40 => 2,
            41..=64 => 3,
            65..=75 => 4,
            76..=82 => 5,
            83..=91 => 6,
            92..=95 => 7,
            96 => 8,
            97..=99 => 9,
            _ => 10,
        };
        self.chapters[chapter_idx].articles.push(article);
    }

    /// Gets an article by number.
    #[must_use]
    pub fn get_article(&self, number: u32) -> Option<&ConstitutionArticle> {
        for chapter in &self.chapters {
            if let Some(article) = chapter.articles.iter().find(|a| a.number == number) {
                return Some(article);
            }
        }
        None
    }

    /// Returns all fundamental rights articles (Chapter 3).
    #[must_use]
    pub fn fundamental_rights(&self) -> Vec<&ConstitutionArticle> {
        self.chapters[2]
            .articles
            .iter()
            .filter(|a| a.fundamental_right)
            .collect()
    }

    /// Converts to Legalis core statutes.
    #[must_use]
    pub fn to_statutes(&self) -> Vec<legalis_core::Statute> {
        let mut statutes = Vec::new();

        for chapter in &self.chapters {
            for article in &chapter.articles {
                let id = format!("jp-constitution-article-{}", article.number);
                let title = format!("第{}条 (Article {})", article.number, article.number);

                let description = article
                    .paragraphs
                    .iter()
                    .map(|p| format!("{}. {}", p.number, p.text.ja))
                    .collect::<Vec<_>>()
                    .join("\n");

                let mut statute = legalis_core::Statute::new(
                    &id,
                    &title,
                    legalis_core::Effect::new(legalis_core::EffectType::Grant, &description),
                )
                .with_jurisdiction("JP");

                if article.fundamental_right {
                    statute = statute.with_discretion(
                        "Constitutional fundamental right - requires judicial interpretation",
                    );
                }

                statutes.push(statute);
            }
        }

        statutes
    }

    /// Creates Article 9 (戦争の放棄).
    #[must_use]
    pub fn article_9() -> ConstitutionArticle {
        ConstitutionArticle {
            number: 9,
            caption: Some(BilingualText {
                ja: "戦争の放棄".to_string(),
                en: "Renunciation of War".to_string(),
            }),
            paragraphs: vec![
                ArticleParagraph {
                    number: 1,
                    text: BilingualText {
                        ja: "日本国民は、正義と秩序を基調とする国際平和を誠実に希求し、国権の発動たる戦争と、武力による威嚇又は武力の行使は、国際紛争を解決する手段としては、永久にこれを放棄する。".to_string(),
                        en: "Aspiring sincerely to an international peace based on justice and order, the Japanese people forever renounce war as a sovereign right of the nation and the threat or use of force as means of settling international disputes.".to_string(),
                    },
                },
                ArticleParagraph {
                    number: 2,
                    text: BilingualText {
                        ja: "前項の目的を達するため、陸海空軍その他の戦力は、これを保持しない。国の交戦権は、これを認めない。".to_string(),
                        en: "In order to accomplish the aim of the preceding paragraph, land, sea, and air forces, as well as other war potential, will never be maintained. The right of belligerency of the state will not be recognized.".to_string(),
                    },
                },
            ],
            fundamental_right: false,
            keywords: vec!["peace".to_string(), "war".to_string(), "military".to_string()],
        }
    }

    /// Creates Article 25 (生存権).
    #[must_use]
    pub fn article_25() -> ConstitutionArticle {
        ConstitutionArticle {
            number: 25,
            caption: Some(BilingualText {
                ja: "生存権".to_string(),
                en: "Right to Life".to_string(),
            }),
            paragraphs: vec![
                ArticleParagraph {
                    number: 1,
                    text: BilingualText {
                        ja: "すべて国民は、健康で文化的な最低限度の生活を営む権利を有する。".to_string(),
                        en: "All people shall have the right to maintain the minimum standards of wholesome and cultured living.".to_string(),
                    },
                },
                ArticleParagraph {
                    number: 2,
                    text: BilingualText {
                        ja: "国は、すべての生活部面について、社会福祉、社会保障及び公衆衛生の向上及び増進に努めなければならない。".to_string(),
                        en: "In all spheres of life, the State shall use its endeavors for the promotion and extension of social welfare and security, and of public health.".to_string(),
                    },
                },
            ],
            fundamental_right: true,
            keywords: vec!["welfare".to_string(), "health".to_string(), "life".to_string()],
        }
    }
}

impl Default for Constitution {
    fn default() -> Self {
        Self::new()
    }
}

/// Japanese preamble text.
const PREAMBLE_JA: &str = "日本国民は、正当に選挙された国会における代表者を通じて行動し、われらとわれらの子孫のために、諸国民との協和による成果と、わが国全土にわたつて自由のもたらす恵沢を確保し、政府の行為によつて再び戦争の惨禍が起ることのないやうにすることを決意し、ここに主権が国民に存することを宣言し、この憲法を確定する。";

/// English preamble text.
const PREAMBLE_EN: &str = "We, the Japanese people, acting through our duly elected representatives in the National Diet, determined that we shall secure for ourselves and our posterity the fruits of peaceful cooperation with all nations and the blessings of liberty throughout this land, and resolved that never again shall we be visited with the horrors of war through the action of government, do proclaim that sovereign power resides with the people and do firmly establish this Constitution.";

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;

    #[test]
    fn test_constitution_dates() {
        let constitution = Constitution::new();

        // Promulgation: November 3, Showa 21 (1946)
        let promulgation = constitution.promulgation_date.to_western().unwrap();
        assert_eq!(promulgation.year(), 1946);
        assert_eq!(promulgation.month(), 11);
        assert_eq!(promulgation.day(), 3);

        // Effective: May 3, Showa 22 (1947)
        let effective = constitution.effective_date.to_western().unwrap();
        assert_eq!(effective.year(), 1947);
        assert_eq!(effective.month(), 5);
        assert_eq!(effective.day(), 3);
    }

    #[test]
    fn test_chapter_structure() {
        let constitution = Constitution::new();
        assert_eq!(constitution.chapters.len(), 11);
        assert_eq!(constitution.chapters[0].title_ja, "天皇");
        assert_eq!(constitution.chapters[1].title_ja, "戦争の放棄");
        assert_eq!(constitution.chapters[2].title_ja, "国民の権利及び義務");
    }

    #[test]
    fn test_article_9() {
        let article = Constitution::article_9();
        assert_eq!(article.number, 9);
        assert_eq!(article.paragraphs.len(), 2);
        assert!(article.paragraphs[0].text.ja.contains("戦争"));
        assert!(article.paragraphs[0].text.en.contains("war"));
    }

    #[test]
    fn test_article_25() {
        let article = Constitution::article_25();
        assert_eq!(article.number, 25);
        assert!(article.fundamental_right);
        assert!(article.paragraphs[0].text.ja.contains("生活を営む権利"));
    }

    #[test]
    fn test_add_article() {
        let mut constitution = Constitution::new();
        constitution.add_article(Constitution::article_9());
        constitution.add_article(Constitution::article_25());

        assert!(constitution.get_article(9).is_some());
        assert!(constitution.get_article(25).is_some());
        assert!(constitution.get_article(1).is_none());
    }

    #[test]
    fn test_to_statutes() {
        let mut constitution = Constitution::new();
        constitution.add_article(Constitution::article_9());
        constitution.add_article(Constitution::article_25());

        let statutes = constitution.to_statutes();
        assert_eq!(statutes.len(), 2);
        assert!(
            statutes
                .iter()
                .all(|s| s.jurisdiction == Some("JP".to_string()))
        );
    }
}
