//! State Policy Adoption Tracking
//!
//! This module tracks the adoption of key policy areas across US states, providing
//! comprehensive coverage of legislative trends and policy diffusion patterns.
//!
//! # Overview
//!
//! The module tracks three major policy areas that demonstrate significant interstate
//! variation and rapid legislative evolution:
//!
//! ## Cannabis Legalization
//!
//! Cannabis policy has evolved dramatically since 2012, when Colorado and Washington
//! became the first states to legalize recreational use. As of 2024:
//! - **25 states + DC** have legalized recreational cannabis
//! - **16 states** allow medical use only
//! - **2 states** have decriminalized possession
//! - **8 states** maintain full prohibition
//!
//! This represents one of the most significant policy shifts in recent US history,
//! with state-level experimentation leading to federal policy reconsideration.
//!
//! ## Data Privacy Laws
//!
//! Following the EU's GDPR (2018) and California's CCPA (2018), a wave of
//! comprehensive state privacy laws emerged:
//! - **17+ states** have enacted comprehensive privacy legislation
//! - Most laws follow the CCPA/VCDPA model with consumer rights
//! - Common features: right to know, delete, opt-out, non-discrimination
//! - Enforcement mechanisms vary by state
//!
//! The absence of federal privacy legislation has made states the primary
//! regulators of data privacy, creating a complex patchwork of requirements.
//!
//! ## Right to Repair
//!
//! The right to repair movement seeks to ensure consumers and independent repair
//! shops can access tools, parts, and documentation to repair products:
//! - **Massachusetts (2012)**: Pioneer of automotive right to repair
//! - **Electronics**: Recent focus (NY 2022, CA 2023, MN 2023)
//! - **Agricultural**: Farm equipment repair (MN, CO 2023)
//!
//! This movement challenges manufacturer monopolies on repairs and addresses
//! environmental concerns about electronic waste.
//!
//! # Policy Diffusion Patterns
//!
//! These policy areas demonstrate classic patterns of interstate policy diffusion:
//! - **Innovation**: Pioneer states (CO, CA, MA) experiment with new policies
//! - **Learning**: Other states observe outcomes and adapt policies
//! - **Regional Clusters**: Geographically proximate states often adopt similar policies
//! - **Competition**: States compete for residents/businesses through policy choices
//!
//! # Data Sources
//!
//! Policy status reflects enacted legislation as of 2024. For cannabis and privacy,
//! implementation dates are tracked. Future versions may include:
//! - Pending legislation tracking
//! - Enforcement activity monitoring
//! - Policy effectiveness metrics
//! - Interstate coordination mechanisms

use crate::states::types::StateId;
use serde::{Deserialize, Serialize};

/// Cannabis legalization status across jurisdictions
///
///This enum tracks the legal status of cannabis in each US jurisdiction, reflecting
/// the significant policy evolution since 2012. States have adopted four distinct
/// approaches to cannabis regulation:
///
/// # Policy Categories
///
/// ## Recreational Legal
/// Full legalization for adult use (typically 21+). These jurisdictions allow:
/// - Personal possession (usually 1-2 ounces)
/// - Home cultivation (often 6-12 plants)
/// - Licensed retail sales with taxation
/// - Regulated production and distribution
///
/// **Pioneer states**: Colorado and Washington (2012)
/// **Recent adopters**: Ohio, Minnesota, Maryland (2023)
///
/// ## Medical Only
/// Cannabis available with physician recommendation/prescription. Typically includes:
/// - Qualifying medical conditions list
/// - State-issued medical cannabis ID cards
/// - Licensed dispensaries
/// - Restrictions on cultivation and possession
///
/// Programs range from limited (low-THC CBD only) to comprehensive.
///
/// ## Decriminalized
/// Possession remains illegal but penalties reduced to civil fines (like traffic tickets)
/// rather than criminal prosecution. Does not establish legal market.
///
/// ## Fully Illegal
/// Cannabis prohibited for all purposes, with criminal penalties for possession,
/// cultivation, and distribution.
///
/// # Historical Context
///
/// The modern legalization movement began with medical cannabis in the 1990s
/// (California 1996) and accelerated with recreational legalization in 2012.
/// This represents a significant shift from the "War on Drugs" era policies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CannabisStatus {
    /// Recreational use is legal for adults
    ///
    /// The `year_enacted` indicates when the law took effect. Note that voter
    /// approval (via initiative) often precedes implementation by 1-2 years.
    RecreationalLegal {
        /// Year recreational cannabis became legal
        year_enacted: u16,
    },

    /// Medical use only (with prescription/recommendation)
    ///
    /// Includes both comprehensive medical programs and limited CBD-only programs.
    /// The `year_enacted` reflects when medical cannabis first became available.
    MedicalOnly {
        /// Year medical cannabis program established
        year_enacted: u16,
    },

    /// Decriminalized (civil penalty, not criminal)
    ///
    /// Possession penalties reduced to fines rather than criminal prosecution.
    /// Does not establish legal market or cultivation rights.
    Decriminalized {
        /// Year decriminalization took effect
        year_enacted: u16,
    },

    /// Fully illegal under state law
    ///
    /// Cannabis prohibited with criminal penalties. Note that federal prohibition
    /// remains in effect nationwide regardless of state law (Controlled Substances
    /// Act Schedule I classification).
    Illegal,
}

/// Comprehensive state data privacy laws
///
/// This enum tracks comprehensive consumer privacy legislation enacted by US states.
/// Following the EU's GDPR (2018) and California's CCPA (2018), a wave of state
/// privacy laws emerged to fill the federal legislative void.
///
/// # Common Features
///
/// Most state privacy laws share core consumer rights:
/// - **Right to Know**: Access to personal data collected
/// - **Right to Delete**: Deletion of personal data
/// - **Right to Opt-Out**: Opt-out of data sales/sharing
/// - **Non-Discrimination**: No retaliation for exercising rights
///
/// # Applicability Thresholds
///
/// Laws typically apply to businesses meeting revenue/data thresholds:
/// - Annual revenue: $25M-$50M
/// - Consumer data processed: 50,000-100,000+ consumers
/// - Revenue from data sales: 50% threshold (CCPA)
///
/// # Enforcement
///
/// Enforcement mechanisms vary:
/// - **Attorney General enforcement** (most states)
/// - **Private right of action** (California only, limited to data breaches)
/// - **Cure periods**: 30-60 days to remedy violations (many states)
///
/// # CCPA as Model
///
/// The California Consumer Privacy Act (2018) established the template that
/// subsequent state laws largely follow, though with variations in scope,
/// exemptions, and enforcement mechanisms.
///
/// # Notable Variations
///
/// - **CCPA/CPRA** (CA): Broadest scope, strictest requirements, California
///   Privacy Rights Act (CPRA, 2020) enhanced protections
/// - **VCDPA** (VA): First "second generation" law following CCPA model (2021)
/// - **CPA** (CO): Strong consumer rights, universal opt-out requirement
/// - **TDPSA** (TX): Business-friendly, higher thresholds
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DataPrivacyLaw {
    /// California Consumer Privacy Act (CCPA/CPRA, 2018/2020)
    ///
    /// The pioneering state comprehensive privacy law. CPRA (2020) established
    /// the California Privacy Protection Agency and enhanced protections for
    /// sensitive personal information.
    CCPA {
        /// Year CCPA enacted (2018)
        enacted: u16,
        /// Whether enhanced by CPRA (2020)
        cpra_enhanced: bool,
    },

    /// Virginia Consumer Data Protection Act (VCDPA, 2021)
    ///
    /// First state to follow CCPA model. Established "second generation"
    /// template adopted by many subsequent states.
    VCDPA {
        /// Year enacted
        enacted: u16,
    },

    /// Colorado Privacy Act (CPA, 2021)
    ///
    /// Notable for universal opt-out mechanism requirement and strong
    /// consumer rights framework.
    CPA {
        /// Year enacted
        enacted: u16,
    },

    /// Connecticut Data Privacy Act (CTDPA, 2022)
    ///
    /// Follows VCDPA model with some Connecticut-specific variations.
    CTDPA {
        /// Year enacted
        enacted: u16,
    },

    /// Utah Consumer Privacy Act (UCPA, 2022)
    ///
    /// Generally considered more business-friendly with higher thresholds.
    UCPA {
        /// Year enacted
        enacted: u16,
    },

    /// Montana Consumer Data Privacy Act (MCDPA, 2023)
    MCDPA {
        /// Year enacted
        enacted: u16,
    },

    /// Oregon Consumer Privacy Act (OCPA, 2023)
    OCPA {
        /// Year enacted
        enacted: u16,
    },

    /// Texas Data Privacy and Security Act (TDPSA, 2023)
    ///
    /// Business-friendly approach with higher revenue thresholds.
    TDPSA {
        /// Year enacted
        enacted: u16,
    },

    /// Iowa Consumer Data Protection Act (ICDPA, 2023)
    ICDPA {
        /// Year enacted
        enacted: u16,
    },

    /// Tennessee Information Protection Act (TIPA, 2023)
    TIPA {
        /// Year enacted
        enacted: u16,
    },

    /// Delaware Personal Data Privacy Act (DPDPA, 2023)
    DPDPA {
        /// Year enacted
        enacted: u16,
    },

    /// Florida Digital Bill of Rights (FDBR, 2023)
    FDBR {
        /// Year enacted
        enacted: u16,
    },

    /// Indiana Consumer Data Protection Act (INCDPA, 2023)
    INCDPA {
        /// Year enacted
        enacted: u16,
    },

    /// Kentucky Consumer Data Protection Act (KYCDPA, 2024)
    KYCDPA {
        /// Year enacted
        enacted: u16,
    },

    /// Nebraska Data Privacy Act (NDPA, 2024)
    NDPA {
        /// Year enacted
        enacted: u16,
    },

    /// New Hampshire Privacy Act (NHPA, 2024)
    NHPA {
        /// Year enacted
        enacted: u16,
    },

    /// New Jersey Data Protection Act (NJDPA, 2024)
    NJDPA {
        /// Year enacted
        enacted: u16,
    },
}

/// Right to repair legislation status
///
/// The right to repair movement seeks to ensure consumers and independent repair
/// businesses have access to tools, parts, diagnostic equipment, and documentation
/// needed to repair products they own or service.
///
/// # Movement Origins
///
/// The right to repair movement emerged in response to:
/// - Manufacturer restrictions on repair information and tools
/// - "Authorized repair network" monopolies
/// - Product designs preventing independent repair
/// - Environmental concerns about electronic waste
/// - Economic impacts on independent repair businesses
///
/// # Sector-Specific Laws
///
/// Different sectors face distinct repair challenges:
///
/// ## Automotive
/// **Massachusetts (2012)**: Pioneered automotive right to repair via ballot initiative.
/// Requires manufacturers to provide:
/// - On-board diagnostic system information
/// - Repair technical data
/// - Tools for independent repair shops
///
/// Expanded in 2020 to include wireless vehicle data access.
///
/// ## Electronics
/// **Focus**: Smartphones, tablets, computers, appliances
/// **Key requirements**:
/// - Availability of spare parts at fair prices
/// - Access to diagnostic software and firmware
/// - Repair manuals and schematics
/// - Without voiding warranties
///
/// **Recent laws**: New York (2022), California (2023), Minnesota (2023)
///
/// ## Agricultural Equipment
/// **Issue**: Modern tractors and farm equipment use software locks preventing repair
/// **States**: Minnesota (2023), Colorado (2023)
/// **Impact**: Enables farmers to repair expensive equipment rather than relying
/// on distant authorized dealers
///
/// # Industry Opposition
///
/// Manufacturers have opposed right to repair legislation citing:
/// - Intellectual property concerns
/// - Safety and security risks
/// - Quality control issues
/// - Business model disruption
///
/// # Environmental Impact
///
/// Right to repair addresses e-waste by:
/// - Extending product lifespans
/// - Reducing premature disposal
/// - Enabling component-level repair vs. full replacement
/// - Supporting circular economy principles
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RightToRepairStatus {
    /// Comprehensive right to repair law covering multiple sectors
    ///
    /// State has enacted legislation providing broad repair rights across
    /// one or more product categories.
    Comprehensive {
        /// Electronics (phones, computers, appliances) covered
        electronics: bool,
        /// Automotive vehicles covered
        automotive: bool,
        /// Agricultural equipment (tractors, farm machinery) covered
        agricultural: bool,
        /// Year primary legislation enacted
        year_enacted: u16,
    },

    /// Limited right to repair (specific sectors only)
    ///
    /// State has enacted narrow legislation or regulations covering specific
    /// product categories or use cases.
    Limited {
        /// Electronics covered
        electronics: bool,
        /// Automotive covered
        automotive: bool,
        /// Agricultural equipment covered
        agricultural: bool,
        /// Year legislation enacted
        year_enacted: u16,
    },

    /// No right to repair law
    ///
    /// State has not enacted right to repair legislation. Manufacturers retain
    /// control over repair markets. Note that some repairs may still be possible
    /// under federal law (Magnuson-Moss Warranty Act) or common law principles.
    None,
}

/// Policy adoption tracker for a jurisdiction
///
/// Aggregates all policy adoption status for a single jurisdiction, providing
/// a comprehensive view of that state's position on key policy issues.
///
/// # Usage
///
/// This structure is useful for:
/// - Comparing policy positions across states
/// - Tracking policy adoption trends over time
/// - Identifying policy clusters or patterns
/// - Analyzing policy diffusion mechanisms
///
/// # Example
///
/// ```rust
/// use legalis_us::legislative::policy_tracker::*;
/// use legalis_us::states::types::StateId;
///
/// // Create tracker for a progressive state
/// let ca_tracker = PolicyAdoptionTracker {
///     state_id: StateId::from_code("CA"),
///     cannabis: CannabisStatus::RecreationalLegal { year_enacted: 2016 },
///     privacy_laws: vec![
///         DataPrivacyLaw::CCPA { enacted: 2018, cpra_enhanced: true }
///     ],
///     right_to_repair: RightToRepairStatus::Comprehensive {
///         electronics: true,
///         automotive: false,
///         agricultural: false,
///         year_enacted: 2023,
///     },
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PolicyAdoptionTracker {
    /// State identifier
    pub state_id: StateId,

    /// Cannabis legalization status
    ///
    /// Tracks whether the state has legalized recreational use, allows medical
    /// use only, has decriminalized possession, or maintains full prohibition.
    pub cannabis: CannabisStatus,

    /// Comprehensive data privacy laws enacted
    ///
    /// Vector of all comprehensive privacy laws. Empty for states without
    /// comprehensive privacy legislation. Most states have 0-1 laws; multiple
    /// laws indicate amendments or supplementary legislation.
    pub privacy_laws: Vec<DataPrivacyLaw>,

    /// Right to repair legislation status
    ///
    /// Indicates whether the state has enacted comprehensive, limited, or no
    /// right to repair legislation.
    pub right_to_repair: RightToRepairStatus,
}

/// Get cannabis legalization status for a state
///
/// # Example
/// ```
/// use legalis_us::legislative::policy_tracker::{cannabis_status, CannabisStatus};
///
/// let ca = cannabis_status("CA");
/// assert_eq!(ca, CannabisStatus::RecreationalLegal { year_enacted: 2016 });
/// ```
pub fn cannabis_status(state_code: &str) -> CannabisStatus {
    match state_code {
        // Recreational Legal (24 states + DC as of 2024)
        "AK" => CannabisStatus::RecreationalLegal { year_enacted: 2014 },
        "AZ" => CannabisStatus::RecreationalLegal { year_enacted: 2020 },
        "CA" => CannabisStatus::RecreationalLegal { year_enacted: 2016 },
        "CO" => CannabisStatus::RecreationalLegal { year_enacted: 2012 },
        "CT" => CannabisStatus::RecreationalLegal { year_enacted: 2021 },
        "DC" => CannabisStatus::RecreationalLegal { year_enacted: 2014 },
        "DE" => CannabisStatus::RecreationalLegal { year_enacted: 2023 },
        "IL" => CannabisStatus::RecreationalLegal { year_enacted: 2019 },
        "ME" => CannabisStatus::RecreationalLegal { year_enacted: 2016 },
        "MD" => CannabisStatus::RecreationalLegal { year_enacted: 2023 },
        "MA" => CannabisStatus::RecreationalLegal { year_enacted: 2016 },
        "MI" => CannabisStatus::RecreationalLegal { year_enacted: 2018 },
        "MN" => CannabisStatus::RecreationalLegal { year_enacted: 2023 },
        "MO" => CannabisStatus::RecreationalLegal { year_enacted: 2022 },
        "MT" => CannabisStatus::RecreationalLegal { year_enacted: 2020 },
        "NJ" => CannabisStatus::RecreationalLegal { year_enacted: 2021 },
        "NM" => CannabisStatus::RecreationalLegal { year_enacted: 2021 },
        "NV" => CannabisStatus::RecreationalLegal { year_enacted: 2016 },
        "NY" => CannabisStatus::RecreationalLegal { year_enacted: 2021 },
        "OH" => CannabisStatus::RecreationalLegal { year_enacted: 2023 },
        "OR" => CannabisStatus::RecreationalLegal { year_enacted: 2014 },
        "RI" => CannabisStatus::RecreationalLegal { year_enacted: 2022 },
        "VT" => CannabisStatus::RecreationalLegal { year_enacted: 2018 },
        "VA" => CannabisStatus::RecreationalLegal { year_enacted: 2021 },
        "WA" => CannabisStatus::RecreationalLegal { year_enacted: 2012 },

        // Medical Only (16 states - comprehensive medical programs)
        "AL" => CannabisStatus::MedicalOnly { year_enacted: 2021 },
        "AR" => CannabisStatus::MedicalOnly { year_enacted: 2016 },
        "FL" => CannabisStatus::MedicalOnly { year_enacted: 2016 },
        "HI" => CannabisStatus::MedicalOnly { year_enacted: 2000 },
        "LA" => CannabisStatus::MedicalOnly { year_enacted: 2015 },
        "MS" => CannabisStatus::MedicalOnly { year_enacted: 2022 },
        "ND" => CannabisStatus::MedicalOnly { year_enacted: 2016 },
        "NH" => CannabisStatus::MedicalOnly { year_enacted: 2013 },
        "OK" => CannabisStatus::MedicalOnly { year_enacted: 2018 },
        "PA" => CannabisStatus::MedicalOnly { year_enacted: 2016 },
        "SD" => CannabisStatus::MedicalOnly { year_enacted: 2020 },
        "TX" => CannabisStatus::MedicalOnly { year_enacted: 2015 }, // Limited
        "UT" => CannabisStatus::MedicalOnly { year_enacted: 2018 },
        "WV" => CannabisStatus::MedicalOnly { year_enacted: 2017 },
        "WI" => CannabisStatus::MedicalOnly { year_enacted: 2017 }, // CBD only
        "WY" => CannabisStatus::MedicalOnly { year_enacted: 2015 }, // CBD only

        // Decriminalized (possession decriminalized but not fully legal)
        "NC" => CannabisStatus::Decriminalized { year_enacted: 1977 },
        "NE" => CannabisStatus::Decriminalized { year_enacted: 1978 },

        // Fully Illegal (8 states)
        "GA" | "ID" | "IN" | "IA" | "KS" | "KY" | "SC" | "TN" => CannabisStatus::Illegal,

        _ => CannabisStatus::Illegal,
    }
}

/// Check if state has comprehensive data privacy law
pub fn has_comprehensive_privacy_law(state_code: &str) -> bool {
    !comprehensive_privacy_laws(state_code).is_empty()
}

/// Get comprehensive data privacy laws for a state
///
/// Returns all comprehensive privacy laws enacted by the state.
/// Does not include sector-specific laws (e.g., biometric, health data).
pub fn comprehensive_privacy_laws(state_code: &str) -> Vec<DataPrivacyLaw> {
    match state_code {
        "CA" => vec![DataPrivacyLaw::CCPA {
            enacted: 2018,
            cpra_enhanced: true, // CPRA amendments 2020
        }],
        "VA" => vec![DataPrivacyLaw::VCDPA { enacted: 2021 }],
        "CO" => vec![DataPrivacyLaw::CPA { enacted: 2021 }],
        "CT" => vec![DataPrivacyLaw::CTDPA { enacted: 2022 }],
        "UT" => vec![DataPrivacyLaw::UCPA { enacted: 2022 }],
        "MT" => vec![DataPrivacyLaw::MCDPA { enacted: 2023 }],
        "OR" => vec![DataPrivacyLaw::OCPA { enacted: 2023 }],
        "TX" => vec![DataPrivacyLaw::TDPSA { enacted: 2023 }],
        "IA" => vec![DataPrivacyLaw::ICDPA { enacted: 2023 }],
        "TN" => vec![DataPrivacyLaw::TIPA { enacted: 2023 }],
        "DE" => vec![DataPrivacyLaw::DPDPA { enacted: 2023 }],
        "FL" => vec![DataPrivacyLaw::FDBR { enacted: 2023 }],
        "IN" => vec![DataPrivacyLaw::INCDPA { enacted: 2023 }],
        "KY" => vec![DataPrivacyLaw::KYCDPA { enacted: 2024 }],
        "NE" => vec![DataPrivacyLaw::NDPA { enacted: 2024 }],
        "NH" => vec![DataPrivacyLaw::NHPA { enacted: 2024 }],
        "NJ" => vec![DataPrivacyLaw::NJDPA { enacted: 2024 }],
        _ => vec![],
    }
}

/// Get right to repair status for a state
pub fn right_to_repair_status(state_code: &str) -> RightToRepairStatus {
    match state_code {
        // Comprehensive right to repair laws
        "MA" => RightToRepairStatus::Comprehensive {
            electronics: true,
            automotive: true,
            agricultural: false,
            year_enacted: 2012, // Automotive (expanded 2020)
        },
        "NY" => RightToRepairStatus::Comprehensive {
            electronics: true,
            automotive: false,
            agricultural: false,
            year_enacted: 2022, // Electronics
        },
        "CA" => RightToRepairStatus::Comprehensive {
            electronics: true,
            automotive: false,
            agricultural: false,
            year_enacted: 2023, // Electronics (SB 244)
        },
        "MN" => RightToRepairStatus::Comprehensive {
            electronics: true,
            automotive: false,
            agricultural: true,
            year_enacted: 2023, // Electronics and agricultural
        },
        "CO" => RightToRepairStatus::Comprehensive {
            electronics: true,
            automotive: false,
            agricultural: true,
            year_enacted: 2023, // Agricultural equipment (HB 1011)
        },

        // Limited right to repair
        "OR" => RightToRepairStatus::Limited {
            electronics: true,
            automotive: false,
            agricultural: false,
            year_enacted: 2023,
        },

        // No right to repair law
        _ => RightToRepairStatus::None,
    }
}

/// Get list of states with recreational cannabis
pub fn states_with_recreational_cannabis() -> Vec<&'static str> {
    vec![
        "AK", "AZ", "CA", "CO", "CT", "DC", "DE", "IL", "ME", "MD", "MA", "MI", "MN", "MO", "MT",
        "NJ", "NM", "NV", "NY", "OH", "OR", "RI", "VT", "VA", "WA",
    ]
}

/// Get list of states with comprehensive privacy laws
pub fn states_with_privacy_laws() -> Vec<&'static str> {
    vec![
        "CA", "CO", "CT", "DE", "FL", "IA", "IN", "KY", "MT", "NE", "NH", "NJ", "OR", "TN", "TX",
        "UT", "VA",
    ]
}

/// Get list of states with right to repair laws
pub fn states_with_right_to_repair() -> Vec<&'static str> {
    vec!["CA", "CO", "MA", "MN", "NY", "OR"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cannabis_recreational_states() {
        assert_eq!(
            cannabis_status("CA"),
            CannabisStatus::RecreationalLegal { year_enacted: 2016 }
        );
        assert_eq!(
            cannabis_status("CO"),
            CannabisStatus::RecreationalLegal { year_enacted: 2012 }
        );
        assert_eq!(
            cannabis_status("WA"),
            CannabisStatus::RecreationalLegal { year_enacted: 2012 }
        );
    }

    #[test]
    fn test_cannabis_medical_only() {
        assert_eq!(
            cannabis_status("FL"),
            CannabisStatus::MedicalOnly { year_enacted: 2016 }
        );
        assert_eq!(
            cannabis_status("TX"),
            CannabisStatus::MedicalOnly { year_enacted: 2015 }
        );
    }

    #[test]
    fn test_cannabis_decriminalized() {
        assert_eq!(
            cannabis_status("NC"),
            CannabisStatus::Decriminalized { year_enacted: 1977 }
        );
    }

    #[test]
    fn test_cannabis_illegal() {
        assert_eq!(cannabis_status("ID"), CannabisStatus::Illegal);
        assert_eq!(cannabis_status("KS"), CannabisStatus::Illegal);
    }

    #[test]
    fn test_cannabis_count_recreational() {
        let recreational_states = states_with_recreational_cannabis();
        assert_eq!(recreational_states.len(), 25); // 24 states + DC
    }

    #[test]
    fn test_comprehensive_privacy_law_california() {
        assert!(has_comprehensive_privacy_law("CA"));
        let ca_laws = comprehensive_privacy_laws("CA");
        assert_eq!(ca_laws.len(), 1);
        assert!(matches!(
            ca_laws[0],
            DataPrivacyLaw::CCPA {
                enacted: 2018,
                cpra_enhanced: true
            }
        ));
    }

    #[test]
    fn test_comprehensive_privacy_law_virginia() {
        assert!(has_comprehensive_privacy_law("VA"));
        let va_laws = comprehensive_privacy_laws("VA");
        assert_eq!(va_laws.len(), 1);
        assert!(matches!(
            va_laws[0],
            DataPrivacyLaw::VCDPA { enacted: 2021 }
        ));
    }

    #[test]
    fn test_no_comprehensive_privacy_law() {
        assert!(!has_comprehensive_privacy_law("AL"));
        assert!(!has_comprehensive_privacy_law("AK"));
        assert!(comprehensive_privacy_laws("AL").is_empty());
    }

    #[test]
    fn test_privacy_law_count() {
        let privacy_states = states_with_privacy_laws();
        assert!(privacy_states.len() >= 17); // At least 17 states as of 2024
    }

    #[test]
    fn test_right_to_repair_comprehensive() {
        let ma = right_to_repair_status("MA");
        assert!(matches!(ma, RightToRepairStatus::Comprehensive { .. }));

        if let RightToRepairStatus::Comprehensive {
            electronics,
            automotive,
            ..
        } = ma
        {
            assert!(automotive); // MA has automotive right to repair
            assert!(electronics);
        }
    }

    #[test]
    fn test_right_to_repair_ny() {
        let ny = right_to_repair_status("NY");
        assert!(matches!(ny, RightToRepairStatus::Comprehensive { .. }));

        if let RightToRepairStatus::Comprehensive {
            electronics,
            year_enacted,
            ..
        } = ny
        {
            assert!(electronics);
            assert_eq!(year_enacted, 2022);
        }
    }

    #[test]
    fn test_right_to_repair_california() {
        let ca = right_to_repair_status("CA");
        assert!(matches!(ca, RightToRepairStatus::Comprehensive { .. }));
    }

    #[test]
    fn test_right_to_repair_none() {
        assert_eq!(right_to_repair_status("TX"), RightToRepairStatus::None);
        assert_eq!(right_to_repair_status("FL"), RightToRepairStatus::None);
    }

    #[test]
    fn test_right_to_repair_count() {
        let repair_states = states_with_right_to_repair();
        assert_eq!(repair_states.len(), 6); // 6 states with right to repair laws
    }
}
