//! Federal Economic Competition Law (Ley Federal de Competencia Económica - LFCE)

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Market concentration (Concentración)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarketConcentration {
    /// Market participants
    pub participantes: Vec<MarketParticipant>,
    /// Relevant market
    pub mercado_relevante: String,
    /// Type of concentration
    pub tipo: ConcentrationType,
}

/// Market participant
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarketParticipant {
    /// Participant name
    pub nombre: String,
    /// Market share (percentage)
    pub participacion_mercado: u8,
}

/// Concentration type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConcentrationType {
    /// Merger (Fusión)
    Merger,
    /// Acquisition (Adquisición)
    Acquisition,
    /// Joint venture (Coinversión)
    JointVenture,
}

/// Anticompetitive practices (Prácticas monopólicas)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnticompetitivePractice {
    /// Absolute monopolistic practices (Article 53)
    Absolute(AbsolutePractice),
    /// Relative monopolistic practices (Article 54)
    Relative(RelativePractice),
}

/// Absolute monopolistic practices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AbsolutePractice {
    /// Price fixing (Fijación de precios)
    PriceFixing,
    /// Market division (División de mercados)
    MarketDivision,
    /// Output restriction (Restricción de producción)
    OutputRestriction,
    /// Bid rigging (Manipulación de licitaciones)
    BidRigging,
}

/// Relative monopolistic practices
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelativePractice {
    /// Vertical price fixing (Fijación vertical de precios)
    VerticalPriceFixing,
    /// Exclusive dealing (Ventas atadas)
    ExclusiveDealing,
    /// Predatory pricing (Precios predatorios)
    PredatoryPricing,
    /// Refusal to deal (Negativa de trato)
    RefusalToDeal,
}

/// Market power threshold
pub mod thresholds {
    /// Substantial market power (Article 13)
    pub const SUBSTANTIAL_POWER_PERCENT: u8 = 50;

    /// Mandatory notification threshold (UMA)
    pub const NOTIFICATION_THRESHOLD_UMA: u64 = 18_000_000;
}

/// Competition law errors
#[derive(Debug, Error)]
pub enum CompetitionError {
    #[error("Anticompetitive practice detected: {0:?}")]
    AnticompetitivePractice(AnticompetitivePractice),
    #[error("Market concentration requires notification")]
    NotificationRequired,
    #[error("Substantial market power: {0}%")]
    SubstantialPower(u8),
}

impl MarketConcentration {
    /// Check if concentration requires COFECE notification
    pub fn requires_notification(&self, transaction_value_uma: u64) -> bool {
        transaction_value_uma >= thresholds::NOTIFICATION_THRESHOLD_UMA
    }

    /// Calculate combined market share
    pub fn combined_market_share(&self) -> u8 {
        self.participantes
            .iter()
            .map(|p| p.participacion_mercado)
            .sum()
    }

    /// Check if concentration creates substantial market power
    pub fn creates_substantial_power(&self) -> bool {
        self.combined_market_share() >= thresholds::SUBSTANTIAL_POWER_PERCENT
    }
}

impl MarketParticipant {
    /// Check if participant has substantial market power
    pub fn has_substantial_power(&self) -> bool {
        self.participacion_mercado >= thresholds::SUBSTANTIAL_POWER_PERCENT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_market_concentration() {
        let concentration = MarketConcentration {
            participantes: vec![
                MarketParticipant {
                    nombre: "Empresa A".to_string(),
                    participacion_mercado: 30,
                },
                MarketParticipant {
                    nombre: "Empresa B".to_string(),
                    participacion_mercado: 25,
                },
            ],
            mercado_relevante: "Telecomunicaciones".to_string(),
            tipo: ConcentrationType::Merger,
        };

        assert_eq!(concentration.combined_market_share(), 55);
        assert!(concentration.creates_substantial_power());
    }

    #[test]
    fn test_notification_requirement() {
        let concentration = MarketConcentration {
            participantes: vec![],
            mercado_relevante: "Test".to_string(),
            tipo: ConcentrationType::Acquisition,
        };

        assert!(concentration.requires_notification(20_000_000));
        assert!(!concentration.requires_notification(10_000_000));
    }
}
