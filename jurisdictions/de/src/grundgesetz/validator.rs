//! Validation functions for German Constitutional Law
//!
//! Implements Grundgesetz validation logic for basic rights, proportionality,
//! and constitutional structure.

use chrono::Utc;

use super::error::{ConstitutionalError, Result};
use super::types::*;

/// Validate basic right holder
pub fn validate_right_holder(holder: &RightHolder, article: BasicRightArticle) -> Result<()> {
    if holder.name.trim().is_empty() {
        return Err(ConstitutionalError::EmptyName);
    }

    // Public authorities cannot be basic right holders
    if matches!(holder.holder_type, RightHolderType::PublicAuthority) {
        return Err(ConstitutionalError::PublicAuthorityNotRightHolder);
    }

    // Some rights are limited to German citizens (Deutschenrechte)
    if !holder.german_citizen {
        match article {
            BasicRightArticle::FreedomOfAssembly
            | BasicRightArticle::FreedomOfAssociation
            | BasicRightArticle::FreedomOfMovement
            | BasicRightArticle::OccupationalFreedom => {
                return Err(ConstitutionalError::RightLimitedToGermanCitizens {
                    article: format!("{:?}", article),
                });
            }
            _ => {}
        }
    }

    Ok(())
}

/// Validate basic right with restrictions
pub fn validate_basic_right(right: &BasicRight) -> Result<()> {
    validate_right_holder(&right.holder, right.article)?;

    if right.content.trim().is_empty() {
        return Err(ConstitutionalError::MissingJustification);
    }

    // Validate each restriction
    for restriction in &right.restrictions {
        validate_rights_restriction(restriction)?;
    }

    Ok(())
}

/// Validate rights restriction
pub fn validate_rights_restriction(restriction: &RightsRestriction) -> Result<()> {
    // Restriction must have legal basis
    if restriction.legal_basis.trim().is_empty() {
        return Err(ConstitutionalError::RestrictionWithoutLegalBasis);
    }

    // Restriction must have justification
    if restriction.justification.trim().is_empty() {
        return Err(ConstitutionalError::MissingJustification);
    }

    // Validate restricting authority
    validate_public_authority(&restriction.restricting_authority)?;

    // Date cannot be in the future
    if restriction.date_of_restriction > Utc::now().date_naive() {
        return Err(ConstitutionalError::InvalidDate {
            date: restriction.date_of_restriction.to_string(),
        });
    }

    Ok(())
}

/// Validate public authority
pub fn validate_public_authority(authority: &PublicAuthority) -> Result<()> {
    if authority.name.trim().is_empty() {
        return Err(ConstitutionalError::EmptyName);
    }

    Ok(())
}

/// Validate proportionality test (Verhältnismäßigkeitsprüfung)
///
/// Three-step test: Suitability, Necessity, Proportionality stricto sensu
pub fn validate_proportionality_test(test: &ProportionalityTest) -> Result<()> {
    validate_rights_restriction(&test.restriction)?;

    if test.legitimate_purpose.trim().is_empty() {
        return Err(ConstitutionalError::MissingJustification);
    }

    // Suitability (Geeignetheit)
    if !test.suitable.is_suitable {
        return Err(ConstitutionalError::NotSuitable {
            reason: test.suitable.reasoning.clone(),
        });
    }

    // Necessity (Erforderlichkeit)
    if !test.necessary.is_necessary {
        return Err(ConstitutionalError::NotNecessary {
            alternatives: test.necessary.alternative_measures.join(", "),
        });
    }

    // Proportionality stricto sensu (Angemessenheit)
    if !test.proportionate_stricto_sensu.is_proportionate {
        return Err(ConstitutionalError::NotProportionate);
    }

    Ok(())
}

/// Validate constitutional complaint (Verfassungsbeschwerde)
pub fn validate_constitutional_complaint(complaint: &ConstitutionalComplaint) -> Result<()> {
    // Validate complainant
    validate_right_holder(&complaint.complainant, complaint.violated_right)?;

    // Subsidiarity requirement (Art. 90 Para. 2 BVerfGG)
    if !complaint.subsidiarity_met {
        return Err(ConstitutionalError::SubsidiarityNotMet);
    }

    // Must be personally, currently, and directly affected (selbst, gegenwärtig, unmittelbar)
    if !complaint.directly_affected {
        return Err(ConstitutionalError::NotDirectlyAffected);
    }

    // Deadline check
    if !complaint.filed_within_deadline {
        // Calculate days since infringement (simplified)
        return Err(ConstitutionalError::ComplaintDeadlineExpired { days: 365 });
    }

    // Complaint date cannot be in the future
    if complaint.complaint_date > Utc::now().date_naive() {
        return Err(ConstitutionalError::InvalidDate {
            date: complaint.complaint_date.to_string(),
        });
    }

    Ok(())
}

/// Validate Bundestag structure
pub fn validate_bundestag(bundestag: &Bundestag) -> Result<()> {
    if bundestag.president.trim().is_empty() {
        return Err(ConstitutionalError::EmptyName);
    }

    // Validate each member
    for member in &bundestag.members {
        validate_bundestag_member(member)?;
    }

    Ok(())
}

/// Validate Bundestag member (Art. 38 GG)
pub fn validate_bundestag_member(member: &BundestagMember) -> Result<()> {
    if member.name.trim().is_empty() {
        return Err(ConstitutionalError::EmptyName);
    }

    // Free mandate (Art. 38 Para. 1 Sent. 2 GG)
    // Members represent the whole people, not bound by instructions
    if !member.free_mandate {
        return Err(ConstitutionalError::FreeMandateViolated);
    }

    Ok(())
}

/// Validate Bundesrat structure
pub fn validate_bundesrat(bundesrat: &Bundesrat) -> Result<()> {
    // Validate each state delegation
    for delegation in &bundesrat.state_delegations {
        validate_state_delegation(delegation)?;
    }

    Ok(())
}

/// Validate state delegation to Bundesrat (Art. 51 Para. 2 GG)
pub fn validate_state_delegation(delegation: &StateDelegation) -> Result<()> {
    if delegation.state_name.trim().is_empty() {
        return Err(ConstitutionalError::EmptyName);
    }

    // Votes must be 3-6 based on population
    if !(3..=6).contains(&delegation.votes) {
        return Err(ConstitutionalError::InvalidBundesratVotes {
            actual: delegation.votes,
            population: 0, // Would need actual population data
        });
    }

    Ok(())
}

/// Validate Federal President (Art. 54 GG)
pub fn validate_federal_president(president: &FederalPresident) -> Result<()> {
    if president.name.trim().is_empty() {
        return Err(ConstitutionalError::EmptyName);
    }

    // Maximum two consecutive terms (Art. 54 Para. 2 GG)
    if president.term_number > 2 {
        return Err(ConstitutionalError::PresidentTooManyTerms);
    }

    // Term start cannot be in the future
    if president.term_start > Utc::now().date_naive() {
        return Err(ConstitutionalError::InvalidDate {
            date: president.term_start.to_string(),
        });
    }

    Ok(())
}

/// Validate Federal Government (Art. 62-69 GG)
pub fn validate_federal_government(government: &FederalGovernment) -> Result<()> {
    validate_federal_chancellor(&government.chancellor)?;

    // Validate each minister
    for minister in &government.ministers {
        validate_federal_minister(minister)?;
    }

    Ok(())
}

/// Validate Federal Chancellor (Art. 65 GG)
pub fn validate_federal_chancellor(chancellor: &FederalChancellor) -> Result<()> {
    if chancellor.name.trim().is_empty() {
        return Err(ConstitutionalError::EmptyName);
    }

    // Chancellor determines policy guidelines (Art. 65 Sent. 1 GG)
    if !chancellor.policy_guidelines {
        return Err(ConstitutionalError::PolicyGuidelinesViolated);
    }

    // Election date cannot be in the future
    if chancellor.elected_date > Utc::now().date_naive() {
        return Err(ConstitutionalError::InvalidDate {
            date: chancellor.elected_date.to_string(),
        });
    }

    Ok(())
}

/// Validate Federal Minister (Art. 65 GG)
pub fn validate_federal_minister(minister: &FederalMinister) -> Result<()> {
    if minister.name.trim().is_empty() {
        return Err(ConstitutionalError::EmptyName);
    }

    if minister.ministry.trim().is_empty() {
        return Err(ConstitutionalError::InvalidAuthority);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;

    fn create_valid_right_holder() -> RightHolder {
        RightHolder {
            name: "Max Mustermann".to_string(),
            holder_type: RightHolderType::NaturalPerson,
            german_citizen: true,
        }
    }

    fn create_valid_public_authority() -> PublicAuthority {
        PublicAuthority {
            name: "Bundestag".to_string(),
            authority_type: AuthorityType::Legislative,
            level: FederalLevel::Federal,
        }
    }

    #[test]
    fn test_valid_right_holder() {
        let holder = create_valid_right_holder();
        assert!(validate_right_holder(&holder, BasicRightArticle::FreedomOfExpression).is_ok());
    }

    #[test]
    fn test_public_authority_not_right_holder() {
        let holder = RightHolder {
            name: "Government".to_string(),
            holder_type: RightHolderType::PublicAuthority,
            german_citizen: true,
        };
        assert!(matches!(
            validate_right_holder(&holder, BasicRightArticle::FreedomOfExpression),
            Err(ConstitutionalError::PublicAuthorityNotRightHolder)
        ));
    }

    #[test]
    fn test_german_only_right() {
        let holder = RightHolder {
            name: "John Doe".to_string(),
            holder_type: RightHolderType::NaturalPerson,
            german_citizen: false,
        };
        assert!(matches!(
            validate_right_holder(&holder, BasicRightArticle::FreedomOfAssembly),
            Err(ConstitutionalError::RightLimitedToGermanCitizens { .. })
        ));
    }

    #[test]
    fn test_valid_proportionality_test() {
        let test = ProportionalityTest {
            restriction: RightsRestriction {
                restricting_authority: create_valid_public_authority(),
                legal_basis: "BVerfSchG".to_string(),
                restriction_type: RestrictionType::PermitRequirement,
                date_of_restriction: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                justification: "Protection of public order".to_string(),
            },
            legitimate_purpose: "Ensure public safety".to_string(),
            suitable: SuitabilityAssessment {
                is_suitable: true,
                reasoning: "Measure achieves stated purpose".to_string(),
            },
            necessary: NecessityAssessment {
                is_necessary: true,
                alternative_measures: vec![],
                reasoning: "No less restrictive alternatives available".to_string(),
            },
            proportionate_stricto_sensu: ProportionalityStrictoSensu {
                is_proportionate: true,
                public_interest: "Public safety".to_string(),
                private_interest: "Individual freedom".to_string(),
                balancing: "Public safety outweighs minor restriction".to_string(),
            },
        };

        assert!(validate_proportionality_test(&test).is_ok());
        assert!(test.passes_test());
    }

    #[test]
    fn test_proportionality_test_not_suitable() {
        let test = ProportionalityTest {
            restriction: RightsRestriction {
                restricting_authority: create_valid_public_authority(),
                legal_basis: "Law".to_string(),
                restriction_type: RestrictionType::Prohibition,
                date_of_restriction: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
                justification: "Test".to_string(),
            },
            legitimate_purpose: "Purpose".to_string(),
            suitable: SuitabilityAssessment {
                is_suitable: false,
                reasoning: "Does not achieve purpose".to_string(),
            },
            necessary: NecessityAssessment {
                is_necessary: true,
                alternative_measures: vec![],
                reasoning: "Test".to_string(),
            },
            proportionate_stricto_sensu: ProportionalityStrictoSensu {
                is_proportionate: true,
                public_interest: "Test".to_string(),
                private_interest: "Test".to_string(),
                balancing: "Test".to_string(),
            },
        };

        assert!(matches!(
            validate_proportionality_test(&test),
            Err(ConstitutionalError::NotSuitable { .. })
        ));
        assert!(!test.passes_test());
    }

    #[test]
    fn test_valid_constitutional_complaint() {
        let complaint = ConstitutionalComplaint {
            complainant: create_valid_right_holder(),
            violated_right: BasicRightArticle::FreedomOfExpression,
            infringing_act: InfringingAct::Statute {
                name: "Test Law".to_string(),
                date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            },
            subsidiarity_met: true,
            directly_affected: true,
            filed_within_deadline: true,
            complaint_date: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        };

        assert!(validate_constitutional_complaint(&complaint).is_ok());
    }

    #[test]
    fn test_complaint_subsidiarity_not_met() {
        let complaint = ConstitutionalComplaint {
            complainant: create_valid_right_holder(),
            violated_right: BasicRightArticle::FreedomOfExpression,
            infringing_act: InfringingAct::Statute {
                name: "Test Law".to_string(),
                date: NaiveDate::from_ymd_opt(2023, 1, 1).unwrap(),
            },
            subsidiarity_met: false,
            directly_affected: true,
            filed_within_deadline: true,
            complaint_date: NaiveDate::from_ymd_opt(2023, 6, 1).unwrap(),
        };

        assert!(matches!(
            validate_constitutional_complaint(&complaint),
            Err(ConstitutionalError::SubsidiarityNotMet)
        ));
    }

    #[test]
    fn test_valid_federal_president() {
        let president = FederalPresident {
            name: "Frank-Walter Steinmeier".to_string(),
            term_start: NaiveDate::from_ymd_opt(2017, 3, 19).unwrap(),
            term_number: 1,
        };

        assert!(validate_federal_president(&president).is_ok());
    }

    #[test]
    fn test_president_too_many_terms() {
        let president = FederalPresident {
            name: "Test President".to_string(),
            term_start: NaiveDate::from_ymd_opt(2010, 1, 1).unwrap(),
            term_number: 3,
        };

        assert!(matches!(
            validate_federal_president(&president),
            Err(ConstitutionalError::PresidentTooManyTerms)
        ));
    }

    #[test]
    fn test_valid_bundestag_member() {
        let member = BundestagMember {
            name: "Member Name".to_string(),
            party: Some("Party".to_string()),
            constituency: Some("Constituency 1".to_string()),
            free_mandate: true,
        };

        assert!(validate_bundestag_member(&member).is_ok());
    }

    #[test]
    fn test_free_mandate_violated() {
        let member = BundestagMember {
            name: "Member Name".to_string(),
            party: Some("Party".to_string()),
            constituency: Some("Constituency 1".to_string()),
            free_mandate: false,
        };

        assert!(matches!(
            validate_bundestag_member(&member),
            Err(ConstitutionalError::FreeMandateViolated)
        ));
    }
}
