//! Italy — Codice in materia di protezione dei dati personali (Codice Privacy).
//!
//! Italy implements the GDPR through the **Codice in materia di protezione dei dati
//! personali** (the "Codice Privacy"), originally enacted as **Decreto Legislativo 30
//! giugno 2003, n. 196** and substantially amended to align with the GDPR by **Decreto
//! Legislativo 10 agosto 2018, n. 101** (in force 19 September 2018). D.Lgs. 101/2018
//! inserted a series of "bis/ter/quater/quinquies" articles into the 2003 Code.
//!
//! Key Italian specifics modelled here:
//!
//! - **Age of digital consent: 14** — Italy lowered the Article 8(1) GDPR default of 16
//!   to **14** via **Article 2-quinquies** of the Codice Privacy (inserted by D.Lgs.
//!   101/2018). A minor aged 14 or over may give valid consent to information-society
//!   services; below 14 consent must be given by the holder of parental responsibility.
//! - **Supervisory authority** — the **Garante per la protezione dei dati personali**
//!   (the "Garante"), the Italian Data Protection Authority (Articles 153 ff. Codice).
//! - **Special categories & genetic/biometric/health data** — Article 2-sexies (public
//!   interest) and Article 2-septies (Garante safeguarding measures for genetic,
//!   biometric and health data) Codice Privacy specify the conditions of Art 9(2)/(4)
//!   GDPR; Article 2-septies in particular requires the Garante's periodic
//!   *provvedimento* setting additional safeguards.
//! - **Criminal-conviction data** — Article 2-octies Codice Privacy regulates processing
//!   of data relating to criminal convictions and offences (Art 10 GDPR).
//! - **Journalism** — Articles 136-139 Codice Privacy and the journalists' code of
//!   conduct reconcile data protection with freedom of expression (Art 85 GDPR).
//! - **Criminal sanctions** — Articles 167-172 Codice Privacy retain national criminal
//!   offences (e.g. unlawful processing) alongside the GDPR's administrative fines.

use crate::member_states::template::{
    MemberStateGdpr, NationalActCitation, NationalDerogation, OpeningClause, SupervisoryAuthority,
};
use crate::shared::MemberState;

/// Italy's age of digital consent under Article 8(1) GDPR, lowered to 14 by Article
/// 2-quinquies of the Codice Privacy.
pub const AGE_OF_DIGITAL_CONSENT: u8 = 14;

/// Build the Italian national GDPR implementation (Codice Privacy).
///
/// ## Example
///
/// ```rust
/// use legalis_eu::member_states::italy;
///
/// let it = italy::implementation();
/// assert_eq!(it.age_of_digital_consent, 14);
/// assert_eq!(it.lead_authority().abbreviation, "Garante");
/// assert!(it.has_lowered_age_of_consent());
/// ```
pub fn implementation() -> MemberStateGdpr {
    MemberStateGdpr::builder(MemberState::Italy)
        .age_of_digital_consent(AGE_OF_DIGITAL_CONSENT)
        .authority(SupervisoryAuthority::national(
            "Garante per la protezione dei dati personali",
            "Garante",
            "Italian Data Protection Authority",
            "https://www.garanteprivacy.it",
        ))
        .national_act(NationalActCitation::new(
            "D.Lgs. 196/2003",
            "Decreto Legislativo 30 giugno 2003, n. 196 — Codice in materia di protezione dei dati personali",
            "Legislative Decree No. 196 of 30 June 2003 — Personal Data Protection Code",
            2003,
        ))
        .national_act(NationalActCitation::new(
            "D.Lgs. 101/2018",
            "Decreto Legislativo 10 agosto 2018, n. 101",
            "Legislative Decree No. 101 of 10 August 2018 (harmonising the Code with the GDPR)",
            2018,
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article8AgeOfConsent,
            "Age of digital consent lowered to 14 (Art. 2-quinquies Codice)",
            "Article 2-quinquies of the Codice Privacy (inserted by D.Lgs. 101/2018) lowers \
             the Art 8(1) GDPR default from 16 to 14: a minor who has reached 14 years of \
             age may give valid consent in relation to an information-society service. For a \
             minor under 14, consent must be given by the holder of parental \
             responsibility.",
            "Art. 2-quinquies Codice (D.Lgs. 196/2003)",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article9SpecialCategories,
            "Genetic, biometric and health data safeguards (Art. 2-septies Codice)",
            "Article 2-septies of the Codice Privacy requires processing of genetic, \
             biometric and health data to comply with additional safeguarding measures \
             adopted by the Garante (provvedimento), reviewed at least every two years, \
             exercising the Art 9(4) GDPR opening clause. Article 2-sexies governs \
             processing of special categories for reasons of substantial public interest.",
            "Arts 2-sexies, 2-septies Codice",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article10CriminalData,
            "Criminal-conviction data (Art. 2-octies Codice)",
            "Article 2-octies of the Codice Privacy lays down the conditions and safeguards \
             for processing personal data relating to criminal convictions and offences not \
             carried out under the control of official authority, exercising the Art 10 \
             GDPR opening clause.",
            "Art. 2-octies Codice",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article85FreedomOfExpression,
            "Journalistic processing (Arts 136-139 Codice)",
            "Articles 136-139 of the Codice Privacy, together with the deontological rules \
             (code of conduct) for journalistic activity, reconcile the protection of \
             personal data with the right to freedom of expression and information under \
             Art 85 GDPR.",
            "Arts 136-139 Codice",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article90Secrecy,
            "Criminal sanctions for unlawful processing (Arts 167-172 Codice)",
            "Italy retains national criminal offences — including unlawful data processing \
             (Art. 167), unlawful communication and dissemination of large-scale data \
             (Art. 167-bis) and fraudulent acquisition of data (Art. 167-ter) — alongside \
             the GDPR administrative fines, reflecting the member-state competence \
             preserved under Recital 149 and Art 84 GDPR.",
            "Arts 167-172 Codice",
        ))
        .build()
        .unwrap_or_else(|_| fallback_implementation())
}

/// Minimal fallback used only if the rich builder unexpectedly fails validation.
fn fallback_implementation() -> MemberStateGdpr {
    MemberStateGdpr {
        state: MemberState::Italy,
        authorities: vec![SupervisoryAuthority::national(
            "Garante per la protezione dei dati personali",
            "Garante",
            "Italian Data Protection Authority",
            "https://www.garanteprivacy.it",
        )],
        age_of_digital_consent: AGE_OF_DIGITAL_CONSENT,
        national_acts: vec![NationalActCitation::new(
            "D.Lgs. 196/2003",
            "Decreto Legislativo 30 giugno 2003, n. 196 — Codice in materia di protezione dei dati personali",
            "Legislative Decree No. 196 of 30 June 2003 — Personal Data Protection Code",
            2003,
        )],
        derogations: vec![NationalDerogation::new(
            OpeningClause::Article8AgeOfConsent,
            "Age of digital consent lowered to 14 (Art. 2-quinquies Codice)",
            "Article 2-quinquies of the Codice Privacy lowers the Art 8(1) GDPR default to 14.",
            "Art. 2-quinquies Codice (D.Lgs. 196/2003)",
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_italy_age_of_consent_is_14() {
        let it = implementation();
        assert_eq!(it.age_of_digital_consent, 14);
        assert!(it.has_lowered_age_of_consent());
        assert!(it.can_child_consent(14));
        assert!(!it.can_child_consent(13));
        assert!(it.requires_parental_consent(13));
    }

    #[test]
    fn test_italy_lead_authority_is_garante() {
        let it = implementation();
        assert_eq!(it.lead_authority().abbreviation, "Garante");
        assert!(it.lead_authority().is_national);
        assert!(it.lead_authority().name.contains("Garante"));
    }

    #[test]
    fn test_italy_age_derogation_article_2_quinquies() {
        let it = implementation();
        assert!(it.has_derogation_for(OpeningClause::Article8AgeOfConsent));
        let derog = it.derogations_for(OpeningClause::Article8AgeOfConsent);
        assert_eq!(derog.len(), 1);
        assert!(derog[0].national_citation.contains("2-quinquies"));
        assert!(derog[0].summary.contains("14"));
    }

    #[test]
    fn test_italy_criminal_data_derogation() {
        let it = implementation();
        assert!(it.has_derogation_for(OpeningClause::Article10CriminalData));
        let crim = it.derogations_for(OpeningClause::Article10CriminalData);
        assert!(crim[0].national_citation.contains("2-octies"));
    }

    #[test]
    fn test_italy_cites_codice() {
        let it = implementation();
        assert!(
            it.national_acts
                .iter()
                .any(|a| a.short_citation == "D.Lgs. 196/2003")
        );
        assert!(
            it.national_acts
                .iter()
                .any(|a| a.short_citation == "D.Lgs. 101/2018")
        );
    }

    #[test]
    fn test_italy_validates() {
        assert!(implementation().validate().is_ok());
    }
}
