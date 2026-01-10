//! French Patent Law Articles (Code de la propriété intellectuelle, Book VI)
//!
//! This module implements patent law provisions from the CPI, including
//! patentability requirements and patent duration.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article L611-10 - Patent requirements (novelty, inventive step, industrial applicability)
///
/// # French Text
/// "Sont brevetables, dans tous les domaines technologiques, les inventions nouvelles
/// impliquant une activité inventive et susceptibles d'application industrielle."
///
/// # English Translation
/// "Inventions which are new, involve an inventive step and are susceptible of industrial
/// application are patentable in all fields of technology."
///
/// # Legal Commentary
///
/// Article L611-10 establishes the **three cumulative requirements** for patentability
/// in French law, derived from the European Patent Convention (EPC) Article 52:
///
/// ## 1. Novelty (Nouveauté)
/// An invention is **novel** if it does not form part of the **state of the art**.
/// The state of the art comprises everything made available to the public anywhere
/// in the world before the filing date, by any means (written, oral, use, etc.).
///
/// ### Absolute Novelty Standard
/// France follows the **absolute novelty** standard (as opposed to relative novelty):
/// - **Worldwide disclosure** destroys novelty, not just French/European disclosure
/// - **Any form of disclosure**: Written publication, oral presentation, public use
/// - **Grace period**: No grace period in France (unlike USA's 12-month grace period)
///
/// ### Novelty Examples
/// **Novel inventions**:
/// - New chemical compound never synthesized before
/// - Novel mechanical device with unique structural features
/// - New method of treatment using known drugs in unprecedented combination
/// - Software algorithm solving problem in non-obvious way
///
/// **Not novel**:
/// - Pharmaceutical compound disclosed in prior research paper
/// - Device demonstrated at public trade show before filing
/// - Invention disclosed in inventor's own publication (no grace period!)
/// - Prior patent application with earlier priority date
///
/// ## 2. Inventive Step / Non-Obviousness (Activité Inventive)
/// An invention involves an **inventive step** if, having regard to the state of the art,
/// it is **not obvious** to a person skilled in the art (skilled artisan test).
///
/// ### Problem-Solution Approach
/// French/EPO courts use the **problem-solution approach**:
/// 1. Identify the closest prior art
/// 2. Determine the technical problem solved by the invention
/// 3. Assess whether the solution would be obvious to skilled artisan
///
/// ### Obviousness Factors
/// - **Combining known elements**: Obvious combination vs. synergistic effect
/// - **Unexpected advantages**: Superior results indicate inventive step
/// - **Long-felt need**: Problem existed but unsolved suggests non-obviousness
/// - **Commercial success**: May indicate inventive merit (weak factor)
/// - **Teaching away**: Prior art suggesting different direction supports inventive step
///
/// ### Inventive Step Examples
/// **Inventive inventions**:
/// - Synergistic drug combination (1+1=3 effect)
/// - Novel use of known compound for different purpose
/// - Combination solving problem prior art taught away from
/// - Unexpected superior performance in known device
///
/// **Obvious inventions**:
/// - Mere aggregation of known features without synergy
/// - Applying known technique to similar problem (routine engineering)
/// - Selecting from limited options suggested by prior art
/// - Obvious to try approach with reasonable expectation of success
///
/// ## 3. Industrial Applicability (Application Industrielle)
/// An invention is **industrially applicable** if it can be **made or used** in any kind
/// of industry, including agriculture, fishing, and services.
///
/// ### Broad Definition
/// "Industry" is interpreted **very broadly**:
/// - Manufacturing (cars, electronics, pharmaceuticals)
/// - Agriculture (plant varieties, farming methods)
/// - Services (logistics methods, data processing)
/// - Software (if technical effect beyond computer itself)
///
/// ### Non-Applicable Inventions
/// **Not industrially applicable**:
/// - Purely aesthetic creations (protectable as designs)
/// - Mathematical methods as such (no technical application)
/// - Business methods as such (unless technical implementation)
/// - Surgical/therapeutic methods practiced on human/animal body (Art. L611-16)
///
/// ### Software Patentability
/// Software is patentable if it has **technical character** beyond normal computer operation:
/// - **Patentable**: Image compression algorithm, industrial control system, medical diagnostic software
/// - **Not patentable**: Business logic, abstract algorithms, user interface designs
///
/// ## Historical Context
/// Article L611-10 implements EPC Article 52, harmonizing French law with European patent law.
/// France is a contracting state to the **European Patent Convention (1973)**, allowing patents
/// to be obtained through national route (INPI) or European route (EPO).
///
/// ### Legislative Evolution
/// - **1844**: First French patent law (ordonnance)
/// - **1968**: Law 68-1 reformed patent system
/// - **1978**: EPC entered into force for France
/// - **1992**: CPI codified IP laws (including patents in Book VI)
/// - **2014**: Patent box regime (reduced tax on patent income)
/// - **2024**: Ongoing harmonization with Unified Patent Court (UPC)
///
/// ## International Comparison
///
/// ### United States (35 USC §101-103)
/// - **Subject matter**: Broader than France (business methods historically patentable)
/// - **Novelty**: Absolute worldwide novelty (since AIA 2011), but 12-month grace period
/// - **Non-obviousness**: Similar to inventive step, Graham factors test
/// - **Utility**: Corresponds to industrial applicability, but more flexible
///
/// ### Germany (PatG §1-5)
/// - **Requirements**: Identical to France (both follow EPC)
/// - **Examination**: German Patent Office (DPMA) applies same standards as EPO
/// - **Utility models**: Germany has utility model (Gebrauchsmuster) for minor inventions
///
/// ### Japan (Patent Act §29)
/// - **Novelty**: Absolute worldwide, but 6-month grace period
/// - **Inventive step**: Similar standard, but more emphasis on unexpected advantages
/// - **Industrial applicability**: Broader than medical methods restriction
///
/// ### United Kingdom (Patents Act 1977 §1)
/// - **Requirements**: Identical (implements EPC)
/// - **Brexit impact**: UK left EU but remains EPC member, standards unchanged
///
/// ### China (Patent Law §22)
/// - **Novelty**: Absolute worldwide (since 2009 amendment)
/// - **Inventiveness**: Lower threshold than France/EPO in practice
/// - **Utility**: Similar standard, but traditional medicine methods patentable
///
/// ## Exclusions from Patentability (Article L611-10 to L611-19)
///
/// ### Article L611-16: Medical Methods
/// Surgical, therapeutic, and diagnostic methods practiced on human/animal body **not patentable**
/// (but pharmaceutical products and medical devices are patentable).
///
/// ### Article L611-17: Ordre Public and Morality
/// Inventions contrary to **public order or morality** not patentable:
/// - Human cloning processes
/// - Modification of human germline genetic identity
/// - Commercial use of human embryos
/// - Processes causing animal suffering without medical benefit
///
/// ### Article L611-19: Biological Inventions
/// - **Plant/animal varieties**: Not patentable (protectable by plant variety rights)
/// - **Essentially biological processes**: Traditional breeding not patentable
/// - **Biotechnological inventions**: Patentable if technical process (recombinant DNA)
/// - **Human gene sequences**: Patentable only if industrial application disclosed
///
/// ## Modern Applications and Controversies
///
/// ### Software Patents
/// **French approach** (via EPO jurisprudence):
/// - Pure software "as such" not patentable (Art. L611-10(2)(3))
/// - Software with **technical effect** beyond computer patentable
/// - **Examples**: Image processing (patentable), accounting software (not patentable)
///
/// **Controversy**: USA historically more permissive (post-Alice 2014, converging toward Europe)
///
/// ### Artificial Intelligence Inventions
/// **Current issues**:
/// - **AI-generated inventions**: Can AI be named as inventor? (UK court: No, 2023)
/// - **AI-implemented methods**: Patentable if technical effect (medical diagnosis, control systems)
/// - **Machine learning algorithms**: Generally patentable if technical application
///
/// ### Pharmaceutical Patents
/// **Special considerations**:
/// - **Second medical use claims**: New therapeutic use of known compound patentable (Swiss-type claims)
/// - **Markush structures**: Generic chemical formulas covering variations
/// - **Evergreening concerns**: Critics argue minor variations extend monopoly unfairly
/// - **Compulsory licenses**: Government can override patent for public health (rare)
///
/// ### Green Technology
/// **Accelerated examination**:
/// - France/EPO offer expedited examination for green tech patents
/// - Incentive to patent clean energy, pollution reduction inventions
/// - Patent box tax benefits may apply (15% reduced rate)
///
/// ## Practical Examination Tips
///
/// ### INPI Examination Procedure
/// 1. **Filing**: Submit application to INPI (national route) or EPO (European route)
/// 2. **Formality check**: INPI verifies formal requirements within 1 month
/// 3. **Search report**: INPI or EPO searches prior art (novelty/inventive step)
/// 4. **Publication**: Application published 18 months after filing (Art. L612-21)
/// 5. **Substantive examination**: Examiner assesses patentability requirements
/// 6. **Office actions**: Applicant may need to respond to objections
/// 7. **Grant or refusal**: If requirements met, patent granted
///
/// ### Costs (INPI, 2024)
/// - **Filing fee**: €26 (electronic) or €36 (paper)
/// - **Search report**: €520
/// - **Grant fee**: €90
/// - **Annual renewal**: €38-800 (increases over 20 years)
/// - **Total for 20 years**: ~€10,000
///
/// **EPO route**: More expensive (~€5,000-10,000 for filing/examination) but covers multiple countries.
///
/// ## Examples
///
/// ```rust
/// use legalis_fr::intellectual_property::{Patent, validate_patent};
/// use chrono::NaiveDate;
///
/// // Example 1: Valid pharmaceutical patent
/// let patent = Patent::builder()
///     .title("Novel EGFR Inhibitor for Cancer Treatment".to_string())
///     .inventor("Dr. Marie Curie".to_string())
///     .filing_date(NaiveDate::from_ymd_opt(2024, 1, 15).unwrap())
///     .novelty(true)  // New chemical structure, not in prior art
///     .inventive_step(true)  // Unexpected superior efficacy vs. known inhibitors
///     .industrial_applicability(true)  // Pharmaceutical manufacturing
///     .build()
///     .unwrap();
///
/// // Patent satisfies Article L611-10 requirements
/// assert!(validate_patent(&patent, NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()).is_ok());
///
/// // Example 2: Software patent with technical effect
/// let ai_patent = Patent::builder()
///     .title("Real-time Image Compression Algorithm".to_string())
///     .inventor("Jean Dupont".to_string())
///     .filing_date(NaiveDate::from_ymd_opt(2024, 3, 1).unwrap())
///     .novelty(true)  // Novel compression technique
///     .inventive_step(true)  // Non-obvious improvement in compression ratio
///     .industrial_applicability(true)  // Technical effect: improved image processing
///     .build()
///     .unwrap();
///
/// // Patentable because technical effect beyond normal computer operation
/// assert!(validate_patent(&ai_patent, NaiveDate::from_ymd_opt(2024, 6, 1).unwrap()).is_ok());
/// ```
pub fn article_l611_10() -> Statute {
    Statute::new(
        "cpi-l611-10",
        "CPI Article L611-10 - Patentability requirements (novelty, inventive step, industrial applicability)",
        Effect::new(
            EffectType::Grant,
            "Invention patentable if novel, involves inventive step, and susceptible of industrial application",
        )
        .with_parameter("requirement_1", "novelty")
        .with_parameter("requirement_2", "inventive_step")
        .with_parameter("requirement_3", "industrial_applicability")
        .with_parameter("novelty_standard", "absolute_worldwide")
        .with_parameter("inventive_step_test", "skilled_artisan_obvious_test")
        .with_parameter("basis", "european_patent_convention_art52")
        .with_parameter("all_fields", "technology_all_domains"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "novelty".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "inventive_step".to_string(),
                value: "true".to_string(),
            }),
        )),
        Box::new(Condition::AttributeEquals {
            key: "industrial_applicability".to_string(),
            value: "true".to_string(),
        }),
    ))
    .with_discretion(
        "Patentability requirements: (1) NOVELTY - not part of state of the art (absolute worldwide \
         standard, any public disclosure anywhere destroys novelty, no grace period unlike USA); \
         (2) INVENTIVE STEP - not obvious to skilled artisan (problem-solution approach, synergistic \
         effect indicates inventiveness); (3) INDUSTRIAL APPLICABILITY - can be made/used in industry \
         (broadly defined: manufacturing, agriculture, services). EPC Art. 52 basis. Software patentable \
         if technical effect beyond computer. Medical methods not patentable (Art. L611-16) but drugs are. \
         Exclusions: human cloning, germline modification, embryo use. Modern issues: AI inventions, \
         software patents, pharmaceutical evergreening, green tech acceleration. Compare: USA 35 USC §101-103 \
         (12-month grace period, broader subject matter post-Alice 2014), Germany PatG §1-5 (identical EPC), \
         Japan Patent Act §29 (6-month grace period), China Patent Law §22 (similar but lower inventiveness bar)."
    )
}

/// Article L611-11 - Patent duration (20 years from filing)
///
/// # French Text
/// "La durée du brevet est de vingt ans à compter de la date de dépôt de la demande."
///
/// # English Translation
/// "The term of a patent shall be twenty years from the filing date of the application."
///
/// # Legal Commentary
///
/// Article L611-11 establishes the **20-year patent term** from the filing date,
/// harmonized internationally by the TRIPS Agreement (Article 33).
///
/// ## Patent Term Calculation
///
/// ### Filing Date (Date de Dépôt)
/// - Term runs from **filing date**, not grant date
/// - **Priority date** (Paris Convention): Can claim earlier filing date from foreign application (12 months)
/// - **Effective date**: Patent rights only enforceable from grant date, but term starts from filing
///
/// ### 20-Year Term
/// **Rationale for 20 years**:
/// - Balance between inventor reward and public access
/// - International harmonization (TRIPS Art. 33)
/// - Sufficient time to recoup R&D investment (especially pharmaceuticals)
///
/// **Not extendable** except:
/// - Supplementary Protection Certificates (SPCs) for pharmaceuticals/plant protection products (+5 years max)
/// - No general extension mechanism (unlike USA's patent term adjustment for PTO delays)
///
/// ## Supplementary Protection Certificates (SPCs)
///
/// ### Legal Basis
/// - **EU Regulation 469/2009** (pharmaceuticals)
/// - **EU Regulation 1610/96** (plant protection products)
/// - Implemented in France via Article L611-3
///
/// ### Eligibility
/// **Criteria**:
/// 1. Valid patent protecting pharmaceutical/plant product
/// 2. Valid marketing authorization (MA) required before commercialization
/// 3. First MA in EU for that product
/// 4. No prior SPC for same product
///
/// ### Duration Calculation
/// **SPC term** = Time between patent filing and MA grant - 5 years (max 5 years SPC)
///
/// **Example**:
/// - Patent filed: 2000
/// - MA granted: 2012 (12 years later)
/// - SPC duration: 12 - 5 = 7 years, capped at 5 years max
/// - **Result**: 5-year SPC (patent expires 2020 + 5 = 2025 instead of 2020)
///
/// ### Pediatric Extension
/// **Additional 6 months** if pediatric studies completed (Regulation 1901/2006)
/// - Total max protection: 20 + 5 + 0.5 = 25.5 years
///
/// ## Historical Context
///
/// ### International Harmonization
/// - **Paris Convention (1883)**: Minimum patent protection, priority system
/// - **TRIPS Agreement (1994)**: Mandates 20-year term from filing (Art. 33)
/// - **Pre-TRIPS**: Some countries had shorter terms (e.g., 15-17 years)
/// - **France pre-1992**: 20-year term already (ahead of TRIPS)
///
/// ### Rationale Evolution
/// **1800s**: Shorter terms (7-14 years) sufficient for mechanical inventions
/// **1900s**: Chemical/pharmaceutical inventions require longer R&D recoupment
/// **Today**: Debate over whether 20 years appropriate for fast-moving fields (software, AI)
///
/// ## International Comparison
///
/// ### United States (35 USC §154)
/// - **Standard term**: 20 years from filing (post-TRIPS)
/// - **Patent Term Adjustment (PTA)**: Additional time for PTO delays
/// - **Patent Term Extension (PTE)**: Up to 5 years for regulatory review (similar to SPC)
/// - **Maintenance fees**: Required at 3.5, 7.5, 11.5 years (or patent expires)
///
/// ### Germany (PatG §16)
/// - **Standard term**: 20 years from filing (EPC harmonized)
/// - **SPCs**: Same EU regulations apply
/// - **Utility models**: Separate 10-year protection for minor inventions
///
/// ### Japan (Patent Act §67)
/// - **Standard term**: 20 years from filing
/// - **Extension**: Up to 5 years for regulatory delay (pharmaceuticals, agrochemicals)
/// - **No PTA**: Unlike USA, no adjustment for JPO examination delays
///
/// ### United Kingdom (Patents Act 1977 §25)
/// - **Standard term**: 20 years from filing (EPC member)
/// - **SPCs**: Applied EU regulations (post-Brexit, retained in UK law)
///
/// ### China (Patent Law §42)
/// - **Invention patents**: 20 years from filing
/// - **Utility models**: 10 years from filing (lower inventiveness threshold)
/// - **Design patents**: 15 years from filing
///
/// ## Maintenance and Renewal
///
/// ### Annual Fees (Annuités)
/// French patents require **annual maintenance fees** to remain in force:
/// - Years 1-5: €38/year
/// - Years 6-10: €70-130/year (increasing)
/// - Years 11-20: €240-800/year (increasing)
///
/// **Failure to pay**: Patent lapses, cannot be revived (unlike USA's 2-year revival window)
///
/// ### Strategic Abandonment
/// Patent owners often **abandon patents early** if:
/// - Product no longer commercially viable
/// - Maintenance costs exceed value
/// - Statistics: ~50% of patents abandoned before year 10
///
/// ## Expiry and Public Domain
///
/// ### Patent Expiry Effects
/// Upon expiry (20 years + any SPC):
/// 1. **Invention enters public domain**: Anyone can freely use
/// 2. **Generic competition**: Pharmaceutical generics can launch
/// 3. **No revival**: Term cannot be extended (except SPC if eligible)
/// 4. **Knowledge remains**: Technical disclosure remains publicly available
///
/// ### Post-Expiry Considerations
/// - **Trade secrets**: May protect know-how beyond patent term
/// - **Trademark**: Brand remains protected indefinitely
/// - **Regulatory exclusivity**: Some countries grant separate data exclusivity (EU: 8+2+1 years)
///
/// ## Modern Issues and Debates
///
/// ### Patent Term Adequacy
/// **Criticism of 20-year term**:
/// - **Pharmaceuticals**: Long R&D (10-15 years) leaves short market exclusivity
/// - **Software**: Rapid innovation cycle makes 20 years excessive (obsolete before expiry)
/// - **Proposal**: Variable terms by technology field (rejected for complexity)
///
/// ### Evergreening Concerns
/// **Strategies to extend effective monopoly**:
/// - Filing continuation patents on minor improvements
/// - Patenting metabolites, formulations, delivery methods
/// - **Criticism**: Delays generic entry, raises drug costs
/// - **Defense**: Each patent must meet patentability requirements
///
/// ### Regulatory Data Exclusivity
/// **Separate from patents**:
/// - EU grants 8+2+1 years data exclusivity (independent of patent)
/// - Prevents generics from relying on originator's clinical data
/// - **Cumulative protection**: Patent + data exclusivity can exceed 25 years
///
/// ## Examples
///
/// ```rust
/// use legalis_fr::intellectual_property::{Patent, validate_patent_duration};
/// use chrono::{NaiveDate, Datelike};
///
/// // Example 1: Patent within 20-year term
/// let patent = Patent::builder()
///     .title("Novel Drug Compound".to_string())
///     .inventor("Dr. Pasteur".to_string())
///     .filing_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
///     .novelty(true)
///     .inventive_step(true)
///     .industrial_applicability(true)
///     .build()
///     .unwrap();
///
/// // Check in 2025 - patent still valid (5 years elapsed, 15 remaining)
/// let current = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
/// assert!(!patent.is_expired(current));
/// assert!(validate_patent_duration(&patent, current).is_ok());
///
/// // Expiry date is 2039 (filing date + 365*20 days)
/// let expiry = patent.expiry_date();
/// assert_eq!(expiry.year(), 2039);
///
/// // Example 2: Expired patent
/// let old_patent = Patent::builder()
///     .title("19th Century Invention".to_string())
///     .inventor("M. Curie".to_string())
///     .filing_date(NaiveDate::from_ymd_opt(1990, 1, 1).unwrap())
///     .novelty(true)
///     .inventive_step(true)
///     .industrial_applicability(true)
///     .build()
///     .unwrap();
///
/// // In 2025, patent expired in 2010 (20 years after 1990 filing)
/// assert!(old_patent.is_expired(current));
/// assert!(validate_patent_duration(&old_patent, current).is_err());
/// ```
pub fn article_l611_11() -> Statute {
    Statute::new(
        "cpi-l611-11",
        "CPI Article L611-11 - Patent duration (20 years from filing date)",
        Effect::new(
            EffectType::Grant,
            "Patent protection lasts 20 years from filing date, not extendable except SPCs",
        )
        .with_parameter("duration_years", "20")
        .with_parameter("start_date", "filing_date")
        .with_parameter("not_grant_date", "true")
        .with_parameter("trips_basis", "trips_art33")
        .with_parameter("extension_spc", "pharmaceuticals_plant_protection_max_5years")
        .with_parameter("maintenance_fees", "annual_fees_required"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::HasAttribute {
            key: "patent_filing_date".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "patent_type".to_string(),
            value: "invention".to_string(),
        }),
    ))
    .with_discretion(
        "Patent term: 20 years from filing date (not grant date), per TRIPS Art. 33. Term not \
         extendable EXCEPT: (1) Supplementary Protection Certificates (SPCs) for pharmaceuticals/plant \
         protection products (+5 years max, EU Reg 469/2009, pediatric +6 months possible = 25.5 years max); \
         (2) No general extension for PTO delays (unlike USA PTA). Annual maintenance fees required (€38-800, \
         increasing over time) or patent lapses. Priority date (Paris Convention) can establish earlier \
         filing date (12-month priority period). Upon expiry: invention enters public domain, generic \
         competition allowed, no revival. Debate: 20 years excessive for fast fields (software), insufficient \
         for slow fields (pharma with 10-15 year R&D). Evergreening criticism: continuation patents extend \
         effective monopoly. Compare: USA 35 USC §154 (20 years + PTA/PTE), Germany PatG §16 (identical), \
         Japan Patent Act §67 (20 years + 5-year extension), China Patent Law §42 (invention 20y, utility 10y)."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_l611_10_structure() {
        let statute = article_l611_10();
        assert_eq!(statute.id, "cpi-l611-10");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("L611-10"));
        assert!(statute.title.contains("Patentability"));
    }

    #[test]
    fn test_article_l611_10_parameters() {
        let statute = article_l611_10();
        let params = &statute.effect.parameters;
        assert_eq!(params.get("requirement_1").unwrap(), "novelty");
        assert_eq!(params.get("requirement_2").unwrap(), "inventive_step");
        assert_eq!(
            params.get("requirement_3").unwrap(),
            "industrial_applicability"
        );
        assert_eq!(
            params.get("novelty_standard").unwrap(),
            "absolute_worldwide"
        );
    }

    #[test]
    fn test_article_l611_10_preconditions() {
        let statute = article_l611_10();
        assert_eq!(statute.preconditions.len(), 1);
    }

    #[test]
    fn test_article_l611_10_discretion() {
        let statute = article_l611_10();
        assert!(statute.has_discretion());
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("NOVELTY"));
        assert!(discretion.contains("INVENTIVE STEP"));
        assert!(discretion.contains("INDUSTRIAL APPLICABILITY"));
    }

    #[test]
    fn test_article_l611_11_structure() {
        let statute = article_l611_11();
        assert_eq!(statute.id, "cpi-l611-11");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("L611-11"));
        assert!(statute.title.contains("duration"));
    }

    #[test]
    fn test_article_l611_11_parameters() {
        let statute = article_l611_11();
        let params = &statute.effect.parameters;
        assert_eq!(params.get("duration_years").unwrap(), "20");
        assert_eq!(params.get("start_date").unwrap(), "filing_date");
        assert_eq!(params.get("trips_basis").unwrap(), "trips_art33");
    }

    #[test]
    fn test_article_l611_11_preconditions() {
        let statute = article_l611_11();
        assert_eq!(statute.preconditions.len(), 1);
    }

    #[test]
    fn test_article_l611_11_discretion() {
        let statute = article_l611_11();
        assert!(statute.has_discretion());
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("20 years"));
        assert!(discretion.contains("filing date"));
        assert!(discretion.contains("TRIPS"));
    }

    #[test]
    fn test_all_patent_articles_have_effect_type() {
        let articles = vec![article_l611_10(), article_l611_11()];
        for article in articles {
            assert!(matches!(article.effect.effect_type, EffectType::Grant));
        }
    }
}
