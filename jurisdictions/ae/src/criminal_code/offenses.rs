//! Criminal Offenses under UAE Penal Code

use serde::{Deserialize, Serialize};

/// Common criminal offenses
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CriminalOffense {
    Murder,
    Manslaughter,
    Assault,
    Theft,
    Fraud,
    Embezzlement,
    Bribery,
    Forgery,
    DrugTrafficking,
    MoneyLaundering,
    Defamation,
    Insult,
}

impl CriminalOffense {
    pub fn name_en(&self) -> &'static str {
        match self {
            Self::Murder => "Murder",
            Self::Manslaughter => "Manslaughter",
            Self::Assault => "Assault",
            Self::Theft => "Theft",
            Self::Fraud => "Fraud",
            Self::Embezzlement => "Embezzlement",
            Self::Bribery => "Bribery",
            Self::Forgery => "Forgery",
            Self::DrugTrafficking => "Drug Trafficking",
            Self::MoneyLaundering => "Money Laundering",
            Self::Defamation => "Defamation",
            Self::Insult => "Insult",
        }
    }
}
