//! Core types for French evidence law

use chrono::NaiveDate;

/// Represents a piece of evidence in French civil proceedings
#[derive(Debug, Clone, PartialEq)]
pub struct Evidence {
    pub evidence_type: EvidenceType,
    pub description: String,
    pub date_obtained: NaiveDate,
    pub authenticity_verified: bool,
}

impl Evidence {
    pub fn new(
        evidence_type: EvidenceType,
        description: String,
        date_obtained: NaiveDate,
        authenticity_verified: bool,
    ) -> Self {
        Self {
            evidence_type,
            description,
            date_obtained,
            authenticity_verified,
        }
    }

    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }

    pub fn with_verified(mut self, verified: bool) -> Self {
        self.authenticity_verified = verified;
        self
    }

    pub fn is_electronic(&self) -> bool {
        matches!(
            self.evidence_type,
            EvidenceType::WrittenDocument {
                electronic: true,
                ..
            }
        )
    }

    pub fn is_witness_testimony(&self) -> bool {
        matches!(self.evidence_type, EvidenceType::WitnessTestimony { .. })
    }

    pub fn is_presumption(&self) -> bool {
        matches!(self.evidence_type, EvidenceType::Presumption { .. })
    }
}

/// Types of evidence recognized by French law
#[derive(Debug, Clone, PartialEq)]
pub enum EvidenceType {
    /// Written document (paper or electronic)
    WrittenDocument { electronic: bool, signed: bool },
    /// Witness testimony (Article 1378)
    WitnessTestimony { witness: WitnessTestimony },
    /// Expert report (CPC Articles 227-229)
    ExpertReport { expert: ExpertReport },
    /// Legal presumption (Articles 1354-1355)
    Presumption { presumption_type: PresumptionType },
    /// Confession by a party (aveu)
    Confession {
        party: String,
        judicial: bool, // judicial vs. extrajudicial
    },
    /// Oath (serment)
    Oath {
        party: String,
        decisive: bool, // decisive vs. supplementary
    },
}

/// Witness testimony details
#[derive(Debug, Clone, PartialEq)]
pub struct WitnessTestimony {
    pub witness_name: String,
    pub sworn: bool,
    pub testimony_date: NaiveDate,
    pub credibility: Option<f64>, // 0.0-1.0
}

impl WitnessTestimony {
    pub fn new(witness_name: String, sworn: bool, testimony_date: NaiveDate) -> Self {
        Self {
            witness_name,
            sworn,
            testimony_date,
            credibility: None,
        }
    }

    pub fn with_credibility(mut self, credibility: f64) -> Self {
        self.credibility = Some(credibility);
        self
    }
}

/// Expert report details
#[derive(Debug, Clone, PartialEq)]
pub struct ExpertReport {
    pub expert_name: String,
    pub field: String,
    pub report_date: NaiveDate,
    pub court_appointed: bool,
}

impl ExpertReport {
    pub fn new(
        expert_name: String,
        field: String,
        report_date: NaiveDate,
        court_appointed: bool,
    ) -> Self {
        Self {
            expert_name,
            field,
            report_date,
            court_appointed,
        }
    }
}

/// Types of legal presumptions (Article 1354)
///
/// French law recognizes three types of presumptions with varying evidentiary weight:
///
/// - **Simple** (présomption simple): Rebuttable by counter-evidence
/// - **Mixed** (présomption mixte): Rebuttable only by specific means
/// - **Irrebuttable** (présomption irréfragable): Cannot be rebutted
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresumptionType {
    /// Rebuttable presumption (can be overcome by counter-evidence)
    Simple,
    /// Mixed presumption (partially rebuttable)
    Mixed,
    /// Irrebuttable presumption (conclusive)
    Irrebuttable,
}

impl PresumptionType {
    pub fn can_rebut(&self) -> bool {
        matches!(self, PresumptionType::Simple | PresumptionType::Mixed)
    }

    pub fn is_conclusive(&self) -> bool {
        matches!(self, PresumptionType::Irrebuttable)
    }
}

/// Burden of proof allocation (Article 1353)
///
/// Under French law, the burden of proof follows the principle:
/// "Actori incumbit probatio" - the burden is on the claimant.
///
/// **Original French** (Article 1353):
/// > "Celui qui réclame l'exécution d'une obligation doit la prouver.
/// > Réciproquement, celui qui se prétend libéré doit justifier le paiement
/// > ou le fait qui a produit l'extinction de son obligation."
///
/// **English Translation**:
/// > "The person who claims performance of an obligation must prove it.
/// > Conversely, the person who claims to be released must justify payment
/// > or the fact that extinguished their obligation."
#[derive(Debug, Clone, PartialEq)]
pub struct BurdenOfProof {
    pub claimant_must_prove: Vec<String>,
    pub defendant_must_prove: Vec<String>,
}

impl BurdenOfProof {
    pub fn new() -> Self {
        Self {
            claimant_must_prove: Vec::new(),
            defendant_must_prove: Vec::new(),
        }
    }

    pub fn with_claimant_burden(mut self, fact: String) -> Self {
        self.claimant_must_prove.push(fact);
        self
    }

    pub fn with_defendant_burden(mut self, fact: String) -> Self {
        self.defendant_must_prove.push(fact);
        self
    }

    pub fn claimant_burden_count(&self) -> usize {
        self.claimant_must_prove.len()
    }

    pub fn defendant_burden_count(&self) -> usize {
        self.defendant_must_prove.len()
    }
}

impl Default for BurdenOfProof {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_creation() {
        let evidence = Evidence::new(
            EvidenceType::WrittenDocument {
                electronic: true,
                signed: true,
            },
            "Contract signed on blockchain".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(evidence.is_electronic());
        assert!(evidence.authenticity_verified);
    }

    #[test]
    fn test_witness_testimony_builder() {
        let witness = WitnessTestimony::new(
            "Jean Dupont".to_string(),
            true,
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
        )
        .with_credibility(0.8);

        assert_eq!(witness.witness_name, "Jean Dupont");
        assert!(witness.sworn);
        assert_eq!(witness.credibility, Some(0.8));
    }

    #[test]
    fn test_presumption_types() {
        assert!(PresumptionType::Simple.can_rebut());
        assert!(PresumptionType::Mixed.can_rebut());
        assert!(!PresumptionType::Irrebuttable.can_rebut());
        assert!(PresumptionType::Irrebuttable.is_conclusive());
    }

    #[test]
    fn test_burden_of_proof_builder() {
        let burden = BurdenOfProof::new()
            .with_claimant_burden("Contract existence".to_string())
            .with_claimant_burden("Breach of contract".to_string())
            .with_defendant_burden("Payment made".to_string());

        assert_eq!(burden.claimant_burden_count(), 2);
        assert_eq!(burden.defendant_burden_count(), 1);
    }

    #[test]
    fn test_expert_report_creation() {
        let expert = ExpertReport::new(
            "Dr. Marie Curie".to_string(),
            "Nuclear physics".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert_eq!(expert.expert_name, "Dr. Marie Curie");
        assert!(expert.court_appointed);
    }

    #[test]
    fn test_evidence_type_checks() {
        let doc = Evidence::new(
            EvidenceType::WrittenDocument {
                electronic: false,
                signed: true,
            },
            "Paper contract".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(!doc.is_electronic());
        assert!(!doc.is_witness_testimony());

        let testimony = Evidence::new(
            EvidenceType::WitnessTestimony {
                witness: WitnessTestimony::new(
                    "John Doe".to_string(),
                    true,
                    NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                ),
            },
            "Testimony".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(testimony.is_witness_testimony());
    }

    #[test]
    fn test_confession_evidence() {
        let confession = EvidenceType::Confession {
            party: "Defendant".to_string(),
            judicial: true,
        };
        assert!(matches!(confession, EvidenceType::Confession { .. }));
    }

    #[test]
    fn test_oath_evidence() {
        let oath = EvidenceType::Oath {
            party: "Plaintiff".to_string(),
            decisive: true,
        };
        assert!(matches!(oath, EvidenceType::Oath { .. }));
    }

    #[test]
    fn test_evidence_builder_pattern() {
        let evidence = Evidence::new(
            EvidenceType::WrittenDocument {
                electronic: true,
                signed: false,
            },
            "Initial desc".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            false,
        )
        .with_description("Updated desc".to_string())
        .with_verified(true);

        assert_eq!(evidence.description, "Updated desc");
        assert!(evidence.authenticity_verified);
    }

    #[test]
    fn test_presumption_evidence() {
        let presumption = Evidence::new(
            EvidenceType::Presumption {
                presumption_type: PresumptionType::Simple,
            },
            "Legal presumption".to_string(),
            NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            true,
        );
        assert!(presumption.is_presumption());
    }
}
