//! Indian Evidence Act 1872 / BSA 2023 Types

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EvidenceType {
    Oral,
    Documentary,
    Electronic,
    Material,
    Circumstantial,
    Hearsay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Admissibility {
    Admissible,
    Inadmissible,
    ConditionallyAdmissible,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub description: String,
    pub admissibility: Admissibility,
    pub relevant_sections: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BurdenOfProof {
    Prosecution,
    Defence,
    Shifting,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Presumption {
    FactPresumption,
    LawPresumption,
    RebuttablePresumption,
    ConclusivePresumption,
}
