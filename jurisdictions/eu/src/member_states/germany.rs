//! Germany — Bundesdatenschutzgesetz (BDSG) national GDPR implementation.
//!
//! Germany implements the GDPR's opening clauses through the **Bundesdatenschutzgesetz**
//! (BDSG) of 30 June 2017 (BGBl. I S. 2097), which entered into force together with the
//! GDPR on **25 May 2018** and replaced the BDSG of 1990/2003. The 2017 BDSG was adopted
//! by the *Datenschutz-Anpassungs- und -Umsetzungsgesetz EU* (DSAnpUG-EU) and also
//! transposes the Law Enforcement Directive (EU) 2016/680 (Part 3 BDSG).
//!
//! Key German specifics modelled here:
//!
//! - **Age of digital consent: 16** — Germany did *not* lower the Article 8(1) GDPR
//!   default; the default of 16 applies.
//! - **Supervisory authorities** — at federal level the **BfDI** (Bundesbeauftragte für
//!   den Datenschutz und die Informationsfreiheit, § 8 BDSG) supervises federal public
//!   bodies and telecommunications/postal providers; the **private sector** and *Land*
//!   public bodies are supervised by the **16 *Land* data protection authorities**
//!   (§ 40 BDSG). Coordination occurs through the *Datenschutzkonferenz* (DSK).
//! - **Employee data (§ 26 BDSG, Art 88 GDPR)** — special rules for processing in the
//!   employment context. Note: the CJEU in *Case C-34/21, Hauptpersonalrat der
//!   Lehrerinnen und Lehrer* (30 March 2023) held that § 23(1) sentence 1 of the Hessian
//!   Data Protection Act (a near-identical wording to § 26(1) BDSG) did not satisfy the
//!   conditions of Art 88(2) GDPR; this is recorded in the derogation summary.
//! - **DPO designation (§ 38 BDSG, Art 37(4) GDPR)** — Germany requires a DPO where the
//!   controller/processor constantly employs as a rule at least **20 persons** with the
//!   automated processing of personal data (a national addition to Art 37(1)).
//! - **Processing for other purposes & special categories** — §§ 22-24 BDSG specify
//!   conditions for processing special categories and for further processing by public
//!   and private bodies (Art 6(4)/Art 9(2) GDPR opening clauses).

use crate::member_states::template::{
    MemberStateGdpr, NationalActCitation, NationalDerogation, OpeningClause, SupervisoryAuthority,
};
use crate::shared::MemberState;

/// Germany's age of digital consent under Article 8(1) GDPR (the GDPR default; § 8(1)
/// GDPR not lowered by the BDSG).
pub const AGE_OF_DIGITAL_CONSENT: u8 = 16;

/// Build the German national GDPR implementation (BDSG).
///
/// ## Example
///
/// ```rust
/// use legalis_eu::member_states::germany;
///
/// let de = germany::implementation();
/// assert_eq!(de.age_of_digital_consent, 16);
/// assert_eq!(de.lead_authority().abbreviation, "BfDI");
/// // 16 Land DPAs are recorded in addition to the federal BfDI.
/// assert_eq!(de.regional_authorities().len(), 16);
/// ```
pub fn implementation() -> MemberStateGdpr {
    let builder = MemberStateGdpr::builder(MemberState::Germany)
        .age_of_digital_consent(AGE_OF_DIGITAL_CONSENT)
        .authority(SupervisoryAuthority::national(
            "Die Bundesbeauftragte für den Datenschutz und die Informationsfreiheit",
            "BfDI",
            "Federal Commissioner for Data Protection and Freedom of Information",
            "https://www.bfdi.bund.de",
        ))
        .national_act(NationalActCitation::new(
            "BDSG",
            "Bundesdatenschutzgesetz",
            "Federal Data Protection Act",
            2018,
        ))
        .national_act(NationalActCitation::new(
            "DSAnpUG-EU",
            "Datenschutz-Anpassungs- und -Umsetzungsgesetz EU",
            "EU Data Protection Adaptation and Implementation Act",
            2017,
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article88Employment,
            "Employee data protection (§ 26 BDSG)",
            "§ 26 BDSG permits processing of employees' personal data where necessary for \
             the decision to establish, or after establishment for the performance or \
             termination of, the employment relationship, and regulates consent and the \
             processing of special categories in the employment context. Note: the CJEU \
             (C-34/21, 30 March 2023) found materially identical wording insufficient under \
             Art 88(2) GDPR, casting doubt on § 26(1) sentence 1 as a stand-alone basis.",
            "§ 26 BDSG",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article37DpoDesignation,
            "Mandatory DPO designation threshold (§ 38 BDSG)",
            "§ 38(1) BDSG requires controllers and processors to designate a data \
             protection officer where they constantly employ, as a rule, at least 20 \
             persons in the automated processing of personal data — a national addition to \
             the Art 37(1) GDPR criteria.",
            "§ 38 BDSG",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article9SpecialCategories,
            "Processing of special categories (§ 22 BDSG)",
            "§ 22 BDSG sets out the conditions under which public and private bodies may \
             process special categories of personal data (e.g. for health care, social \
             security, substantial public interest) together with the specific safeguards \
             required, exercising the Art 9(2)/(4) GDPR opening clause.",
            "§ 22 BDSG",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article6SpecificProvisions,
            "Processing for other purposes (§§ 23-24 BDSG)",
            "§ 23 BDSG (public bodies) and § 24 BDSG (private bodies) specify when further \
             processing for a purpose other than that for which the data were collected is \
             permitted, implementing the Art 6(4) compatibility assessment in national law.",
            "§§ 23-24 BDSG",
        ))
        .derogation(NationalDerogation::new(
            OpeningClause::Article10CriminalData,
            "Processing of criminal-conviction data (§ 22 BDSG)",
            "Germany permits the processing of personal data relating to criminal \
             convictions and offences under the conditions of § 22 BDSG read with Art 10 \
             GDPR, outside the control of an official authority only where authorised by \
             law.",
            "§ 22 BDSG / Art 10 GDPR",
        ));

    // Add the 16 Land (state) supervisory authorities (§ 40 BDSG). Each Land DPA
    // supervises the private sector and Land public bodies in its territory.
    land_authorities()
        .into_iter()
        .fold(builder, |b, authority| b.authority(authority))
        .build()
        .unwrap_or_else(|_| fallback_implementation())
}

/// The 16 *Land* (federal-state) supervisory authorities under § 40 BDSG.
fn land_authorities() -> Vec<SupervisoryAuthority> {
    let entries: [(&str, &str, &str); 16] = [
        (
            "Der Landesbeauftragte für den Datenschutz Baden-Württemberg",
            "LfDI BW",
            "State Commissioner for Data Protection of Baden-Württemberg",
        ),
        (
            "Bayerisches Landesamt für Datenschutzaufsicht",
            "BayLDA",
            "Bavarian State Office for Data Protection Supervision",
        ),
        (
            "Berliner Beauftragte für Datenschutz und Informationsfreiheit",
            "BlnBDI",
            "Berlin Commissioner for Data Protection and Freedom of Information",
        ),
        (
            "Die Landesbeauftragte für den Datenschutz und für das Recht auf Akteneinsicht Brandenburg",
            "LDA Brandenburg",
            "State Commissioner for Data Protection Brandenburg",
        ),
        (
            "Die Landesbeauftragte für Datenschutz und Informationsfreiheit der Freien Hansestadt Bremen",
            "LfDI Bremen",
            "State Commissioner for Data Protection of Bremen",
        ),
        (
            "Der Hamburgische Beauftragte für Datenschutz und Informationsfreiheit",
            "HmbBfDI",
            "Hamburg Commissioner for Data Protection and Freedom of Information",
        ),
        (
            "Der Hessische Beauftragte für Datenschutz und Informationsfreiheit",
            "HBDI",
            "Hessian Commissioner for Data Protection and Freedom of Information",
        ),
        (
            "Der Landesbeauftragte für Datenschutz und Informationsfreiheit Mecklenburg-Vorpommern",
            "LfDI MV",
            "State Commissioner for Data Protection Mecklenburg-Vorpommern",
        ),
        (
            "Die Landesbeauftragte für den Datenschutz Niedersachsen",
            "LfD Niedersachsen",
            "State Commissioner for Data Protection of Lower Saxony",
        ),
        (
            "Die Landesbeauftragte für Datenschutz und Informationsfreiheit Nordrhein-Westfalen",
            "LDI NRW",
            "State Commissioner for Data Protection North Rhine-Westphalia",
        ),
        (
            "Der Landesbeauftragte für den Datenschutz und die Informationsfreiheit Rheinland-Pfalz",
            "LfDI RLP",
            "State Commissioner for Data Protection Rhineland-Palatinate",
        ),
        (
            "Unabhängiges Datenschutzzentrum Saarland",
            "UDS Saarland",
            "Independent Data Protection Centre Saarland",
        ),
        (
            "Die Sächsische Datenschutz- und Transparenzbeauftragte",
            "SDTB",
            "Saxon Commissioner for Data Protection and Transparency",
        ),
        (
            "Landesbeauftragter für den Datenschutz Sachsen-Anhalt",
            "LfD Sachsen-Anhalt",
            "State Commissioner for Data Protection Saxony-Anhalt",
        ),
        (
            "Unabhängiges Landeszentrum für Datenschutz Schleswig-Holstein",
            "ULD",
            "Independent State Centre for Data Protection Schleswig-Holstein",
        ),
        (
            "Der Thüringer Landesbeauftragte für den Datenschutz und die Informationsfreiheit",
            "TLfDI",
            "Thuringian State Commissioner for Data Protection and Freedom of Information",
        ),
    ];

    entries
        .iter()
        .map(|(name, abbr, name_en)| {
            SupervisoryAuthority::regional(
                *name,
                *abbr,
                *name_en,
                "https://www.datenschutzkonferenz-online.de",
            )
        })
        .collect()
}

/// Minimal fallback used only if the rich builder unexpectedly fails validation.
///
/// This keeps [`implementation`] total without `unwrap`/`panic`. It encodes the same
/// load-bearing facts (BfDI, age 16, BDSG) and always passes validation.
fn fallback_implementation() -> MemberStateGdpr {
    MemberStateGdpr {
        state: MemberState::Germany,
        authorities: vec![SupervisoryAuthority::national(
            "Die Bundesbeauftragte für den Datenschutz und die Informationsfreiheit",
            "BfDI",
            "Federal Commissioner for Data Protection and Freedom of Information",
            "https://www.bfdi.bund.de",
        )],
        age_of_digital_consent: AGE_OF_DIGITAL_CONSENT,
        national_acts: vec![NationalActCitation::new(
            "BDSG",
            "Bundesdatenschutzgesetz",
            "Federal Data Protection Act",
            2018,
        )],
        derogations: vec![NationalDerogation::new(
            OpeningClause::Article88Employment,
            "Employee data protection (§ 26 BDSG)",
            "§ 26 BDSG governs processing of employees' personal data.",
            "§ 26 BDSG",
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_germany_age_of_consent_is_default_16() {
        let de = implementation();
        assert_eq!(de.age_of_digital_consent, 16);
        assert!(!de.has_lowered_age_of_consent());
        assert!(de.can_child_consent(16));
        assert!(de.requires_parental_consent(15));
    }

    #[test]
    fn test_germany_lead_authority_is_bfdi() {
        let de = implementation();
        assert_eq!(de.lead_authority().abbreviation, "BfDI");
        assert!(de.lead_authority().is_national);
        assert!(de.lead_authority().name.contains("Bundesbeauftragte"));
    }

    #[test]
    fn test_germany_has_16_land_authorities() {
        let de = implementation();
        assert_eq!(de.regional_authorities().len(), 16);
        // Spot-check a couple of well-known Land DPAs.
        let abbrevs: Vec<&str> = de
            .regional_authorities()
            .iter()
            .map(|a| a.abbreviation.as_str())
            .collect();
        assert!(abbrevs.contains(&"BayLDA"));
        assert!(abbrevs.contains(&"ULD"));
    }

    #[test]
    fn test_germany_employee_derogation_section_26() {
        let de = implementation();
        assert!(de.has_derogation_for(OpeningClause::Article88Employment));
        let derogations = de.derogations_for(OpeningClause::Article88Employment);
        assert_eq!(derogations.len(), 1);
        assert_eq!(derogations[0].national_citation, "§ 26 BDSG");
        assert!(derogations[0].summary.contains("employment"));
    }

    #[test]
    fn test_germany_dpo_threshold_derogation() {
        let de = implementation();
        assert!(de.has_derogation_for(OpeningClause::Article37DpoDesignation));
        let dpo = de.derogations_for(OpeningClause::Article37DpoDesignation);
        assert!(dpo[0].summary.contains("20"));
        assert_eq!(dpo[0].national_citation, "§ 38 BDSG");
    }

    #[test]
    fn test_germany_validates() {
        assert!(implementation().validate().is_ok());
    }
}
