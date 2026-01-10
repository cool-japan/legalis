//! CJEU Landmark Cases

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// CJEU legal principles established by case law
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum CjeuPrinciple {
    /// Direct effect - EU law creates individual rights enforceable in national courts
    DirectEffect,

    /// Supremacy - EU law prevails over conflicting national law
    Supremacy,

    /// State liability - Member states liable for damages from EU law violations
    StateLiability,

    /// Mutual recognition - Products lawfully marketed in one MS can be sold in others
    MutualRecognition,

    /// Proportionality - EU action must not exceed what is necessary
    Proportionality,

    /// Legal certainty - Laws must be clear and predictable
    LegalCertainty,
}

/// Landmark CJEU case
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CjeuCase {
    /// Case number (e.g., "C-26/62")
    pub case_number: String,

    /// Case name
    pub name: String,

    /// Year decided
    pub year: u16,

    /// ECLI (European Case Law Identifier) if available
    pub ecli: Option<String>,

    /// Legal principle established
    pub principle: CjeuPrinciple,

    /// Brief summary
    pub summary: String,
}

impl CjeuCase {
    pub fn new(
        case_number: impl Into<String>,
        name: impl Into<String>,
        year: u16,
        principle: CjeuPrinciple,
    ) -> Self {
        Self {
            case_number: case_number.into(),
            name: name.into(),
            year,
            ecli: None,
            principle,
            summary: String::new(),
        }
    }

    pub fn with_ecli(mut self, ecli: impl Into<String>) -> Self {
        self.ecli = Some(ecli.into());
        self
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = summary.into();
        self
    }

    /// Format citation
    pub fn format(&self) -> String {
        format!("{} {} ({})", self.case_number, self.name, self.year)
    }
}

/// Famous CJEU landmark cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LandmarkCase {
    /// Van Gend en Loos (C-26/62) - Direct effect
    VanGendEnLoos,

    /// Costa v ENEL (C-6/64) - Supremacy
    CostaVEnel,

    /// Cassis de Dijon (C-120/78) - Mutual recognition
    CassisDeDijon,

    /// Francovich (C-6/90, C-9/90) - State liability
    Francovich,
}

impl LandmarkCase {
    /// Get case details
    pub fn case(&self) -> CjeuCase {
        match self {
            LandmarkCase::VanGendEnLoos => CjeuCase::new(
                "C-26/62",
                "Van Gend en Loos v Nederlandse Administratie der Belastingen",
                1963,
                CjeuPrinciple::DirectEffect,
            )
            .with_summary(
                "Established direct effect doctrine: EU law creates individual rights \
                 that national courts must protect, even against conflicting national law.",
            ),

            LandmarkCase::CostaVEnel => {
                CjeuCase::new("C-6/64", "Costa v ENEL", 1964, CjeuPrinciple::Supremacy)
                    .with_summary(
                        "Established supremacy doctrine: EU law takes precedence over all \
                 conflicting provisions of national law, including constitutional provisions.",
                    )
            }

            LandmarkCase::CassisDeDijon => CjeuCase::new(
                "C-120/78",
                "Rewe-Zentral AG v Bundesmonopolverwaltung für Branntwein (Cassis de Dijon)",
                1979,
                CjeuPrinciple::MutualRecognition,
            )
            .with_summary(
                "Established mutual recognition principle: Products lawfully marketed in one \
                 Member State can generally be sold in all Member States. Member states cannot \
                 restrict imports unless justified by mandatory requirements.",
            ),

            LandmarkCase::Francovich => CjeuCase::new(
                "C-6/90 and C-9/90",
                "Francovich and Bonifaci v Italy",
                1991,
                CjeuPrinciple::StateLiability,
            )
            .with_summary(
                "Established state liability doctrine: Member States must compensate individuals \
                 for damage caused by failure to implement EU directives correctly.",
            ),
        }
    }

    /// Get case number
    pub fn case_number(&self) -> &str {
        match self {
            LandmarkCase::VanGendEnLoos => "C-26/62",
            LandmarkCase::CostaVEnel => "C-6/64",
            LandmarkCase::CassisDeDijon => "C-120/78",
            LandmarkCase::Francovich => "C-6/90 and C-9/90",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landmark_cases() {
        let van_gend = LandmarkCase::VanGendEnLoos.case();
        assert_eq!(van_gend.case_number, "C-26/62");
        assert_eq!(van_gend.year, 1963);
        assert_eq!(van_gend.principle, CjeuPrinciple::DirectEffect);
        assert!(!van_gend.summary.is_empty());
    }

    #[test]
    fn test_case_formatting() {
        let costa = LandmarkCase::CostaVEnel.case();
        assert_eq!(costa.format(), "C-6/64 Costa v ENEL (1964)");
    }

    #[test]
    fn test_all_landmark_cases() {
        // Ensure all landmark cases can be instantiated
        let _ = LandmarkCase::VanGendEnLoos.case();
        let _ = LandmarkCase::CostaVEnel.case();
        let _ = LandmarkCase::CassisDeDijon.case();
        let _ = LandmarkCase::Francovich.case();
    }
}
