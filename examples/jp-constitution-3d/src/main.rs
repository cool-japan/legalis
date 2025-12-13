//! Japanese Constitution 3D Visualization Example
//!
//! This example demonstrates how to:
//! 1. Create a structured representation of the Japanese Constitution
//! 2. Generate hierarchical visualizations (chapters → articles → paragraphs)
//! 3. Export to various formats (JSON, Mermaid, ASCII tree)
//!
//! The "3D" aspect refers to the three-dimensional structure:
//! - Layer 1: Chapters (章)
//! - Layer 2: Articles (条)
//! - Layer 3: Paragraphs/Items (項/号)

use legalis_jp::constitution::{
    ArticleParagraph, BilingualText, Constitution, ConstitutionArticle,
};
use legalis_viz::DecisionTree;

fn main() {
    println!("=== Japanese Constitution 3D Structure ===\n");

    // Create constitution with key articles
    let constitution = create_sample_constitution();

    // Display structure overview
    display_structure(&constitution);

    // Generate visualizations
    generate_chapter_diagram(&constitution);

    // Export to JSON
    export_to_json(&constitution);

    // Generate statute visualizations
    generate_statute_visualizations(&constitution);
}

/// Creates a sample constitution with key articles.
fn create_sample_constitution() -> Constitution {
    let mut constitution = Constitution::new();

    // Chapter 1: The Emperor (Articles 1-8)
    constitution.add_article(create_article_1());

    // Chapter 2: Renunciation of War (Article 9)
    constitution.add_article(Constitution::article_9());

    // Chapter 3: Rights and Duties of the People (Articles 10-40)
    constitution.add_article(create_article_11()); // Fundamental human rights
    constitution.add_article(create_article_13()); // Individual respect
    constitution.add_article(create_article_14()); // Equality under the law
    constitution.add_article(create_article_19()); // Freedom of thought
    constitution.add_article(create_article_21()); // Freedom of expression
    constitution.add_article(Constitution::article_25()); // Right to life

    // Chapter 10: Supreme Law (Articles 97-99)
    constitution.add_article(create_article_97()); // Fundamental human rights as eternal

    constitution
}

fn create_article_1() -> ConstitutionArticle {
    ConstitutionArticle {
        number: 1,
        caption: Some(BilingualText {
            ja: "天皇の地位と主権".to_string(),
            en: "The Emperor, Symbol of the State".to_string(),
        }),
        paragraphs: vec![ArticleParagraph {
            number: 1,
            text: BilingualText {
                ja: "天皇は、日本国の象徴であり日本国民統合の象徴であつて、この地位は、主権の存する日本国民の総意に基く。".to_string(),
                en: "The Emperor shall be the symbol of the State and of the unity of the People, deriving his position from the will of the people with whom resides sovereign power.".to_string(),
            },
        }],
        fundamental_right: false,
        keywords: vec!["emperor".to_string(), "symbol".to_string(), "sovereignty".to_string()],
    }
}

fn create_article_11() -> ConstitutionArticle {
    ConstitutionArticle {
        number: 11,
        caption: Some(BilingualText {
            ja: "基本的人権の享有".to_string(),
            en: "Enjoyment of Fundamental Human Rights".to_string(),
        }),
        paragraphs: vec![ArticleParagraph {
            number: 1,
            text: BilingualText {
                ja: "国民は、すべての基本的人権の享有を妨げられない。この憲法が国民に保障する基本的人権は、侵すことのできない永久の権利として、現在及び将来の国民に与へられる。".to_string(),
                en: "The people shall not be prevented from enjoying any of the fundamental human rights. These fundamental human rights guaranteed to the people by this Constitution shall be conferred upon the people of this and future generations as eternal and inviolate rights.".to_string(),
            },
        }],
        fundamental_right: true,
        keywords: vec!["human rights".to_string(), "fundamental".to_string(), "eternal".to_string()],
    }
}

fn create_article_13() -> ConstitutionArticle {
    ConstitutionArticle {
        number: 13,
        caption: Some(BilingualText {
            ja: "個人の尊重".to_string(),
            en: "Respect for Individuals".to_string(),
        }),
        paragraphs: vec![ArticleParagraph {
            number: 1,
            text: BilingualText {
                ja: "すべて国民は、個人として尊重される。生命、自由及び幸福追求に対する国民の権利については、公共の福祉に反しない限り、立法その他の国政の上で、最大の尊重を必要とする。".to_string(),
                en: "All of the people shall be respected as individuals. Their right to life, liberty, and the pursuit of happiness shall, to the extent that it does not interfere with the public welfare, be the supreme consideration in legislation and in other governmental affairs.".to_string(),
            },
        }],
        fundamental_right: true,
        keywords: vec!["individual".to_string(), "life".to_string(), "liberty".to_string(), "happiness".to_string()],
    }
}

fn create_article_14() -> ConstitutionArticle {
    ConstitutionArticle {
        number: 14,
        caption: Some(BilingualText {
            ja: "法の下の平等".to_string(),
            en: "Equality Under the Law".to_string(),
        }),
        paragraphs: vec![
            ArticleParagraph {
                number: 1,
                text: BilingualText {
                    ja: "すべて国民は、法の下に平等であつて、人種、信条、性別、社会的身分又は門地により、政治的、経済的又は社会的関係において、差別されない。".to_string(),
                    en: "All of the people are equal under the law and there shall be no discrimination in political, economic or social relations because of race, creed, sex, social status or family origin.".to_string(),
                },
            },
            ArticleParagraph {
                number: 2,
                text: BilingualText {
                    ja: "華族その他の貴族の制度は、これを認めない。".to_string(),
                    en: "Peers and peerage shall not be recognized.".to_string(),
                },
            },
            ArticleParagraph {
                number: 3,
                text: BilingualText {
                    ja: "栄誉、勲章その他の栄典の授与は、いかなる特権も伴はない。栄典の授与は、現にこれを有し、又は将来これを受ける者の一代に限り、その効力を有する。".to_string(),
                    en: "No privilege shall accompany any award of honor, decoration or any distinction, nor shall any such award be valid beyond the lifetime of the individual who now holds or hereafter may receive it.".to_string(),
                },
            },
        ],
        fundamental_right: true,
        keywords: vec!["equality".to_string(), "discrimination".to_string(), "race".to_string(), "sex".to_string()],
    }
}

fn create_article_19() -> ConstitutionArticle {
    ConstitutionArticle {
        number: 19,
        caption: Some(BilingualText {
            ja: "思想及び良心の自由".to_string(),
            en: "Freedom of Thought and Conscience".to_string(),
        }),
        paragraphs: vec![ArticleParagraph {
            number: 1,
            text: BilingualText {
                ja: "思想及び良心の自由は、これを侵してはならない。".to_string(),
                en: "Freedom of thought and conscience shall not be violated.".to_string(),
            },
        }],
        fundamental_right: true,
        keywords: vec![
            "thought".to_string(),
            "conscience".to_string(),
            "freedom".to_string(),
        ],
    }
}

fn create_article_21() -> ConstitutionArticle {
    ConstitutionArticle {
        number: 21,
        caption: Some(BilingualText {
            ja: "表現の自由".to_string(),
            en: "Freedom of Expression".to_string(),
        }),
        paragraphs: vec![
            ArticleParagraph {
                number: 1,
                text: BilingualText {
                    ja: "集会、結社及び言論、出版その他一切の表現の自由は、これを保障する。".to_string(),
                    en: "Freedom of assembly and association as well as speech, press and all other forms of expression are guaranteed.".to_string(),
                },
            },
            ArticleParagraph {
                number: 2,
                text: BilingualText {
                    ja: "検閲は、これをしてはならない。通信の秘密は、これを侵してはならない。".to_string(),
                    en: "No censorship shall be maintained, nor shall the secrecy of any means of communication be violated.".to_string(),
                },
            },
        ],
        fundamental_right: true,
        keywords: vec!["expression".to_string(), "speech".to_string(), "press".to_string(), "censorship".to_string()],
    }
}

fn create_article_97() -> ConstitutionArticle {
    ConstitutionArticle {
        number: 97,
        caption: Some(BilingualText {
            ja: "基本的人権の本質".to_string(),
            en: "Fundamental Human Rights as Eternal".to_string(),
        }),
        paragraphs: vec![ArticleParagraph {
            number: 1,
            text: BilingualText {
                ja: "この憲法が日本国民に保障する基本的人権は、人類の多年にわたる自由獲得の努力の成果であつて、これらの権利は、過去幾多の試錬に堪へ、現在及び将来の国民に対し、侵すことのできない永久の権利として信託されたものである。".to_string(),
                en: "The fundamental human rights by this Constitution guaranteed to the people of Japan are fruits of the age-old struggle of man to be free; they have survived the many exacting tests for durability and are conferred upon this and future generations in trust, to be held for all time inviolate.".to_string(),
            },
        }],
        fundamental_right: true,
        keywords: vec!["human rights".to_string(), "eternal".to_string(), "trust".to_string()],
    }
}

/// Displays the hierarchical structure of the constitution.
fn display_structure(constitution: &Constitution) {
    println!("📜 {} ({})", constitution.title_ja, constitution.title_en);
    println!(
        "   公布: {} ({})",
        constitution.promulgation_date,
        constitution.promulgation_date.to_western().unwrap()
    );
    println!(
        "   施行: {} ({})\n",
        constitution.effective_date,
        constitution.effective_date.to_western().unwrap()
    );

    println!("=== Chapter Structure (章構成) ===\n");

    for chapter in &constitution.chapters {
        let article_count = chapter.articles.len();
        if article_count > 0 {
            println!(
                "📂 第{}章 {} ({})",
                chapter.number, chapter.title_ja, chapter.title_en
            );

            for article in &chapter.articles {
                let caption = article
                    .caption
                    .as_ref()
                    .map(|c| format!(" - {}", c.ja))
                    .unwrap_or_default();
                let rights_marker = if article.fundamental_right {
                    "🔵"
                } else {
                    "⚪"
                };

                println!("   {} 第{}条{}", rights_marker, article.number, caption);

                for para in &article.paragraphs {
                    let preview: String = para.text.ja.chars().take(40).collect();
                    println!("      {}. {}...", para.number, preview);
                }
            }
            println!();
        }
    }

    // Summary
    let total_articles: usize = constitution.chapters.iter().map(|c| c.articles.len()).sum();
    let fundamental_rights = constitution.fundamental_rights().len();

    println!("=== Summary ===");
    println!("Total chapters: 11");
    println!("Articles in this example: {}", total_articles);
    println!("Fundamental rights articles: {} 🔵", fundamental_rights);
    println!();
}

/// Generates a Mermaid diagram of chapter relationships.
fn generate_chapter_diagram(constitution: &Constitution) {
    println!("=== Mermaid Chapter Diagram ===\n");

    println!("```mermaid");
    println!("graph TD");
    println!("    Constitution[\"🏛️ 日本国憲法<br/>The Constitution of Japan\"]");

    for chapter in &constitution.chapters {
        if !chapter.articles.is_empty() {
            let chapter_id = format!("Ch{}", chapter.number);
            println!(
                "    {}[\"第{}章 {}<br/>{}\"]",
                chapter_id, chapter.number, chapter.title_ja, chapter.title_en
            );
            println!("    Constitution --> {}", chapter_id);

            for article in &chapter.articles {
                let article_id = format!("Art{}", article.number);
                let style = if article.fundamental_right {
                    ":::fundamental"
                } else {
                    ""
                };
                println!("    {}[\"第{}条\"]{}", article_id, article.number, style);
                println!("    {} --> {}", chapter_id, article_id);
            }
        }
    }

    println!();
    println!("    classDef fundamental fill:#e3f2fd,stroke:#1976d2");
    println!("```");
    println!();
}

/// Exports the constitution to JSON.
fn export_to_json(constitution: &Constitution) {
    println!("=== JSON Export (sample) ===\n");

    #[derive(serde::Serialize)]
    struct ArticleSummary {
        number: u32,
        title_ja: String,
        title_en: String,
        fundamental_right: bool,
        paragraph_count: usize,
        keywords: Vec<String>,
    }

    let summaries: Vec<_> = constitution
        .chapters
        .iter()
        .flat_map(|c| &c.articles)
        .map(|a| ArticleSummary {
            number: a.number,
            title_ja: a.caption.as_ref().map(|c| c.ja.clone()).unwrap_or_default(),
            title_en: a.caption.as_ref().map(|c| c.en.clone()).unwrap_or_default(),
            fundamental_right: a.fundamental_right,
            paragraph_count: a.paragraphs.len(),
            keywords: a.keywords.clone(),
        })
        .collect();

    let json = serde_json::to_string_pretty(&summaries).unwrap();
    println!("{}\n", json);
}

/// Generates statute visualizations using legalis-viz.
fn generate_statute_visualizations(constitution: &Constitution) {
    println!("=== Statute Visualizations ===\n");

    let statutes = constitution.to_statutes();

    if let Some(statute) = statutes.first()
        && let Ok(tree) = DecisionTree::from_statute(statute)
    {
        println!("ASCII Tree for Article 9:");
        println!("{}\n", tree.to_ascii());
    }

    // Show fundamental rights as a list
    println!("Fundamental Rights Articles:");
    for article in constitution.fundamental_rights() {
        println!(
            "  • 第{}条: {}",
            article.number,
            article
                .caption
                .as_ref()
                .map(|c| c.ja.as_str())
                .unwrap_or("")
        );
    }
}
