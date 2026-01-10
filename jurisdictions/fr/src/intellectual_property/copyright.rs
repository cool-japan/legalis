//! French Copyright Law Articles (Code de la propriété intellectuelle, Books I & III)
//!
//! This module implements copyright (droit d'auteur) provisions from the CPI,
//! including copyright scope and duration.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article L122-1 - Copyright scope (moral rights + economic rights)
///
/// # French Text
/// "Le droit d'exploitation appartenant à l'auteur comprend le droit de représentation
/// et le droit de reproduction. L'auteur a seul le droit d'autoriser la représentation
/// et la reproduction de son œuvre sous quelque forme que ce soit."
///
/// # English Translation
/// "The exploitation right belonging to the author includes the right of public performance
/// and the right of reproduction. The author alone has the right to authorize the performance
/// and reproduction of their work in any form whatsoever."
///
/// # Legal Commentary
///
/// Article L122-1 establishes the **dual nature of French copyright**: moral rights
/// (droits moraux) and economic rights (droits patrimoniaux). This reflects the
/// **author-centric philosophy** of French copyright law (droit d'auteur = author's right),
/// contrasting with the Anglo-American "copyright" concept.
///
/// ## Economic Rights (Droits Patrimoniaux)
///
/// ### 1. Right of Reproduction (Droit de Reproduction)
/// **Definition**: Fixed or transient copy in any medium (Article L122-3)
///
/// **Forms of reproduction**:
/// - **Physical**: Printing books, photocopying, manufacturing CDs/DVDs
/// - **Digital**: Downloading files, copying to hard drive, RAM copying
/// - **Broadcasting**: Radio/TV transmission creates temporary reproduction
/// - **Streaming**: Transient buffer copies constitute reproduction
///
/// **Modern applications**:
/// - **Cloud storage**: Uploading to servers = reproduction requiring authorization
/// - **Email forwarding**: Attaching copyrighted work = reproduction
/// - **Social media**: Posting images/videos = reproduction (platform needs license)
/// - **NFTs**: Minting NFT may require reproduction authorization (artwork metadata)
///
/// ### 2. Right of Public Performance (Droit de Représentation)
/// **Definition**: Communication of work to the public by any means (Article L122-2)
///
/// **Forms of public performance**:
/// - **Physical performance**: Theater, concert, film screening
/// - **Broadcasting**: Radio, television, cable transmission
/// - **Online**: Streaming, webcasting, making available on-demand
/// - **Public display**: Gallery exhibition, public projection
///
/// **Public vs. Private**:
/// - **Private circle** (cercle de famille): No authorization needed for family/friends
/// - **Public**: Beyond intimate circle, even if admission free
/// - **Example**: Screening film at large party = public (authorization needed)
///
/// ### 3. Digital Transmission Rights
/// **Making available right** (Article L122-2 as amended by DADVSI 2006):
/// - Implements EU Copyright Directive 2001/29/EC Article 3
/// - Covers on-demand services (Spotify, Netflix, YouTube)
/// - User chooses when/where to access work
///
/// ## Moral Rights (Droits Moraux)
///
/// French copyright grants **perpetual, inalienable moral rights** (Articles L121-1 to L121-9):
///
/// ### 1. Right of Disclosure (Droit de Divulgation)
/// - Author decides **if and when** to publish work
/// - Cannot be forced to disclose unfinished work
/// - **Example**: Kafka's executor violated this by publishing per Kafka's will to destroy
///
/// ### 2. Right of Attribution (Droit de Paternité)
/// - Author's name must appear on work
/// - Cannot be published anonymously without author consent
/// - Protects against false attribution
///
/// ### 3. Right of Integrity (Droit au Respect de l'Œuvre)
/// - Work cannot be modified without author consent
/// - Protects against distortion, mutilation
/// - **Example**: Colorizing black-and-white film may violate integrity right
///
/// ### 4. Right of Withdrawal (Droit de Retrait ou Repentir)
/// - Author can withdraw work from circulation
/// - Must compensate rights holders for damages
/// - Rarely exercised due to compensation requirement
///
/// ### Moral Rights vs. Economic Rights
/// **Key differences**:
/// - **Duration**: Moral rights perpetual (even after 70-year copyright expires)
/// - **Alienability**: Moral rights **inalienable** (cannot be sold/transferred)
/// - **Heirs**: Moral rights pass to heirs who can enforce (but not exploit economically)
/// - **Waiver**: Economic rights can be licensed; moral rights cannot be waived in France
///
/// ## Historical Context
///
/// ### Beaux-Arts Tradition (19th Century)
/// French copyright law evolved from **visual arts protection**:
/// - **1793**: Revolutionary decree protected theatrical works
/// - **1793**: Second decree protected musical works
/// - **1857**: Right of adaptation recognized (Les Misérables case)
/// - **1902**: Literary and artistic property law consolidated
/// - **1957**: Law 57-298 codified modern copyright principles
/// - **1992**: CPI codified all IP laws including copyright (Book I)
///
/// ### Author-Centric Philosophy
/// French law prioritizes **author personality** over economic exploitation:
/// - Work as extension of author's personality (romantic author concept)
/// - Moral rights reflect personal connection to work
/// - Contrasts with Anglo-American **copyright** (economic right focus)
///
/// ## International Comparison
///
/// ### United States (17 USC §106)
/// **Economic rights** similar to France:
/// - Reproduction, distribution, public performance, display, derivative works
///
/// **Moral rights** very limited:
/// - Visual arts only (VARA 1990): attribution, integrity for paintings/sculptures
/// - No moral rights for literature, music, film (except contractual)
/// - Work-for-hire doctrine: Employer owns copyright (rare in France)
///
/// ### Germany (UrhG §11-14)
/// **Monist system**: Economic and moral rights inseparable
/// - Cannot transfer copyright itself, only grant licenses
/// - Moral rights similar to France (attribution, integrity, disclosure)
/// - Creator principle: Only natural persons can be authors (not corporations)
///
/// ### United Kingdom (CDPA 1988 §77-89)
/// **Moral rights** (implementing Berne Convention):
/// - Attribution, integrity, false attribution, privacy (photographs)
/// - **Waivable** (unlike France): Authors can contract away moral rights
/// - Duration: Lifetime + 70 years (not perpetual like France)
///
/// ### Japan (Chosakuken Hō §17-20, §59)
/// **Strong moral rights** similar to France:
/// - Publication, attribution, integrity
/// - Inalienable, but waiver possible by contract (controversy)
/// - Duration: Perpetual attribution right, integrity ends with copyright
///
/// ### China (Copyright Law §10)
/// **Moral rights** recognized:
/// - Publication, attribution, integrity, modification
/// - Duration: Perpetual for attribution/integrity
/// - Practical enforcement weaker than France
///
/// ## Modern Applications and Controversies
///
/// ### AI-Generated Works
/// **French law** (as of 2024):
/// - **AI cannot be author**: Only natural persons have moral rights
/// - **User as author**: Person using AI tool may claim copyright if creative input sufficient
/// - **Originality question**: Does AI-generated work bear author's personality imprint?
/// - **Pending issues**: EU AI Act may clarify (2024-2026)
///
/// ### NFTs and Blockchain
/// **Copyright implications**:
/// - **Minting NFT**: May require reproduction authorization (embedding image/metadata)
/// - **NFT sale**: Does not transfer copyright (only token ownership)
/// - **Smart contracts**: Cannot override moral rights (inalienable in France)
/// - **Resale royalty**: Droit de suite may apply (Article L122-8: 4% on resales)
///
/// ### Streaming and Cloud Services
/// **Licensing requirements**:
/// - **Spotify**: Reproduction (download/cache) + performance (streaming) licenses needed
/// - **Netflix**: Reproduction (storage) + performance (streaming) + distribution licenses
/// - **YouTube**: User uploads may infringe unless fair use (exceptions apply)
/// - **Cloud storage**: Dropbox/Google Drive need licenses for cached copies
///
/// ### User-Generated Content (UGC)
/// **Platform liability**:
/// - **EU Copyright Directive 2019/790**: Platforms liable for user uploads
/// - **Article 17**: YouTube/Facebook must license or filter copyrighted content
/// - **France implemented 2021**: DADVSI + Hadopi enforcement
/// - **Safe harbor limited**: Platforms cannot rely on passive host exemption
///
/// ### Open Source and Creative Commons
/// **Compatibility with French moral rights**:
/// - **CC licenses**: Attempt to waive moral rights (unenforceable in France)
/// - **Attribution requirement**: Aligns with French droit de paternité
/// - **Integrity concerns**: CC allows modifications (may conflict with droit au respect)
/// - **Practical solution**: Authors accept CC terms despite theoretical conflict
///
/// ## Fair Use vs. Exceptions (Articles L122-5 to L122-5-4)
///
/// ### French Exceptions System
/// **Closed list of exceptions** (no general fair use):
/// 1. **Private copy** (copie privée): Personal use copy (not fair use)
/// 2. **Quotation**: Short excerpts for criticism, review (with attribution)
/// 3. **Parody**: Transformation for humorous effect
/// 4. **Education**: Teaching illustration (limited scope)
/// 5. **Library/archive**: Preservation copies
/// 6. **Disability**: Accessible format for handicapped
/// 7. **News reporting**: Current events coverage
///
/// **No transformative use doctrine**: Unlike USA's Campbell v. Acuff-Rose
///
/// ### US Fair Use (17 USC §107)
/// **Open-ended factors**:
/// 1. Purpose and character (commercial vs. educational)
/// 2. Nature of work (factual vs. creative)
/// 3. Amount used (substantiality)
/// 4. Market effect
///
/// **Transformative use**: New meaning/purpose = more likely fair use
///
/// ## Examples
///
/// ```rust
/// use legalis_fr::intellectual_property::{Copyright, WorkType, validate_copyright};
/// use chrono::NaiveDate;
///
/// // Example 1: Literary work with moral and economic rights
/// let novel = Copyright::builder()
///     .work_title("Les Misérables".to_string())
///     .author("Victor Hugo".to_string())
///     .creation_date(NaiveDate::from_ymd_opt(1862, 4, 3).unwrap())
///     .author_death_date(NaiveDate::from_ymd_opt(1885, 5, 22).unwrap())
///     .work_type(WorkType::Literary)
///     .build()
///     .unwrap();
///
/// // Copyright expired in 1955 (70 years after Hugo's death in 1885)
/// // But moral rights (attribution, integrity) remain perpetual
/// let current = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// assert!(novel.is_expired(current)); // Economic rights expired
/// // Note: Moral rights never expire - heirs can still enforce attribution/integrity
///
/// // Example 2: Software copyright (still protected)
/// let software = Copyright::builder()
///     .work_title("Revolutionary AI Algorithm".to_string())
///     .author("Marie Curie".to_string())
///     .creation_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
///     .work_type(WorkType::Software)
///     .build()
///     .unwrap();
///
/// // Software protected like any literary work (Article L112-2)
/// // Author has reproduction + performance rights (Article L122-1)
/// assert!(validate_copyright(&software, current).is_ok());
/// ```
pub fn article_l122_1() -> Statute {
    Statute::new(
        "cpi-l122-1",
        "CPI Article L122-1 - Copyright scope (moral rights + economic rights: reproduction, performance)",
        Effect::new(
            EffectType::Grant,
            "Author has exclusive right to authorize reproduction and public performance of work",
        )
        .with_parameter("economic_right_1", "reproduction")
        .with_parameter("economic_right_2", "public_performance")
        .with_parameter("moral_right_1", "disclosure")
        .with_parameter("moral_right_2", "attribution")
        .with_parameter("moral_right_3", "integrity")
        .with_parameter("moral_right_4", "withdrawal")
        .with_parameter("moral_rights_duration", "perpetual")
        .with_parameter("moral_rights_alienability", "inalienable")
        .with_parameter("basis", "berne_convention_art6bis"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::HasAttribute {
            key: "work".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "originality".to_string(),
            value: "true".to_string(),
        }),
    ))
    .with_discretion(
        "Copyright scope: ECONOMIC RIGHTS (droits patrimoniaux) - (1) Reproduction right: fixed/transient \
         copy in any medium (physical, digital, cloud storage, streaming buffer); (2) Public performance \
         right: communication to public (broadcasting, online streaming, making available on-demand). \
         MORAL RIGHTS (droits moraux, perpetual & inalienable) - (1) Disclosure: decide if/when publish; \
         (2) Attribution: name must appear; (3) Integrity: no modification without consent; (4) Withdrawal: \
         can retract (with compensation). French author-centric philosophy vs. Anglo-American copyright. \
         Moral rights cannot be waived (Creative Commons conflict). Modern issues: AI-generated works \
         (AI not author), NFTs (minting needs reproduction license, no copyright transfer), streaming \
         (Netflix/Spotify need reproduction+performance licenses), UGC (EU Directive 2019/790 platform \
         liability). Fair use: Closed exceptions list (private copy, quotation, parody, education) vs. \
         USA open-ended fair use. Compare: USA 17 USC §106 (economic focus, minimal moral rights via VARA), \
         Germany UrhG §11-14 (monist system), UK CDPA §77-89 (waivable moral rights), Japan Chosakuken \
         (strong moral rights), China Copyright Law §10 (perpetual attribution/integrity)."
    )
}

/// Article L123-1 - Copyright duration (70 years post-mortem)
///
/// # French Text
/// "L'auteur jouit, sa vie durant, du droit exclusif d'exploiter son œuvre sous quelque
/// forme que ce soit et d'en tirer un profit pécuniaire. Au décès de l'auteur, ce droit
/// persiste au bénéfice de ses ayants droit pendant l'année civile en cours et les soixante-
/// dix années qui suivent."
///
/// # English Translation
/// "The author shall enjoy, during their lifetime, the exclusive right to exploit their work
/// in any form whatsoever and to derive financial profit from it. Upon the author's death,
/// this right persists for the benefit of their successors during the current calendar year
/// and the seventy years that follow."
///
/// # Legal Commentary
///
/// Article L123-1 establishes the **70-year post-mortem** copyright duration, harmonized
/// by EU Directive 2006/116/EC (Term Directive). This represents a balance between author
/// reward and public domain enrichment.
///
/// ## Duration Calculation
///
/// ### Lifetime + 70 Years
/// **Standard term**:
/// - Author's entire lifetime (no limit)
/// - Plus current calendar year of death
/// - Plus 70 years from January 1 of following year
///
/// **Example**:
/// - Author dies June 15, 2024
/// - Copyright expires December 31, 2094 (2024 + 70 years)
///
/// ### Special Provisions
///
/// **Collaborative works** (Article L123-2):
/// - 70 years from death of **last surviving co-author**
/// - **Example**: Truffaut & Godard collaborate; Godard dies later; 70 years from Godard's death
///
/// **Anonymous/pseudonymous works** (Article L123-3):
/// - 70 years from **publication** (not death)
/// - If author identity revealed within 70 years, standard term applies
///
/// **Posthumous works** (Article L123-4):
/// - Published after author's death: 25 years from publication
/// - Applies to works unpublished during lifetime
///
/// **Collective works** (Article L123-3):
/// - 70 years from publication (corporation/entity holds rights)
///
/// ## Historical Evolution
///
/// ### Progressive Term Extensions
/// - **1793**: Life + 10 years (Revolutionary decree)
/// - **1866**: Life + 50 years
/// - **1957**: Life + 50 years (confirmed by 1957 Law)
/// - **1985**: Life + 70 years for musical works
/// - **1997**: Life + 70 years for all works (EU harmonization)
///
/// ### War Extensions (Prorogations de Guerre)
/// **World War I extension**: +6 years, 152 days (for works by French authors published before WWI)
/// **World War II extension**: +8 years, 120 days (for works by French authors published before WWII)
///
/// **Rationale**: Compensate for lost exploitation during war years
///
/// **Example**: Victor Hugo died 1885; normally expires 1955; with WWI/WWII extensions: ~1970
///
/// ## International Harmonization
///
/// ### Berne Convention (1886)
/// **Minimum term**: Life + 50 years (Article 7)
/// - Most contracting countries exceed minimum
/// - France's 70-year term complies (exceeds minimum)
///
/// ### TRIPS Agreement (1994)
/// **Minimum term**: 50 years from death or publication (Article 12)
/// - Developing countries may use shorter terms
/// - France's 70-year term complies
///
/// ### EU Term Directive (2006/116/EC, amended 2011/77/EU)
/// **Harmonized term**: 70 years post-mortem across EU
/// - Musical performers: 70 years from performance/publication (extended 2011)
/// - Sound recordings: 70 years from publication (extended from 50 in 2011)
/// - Films: 70 years from death of last surviving among director, screenwriter, dialogue writer, composer
///
/// ## International Comparison
///
/// ### United States (17 USC §302)
/// **Post-1978 works**:
/// - Life + 70 years (harmonized with EU)
/// - Corporate authorship: 95 years from publication or 120 from creation (shorter)
///
/// **Pre-1978 works**:
/// - Complex rules: Initial 28-year term + 67-year renewal = 95 years max
/// - Public domain: Works published before 1928 (as of 2024)
///
/// **Notable difference**: USA's work-for-hire (employer = author) reduces effective term
///
/// ### United Kingdom (CDPA 1988 §12)
/// **Standard term**: Life + 70 years (EU harmonized)
/// - Post-Brexit: UK retained 70-year term (unchanged)
/// - Crown copyright: 125 years from creation (government works)
///
/// ### Germany (UrhG §64)
/// **Standard term**: Life + 70 years (EU harmonized)
/// - Film: 70 years from director's death (main director only, not multiple as France)
///
/// ### Japan (Chosakuken Hō §51)
/// **Standard term**: Life + 70 years (extended from 50 in 2018 for TPP)
/// - Previously 50 years (1970-2018), extended to match USA/EU
/// - Films: 70 years from publication (not linked to individual deaths)
///
/// ### Canada (Copyright Act §6)
/// **Standard term**: Life + 70 years (as of 2022, extended from 50)
/// - Extended to comply with USMCA (NAFTA successor)
/// - Previously 50 years (aligned with Berne minimum)
///
/// ### China (Copyright Law §21)
/// **Standard term**: Life + 50 years
/// - Shorter than France/EU/USA/Japan
/// - Legal person authorship: 50 years from publication
///
/// ### Mexico (Ley Federal del Derecho de Autor Art. 29)
/// **Standard term**: Life + 100 years (longest in world)
/// - Extended in 1998 (lobbying by artist heirs)
/// - Controversial: Delays public domain enrichment
///
/// ## Public Domain and Term Debates
///
/// ### Public Domain Entry
/// **Benefits**:
/// - Free access to cultural heritage
/// - Enables derivative works (adaptations, translations)
/// - Educational use without licensing costs
/// - Preservation by multiple parties
///
/// **Recent entries** (as of 2024):
/// - 2024: Works by authors who died in 1953 (e.g., Eugene O'Neill plays)
/// - 2023: Works by authors who died in 1952 (e.g., Hemingway's The Old Man and the Sea)
///
/// ### Criticism of Term Extensions
/// **Economic arguments**:
/// - **Deadweight loss**: Monopoly pricing restricts access without incentivizing creation
/// - **Retroactive extensions**: Cannot incentivize dead authors (yet terms extended)
/// - **Optimal term**: Economic studies suggest 15-25 years sufficient for incentive
///
/// **Cultural arguments**:
/// - **Orphan works**: Rights holder unknown, work unavailable (too risky to use)
/// - **Preservation**: Concentrated ownership may lead to neglect/loss
/// - **Creativity barrier**: Limits remix, adaptation, cultural evolution
///
/// **Defense of long terms**:
/// - **Family support**: Authors' descendants deserve continued income
/// - **Cultural investment**: Long term encourages preservation, promotion
/// - **International competitiveness**: Shorter term disadvantages domestic creators
///
/// ### Copyright Term Extension Controversy
/// **Sonny Bono Act (USA 1998)**: Extended term by 20 years
/// - Critics: "Mickey Mouse Protection Act" (Disney lobbying)
/// - Eldred v. Ashcroft (2003): Supreme Court upheld extension (7-2)
/// - Economic studies: No evidence extension incentivizes creation
///
/// **EU extensions** (2011): Sound recordings 50→70 years
/// - Lobbied by performers (Cliff Richard, Beatles)
/// - Critics: Benefits record labels more than artists
/// - Benefit-cost analysis: Questionable social welfare gain
///
/// ## Modern Issues
///
/// ### AI-Generated Works
/// **Duration question**: If AI-generated works get copyright, what term?
/// - No author death date → Use publication-based term (70 years from publication)?
/// - EU AI Act (2024): May clarify (pending)
///
/// ### NFTs and Digital Art
/// **Copyright term** unaffected by NFT:
/// - NFT ownership ≠ copyright ownership
/// - Copyright remains with creator (70 years post-mortem)
/// - NFT license may grant display rights but not copyright
///
/// ### Orphan Works Directive (EU 2012/28)
/// **Solution for unavailable works**:
/// - Libraries/archives can digitize orphan works (rights holder unknown)
/// - Diligent search required before use
/// - Rights holder can claim compensation if appears later
///
/// ## Examples
///
/// ```rust
/// use legalis_fr::intellectual_property::{Copyright, WorkType, validate_copyright_duration};
/// use chrono::{NaiveDate, Datelike};
///
/// // Example 1: Copyright still active (author alive)
/// let recent_work = Copyright::builder()
///     .work_title("Modern Novel".to_string())
///     .author("Living Author".to_string())
///     .creation_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
///     .work_type(WorkType::Literary)
///     .build()
///     .unwrap();
///
/// let current = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// assert!(!recent_work.is_expired(current)); // Author alive, copyright active
///
/// // Example 2: Copyright expired (70 years post-mortem exceeded)
/// let classic_work = Copyright::builder()
///     .work_title("À la recherche du temps perdu".to_string())
///     .author("Marcel Proust".to_string())
///     .creation_date(NaiveDate::from_ymd_opt(1913, 11, 14).unwrap())
///     .author_death_date(NaiveDate::from_ymd_opt(1922, 11, 18).unwrap())
///     .work_type(WorkType::Literary)
///     .build()
///     .unwrap();
///
/// // Proust died 1922; copyright expired 1992 (70 years later)
/// // Note: War prorogations may extend to ~2000
/// assert!(classic_work.is_expired(current)); // Public domain
///
/// // Example 3: Calculate expiry date
/// let expiry = classic_work.expiry_date().unwrap();
/// assert_eq!(expiry.year(), 1992); // 1922 + 70
/// ```
pub fn article_l123_1() -> Statute {
    Statute::new(
        "cpi-l123-1",
        "CPI Article L123-1 - Copyright duration (lifetime + 70 years post-mortem)",
        Effect::new(
            EffectType::Grant,
            "Copyright protection lasts author's lifetime plus 70 years from death",
        )
        .with_parameter("duration_lifetime", "author_lifetime")
        .with_parameter("duration_post_mortem", "70_years")
        .with_parameter("calculation_start", "january_1_year_after_death")
        .with_parameter("collaborative_works", "70_years_from_last_survivor")
        .with_parameter("anonymous_works", "70_years_from_publication")
        .with_parameter("posthumous_works", "25_years_from_publication")
        .with_parameter("war_extensions", "wwi_6y152d_wwii_8y120d")
        .with_parameter("basis", "berne_convention_art7_eu_directive_2006_116"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::HasAttribute {
            key: "work".to_string(),
        }),
        Box::new(Condition::HasAttribute {
            key: "author".to_string(),
        }),
    ))
    .with_discretion(
        "Copyright duration: Author's lifetime + 70 years from death (calculated from January 1 of year \
         after death). Historical evolution: 1793 (life+10y) → 1866 (life+50y) → 1997 (life+70y, EU \
         harmonization). Special cases: (1) Collaborative works: 70y from last surviving co-author; \
         (2) Anonymous/pseudonymous: 70y from publication; (3) Posthumous: 25y from publication; \
         (4) Collective (corporate): 70y from publication. War prorogations: WWI +6y152d, WWII +8y120d \
         (compensation for lost exploitation). EU Term Directive 2006/116/EC harmonization. Public domain \
         benefits: free access, derivative works, education, preservation. Criticism: 70y excessive \
         (economic studies suggest 15-25y sufficient), retroactive extensions don't incentivize dead authors, \
         orphan works problem. Modern issues: AI-generated works (no death date → publication term?), \
         NFTs (copyright separate from token ownership), Orphan Works Directive 2012/28 (diligent search \
         allows use). Compare: USA 17 USC §302 (life+70y, corporate 95y), UK CDPA §12 (life+70y), Germany \
         UrhG §64 (life+70y), Japan Chosakuken §51 (life+70y since 2018, was 50y), Canada (life+70y since \
         2022, was 50y), China (life+50y), Mexico (life+100y, longest globally)."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_l122_1_structure() {
        let statute = article_l122_1();
        assert_eq!(statute.id, "cpi-l122-1");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("L122-1"));
        assert!(statute.title.contains("Copyright scope"));
    }

    #[test]
    fn test_article_l122_1_parameters() {
        let statute = article_l122_1();
        let params = &statute.effect.parameters;
        assert_eq!(params.get("economic_right_1").unwrap(), "reproduction");
        assert_eq!(
            params.get("economic_right_2").unwrap(),
            "public_performance"
        );
        assert_eq!(params.get("moral_right_1").unwrap(), "disclosure");
        assert_eq!(params.get("moral_right_2").unwrap(), "attribution");
        assert_eq!(params.get("moral_rights_duration").unwrap(), "perpetual");
        assert_eq!(
            params.get("moral_rights_alienability").unwrap(),
            "inalienable"
        );
    }

    #[test]
    fn test_article_l122_1_discretion() {
        let statute = article_l122_1();
        assert!(statute.has_discretion());
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("ECONOMIC RIGHTS"));
        assert!(discretion.contains("MORAL RIGHTS"));
        assert!(discretion.contains("perpetual"));
        assert!(discretion.contains("inalienable"));
    }

    #[test]
    fn test_article_l123_1_structure() {
        let statute = article_l123_1();
        assert_eq!(statute.id, "cpi-l123-1");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("L123-1"));
        assert!(statute.title.contains("duration"));
    }

    #[test]
    fn test_article_l123_1_parameters() {
        let statute = article_l123_1();
        let params = &statute.effect.parameters;
        assert_eq!(params.get("duration_lifetime").unwrap(), "author_lifetime");
        assert_eq!(params.get("duration_post_mortem").unwrap(), "70_years");
        assert!(params.contains_key("war_extensions"));
    }

    #[test]
    fn test_article_l123_1_discretion() {
        let statute = article_l123_1();
        assert!(statute.has_discretion());
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("70 years") || discretion.contains("70y"));
        assert!(discretion.contains("lifetime"));
        assert!(discretion.contains("EU"));
    }

    #[test]
    fn test_all_copyright_articles_have_effect_type() {
        let articles = vec![article_l122_1(), article_l123_1()];
        for article in articles {
            assert!(matches!(article.effect.effect_type, EffectType::Grant));
        }
    }
}
