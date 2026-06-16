//! France — Loi Informatique et Libertés (Loi n° 78-17) national GDPR implementation.
//!
//! France implements the GDPR through the **Loi n° 78-17 du 6 janvier 1978 relative à
//! l'informatique, aux fichiers et aux libertés** (the "Loi Informatique et Libertés"),
//! as substantially amended by **Loi n° 2018-493 du 20 juin 2018** and recast by
//! **Ordonnance n° 2018-1125 du 12 décembre 2018** (in force 1 June 2019). The 2018
//! reforms aligned the 1978 law with the GDPR and transposed the Law Enforcement
//! Directive (EU) 2016/680.
//!
//! Key French specifics modelled here:
//!
//! - **Age of digital consent: 15** — France lowered the Article 8(1) GDPR default of 16
//!   to **15** via **Article 7-1** of Loi 78-17. A minor aged 15 or over may consent
//!   alone to information-society services; for a minor under 15 consent must be given
//!   *jointly* by the minor and the holder(s) of parental authority.
//! - **Supervisory authority** — the **CNIL** (Commission Nationale de l'Informatique
//!   et des Libertés), an independent administrative authority established by the 1978
//!   law (Article 8 Loi 78-17).
//! - **Special categories / health data** — Articles 6 and 44-46 (Chapter III, Section 3)
//!   Loi 78-17 specify conditions for processing sensitive data and impose particular
//!   formalities for health-data processing (Art 9(2)/(4) GDPR opening clause).
//! - **National identification number (NIR)** — Article 30 Loi 78-17 regulates the use of
//!   the *numéro d'inscription au répertoire national d'identification des personnes
//!   physiques* (the social-security number), exercising the Art 87 GDPR opening clause.
//! - **Journalism / freedom of expression** — Article 80 Loi 78-17 reconciles data
//!   protection with freedom of expression and information (Art 85 GDPR).
//! - **Collective redress** — Article 37 Loi 78-17 provides for class actions in data
//!   protection (Art 80 GDPR representation).

use crate::member_states::template::{
    MemberStateGdpr, NationalActCitation, NationalDerogation, OpeningClause, SupervisoryAuthority,
};
use crate::shared::MemberState;

/// France's age of digital consent under Article 8(1) GDPR, lowered to 15 by Article 7-1
/// of Loi n° 78-17.
pub const AGE_OF_DIGITAL_CONSENT: u8 = 15;

/// Build the French national GDPR implementation (Loi Informatique et Libertés).
///
/// ## Example
///
/// ```rust
/// use legalis_eu::member_states::france;
///
/// let fr = france::implementation();
/// assert_eq!(fr.age_of_digital_consent, 15);
/// assert_eq!(fr.lead_authority().abbreviation, "CNIL");
/// assert!(fr.has_lowered_age_of_consent());
/// ```
pub fn implementation() -> MemberStateGdpr {
    MemberStateGdpr::builder(MemberState::France)
        .age_of_digital_consent(AGE_OF_DIGITAL_CONSENT)
        .authority(SupervisoryAuthority::national(
            "Commission Nationale de l'Informatique et des Libertés",
            "CNIL",
            "National Commission on Informatics and Liberty",
            "https://www.cnil.fr",
        ))
        .national_act(NationalActCitation::new(
            "Loi 78-17",
            "Loi n° 78-17 du 6 janvier 1978 relative à l'informatique, aux fichiers et aux libertés",
            "Act No. 78-17 of 6 January 1978 on Information Technology, Data Files and Civil Liberties",
            1978,
        ))
        .national_act(NationalActCitation::new(
            "Loi 2018-493",
            "Loi n° 2018-493 du 20 juin 2018 relative à la protection des données personnelles",
            "Act No. 2018-493 of 20 June 2018 on the protection of personal data",
            2018,
        ))
        .national_act(NationalActCitation::new(
            "Ordonnance 2018-1125",
            "Ordonnance n° 2018-1125 du 12 décembre 2018",
            "Order No. 2018-1125 of 12 December 2018 (recast of Loi 78-17)",
            2018,
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article8AgeOfConsent,
            "Age of digital consent lowered to 15 (Art. 7-1 Loi 78-17)",
            "Article 7-1 of Loi 78-17 lowers the Art 8(1) GDPR default from 16 to 15: a \
             minor may consent alone to the processing of personal data in relation to an \
             information-society service offered directly to them from the age of 15. Below \
             15, the processing is lawful only if consent is given jointly by the minor and \
             the holder(s) of parental authority.",
            "Art. 7-1 Loi 78-17",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article9SpecialCategories,
            "Health-data and sensitive-data processing (Arts 6, 44-46 Loi 78-17)",
            "Articles 6 and 44-46 of Loi 78-17 set the conditions and additional \
             formalities for processing special categories of data, including a specific \
             regime for health data (Chapter III, Section 3), exercising the Art 9(2)/(4) \
             GDPR opening clause.",
            "Arts 6, 44-46 Loi 78-17",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article87NationalIdentifiers,
            "Use of the social-security number / NIR (Art. 30 Loi 78-17)",
            "Article 30 of Loi 78-17 regulates the use of the numéro d'inscription au \
             répertoire national d'identification des personnes physiques (NIR), subjecting \
             certain uses to authorisation by decree of the Conseil d'État after the CNIL's \
             opinion, exercising the Art 87 GDPR opening clause.",
            "Art. 30 Loi 78-17",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article85FreedomOfExpression,
            "Journalistic and literary/artistic expression (Art. 80 Loi 78-17)",
            "Article 80 of Loi 78-17 derogates from several GDPR obligations for processing \
             carried out for journalistic purposes or for the purpose of academic, artistic \
             or literary expression, reconciling data protection with freedom of expression \
             under Art 85 GDPR.",
            "Art. 80 Loi 78-17",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article80Representation,
            "Data-protection collective actions (Art. 37 Loi 78-17)",
            "Article 37 of Loi 78-17 enables approved associations and bodies to bring \
             collective actions (actions de groupe) on behalf of data subjects, including \
             claims for compensation, exercising the Art 80 GDPR representation clause.",
            "Art. 37 Loi 78-17",
        ))
        .build()
        .unwrap_or_else(|_| fallback_implementation())
}

/// Minimal fallback used only if the rich builder unexpectedly fails validation.
fn fallback_implementation() -> MemberStateGdpr {
    MemberStateGdpr {
        state: MemberState::France,
        authorities: vec![SupervisoryAuthority::national(
            "Commission Nationale de l'Informatique et des Libertés",
            "CNIL",
            "National Commission on Informatics and Liberty",
            "https://www.cnil.fr",
        )],
        age_of_digital_consent: AGE_OF_DIGITAL_CONSENT,
        national_acts: vec![NationalActCitation::new(
            "Loi 78-17",
            "Loi n° 78-17 du 6 janvier 1978 relative à l'informatique, aux fichiers et aux libertés",
            "Act No. 78-17 of 6 January 1978 on Information Technology, Data Files and Civil Liberties",
            1978,
        )],
        derogations: vec![NationalDerogation::new(
            OpeningClause::Article8AgeOfConsent,
            "Age of digital consent lowered to 15 (Art. 7-1 Loi 78-17)",
            "Article 7-1 of Loi 78-17 lowers the Art 8(1) GDPR default from 16 to 15.",
            "Art. 7-1 Loi 78-17",
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_france_age_of_consent_is_15() {
        let fr = implementation();
        assert_eq!(fr.age_of_digital_consent, 15);
        assert!(fr.has_lowered_age_of_consent());
        // A 15-year-old can consent alone; a 14-year-old needs joint parental consent.
        assert!(fr.can_child_consent(15));
        assert!(!fr.can_child_consent(14));
        assert!(fr.requires_parental_consent(14));
    }

    #[test]
    fn test_france_lead_authority_is_cnil() {
        let fr = implementation();
        assert_eq!(fr.lead_authority().abbreviation, "CNIL");
        assert!(fr.lead_authority().is_national);
        assert!(fr.lead_authority().name.contains("Commission Nationale"));
    }

    #[test]
    fn test_france_age_derogation_article_7_1() {
        let fr = implementation();
        assert!(fr.has_derogation_for(OpeningClause::Article8AgeOfConsent));
        let derog = fr.derogations_for(OpeningClause::Article8AgeOfConsent);
        assert_eq!(derog.len(), 1);
        assert_eq!(derog[0].national_citation, "Art. 7-1 Loi 78-17");
        assert!(derog[0].summary.contains("15"));
    }

    #[test]
    fn test_france_nir_derogation() {
        let fr = implementation();
        assert!(fr.has_derogation_for(OpeningClause::Article87NationalIdentifiers));
        let nir = fr.derogations_for(OpeningClause::Article87NationalIdentifiers);
        assert_eq!(nir[0].national_citation, "Art. 30 Loi 78-17");
    }

    #[test]
    fn test_france_cites_loi_78_17() {
        let fr = implementation();
        assert!(
            fr.national_acts
                .iter()
                .any(|a| a.short_citation == "Loi 78-17")
        );
    }

    #[test]
    fn test_france_validates() {
        assert!(implementation().validate().is_ok());
    }
}
