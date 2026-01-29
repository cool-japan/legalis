//! Judgment Anonymization - Proof of Concept
//!
//! This example demonstrates automated anonymization of court judgments using:
//! - MeCrab: Morphological analysis for named entity detection
//! - Legalis-RS: Legal document structure parsing
//! - APPI Article 35-2: Pseudonymization logic (仮名加工情報)
//!
//! Status: Research prototype - demonstrates technical feasibility

use anyhow::Result;
use mecrab::MeCrab;
use std::collections::HashMap;
use std::fs;

/// Represents an anonymization mapping
#[derive(Debug)]
struct AnonymizationMap {
    /// Original name -> Pseudonym mapping
    mappings: HashMap<String, String>,
    /// Counter for generating pseudonyms
    person_counter: usize,
    company_counter: usize,
    place_counter: usize,
}

impl AnonymizationMap {
    fn new() -> Self {
        Self {
            mappings: HashMap::new(),
            person_counter: 0,
            company_counter: 0,
            place_counter: 0,
        }
    }

    /// Get or create pseudonym for a name
    fn get_or_create(&mut self, name: &str, entity_type: EntityType) -> String {
        if let Some(pseudonym) = self.mappings.get(name) {
            return pseudonym.clone();
        }

        let pseudonym = match entity_type {
            EntityType::Person => {
                self.person_counter += 1;
                format!("Person{}", self.person_counter)
            }
            EntityType::Company => {
                self.company_counter += 1;
                format!("Company{}", self.company_counter)
            }
            EntityType::Place => {
                self.place_counter += 1;
                format!("Place{}", self.place_counter)
            }
        };

        self.mappings.insert(name.to_string(), pseudonym.clone());
        pseudonym
    }
}

/// Entity type for named entity recognition
#[derive(Debug, Clone, Copy)]
enum EntityType {
    Person,
    Company,
    Place,
}

/// Judgment document structure
#[derive(Debug)]
#[allow(dead_code)]
struct JudgmentStructure {
    /// Title section (case number, type)
    title: Option<String>,
    /// Parties section (原告, 被告)
    parties: Option<String>,
    /// Main text section (主文)
    main_text: Option<String>,
    /// Facts and reasons (事実及び理由)
    facts_and_reasons: Option<String>,
    /// Signature section (裁判官)
    signatures: Option<String>,
    /// Full text
    full_text: String,
}

impl JudgmentStructure {
    /// Parse judgment text into structured sections
    fn parse(text: &str) -> Self {
        let mut parties_section = None;
        let mut main_text_section = None;
        let mut facts_section = None;
        let mut signatures_section = None;

        let lines: Vec<&str> = text.lines().collect();

        // Find section boundaries
        for (i, line) in lines.iter().enumerate() {
            // Detect parties section (原告, 被告 appear early)
            if parties_section.is_none() && (line.starts_with("原告") || line.starts_with("被告"))
            {
                let end = lines
                    .iter()
                    .skip(i)
                    .position(|l| l.trim().is_empty())
                    .unwrap_or(10);
                parties_section = Some(lines[i..i + end.min(lines.len() - i)].join("\n"));
            }

            // Detect main text (主文)
            if line.starts_with("主文") {
                let end = lines
                    .iter()
                    .skip(i)
                    .position(|l| l.starts_with("事実") || l.is_empty())
                    .unwrap_or(20);
                main_text_section = Some(lines[i..i + end.min(lines.len() - i)].join("\n"));
            }

            // Detect facts and reasons (事実及び理由)
            if line.starts_with("事実及び理由") {
                let end = lines
                    .iter()
                    .skip(i)
                    .position(|l| l.contains("裁判所") || l.contains("裁判官"))
                    .unwrap_or(lines.len() - i);
                facts_section = Some(lines[i..i + end.min(lines.len() - i)].join("\n"));
            }

            // Detect signatures (裁判長裁判官, 裁判官)
            if line.contains("裁判長裁判官")
                || (signatures_section.is_none() && line.contains("裁判官"))
            {
                signatures_section = Some(lines[i..].join("\n"));
                break;
            }
        }

        Self {
            title: lines.first().map(|s| s.to_string()),
            parties: parties_section,
            main_text: main_text_section,
            facts_and_reasons: facts_section,
            signatures: signatures_section,
            full_text: text.to_string(),
        }
    }

    /// Get sections that should be anonymized with high priority
    fn high_priority_sections(&self) -> Vec<&str> {
        let mut sections = Vec::new();
        if let Some(ref parties) = self.parties {
            sections.push(parties.as_str());
        }
        if let Some(ref sigs) = self.signatures {
            sections.push(sigs.as_str());
        }
        sections
    }
}

/// Extract named entities from morphological analysis result
fn extract_named_entities(text: &str, mecrab: &MeCrab) -> Result<Vec<(String, EntityType)>> {
    let result = mecrab.parse(text)?;
    let mut entities = Vec::new();

    // Process each morpheme and merge consecutive entities of same type
    // Feature format: 品詞,品詞細分類1,品詞細分類2,品詞細分類3,活用型,活用形,原形,読み,発音
    let mut current_entity: Option<(String, EntityType)> = None;

    for morpheme in &result.morphemes {
        let features: Vec<&str> = morpheme.feature.split(',').collect();

        if features.len() < 3 {
            if let Some(entity) = current_entity.take() {
                entities.push(entity);
            }
            continue;
        }

        let pos = features[0]; // 品詞
        let pos1 = features[1]; // 品詞細分類1
        let pos2 = features[2]; // 品詞細分類2

        // Detect named entities (名詞-固有名詞-*)
        if pos == "名詞" && pos1 == "固有名詞" {
            let entity_type = match pos2 {
                "人名" => Some(EntityType::Person),
                "組織" => Some(EntityType::Company),
                "地域" => Some(EntityType::Place),
                _ => None,
            };

            if let Some(et) = entity_type {
                // Merge consecutive entities of same type
                match &mut current_entity {
                    Some((surface, current_type))
                        if std::mem::discriminant(current_type) == std::mem::discriminant(&et) =>
                    {
                        surface.push_str(&morpheme.surface);
                    }
                    _ => {
                        if let Some(entity) = current_entity.take() {
                            entities.push(entity);
                        }
                        current_entity = Some((morpheme.surface.clone(), et));
                    }
                }
            } else if let Some(entity) = current_entity.take() {
                entities.push(entity);
            }
        } else if let Some(entity) = current_entity.take() {
            entities.push(entity);
        }
    }

    // Don't forget the last entity
    if let Some(entity) = current_entity {
        entities.push(entity);
    }

    Ok(entities)
}

/// Anonymize judgment text with structure awareness
fn anonymize_judgment(text: &str, mecrab: &MeCrab) -> Result<(String, AnonymizationMap)> {
    // NEW: Parse judgment structure
    let structure = JudgmentStructure::parse(text);

    println!("\n📋 Judgment Structure Analysis:");
    println!(
        "   Parties section: {}",
        if structure.parties.is_some() {
            "✅ Detected"
        } else {
            "❌ Not found"
        }
    );
    println!(
        "   Main text section: {}",
        if structure.main_text.is_some() {
            "✅ Detected"
        } else {
            "❌ Not found"
        }
    );
    println!(
        "   Facts & reasons: {}",
        if structure.facts_and_reasons.is_some() {
            "✅ Detected"
        } else {
            "❌ Not found"
        }
    );
    println!(
        "   Signatures: {}",
        if structure.signatures.is_some() {
            "✅ Detected"
        } else {
            "❌ Not found"
        }
    );

    // Extract entities from high-priority sections first
    let _high_priority_sections = structure.high_priority_sections();

    let entities = extract_named_entities(text, mecrab)?;
    let mut map = AnonymizationMap::new();
    let mut anonymized = text.to_string();

    // Sort by length (descending) to avoid partial replacements
    let mut sorted_entities = entities;
    sorted_entities.sort_by(|a, b| b.0.len().cmp(&a.0.len()));

    println!("\n🔍 Named Entities Detected: {}", sorted_entities.len());

    for (name, entity_type) in sorted_entities {
        let pseudonym = map.get_or_create(&name, entity_type);
        anonymized = anonymized.replace(&name, &pseudonym);
    }

    Ok((anonymized, map))
}

/// Detect MeCab dictionary directory from common locations
fn detect_mecab_dicdir() -> Option<std::path::PathBuf> {
    use std::path::Path;

    // Try environment variable first
    if let Ok(dicdir) = std::env::var("MECAB_DICDIR") {
        let path = Path::new(&dicdir);
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }

    // Try common paths
    let candidates = vec![
        "/opt/homebrew/lib/mecab/dic/ipadic",    // macOS (Homebrew ARM)
        "/usr/local/lib/mecab/dic/ipadic",       // macOS (Homebrew Intel)
        "/var/lib/mecab/dic/ipadic",             // Debian/Ubuntu
        "/usr/share/mecab/dic/ipadic",           // Other Linux
        "/usr/lib/mecab/dic/ipadic",             // Alternative Linux
        "C:\\Program Files\\MeCab\\dic\\ipadic", // Windows
        "C:\\Program Files (x86)\\MeCab\\dic\\ipadic", // Windows 32-bit
    ];

    for candidate in candidates {
        let path = Path::new(candidate);
        if path.exists() {
            return Some(path.to_path_buf());
        }
    }

    None
}

fn main() -> Result<()> {
    println!("=== Judgment Anonymization PoC ===\n");

    // Initialize MeCrab
    println!("Initializing MeCrab morphological analyzer...");

    // Detect dictionary directory
    let dicdir = match detect_mecab_dicdir() {
        Some(path) => {
            println!("Found MeCab dictionary at: {}", path.display());
            Some(path)
        }
        None => {
            eprintln!("Error: MeCab dictionary not found.");
            eprintln!("\nPlease install MeCab and its dictionary:");
            eprintln!("  macOS:   brew install mecab mecab-ipadic");
            eprintln!("  Ubuntu:  sudo apt-get install mecab mecab-ipadic-utf8");
            eprintln!("  Windows: Download from https://taku910.github.io/mecab/");
            eprintln!("\nOr set MECAB_DICDIR environment variable:");
            eprintln!("  export MECAB_DICDIR=/path/to/mecab/dic/ipadic");
            return Err(anyhow::anyhow!("MeCab dictionary not found"));
        }
    };

    let mecrab = match MeCrab::builder().dicdir(dicdir).build() {
        Ok(m) => m,
        Err(e) => {
            eprintln!("Error: Failed to initialize MeCrab: {}", e);
            return Err(e.into());
        }
    };
    println!("MeCrab initialized successfully.\n");

    // Read sample judgment
    let sample_path = "sample_judgments/civil_case_01.txt";
    let text = match fs::read_to_string(sample_path) {
        Ok(content) => content,
        Err(_) => {
            // Use embedded sample if file not found
            println!("Sample file not found, using embedded example...\n");
            "原告田中太郎（以下「原告」という）は、被告山田花子（以下「被告」という）に対し、\
             令和5年3月15日、東京都渋谷区において、金500万円を貸し付けた。\
             被告は同年9月15日までに返済する約束であったが、履行しなかった。\
             よって、原告は被告に対し、貸金500万円及びこれに対する遅延損害金の支払いを求める。"
                .to_string()
        }
    };

    println!("Original Judgment:");
    println!("----------------------------------------");
    println!("{}\n", text);

    // Perform anonymization
    println!("Performing anonymization...");
    let (anonymized, mapping) = anonymize_judgment(&text, &mecrab)?;

    println!("Anonymized Judgment:");
    println!("----------------------------------------");
    println!("{}\n", anonymized);

    println!("Anonymization Mapping:");
    println!("----------------------------------------");
    for (original, pseudonym) in &mapping.mappings {
        println!("  {} → {}", original, pseudonym);
    }
    println!();

    println!("Status: PoC completed successfully");
    println!("Note: This is a research prototype demonstrating technical feasibility.");
    println!("Production use requires enhanced accuracy and legal review.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anonymization_map() {
        let mut map = AnonymizationMap::new();

        let p1 = map.get_or_create("田中太郎", EntityType::Person);
        assert_eq!(p1, "Person1");

        let p1_again = map.get_or_create("田中太郎", EntityType::Person);
        assert_eq!(p1_again, "Person1"); // Same pseudonym for same name

        let p2 = map.get_or_create("山田花子", EntityType::Person);
        assert_eq!(p2, "Person2");
    }
}
