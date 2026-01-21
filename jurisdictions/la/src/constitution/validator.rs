//! Validation functions for constitutional compliance.
//!
//! This module provides comprehensive validation for:
//! - Voting rights (Article 35)
//! - Fundamental rights (Articles 34-51)
//! - State structure (Articles 52-100)
//! - Court organization (Articles 88-95)
//! - Constitutional amendments (Articles 105-108)

use crate::constitution::error::{ConstitutionalError, ConstitutionalResult, LimitationFailure};
use crate::constitution::types::{
    AdministrativeLevel, ConstitutionalAmendment, ElectionMethod, FundamentalRight, Government,
    LocalAdministration, NationalAssembly, PeoplesCourt, PeoplesProsecutor, President,
    RightsLimitation,
};

// ============================================================================
// Voting Rights Validation (Article 35)
// ============================================================================

/// Validate voting rights according to Article 35
/// ກວດສອບສິດເລືອກຕັ້ງຕາມມາດຕາ 35
///
/// ## Article 35
/// Lao citizens who are 18 years of age or older have the right to vote.
/// ພົນລະເມືອງລາວທີ່ມີອາຍຸ 18 ປີຂຶ້ນໄປ ມີສິດເລືອກຕັ້ງ.
///
/// ## Examples
/// ```
/// use legalis_la::constitution::validator::validate_voting_rights;
///
/// // Valid: 18 years old
/// assert!(validate_voting_rights(18).is_ok());
///
/// // Valid: Above 18
/// assert!(validate_voting_rights(25).is_ok());
///
/// // Invalid: Below 18
/// assert!(validate_voting_rights(17).is_err());
/// ```
pub fn validate_voting_rights(age: u8) -> ConstitutionalResult<()> {
    const MINIMUM_VOTING_AGE: u8 = 18;

    if age < MINIMUM_VOTING_AGE {
        return Err(ConstitutionalError::voting_rights_violation(
            age,
            MINIMUM_VOTING_AGE,
        ));
    }

    Ok(())
}

/// Validate eligibility to run for National Assembly (Article 53)
/// ກວດສອບສິດສະໝັກເປັນສະມາຊິກສະພາແຫ່ງຊາດ
///
/// ## Article 53
/// Lao citizens who are at least 21 years old have the right to be elected to the National Assembly.
/// ພົນລະເມືອງລາວທີ່ມີອາຍຸ 21 ປີຂຶ້ນໄປ ມີສິດຖືກເລືອກຕັ້ງເຂົ້າສະພາແຫ່ງຊາດ.
pub fn validate_na_candidacy(age: u8, is_lao_citizen: bool) -> ConstitutionalResult<()> {
    const MINIMUM_CANDIDACY_AGE: u8 = 21;

    if !is_lao_citizen {
        return Err(ConstitutionalError::national_assembly_violation(
            "ຕ້ອງເປັນພົນລະເມືອງລາວ",
            "Must be a Lao citizen",
            53,
        ));
    }

    if age < MINIMUM_CANDIDACY_AGE {
        return Err(ConstitutionalError::national_assembly_violation(
            format!(
                "ອາຍຸ {} ປີ ບໍ່ເຖິງອາຍຸຂັ້ນຕ່ຳ {} ປີສຳລັບການສະໝັກເຂົ້າສະພາແຫ່ງຊາດ",
                age, MINIMUM_CANDIDACY_AGE
            ),
            format!(
                "Age {} is below minimum candidacy age of {}",
                age, MINIMUM_CANDIDACY_AGE
            ),
            53,
        ));
    }

    Ok(())
}

// ============================================================================
// Fundamental Rights Validation (Articles 34-51)
// ============================================================================

/// Validate fundamental right according to Constitution Chapter VI
/// ກວດສອບສິດພື້ນຖານຕາມບົດບັນຍັດທີ VI ຂອງລັດຖະທຳມະນູນ
///
/// ## Examples
/// ```
/// use legalis_la::constitution::{types::FundamentalRight, validator::validate_fundamental_right};
///
/// // Validate right to vote for person aged 18
/// assert!(validate_fundamental_right(
///     FundamentalRight::VotingRight { age_requirement: 18 },
///     Some(18)
/// ).is_ok());
///
/// // Validate equality (no age requirement)
/// assert!(validate_fundamental_right(
///     FundamentalRight::Equality,
///     None
/// ).is_ok());
/// ```
pub fn validate_fundamental_right(
    right: FundamentalRight,
    age: Option<u8>,
) -> ConstitutionalResult<()> {
    match right {
        FundamentalRight::VotingRight { age_requirement } => {
            if let Some(actual_age) = age
                && actual_age < age_requirement
            {
                return Err(ConstitutionalError::fundamental_right_violation(
                    right,
                    format!("ອາຍຸ {} ປີ ບໍ່ເຖິງອາຍຸຂັ້ນຕ່ຳ {} ປີ", actual_age, age_requirement),
                    format!(
                        "Age {} is below minimum age of {}",
                        actual_age, age_requirement
                    ),
                    35,
                ));
            }
            Ok(())
        }
        // All other rights are guaranteed without age requirements
        _ => Ok(()),
    }
}

/// Validate rights limitation using proportionality test
/// ກວດສອບການຈຳກັດສິດດ້ວຍການທົດສອບຄວາມສົມເຫດສົມຜົນ
///
/// ## Proportionality Test
/// A limitation on fundamental rights must satisfy:
/// 1. Legitimate aim (public order, national security, health, morals, or rights of others)
/// 2. Necessity (the limitation is necessary to achieve the aim)
/// 3. Proportionality (the limitation is proportional to the aim)
/// 4. Legal basis (the limitation is prescribed by law)
///
/// ## Examples
/// ```
/// use legalis_la::constitution::{
///     types::{FundamentalRight, LegitimateAim, RightsLimitation},
///     validator::validate_rights_limitation,
/// };
///
/// let limitation = RightsLimitation {
///     right: FundamentalRight::FreedomOfExpression,
///     legitimate_aim: LegitimateAim::PublicOrder,
///     is_necessary: true,
///     is_proportional: true,
///     legal_basis: "Public Order Law 2020, Article 15".to_string(),
/// };
///
/// assert!(validate_rights_limitation(&limitation).is_ok());
/// ```
pub fn validate_rights_limitation(limitation: &RightsLimitation) -> ConstitutionalResult<()> {
    // Check for legitimate aim (implicit in enum)
    // No need to check as LegitimateAim enum ensures valid aim

    // Check necessity
    if !limitation.is_necessary {
        return Err(ConstitutionalError::unjustified_limitation(
            limitation.right,
            LimitationFailure::NotNecessary,
        ));
    }

    // Check proportionality
    if !limitation.is_proportional {
        return Err(ConstitutionalError::unjustified_limitation(
            limitation.right,
            LimitationFailure::NotProportional,
        ));
    }

    // Check legal basis
    if limitation.legal_basis.trim().is_empty() {
        return Err(ConstitutionalError::unjustified_limitation(
            limitation.right,
            LimitationFailure::NoLegalBasis,
        ));
    }

    Ok(())
}

// ============================================================================
// State Structure Validation (Articles 52-100)
// ============================================================================

/// Validate National Assembly structure (Articles 52-65)
/// ກວດສອບໂຄງສ້າງສະພາແຫ່ງຊາດ
///
/// ## Article 53
/// - Term: 5 years
/// - Election: universal, direct, and secret ballot
///
/// ## Examples
/// ```
/// use legalis_la::constitution::{
///     types::{ElectionMethod, NationalAssembly},
///     validator::validate_national_assembly,
/// };
///
/// let na = NationalAssembly::builder()
///     .session(9)
///     .members(164)
///     .term_years(5)
///     .election_method(ElectionMethod::default())
///     .build()
///     .unwrap();
///
/// assert!(validate_national_assembly(&na).is_ok());
/// ```
pub fn validate_national_assembly(assembly: &NationalAssembly) -> ConstitutionalResult<()> {
    // Validate term (Article 53: must be 5 years)
    if assembly.term_years != 5 {
        return Err(ConstitutionalError::national_assembly_violation(
            format!("ວາລະ {} ປີ ບໍ່ຖືກຕ້ອງ, ຕ້ອງເປັນ 5 ປີ", assembly.term_years),
            format!(
                "Term of {} years is invalid, must be 5 years",
                assembly.term_years
            ),
            53,
        ));
    }

    // Validate election method (Article 53: universal, direct, and secret)
    validate_election_method(&assembly.election_method)?;

    // Validate minimum number of members (reasonable check)
    if assembly.members < 10 {
        return Err(ConstitutionalError::national_assembly_violation(
            format!("ຈຳນວນສະມາຊິກ {} ຄົນ ຕ່ຳເກີນໄປ", assembly.members),
            format!("Number of members ({}) is too low", assembly.members),
            52,
        ));
    }

    Ok(())
}

/// Validate election method (Article 53)
/// ກວດສອບວິທີການເລືອກຕັ້ງ
fn validate_election_method(method: &ElectionMethod) -> ConstitutionalResult<()> {
    if !method.universal {
        return Err(ConstitutionalError::national_assembly_violation(
            "ການເລືອກຕັ້ງຕ້ອງເປັນການເລືອກຕັ້ງທົ່ວໄປ",
            "Election must be universal",
            53,
        ));
    }

    if !method.direct {
        return Err(ConstitutionalError::national_assembly_violation(
            "ການເລືອກຕັ້ງຕ້ອງເປັນການເລືອກຕັ້ງໂດຍກົງ",
            "Election must be direct",
            53,
        ));
    }

    if !method.secret {
        return Err(ConstitutionalError::national_assembly_violation(
            "ການເລືອກຕັ້ງຕ້ອງເປັນການລົງຄະແນນລັບ",
            "Election must be by secret ballot",
            53,
        ));
    }

    Ok(())
}

/// Validate President structure (Articles 66-70)
/// ກວດສອບໂຄງສ້າງປະທານປະເທດ
///
/// ## Article 67
/// - Term: 5 years
/// - Elected by National Assembly
pub fn validate_president(president: &President) -> ConstitutionalResult<()> {
    // Validate term (Article 67: must be 5 years)
    if president.term_years != 5 {
        return Err(ConstitutionalError::presidential_violation(
            format!("ວາລະ {} ປີ ບໍ່ຖືກຕ້ອງ, ຕ້ອງເປັນ 5 ປີ", president.term_years),
            format!(
                "Term of {} years is invalid, must be 5 years",
                president.term_years
            ),
            67,
        ));
    }

    // Validate that powers are not empty
    if president.powers.is_empty() {
        return Err(ConstitutionalError::presidential_violation(
            "ປະທານປະເທດຕ້ອງມີອຳນາດ",
            "President must have powers",
            68,
        ));
    }

    Ok(())
}

/// Validate Government structure (Articles 71-80)
/// ກວດສອບໂຄງສ້າງລັດຖະບານ
///
/// ## Article 71
/// Government composition:
/// - Prime Minister
/// - Deputy Prime Ministers
/// - Ministers
pub fn validate_government(government: &Government) -> ConstitutionalResult<()> {
    // Validate Prime Minister exists
    if government.prime_minister.trim().is_empty() {
        return Err(ConstitutionalError::government_violation(
            "ລັດຖະບານຕ້ອງມີນາຍົກລັດຖະມົນຕີ",
            "Government must have a Prime Minister",
            71,
        ));
    }

    // Validate at least one minister exists
    if government.ministers.is_empty() {
        return Err(ConstitutionalError::government_violation(
            "ລັດຖະບານຕ້ອງມີລັດຖະມົນຕີຢ່າງໜ້ອຍໜຶ່ງຄົນ",
            "Government must have at least one minister",
            71,
        ));
    }

    // Validate that powers are not empty
    if government.powers.is_empty() {
        return Err(ConstitutionalError::government_violation(
            "ລັດຖະບານຕ້ອງມີອຳນາດ",
            "Government must have powers",
            73,
        ));
    }

    Ok(())
}

/// Validate Local Administration structure (Articles 81-87)
/// ກວດສອບໂຄງສ້າງການປົກຄອງທ້ອງຖິ່ນ
///
/// ## Article 81
/// Administrative levels:
/// - Province (ແຂວງ)
/// - District (ເມືອງ)
/// - Village (ບ້ານ)
pub fn validate_local_administration(admin: &LocalAdministration) -> ConstitutionalResult<()> {
    // Validate name is not empty
    if admin.name.trim().is_empty() {
        return Err(ConstitutionalError::local_administration_violation(
            "ຊື່ຫົວໜ່ວຍການປົກຄອງບໍ່ສາມາດເປັນຄ່າວ່າງ",
            "Administrative unit name cannot be empty",
            81,
        ));
    }

    // Validate chief is not empty
    if admin.administrative_authority.chief.trim().is_empty() {
        let level_str = match admin.level {
            AdministrativeLevel::Province => "ເຈົ້າແຂວງ",
            AdministrativeLevel::District => "ເຈົ້າເມືອງ",
            AdministrativeLevel::Village => "ນາຍບ້ານ",
        };

        return Err(ConstitutionalError::local_administration_violation(
            format!("{} ບໍ່ສາມາດເປັນຄ່າວ່າງ", level_str),
            "Administrative chief cannot be empty",
            83,
        ));
    }

    // Validate People's Council term if exists (Article 82: must be 5 years)
    if let Some(council) = &admin.peoples_council
        && council.term_years != 5
    {
        return Err(ConstitutionalError::local_administration_violation(
            format!("ວາລະສະພາປະຊາຊົນ {} ປີ ບໍ່ຖືກຕ້ອງ, ຕ້ອງເປັນ 5 ປີ", council.term_years),
            format!(
                "People's Council term of {} years is invalid, must be 5 years",
                council.term_years
            ),
            82,
        ));
    }

    Ok(())
}

/// Validate overall state structure
/// ກວດສອບໂຄງສ້າງລັດໂດຍລວມ
pub fn validate_state_structure(
    assembly: Option<&NationalAssembly>,
    president: Option<&President>,
    government: Option<&Government>,
) -> ConstitutionalResult<()> {
    if let Some(na) = assembly {
        validate_national_assembly(na)?;
    }

    if let Some(pres) = president {
        validate_president(pres)?;
    }

    if let Some(gov) = government {
        validate_government(gov)?;
    }

    Ok(())
}

// ============================================================================
// Judicial System Validation (Articles 88-100)
// ============================================================================

/// Validate court organization (Articles 88-95)
/// ກວດສອບໂຄງສ້າງສານ
///
/// ## Article 88
/// Courts are independent and adjudicate cases according to law only.
/// ສານເປັນເອກະລາດ ແລະ ພິຈາລະນາຄະດີຕາມກົດໝາຍເທົ່ານັ້ນ.
///
/// ## Examples
/// ```
/// use legalis_la::constitution::{
///     types::{CourtLevel, Judge, PeoplesCourt, CourtPower},
///     validator::validate_court_organization,
/// };
///
/// let court = PeoplesCourt {
///     level: CourtLevel::Supreme,
///     is_independent: true,
///     judges: vec![
///         Judge {
///             name: "Judge A".to_string(),
///             court_level: CourtLevel::Supreme,
///             independent_judgment: true,
///         }
///     ],
///     powers: vec![CourtPower::AdjudicateCriminal, CourtPower::AdjudicateCivil],
/// };
///
/// assert!(validate_court_organization(&court).is_ok());
/// ```
pub fn validate_court_organization(court: &PeoplesCourt) -> ConstitutionalResult<()> {
    // Validate independence (Article 88)
    if !court.is_independent {
        return Err(ConstitutionalError::judicial_independence_violation(
            "ສານຕ້ອງເປັນເອກະລາດ",
            "Courts must be independent",
        ));
    }

    // Validate judges exist
    if court.judges.is_empty() {
        return Err(ConstitutionalError::court_violation(
            court.level,
            "ສານຕ້ອງມີຜູ້ພິພາກສາຢ່າງໜ້ອຍໜຶ່ງຄົນ",
            "Court must have at least one judge",
            92,
        ));
    }

    // Validate all judges have independent judgment (Article 92)
    for judge in &court.judges {
        if !judge.independent_judgment {
            return Err(ConstitutionalError::judicial_independence_violation(
                format!("ຜູ້ພິພາກສາ {} ຕ້ອງມີຄວາມເປັນເອກະລາດໃນການພິພາກສາ", judge.name),
                format!("Judge {} must have independent judgment", judge.name),
            ));
        }

        // Validate judge's court level matches court level
        if judge.court_level != court.level {
            return Err(ConstitutionalError::court_violation(
                court.level,
                format!("ລະດັບສານຂອງຜູ້ພິພາກສາ {} ບໍ່ກົງກັບລະດັບສານ", judge.name),
                format!(
                    "Judge {}'s court level does not match court level",
                    judge.name
                ),
                89,
            ));
        }
    }

    // Validate powers exist
    if court.powers.is_empty() {
        return Err(ConstitutionalError::court_violation(
            court.level,
            "ສານຕ້ອງມີອຳນາດ",
            "Court must have powers",
            89,
        ));
    }

    Ok(())
}

/// Validate prosecutor organization (Articles 96-100)
/// ກວດສອບໂຄງສ້າງອົງການໄອຍະການ
///
/// ## Article 96
/// The People's Prosecutors have the right to prosecute and supervise investigation,
/// detention, and legality of court proceedings.
pub fn validate_prosecutor_organization(
    prosecutor: &PeoplesProsecutor,
) -> ConstitutionalResult<()> {
    // Validate powers exist
    if prosecutor.powers.is_empty() {
        return Err(ConstitutionalError::prosecutor_violation(
            prosecutor.level,
            "ອົງການໄອຍະການຕ້ອງມີອຳນາດ",
            "Prosecutor must have powers",
            97,
        ));
    }

    Ok(())
}

// ============================================================================
// Constitutional Amendment Validation (Articles 105-108)
// ============================================================================

/// Validate constitutional amendment procedure (Articles 105-108)
/// ກວດສອບຂັ້ນຕອນການແກ້ໄຂລັດຖະທຳມະນູນ
///
/// ## Article 106
/// Constitutional amendment requires 2/3 majority of National Assembly members.
/// ການແກ້ໄຂລັດຖະທຳມະນູນຕ້ອງໄດ້ຮັບຄະແນນສຽງ 2/3 ຂອງສະມາຊິກສະພາແຫ່ງຊາດ.
///
/// ## Examples
/// ```
/// use legalis_la::constitution::{
///     types::{AmendmentProposer, ConstitutionalAmendment},
///     validator::validate_constitutional_amendment,
/// };
///
/// let total_members = 164u32;
/// let amendment = ConstitutionalAmendment {
///     proposed_by: AmendmentProposer::President,
///     proposed_changes_lao: "ແກ້ໄຂມາດຕາ 1".to_string(),
///     proposed_changes_english: "Amend Article 1".to_string(),
///     required_votes: (total_members * 2).div_ceil(3),  // 110 votes
///     votes_received: Some(120),
///     approved: true,
///     amendment_date: None,
/// };
///
/// assert!(validate_constitutional_amendment(&amendment, total_members).is_ok());
/// ```
pub fn validate_constitutional_amendment(
    amendment: &ConstitutionalAmendment,
    total_na_members: u32,
) -> ConstitutionalResult<()> {
    // Calculate required 2/3 majority (Article 106)
    let required_votes = (total_na_members * 2).div_ceil(3);

    // Validate required_votes is correctly calculated
    if amendment.required_votes != required_votes {
        return Err(ConstitutionalError::amendment_violation(
            format!(
                "ຄະແນນສຽງທີ່ຕ້ອງການ {} ບໍ່ຖືກຕ້ອງ, ຄວນເປັນ {} (2/3 ຂອງ {})",
                amendment.required_votes, required_votes, total_na_members
            ),
            format!(
                "Required votes {} is incorrect, should be {} (2/3 of {})",
                amendment.required_votes, required_votes, total_na_members
            ),
            106,
        ));
    }

    // If votes have been received, validate they meet the requirement
    if let Some(votes) = amendment.votes_received {
        if votes < required_votes {
            return Err(ConstitutionalError::insufficient_amendment_votes(
                required_votes,
                votes,
            ));
        }

        // Validate approval status is consistent with votes
        if amendment.approved && votes < required_votes {
            return Err(ConstitutionalError::amendment_violation(
                "ການຮັບຮອງບໍ່ສອດຄ່ອງກັບຄະແນນສຽງ",
                "Approval status inconsistent with votes",
                106,
            ));
        }
    }

    // Validate proposed changes are not empty
    if amendment.proposed_changes_lao.trim().is_empty()
        || amendment.proposed_changes_english.trim().is_empty()
    {
        return Err(ConstitutionalError::amendment_violation(
            "ການແກ້ໄຂທີ່ສະເໜີບໍ່ສາມາດເປັນຄ່າວ່າງ",
            "Proposed changes cannot be empty",
            105,
        ));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::constitution::{AmendmentProposer, ConstitutionalAmendment};

    #[test]
    fn test_voting_rights() {
        assert!(validate_voting_rights(18).is_ok());
        assert!(validate_voting_rights(25).is_ok());
        assert!(validate_voting_rights(17).is_err());
        assert!(validate_voting_rights(0).is_err());
    }

    #[test]
    fn test_na_candidacy() {
        assert!(validate_na_candidacy(21, true).is_ok());
        assert!(validate_na_candidacy(25, true).is_ok());
        assert!(validate_na_candidacy(20, true).is_err());
        assert!(validate_na_candidacy(25, false).is_err());
    }

    #[test]
    fn test_election_method() {
        let valid_method = ElectionMethod {
            universal: true,
            direct: true,
            secret: true,
        };
        assert!(validate_election_method(&valid_method).is_ok());

        let invalid_method = ElectionMethod {
            universal: false,
            direct: true,
            secret: true,
        };
        assert!(validate_election_method(&invalid_method).is_err());
    }

    #[test]
    fn test_constitutional_amendment_votes() {
        let total_members: u32 = 164;
        let required = (total_members * 2).div_ceil(3); // 110

        let amendment = ConstitutionalAmendment {
            proposed_by: AmendmentProposer::President,
            proposed_changes_lao: "ແກ້ໄຂມາດຕາ 1".to_string(),
            proposed_changes_english: "Amend Article 1".to_string(),
            required_votes: required,
            votes_received: Some(120),
            approved: true,
            amendment_date: None,
        };

        assert!(validate_constitutional_amendment(&amendment, total_members).is_ok());

        // Insufficient votes
        let failed_amendment = ConstitutionalAmendment {
            votes_received: Some(100),
            ..amendment.clone()
        };

        assert!(validate_constitutional_amendment(&failed_amendment, total_members).is_err());
    }
}
