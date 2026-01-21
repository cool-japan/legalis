//! UK Land Law - Estates Module
//!
//! This module provides analysis of estates in land under English law:
//! - Freehold estates (fee simple absolute in possession)
//! - Leasehold estates (term of years absolute)
//! - Lease vs licence distinction (Street v Mountford)
//! - LTA 1954 business tenancy protection
//!
//! Key statutes:
//! - Law of Property Act 1925 (s.1 - legal estates)
//! - Landlord and Tenant Act 1954 Part II (business tenancies)
//!
//! Key cases:
//! - Street v Mountford \[1985\] AC 809 (lease vs licence)
//! - Lace v Chantler \[1944\] KB 368 (certainty of term)
//! - Prudential Assurance v London Residuary Body \[1992\] (periodic tenancy)

#![allow(missing_docs)]

use serde::{Deserialize, Serialize};

use super::types::{
    CoOwnershipType, FreeholdEstate, LandLawCase, LeaseDuration, LeaseholdEstate, TitleClass,
};

// ============================================================================
// Lease vs Licence Analyzer (Street v Mountford)
// ============================================================================

/// Facts for lease/licence analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseOrLicenceFacts {
    /// Description of the arrangement
    pub description: String,
    /// Has exclusive possession of defined space
    pub exclusive_possession: bool,
    /// Term is certain/ascertainable
    pub certain_term: bool,
    /// Rent or other valuable consideration
    pub rent_payable: bool,
    /// Shared occupation with others chosen by grantor
    pub shared_occupation: bool,
    /// Grantor retains general access rights
    pub grantor_access: bool,
    /// Service element (hotel, lodging)
    pub service_element: bool,
    /// Label used in agreement
    pub label_used: String,
    /// Any sham arrangement indicators
    pub sham_indicators: Vec<String>,
}

/// Result of lease/licence analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseOrLicenceResult {
    pub is_lease: bool,
    pub exclusive_possession_found: bool,
    pub term_certain: bool,
    pub rent_requirement_met: bool,
    pub exceptions_apply: Vec<LicenceException>,
    pub reasoning: String,
    pub key_cases: Vec<LandLawCase>,
}

/// Exceptions to lease finding
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LicenceException {
    /// No intention to create legal relations
    NoIntention,
    /// Service occupancy (employee)
    ServiceOccupancy,
    /// Acts of friendship or generosity
    Generosity,
    /// Shared occupation negating exclusivity
    SharedOccupation,
    /// Genuine lodger arrangement
    Lodger,
}

/// Analyzes whether arrangement is lease or licence
pub struct LeaseOrLicenceAnalyzer;

impl LeaseOrLicenceAnalyzer {
    /// Analyze whether arrangement creates lease or licence
    pub fn analyze(facts: &LeaseOrLicenceFacts) -> LeaseOrLicenceResult {
        let mut key_cases = vec![LandLawCase::street_v_mountford()];
        let mut exceptions = Vec::new();

        // Check exclusive possession (primary test)
        let exclusive_possession_found =
            facts.exclusive_possession && !facts.shared_occupation && !facts.grantor_access;

        // Check certain term
        let term_certain = facts.certain_term;

        // Check rent (consideration)
        let rent_requirement_met = facts.rent_payable;

        // Check for exceptions
        if facts.service_element {
            exceptions.push(LicenceException::Lodger);
        }
        if facts.shared_occupation {
            exceptions.push(LicenceException::SharedOccupation);
        }

        // Determine if lease
        let is_lease = exclusive_possession_found && term_certain && exceptions.is_empty();

        let reasoning = if is_lease {
            format!(
                "Following Street v Mountford, this arrangement creates a lease. \
                 The occupier has exclusive possession of defined premises for a \
                 certain term at a rent. The label '{}' is not determinative; \
                 substance prevails over form.",
                facts.label_used
            )
        } else if !exclusive_possession_found {
            "This is a licence, not a lease. The grantor has retained general \
             access rights or the occupier shares with others chosen by grantor, \
             negating exclusive possession (AG Securities v Vaughan)."
                .into()
        } else if !term_certain {
            key_cases.push(LandLawCase {
                name: "Lace v Chantler".into(),
                citation: "[1944] KB 368".into(),
                year: 1944,
                principle: "A lease must have a certain term or one capable of \
                    being rendered certain."
                    .into(),
            });
            "This cannot be a lease as the term is uncertain (Lace v Chantler). \
             A lease requires a term that is certain at commencement."
                .into()
        } else {
            format!(
                "Although exclusive possession exists, an exception applies: {:?}. \
                 This arrangement is therefore a licence.",
                exceptions
            )
        };

        LeaseOrLicenceResult {
            is_lease,
            exclusive_possession_found,
            term_certain,
            rent_requirement_met,
            exceptions_apply: exceptions,
            reasoning,
            key_cases,
        }
    }
}

// ============================================================================
// Freehold Analyzer
// ============================================================================

/// Facts for freehold analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeholdFacts {
    /// Property details
    pub property: FreeholdEstate,
    /// Type of acquisition (purchase, gift, inheritance)
    pub acquisition_type: AcquisitionType,
    /// Consideration paid (pence)
    pub consideration_pence: Option<u64>,
    /// Any covenants affecting
    pub restrictive_covenants: Vec<String>,
    /// Any easements benefiting
    pub easements_benefiting: Vec<String>,
    /// Any easements burdening
    pub easements_burdening: Vec<String>,
}

/// Type of acquisition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquisitionType {
    /// Purchase for value
    Purchase,
    /// Gift (voluntary transfer)
    Gift,
    /// Inheritance (assent from PR)
    Inheritance,
    /// Adverse possession
    AdversePossession,
}

/// Result of freehold analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FreeholdAnalysisResult {
    pub valid_estate: bool,
    pub title_quality: TitleQuality,
    pub registration_required: bool,
    pub encumbrances: Vec<String>,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
}

/// Quality of title
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TitleQuality {
    /// Good and marketable title
    Good,
    /// Minor defects but acceptable
    Acceptable,
    /// Defective title
    Defective,
    /// Title doubtful
    Doubtful,
}

/// Analyzes freehold estates
pub struct FreeholdAnalyzer;

impl FreeholdAnalyzer {
    /// Analyze freehold estate
    pub fn analyze(facts: &FreeholdFacts) -> FreeholdAnalysisResult {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut encumbrances = Vec::new();

        // Check registration status
        let registration_required = !facts.property.registered;
        if registration_required {
            recommendations
                .push("First registration required within 2 months of triggering event".into());
        }

        // Check title class
        let title_quality = match facts.property.title_class {
            Some(TitleClass::Absolute) => TitleQuality::Good,
            Some(TitleClass::GoodLeasehold) => TitleQuality::Good,
            Some(TitleClass::Qualified) => {
                issues.push("Qualified title - check nature of qualification".into());
                TitleQuality::Acceptable
            }
            Some(TitleClass::Possessory) => {
                issues.push("Possessory title only - risk of superior claim".into());
                TitleQuality::Defective
            }
            None if !facts.property.registered => TitleQuality::Acceptable,
            None => TitleQuality::Doubtful,
        };

        // Check co-ownership
        if facts.property.owners.len() > 1 {
            match facts.property.co_ownership {
                Some(CoOwnershipType::JointTenancy) => {
                    recommendations
                        .push("Joint tenancy - consider whether severance appropriate".into());
                }
                Some(CoOwnershipType::TenancyInCommon) => {
                    recommendations
                        .push("Tenancy in common - ensure beneficial shares are recorded".into());
                }
                None => {
                    issues.push("Multiple owners but co-ownership type not specified".into());
                }
            }
        }

        // Record encumbrances
        for covenant in &facts.restrictive_covenants {
            encumbrances.push(format!("Restrictive covenant: {covenant}"));
        }
        for easement in &facts.easements_burdening {
            encumbrances.push(format!("Easement burdening: {easement}"));
        }

        let valid_estate = issues.is_empty() || title_quality != TitleQuality::Doubtful;

        FreeholdAnalysisResult {
            valid_estate,
            title_quality,
            registration_required,
            encumbrances,
            issues,
            recommendations,
        }
    }
}

// ============================================================================
// Leasehold Analyzer
// ============================================================================

/// Facts for leasehold analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseholdFacts {
    /// Lease details
    pub lease: LeaseholdEstate,
    /// Tenant's use
    pub use_type: LeaseUseType,
    /// Alterations requested
    pub alterations_proposed: bool,
    /// Assignment or subletting proposed
    pub assignment_proposed: bool,
    /// Rent arrears (pence)
    pub rent_arrears_pence: u64,
    /// Breach of covenant alleged
    pub breach_alleged: Option<String>,
    /// Notice to quit served
    pub notice_served: bool,
}

/// Type of lease use
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseUseType {
    /// Residential
    Residential,
    /// Business/Commercial
    Business,
    /// Agricultural
    Agricultural,
    /// Mixed use
    Mixed,
}

/// Result of leasehold analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseholdAnalysisResult {
    pub valid_lease: bool,
    pub years_remaining: Option<u32>,
    pub lta_1954_protected: bool,
    pub security_of_tenure: bool,
    pub forfeiture_risk: ForfeitureRisk,
    pub issues: Vec<String>,
    pub recommendations: Vec<String>,
    pub key_cases: Vec<LandLawCase>,
}

/// Risk of forfeiture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ForfeitureRisk {
    /// No forfeiture risk
    None,
    /// Low risk
    Low,
    /// Medium risk - s.146 notice likely
    Medium,
    /// High risk - imminent forfeiture
    High,
}

/// Analyzes leasehold estates
pub struct LeaseholdAnalyzer;

impl LeaseholdAnalyzer {
    /// Analyze leasehold estate
    pub fn analyze(facts: &LeaseholdFacts) -> LeaseholdAnalysisResult {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let mut key_cases = Vec::new();

        // Determine years remaining
        let years_remaining = Self::calculate_years_remaining(&facts.lease.duration);

        // Check if short lease
        if let Some(years) = years_remaining
            && years < 80
            && facts.use_type == LeaseUseType::Residential
        {
            issues.push("Short lease - marriage value may apply".into());
            recommendations.push("Consider lease extension under Leasehold Reform Act 1993".into());
        }

        // LTA 1954 analysis for business tenancies
        let lta_1954_protected = facts.use_type == LeaseUseType::Business
            && facts.lease.lta_1954_protected
            && !facts.lease.contracted_out;

        let security_of_tenure = lta_1954_protected;

        if lta_1954_protected {
            recommendations.push("LTA 1954 Part II applies - tenant has security of tenure".into());
        } else if facts.use_type == LeaseUseType::Business && facts.lease.contracted_out {
            issues.push("Tenancy contracted out of LTA 1954 - no security of tenure".into());
        }

        // Forfeiture risk analysis
        let forfeiture_risk = Self::assess_forfeiture_risk(facts);

        if forfeiture_risk == ForfeitureRisk::High {
            issues.push("High forfeiture risk - landlord may seek to forfeit".into());
            recommendations.push("Seek legal advice urgently on relief from forfeiture".into());
        }

        // Validate lease
        let valid_lease = match &facts.lease.duration {
            LeaseDuration::Fixed { years, .. } => *years > 0,
            LeaseDuration::Periodic(_) => true,
            LeaseDuration::AtWill | LeaseDuration::AtSufferance => true,
        };

        if !valid_lease {
            issues.push("Lease term is uncertain or zero".into());
            key_cases.push(LandLawCase {
                name: "Lace v Chantler".into(),
                citation: "[1944] KB 368".into(),
                year: 1944,
                principle: "A lease must have a certain term".into(),
            });
        }

        LeaseholdAnalysisResult {
            valid_lease,
            years_remaining,
            lta_1954_protected,
            security_of_tenure,
            forfeiture_risk,
            issues,
            recommendations,
            key_cases,
        }
    }

    fn calculate_years_remaining(duration: &LeaseDuration) -> Option<u32> {
        match duration {
            LeaseDuration::Fixed { years, .. } => Some(*years),
            LeaseDuration::Periodic(_) => None, // Continues indefinitely
            LeaseDuration::AtWill | LeaseDuration::AtSufferance => None,
        }
    }

    fn assess_forfeiture_risk(facts: &LeaseholdFacts) -> ForfeitureRisk {
        // High risk if significant arrears or serious breach
        if facts.rent_arrears_pence > 1_000_000 {
            // Over £10,000
            return ForfeitureRisk::High;
        }

        if facts.breach_alleged.is_some() && facts.notice_served {
            return ForfeitureRisk::High;
        }

        if facts.breach_alleged.is_some() {
            return ForfeitureRisk::Medium;
        }

        if facts.rent_arrears_pence > 0 {
            return ForfeitureRisk::Low;
        }

        ForfeitureRisk::None
    }
}

// ============================================================================
// Forfeiture Analyzer
// ============================================================================

/// Facts for forfeiture analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForfeitureFacts {
    /// Type of breach
    pub breach_type: BreachType,
    /// Section 146 notice served
    pub section_146_notice: bool,
    /// Reasonable time to remedy
    pub time_to_remedy_given: bool,
    /// Breach remedied
    pub breach_remedied: bool,
    /// Tenant applying for relief
    pub relief_sought: bool,
    /// Any waiver by landlord
    pub waiver_occurred: bool,
    /// Residential premises
    pub residential: bool,
}

/// Type of lease breach
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BreachType {
    /// Non-payment of rent
    RentArrears { amount_pence: u64 },
    /// Breach of repair covenant
    Disrepair,
    /// Breach of user covenant
    UserBreach,
    /// Breach of alienation covenant
    AlienationBreach,
    /// Immoral/illegal use
    ImmoralUse,
    /// Other breach
    Other { description: String },
}

/// Result of forfeiture analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForfeitureAnalysisResult {
    pub forfeiture_available: bool,
    pub section_146_required: bool,
    pub section_146_complied: bool,
    pub relief_likely: ReliefLikelihood,
    pub waiver_bars_forfeiture: bool,
    pub reasoning: String,
    pub recommendations: Vec<String>,
}

/// Likelihood of relief from forfeiture
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReliefLikelihood {
    /// Relief very likely
    VeryLikely,
    /// Relief likely
    Likely,
    /// Relief uncertain
    Uncertain,
    /// Relief unlikely
    Unlikely,
    /// Relief very unlikely
    VeryUnlikely,
}

/// Analyzes forfeiture claims
pub struct ForfeitureAnalyzer;

impl ForfeitureAnalyzer {
    /// Analyze forfeiture claim
    pub fn analyze(facts: &ForfeitureFacts) -> ForfeitureAnalysisResult {
        let mut recommendations = Vec::new();

        // Check for waiver
        let waiver_bars_forfeiture = facts.waiver_occurred;
        if waiver_bars_forfeiture {
            return ForfeitureAnalysisResult {
                forfeiture_available: false,
                section_146_required: false,
                section_146_complied: false,
                relief_likely: ReliefLikelihood::VeryLikely,
                waiver_bars_forfeiture: true,
                reasoning: "Forfeiture has been waived. Landlord's conduct (e.g., \
                    accepting rent with knowledge of breach) constitutes waiver \
                    of the right to forfeit."
                    .into(),
                recommendations: vec!["New breach would be required to pursue forfeiture".into()],
            };
        }

        // Determine if s.146 required (not for rent arrears)
        let section_146_required = !matches!(facts.breach_type, BreachType::RentArrears { .. });

        // Check s.146 compliance
        let section_146_complied =
            !section_146_required || (facts.section_146_notice && facts.time_to_remedy_given);

        // Assess relief likelihood
        let relief_likely = Self::assess_relief_likelihood(facts);

        // Can forfeit?
        let forfeiture_available = !waiver_bars_forfeiture
            && (!section_146_required || section_146_complied)
            && !facts.breach_remedied;

        let reasoning = if forfeiture_available {
            if section_146_required {
                "Forfeiture available. Section 146 LPA 1925 notice has been served \
                 specifying the breach, requiring remedy if capable of remedy, and \
                 requiring compensation. Tenant may apply for relief."
                    .into()
            } else {
                "Forfeiture available for non-payment of rent. Formal demand not \
                 required if lease waives it. Common Law Procedure Act 1852 s.210 \
                 relief available if arrears paid within specified period."
                    .into()
            }
        } else if facts.breach_remedied {
            "Forfeiture not available - breach has been remedied.".into()
        } else if section_146_required && !section_146_complied {
            recommendations.push("Serve valid s.146 notice before proceeding".into());
            "Forfeiture not yet available - section 146 notice requirements not met.".into()
        } else {
            "Forfeiture not available due to waiver or other bar.".into()
        };

        // Residential property protection
        if facts.residential && forfeiture_available {
            recommendations
                .push("Protection from Eviction Act 1977 applies - court order required".into());
        }

        ForfeitureAnalysisResult {
            forfeiture_available,
            section_146_required,
            section_146_complied,
            relief_likely,
            waiver_bars_forfeiture,
            reasoning,
            recommendations,
        }
    }

    fn assess_relief_likelihood(facts: &ForfeitureFacts) -> ReliefLikelihood {
        // Relief from forfeiture - equity's intervention
        if facts.breach_remedied {
            return ReliefLikelihood::VeryLikely;
        }

        match &facts.breach_type {
            BreachType::RentArrears { amount_pence } => {
                // Relief usual if arrears paid
                if *amount_pence < 500_000 {
                    // Under £5,000
                    ReliefLikelihood::VeryLikely
                } else {
                    ReliefLikelihood::Likely
                }
            }
            BreachType::Disrepair => {
                // Relief usually granted if tenant remedies
                ReliefLikelihood::Likely
            }
            BreachType::ImmoralUse => {
                // Courts reluctant to grant relief
                ReliefLikelihood::Unlikely
            }
            _ => ReliefLikelihood::Uncertain,
        }
    }
}

// ============================================================================
// LTA 1954 Analyzer
// ============================================================================

/// Facts for LTA 1954 analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lta1954Facts {
    /// Is a business tenancy
    pub is_business_tenancy: bool,
    /// Tenant occupies for business purposes
    pub occupation_for_business: bool,
    /// Contracted out validly
    pub contracted_out: bool,
    /// Section 25 notice served by landlord
    pub section_25_notice: bool,
    /// Section 26 request served by tenant
    pub section_26_request: bool,
    /// Ground for opposition (if any)
    pub opposition_ground: Option<Lta1954Ground>,
}

/// Grounds for opposing new tenancy (LTA 1954 s.30)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lta1954Ground {
    /// (a) Breach of repair obligation
    GroundA,
    /// (b) Persistent delay in paying rent
    GroundB,
    /// (c) Other substantial breach
    GroundC,
    /// (d) Suitable alternative accommodation
    GroundD,
    /// (e) Uneconomic subdivision
    GroundE,
    /// (f) Landlord intends to demolish/reconstruct
    GroundF,
    /// (g) Landlord intends own occupation
    GroundG,
}

/// Result of LTA 1954 analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lta1954Result {
    pub protection_applies: bool,
    pub new_tenancy_available: bool,
    pub compensation_available: bool,
    pub ground_likely_to_succeed: Option<bool>,
    pub reasoning: String,
    pub key_points: Vec<String>,
}

/// Analyzes LTA 1954 Part II claims
pub struct Lta1954Analyzer;

impl Lta1954Analyzer {
    /// Analyze LTA 1954 application
    pub fn analyze(facts: &Lta1954Facts) -> Lta1954Result {
        let mut key_points = Vec::new();

        // Check if protection applies
        let protection_applies =
            facts.is_business_tenancy && facts.occupation_for_business && !facts.contracted_out;

        if !protection_applies {
            return Lta1954Result {
                protection_applies: false,
                new_tenancy_available: false,
                compensation_available: false,
                ground_likely_to_succeed: None,
                reasoning: "LTA 1954 Part II protection does not apply. Either not a \
                    business tenancy, tenant not in occupation for business purposes, \
                    or tenancy validly contracted out."
                    .into(),
                key_points: vec!["No security of tenure".into()],
            };
        }

        key_points.push("Tenant has security of tenure under LTA 1954 Part II".into());

        // Analyze opposition ground
        let (ground_likely, compensation) = if let Some(ground) = &facts.opposition_ground {
            let likely = Self::assess_ground_success(ground);
            let comp = matches!(
                ground,
                Lta1954Ground::GroundE | Lta1954Ground::GroundF | Lta1954Ground::GroundG
            );
            key_points.push(format!("Landlord opposing on ground {:?}", ground));
            (Some(likely), comp)
        } else {
            key_points.push("No opposition ground specified - new tenancy likely".into());
            (None, false)
        };

        let new_tenancy_available = ground_likely.is_none_or(|success| !success);

        let reasoning = if new_tenancy_available {
            "Tenant is likely entitled to a new tenancy on current terms (or as \
             determined by court). If no opposition, or opposition ground fails, \
             court must grant new tenancy."
                .into()
        } else {
            format!(
                "Opposition ground {:?} appears likely to succeed. Tenant may be \
                 entitled to compensation if ground (e), (f), or (g) applies.",
                facts.opposition_ground
            )
        };

        Lta1954Result {
            protection_applies,
            new_tenancy_available,
            compensation_available: compensation && !new_tenancy_available,
            ground_likely_to_succeed: ground_likely,
            reasoning,
            key_points,
        }
    }

    fn assess_ground_success(ground: &Lta1954Ground) -> bool {
        // Simplified assessment - in practice, highly fact-dependent
        match ground {
            Lta1954Ground::GroundA | Lta1954Ground::GroundB | Lta1954Ground::GroundC => {
                // Discretionary grounds - harder to establish
                false
            }
            Lta1954Ground::GroundD
            | Lta1954Ground::GroundE
            | Lta1954Ground::GroundF
            | Lta1954Ground::GroundG => {
                // Mandatory grounds if requirements met
                true
            }
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lease_or_licence_lease() {
        let facts = LeaseOrLicenceFacts {
            description: "Flat let for 12 months".into(),
            exclusive_possession: true,
            certain_term: true,
            rent_payable: true,
            shared_occupation: false,
            grantor_access: false,
            service_element: false,
            label_used: "Licence".into(),
            sham_indicators: vec![],
        };
        let result = LeaseOrLicenceAnalyzer::analyze(&facts);
        assert!(result.is_lease);
        assert!(result.reasoning.contains("Street v Mountford"));
    }

    #[test]
    fn test_lease_or_licence_no_exclusive() {
        let facts = LeaseOrLicenceFacts {
            description: "Shared room in hostel".into(),
            exclusive_possession: false,
            certain_term: true,
            rent_payable: true,
            shared_occupation: true,
            grantor_access: true,
            service_element: true,
            label_used: "Licence".into(),
            sham_indicators: vec![],
        };
        let result = LeaseOrLicenceAnalyzer::analyze(&facts);
        assert!(!result.is_lease);
    }

    #[test]
    fn test_freehold_analysis() {
        use super::super::types::{Owner, OwnerType, PropertyAddress};

        let facts = FreeholdFacts {
            property: FreeholdEstate {
                title_number: Some("AB123456".into()),
                address: PropertyAddress {
                    address_line_1: "1 Test Street".into(),
                    address_line_2: None,
                    city: "London".into(),
                    postcode: "SW1A 1AA".into(),
                },
                title_class: Some(TitleClass::Absolute),
                registered: true,
                owners: vec![Owner {
                    name: "John Smith".into(),
                    owner_type: OwnerType::Individual,
                }],
                co_ownership: None,
            },
            acquisition_type: AcquisitionType::Purchase,
            consideration_pence: Some(50_000_000),
            restrictive_covenants: vec![],
            easements_benefiting: vec![],
            easements_burdening: vec![],
        };
        let result = FreeholdAnalyzer::analyze(&facts);
        assert!(result.valid_estate);
        assert_eq!(result.title_quality, TitleQuality::Good);
    }

    #[test]
    fn test_leasehold_short_lease() {
        use super::super::types::PropertyAddress;

        let facts = LeaseholdFacts {
            lease: LeaseholdEstate {
                title_number: Some("AB123456".into()),
                address: PropertyAddress {
                    address_line_1: "Flat 1".into(),
                    address_line_2: None,
                    city: "London".into(),
                    postcode: "SW1A 1AA".into(),
                },
                duration: LeaseDuration::Fixed {
                    years: 70,
                    months: 0,
                    weeks: 0,
                },
                start_date: "2020-01-01".into(),
                ground_rent_pence: Some(25000),
                business_tenancy: false,
                lta_1954_protected: false,
                landlord: Some("Freeholder Ltd".into()),
                contracted_out: false,
            },
            use_type: LeaseUseType::Residential,
            alterations_proposed: false,
            assignment_proposed: false,
            rent_arrears_pence: 0,
            breach_alleged: None,
            notice_served: false,
        };
        let result = LeaseholdAnalyzer::analyze(&facts);
        assert!(result.issues.iter().any(|i| i.contains("Short lease")));
    }

    #[test]
    fn test_forfeiture_waiver() {
        let facts = ForfeitureFacts {
            breach_type: BreachType::RentArrears {
                amount_pence: 100_000,
            },
            section_146_notice: false,
            time_to_remedy_given: false,
            breach_remedied: false,
            relief_sought: false,
            waiver_occurred: true,
            residential: true,
        };
        let result = ForfeitureAnalyzer::analyze(&facts);
        assert!(!result.forfeiture_available);
        assert!(result.waiver_bars_forfeiture);
    }

    #[test]
    fn test_lta_1954_protected() {
        let facts = Lta1954Facts {
            is_business_tenancy: true,
            occupation_for_business: true,
            contracted_out: false,
            section_25_notice: false,
            section_26_request: false,
            opposition_ground: None,
        };
        let result = Lta1954Analyzer::analyze(&facts);
        assert!(result.protection_applies);
        assert!(result.new_tenancy_available);
    }

    #[test]
    fn test_lta_1954_contracted_out() {
        let facts = Lta1954Facts {
            is_business_tenancy: true,
            occupation_for_business: true,
            contracted_out: true,
            section_25_notice: false,
            section_26_request: false,
            opposition_ground: None,
        };
        let result = Lta1954Analyzer::analyze(&facts);
        assert!(!result.protection_applies);
    }
}
