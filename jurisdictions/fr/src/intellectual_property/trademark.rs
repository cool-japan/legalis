//! French Trademark and Design Law Articles (CPI, Books V & VII)
//!
//! This module implements trademark (marque) and design (dessin et modèle)
//! provisions from the CPI.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article L711-1 - Trademark distinctiveness requirement
///
/// # French Text
/// "La marque de fabrique, de commerce ou de service est un signe susceptible de
/// représentation graphique servant à distinguer les produits ou services d'une
/// personne physique ou morale. Peuvent notamment constituer un tel signe: les
/// dénominations, les signes sonores, les signes figuratifs, la forme du produit."
///
/// # English Translation
/// "A trademark is a sign capable of graphic representation serving to distinguish
/// the goods or services of a natural or legal person. Such signs may include:
/// names, sound signs, figurative signs, the shape of the product."
///
/// # Legal Commentary
///
/// Article L711-1 establishes the **distinctiveness requirement** for French trademarks,
/// implementing EU Trademark Regulation 2017/1001 and harmonizing with international
/// treaties (Paris Convention, Madrid System).
///
/// ## Trademark Definition
///
/// ### Essential Elements
/// 1. **Sign** (signe): Perceptible representation
/// 2. **Graphic representation**: Visual depiction (relaxed post-2019)
/// 3. **Distinctiveness**: Capability to distinguish source
/// 4. **Commercial use**: Identify goods/services
///
/// ### Types of Signs
/// **Word marks** (marques verbales):
/// - Names, slogans, letters, numbers
/// - Examples: COCA-COLA, NIKE, L'ORÉAL
///
/// **Figurative marks** (marques figuratives):
/// - Logos, designs, images
/// - Examples: Apple logo, Mercedes three-point star
///
/// **Combined marks** (marques mixtes):
/// - Words + images
/// - Examples: McDonald's golden arches + name
///
/// **Three-dimensional marks** (marques tridimensionnelles):
/// - Product shape, packaging shape
/// - Examples: Coca-Cola bottle, Toblerone triangle
///
/// **Sound marks** (marques sonores):
/// - Jingles, sounds
/// - Examples: Intel chime, MGM lion roar
///
/// **Color marks** (marques de couleur):
/// - Single color or combination (rare)
/// - Examples: Tiffany blue, Cadbury purple
///
/// **Motion marks** (marques de mouvement):
/// - Animated sequences
/// - Example: MGM roaring lion sequence
///
/// **Hologram marks** (marques holographiques):
/// - 3D holographic images
///
/// ### Non-Traditional Marks
/// **Post-2019 EU reforms** relax graphic representation requirement:
/// - **Scent marks**: No longer require graphic representation (but distinctiveness hard to prove)
/// - **Taste marks**: Theoretical but almost never granted
/// - **Texture marks**: Surface feel (rarely granted)
///
/// ## Distinctiveness Requirement
///
/// ### Inherent Distinctiveness
/// **Arbitrary/fanciful marks** (strongest):
/// - No connection to product: APPLE for computers, GOOGLE for search
///
/// **Suggestive marks** (strong):
/// - Hint at product quality: COPPERTONE for suntan lotion
///
/// **Descriptive marks** (weak/unregistrable):
/// - Directly describe product: "FAST" for delivery service
/// - Require secondary meaning (acquired distinctiveness)
///
/// **Generic marks** (unregistrable):
/// - Common name for product: "ASPIRIN" for aspirin (lost trademark)
///
/// ### Acquired Distinctiveness (Secondary Meaning)
/// Descriptive marks can become distinctive through **use**:
/// - Long-term use (typically 5+ years)
/// - Extensive advertising
/// - Consumer recognition surveys
/// - Examples: YELLOW PAGES, AMERICAN AIRLINES
///
/// ## Absolute Grounds for Refusal (Article L711-2)
///
/// ### Lack of Distinctiveness
/// - Generic terms: "COMPUTER" for computers
/// - Descriptive terms: "FAST" for delivery
/// - Common surnames: "DUPONT" without distinctive elements
///
/// ### Deceptive Marks
/// - Misleading as to quality, origin, nature
/// - Example: "GOLD" for non-gold jewelry
///
/// ### Contrary to Public Order or Morality
/// - Offensive, obscene marks
/// - Illegal symbols (Nazi swastika)
///
/// ### Bad Faith
/// - Trademark squatting (registering famous mark in different class)
/// - Blocking competitor's legitimate use
///
/// ## Relative Grounds for Refusal (Article L711-3)
///
/// ### Prior Rights Conflicts
/// **Earlier trademarks**:
/// - Identical mark, identical goods/services
/// - Identical mark, similar goods (confusion risk)
/// - Similar mark, similar goods (confusion risk)
///
/// **Well-known marks** (Article 6bis Paris Convention):
/// - Protection even without registration
/// - Example: CHANEL entitled to block "CHANNEL" for perfume
///
/// **Prior rights**:
/// - Copyright, design rights, personality rights
/// - Example: Cannot register celebrity name without permission
///
/// ## Nice Classification System
///
/// ### 45 Classes
/// **Goods** (Classes 1-34):
/// - Class 9: Electronics
/// - Class 25: Clothing
/// - Class 30: Coffee, tea
///
/// **Services** (Classes 35-45):
/// - Class 35: Advertising, retail
/// - Class 42: Scientific services, IT
/// - Class 43: Restaurant services
///
/// **Multi-class applications**:
/// - Single application can cover multiple classes
/// - Fee per class (INPI: €225 first class, €40 each additional)
///
/// ## International Comparison
///
/// ### United States (15 USC §1052, Lanham Act)
/// **Similar distinctiveness requirement**:
/// - Arbitrary, suggestive, descriptive (with secondary meaning), generic
/// - Use-based system: Must use mark before registration (unlike France)
/// - Intent-to-use: Can reserve mark with 1(b) application
///
/// ### Germany (MarkenG §3, §8)
/// **Identical to France** (EU harmonized):
/// - Same distinctiveness standard
/// - Same absolute/relative grounds
///
/// ### United Kingdom (Trade Marks Act 1994 §1, §3)
/// **Identical to France** (EU harmonized, retained post-Brexit):
/// - Same distinctiveness requirement
/// - UK Intellectual Property Office applies same standards
///
/// ### Japan (Trademark Act §3)
/// **Similar but stricter**:
/// - Distinctiveness required
/// - Surnames difficult to register (requires extensive use proof)
/// - Sound/color marks allowed since 2014 amendment
///
/// ### China (Trademark Law §8, §11)
/// **Similar distinctiveness requirement**:
/// - Generic, descriptive marks refused
/// - First-to-file system (unlike USA's use-based)
/// - Bad faith squatting epidemic (notorious problem)
///
/// ## Modern Issues
///
/// ### Non-Traditional Marks
/// **Sound marks**:
/// - Intel chime, T-Mobile jingle registered
/// - Distinctiveness difficult to prove
///
/// **Scent marks**:
/// - Extremely rare (Netherlands: fresh-cut grass for tennis balls)
/// - Graphic representation no longer required (EU 2019 reform)
///
/// **NFT trademarks**:
/// - Do NFTs need separate trademark registration?
/// - Current view: Use in metaverse = use in commerce (requires registration)
///
/// ### Social Media Handles
/// **Username as trademark**:
/// - @nike on Twitter = use as trademark
/// - Cybersquatting on social media (UDRP-like policies)
///
/// ### AI-Generated Marks
/// **Ownership question**:
/// - Can AI create trademarks? Yes (signs are registrable regardless of creation method)
/// - Who owns: AI user, not AI itself
///
/// ## Examples
///
/// ```rust
/// use legalis_fr::intellectual_property::{Trademark, validate_trademark_distinctiveness};
/// use chrono::NaiveDate;
///
/// // Example 1: Strong distinctive trademark
/// let trademark = Trademark::builder()
///     .mark("ACME".to_string())  // Arbitrary mark
///     .owner("ACME Corporation".to_string())
///     .registration_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
///     .classes(vec![9, 35, 42])  // Electronics, advertising, IT
///     .distinctiveness(true)
///     .build()
///     .unwrap();
///
/// // ACME is arbitrary (not descriptive), inherently distinctive
/// assert!(validate_trademark_distinctiveness(&trademark).is_ok());
///
/// // Example 2: Weak descriptive mark (would need secondary meaning)
/// // "FAST" for delivery services = descriptive, not distinctive
/// // Would be refused unless acquired distinctiveness proven
/// ```
pub fn article_l711_1() -> Statute {
    Statute::new(
        "cpi-l711-1",
        "CPI Article L711-1 - Trademark distinctiveness requirement (sign distinguishing goods/services)",
        Effect::new(
            EffectType::Grant,
            "Sign capable of graphic representation distinguishing goods/services registrable as trademark",
        )
        .with_parameter("requirement", "distinctiveness")
        .with_parameter("sign_types", "word,figurative,sound,shape,color,motion")
        .with_parameter("distinctiveness_levels", "arbitrary,suggestive,descriptive,generic")
        .with_parameter("secondary_meaning", "acquired_through_use")
        .with_parameter("nice_classification", "45_classes_1-34_goods_35-45_services")
        .with_parameter("basis", "eu_regulation_2017_1001_paris_convention"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "distinctiveness".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::HasAttribute {
            key: "sign".to_string(),
        }),
    ))
    .with_discretion(
        "Trademark distinctiveness: Sign must distinguish goods/services from others. Types: word (names, \
         slogans), figurative (logos), 3D (product shape), sound (jingles), color (single/combination), \
         motion, hologram. Distinctiveness spectrum: (1) ARBITRARY/FANCIFUL - strongest (APPLE for computers); \
         (2) SUGGESTIVE - strong (COPPERTONE for suntan); (3) DESCRIPTIVE - weak, needs secondary meaning \
         (YELLOW PAGES); (4) GENERIC - unregistrable (ASPIRIN for aspirin). Absolute refusal grounds: generic, \
         descriptive, deceptive, contrary to public order, bad faith. Relative grounds: prior trademarks, \
         well-known marks (Paris Convention Art. 6bis), prior rights (copyright, personality). Nice Classification: \
         45 classes (1-34 goods, 35-45 services), multi-class applications allowed. Non-traditional marks \
         post-2019: scent/taste difficult but possible (no graphic representation required). Modern issues: \
         NFT trademarks (metaverse use = commerce), social media handles (@nike = trademark use), AI-generated \
         marks (user owns). Compare: USA Lanham Act §2 (use-based system, intent-to-use allowed), Germany \
         MarkenG §3 (identical EU), UK Trade Marks Act §1 (retained post-Brexit), Japan Trademark Act §3 \
         (stricter surname registration), China Trademark Law §8 (first-to-file, squatting epidemic)."
    )
}

/// Article L712-1 - Trademark duration (10 years, renewable)
///
/// # French Text
/// "La propriété de la marque s'acquiert par l'enregistrement. La marque est enregistrée
/// pour dix ans, renouvelable indéfiniment."
///
/// # English Translation
/// "Ownership of the trademark is acquired by registration. The trademark is registered
/// for ten years, indefinitely renewable."
///
/// # Legal Commentary
///
/// Article L712-1 establishes the **10-year renewable term** for French trademarks,
/// contrasting with patents (fixed 20 years) and copyright (life + 70 years).
/// Renewable indefinitely means trademarks can last forever if maintained.
///
/// ## Registration System
///
/// ### Constitutive Registration
/// **France uses constitutive system**: Rights arise from registration, not use
/// - **First-to-file**: First applicant wins (unlike USA's first-to-use)
/// - **Pre-application use**: Does not create rights (but can evidence distinctiveness)
/// - **Exception**: Well-known marks protected without registration (Paris Convention Art. 6bis)
///
/// ### INPI Registration Procedure
/// 1. **Filing**: Online or paper application to INPI
/// 2. **Formality check**: INPI verifies application completeness
/// 3. **Publication**: Application published in BOPI (Bulletin Officiel)
/// 4. **Opposition period**: 2 months for third parties to oppose
/// 5. **Examination**: INPI checks absolute grounds (not relative grounds)
/// 6. **Registration**: Certificate issued if no refusal
/// 7. **Duration**: Registration effective 10 years from filing date
///
/// ## 10-Year Term
///
/// ### Term Calculation
/// - Starts from **filing date**, not registration date
/// - **Full 10 years**: No pro-rata reduction if registration delayed
/// - **Renewal**: Apply 6 months before expiry (grace period 6 months after)
///
/// ### Renewal Procedure
/// **Simple renewal**:
/// - No substantive re-examination (unlike patent maintenance)
/// - Pay renewal fee (INPI: €250 for online renewal)
/// - Can renew indefinitely (no maximum renewals)
///
/// **Grace period**:
/// - 6 months after expiry to renew (with surcharge)
/// - If not renewed: Trademark lapses, cannot be revived
///
/// ### Indefinite Renewability
/// **Trademarks vs. other IP**:
/// - Patents: Fixed 20 years (cannot extend except SPC)
/// - Copyright: Life + 70 years (then public domain)
/// - Trademarks: **Forever** if maintained (indefinite renewals)
///
/// **Rationale**: Trademark protects source identification, not creative work
/// - Consumer confusion risk persists regardless of time elapsed
/// - No public domain benefit from trademark expiry
/// - Unlike copyright/patent, no disclosure/publication tradeoff
///
/// ## Use Requirement
///
/// ### Serious Use Obligation (Article L714-5)
/// **5-year use requirement**:
/// - Must use trademark for registered goods/services within 5 years
/// - "Serious use" = genuine commercial use, not token use
/// - **Consequence**: Unused trademarks subject to cancellation (révocation)
///
/// **Genuine use criteria**:
/// - Commercial exploitation, not internal use
/// - Sufficient volume (not isolated sales)
/// - In France (or EU if CTM)
///
/// **Use by licensee** counts as use by owner
/// **Modified form**: Slight variations acceptable if not altering distinctive character
///
/// ### Cancellation for Non-Use
/// **Action en déchéance** (revocation action):
/// - Anyone can challenge unused trademark after 5 years
/// - Burden on trademark owner to prove use
/// - If no use: Trademark cancelled retroactively
///
/// **Examples**:
/// - Company registers trademark but never launches product (cancellable)
/// - Trademark used only internally (not commercial use, cancellable)
/// - Temporary interruption (force majeure) acceptable if resumption
///
/// ## International Comparison
///
/// ### United States (15 USC §1058, §1059)
/// **10-year term with strict maintenance**:
/// - §1058: Declaration of use required between years 5-6 (or trademark cancelled)
/// - §1059: Renewal every 10 years (with declaration of use)
/// - **Incontestability**: After 5 years' use, mark becomes incontestable (limited challenges)
///
/// ### Germany (MarkenG §47)
/// **10-year term, renewable indefinitely** (identical to France):
/// - EU harmonized
/// - Same use requirement (5 years)
///
/// ### United Kingdom (Trade Marks Act 1994 §42)
/// **10-year term, renewable indefinitely** (retained post-Brexit):
/// - Identical to France (was EU harmonized)
/// - Same 5-year use requirement
///
/// ### Japan (Trademark Act §19)
/// **10-year term, renewable indefinitely**:
/// - Similar to France
/// - No automatic use requirement (but can be challenged for non-use)
///
/// ### China (Trademark Law §39-40)
/// **10-year term, renewable indefinitely**:
/// - Similar to France
/// - 3-year use requirement (stricter than France's 5 years)
///
/// ## Historical Context
///
/// ### Term Harmonization
/// - **Paris Convention (1883)**: No minimum term specified
/// - **TRIPS (1994)**: Minimum 7 years, renewable indefinitely (Article 18)
/// - **France**: 10-year term since 1857 law (predates TRIPS)
/// - **EU harmonization (1988)**: First Trademark Directive standardized 10-year term
///
/// ### Rationale for Renewable Term
/// **Trademark as business asset**:
/// - Brand value accumulates over time (unlike patent/copyright)
/// - Coca-Cola trademark (1886): Over 138 years old, still valuable
/// - Perpetual protection justified by continued use requirement
///
/// ## Modern Issues
///
/// ### Trademark Squatting
/// **Defensive registrations**:
/// - Companies register trademarks without intent to use (to block competitors)
/// - **Solution**: 5-year use requirement allows challenges
/// - **Problem**: Squatters can provide minimal token use
///
/// ### Domain Names and Social Media
/// **Digital trademark use**:
/// - Website use = commercial use (satisfies use requirement)
/// - Social media handle use = trademark use
/// - Domain name registration alone ≠ use (must be active website)
///
/// ### NFTs and Metaverse
/// **Virtual goods trademarks**:
/// - Nike, Gucci register trademarks for virtual goods (Class 9)
/// - Use in metaverse = commercial use?
/// - Current view: Yes, if genuine virtual commerce
///
/// ## Examples
///
/// ```rust
/// use legalis_fr::intellectual_property::{Trademark, validate_trademark_duration};
/// use chrono::{NaiveDate, Datelike};
///
/// // Example 1: Trademark within 10-year term
/// let trademark = Trademark::builder()
///     .mark("LAFONTAINE".to_string())
///     .owner("Lafontaine SA".to_string())
///     .registration_date(NaiveDate::from_ymd_opt(2020, 1, 1).unwrap())
///     .classes(vec![25, 35])  // Clothing, retail
///     .distinctiveness(true)
///     .build()
///     .unwrap();
///
/// // In 2024, trademark still valid (4 years elapsed, 6 remaining)
/// let current = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
/// assert!(!trademark.is_expired(current));
/// assert!(validate_trademark_duration(&trademark, current).is_ok());
///
/// // Expiry date is 2029 (registration + 365*10 days)
/// // But can renew indefinitely
/// let expiry = trademark.expiry_date();
/// assert_eq!(expiry.year(), 2029);
///
/// // Example 2: Expired trademark (not renewed)
/// let old_trademark = Trademark::builder()
///     .mark("OBSOLETE".to_string())
///     .owner("Old Company".to_string())
///     .registration_date(NaiveDate::from_ymd_opt(2010, 1, 1).unwrap())
///     .classes(vec![9])
///     .distinctiveness(true)
///     .build()
///     .unwrap();
///
/// // In 2024, trademark expired in 2020 (10 years after 2010)
/// // If not renewed, trademark lapsed
/// assert!(old_trademark.is_expired(current));
/// ```
pub fn article_l712_1() -> Statute {
    Statute::new(
        "cpi-l712-1",
        "CPI Article L712-1 - Trademark duration (10 years from registration, renewable indefinitely)",
        Effect::new(
            EffectType::Grant,
            "Trademark registration lasts 10 years, renewable indefinitely if maintained",
        )
        .with_parameter("duration_years", "10")
        .with_parameter("renewable", "indefinitely")
        .with_parameter("acquisition", "by_registration_not_use")
        .with_parameter("use_requirement", "5_years_serious_use")
        .with_parameter("renewal_fee", "required")
        .with_parameter("grace_period", "6_months_after_expiry")
        .with_parameter("basis", "trips_art18_eu_directive_2008_95"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::HasAttribute {
            key: "trademark_registration".to_string(),
        }),
        Box::new(Condition::HasAttribute {
            key: "registration_date".to_string(),
        }),
    ))
    .with_discretion(
        "Trademark duration: 10 years from filing, renewable indefinitely (forever if maintained). Acquisition \
         by registration (constitutive system), not use (unlike USA). First-to-file wins. INPI procedure: \
         filing → formality check → publication → 2-month opposition → examination (absolute grounds only) → \
         registration. Renewal: simple fee payment (€250 INPI), no re-examination, 6-month grace period \
         after expiry. Use requirement: 5-year serious use obligation (Article L714-5), non-use = cancellation. \
         Indefinite renewability justified: (1) brand value accumulates over time (Coca-Cola 138+ years); \
         (2) consumer confusion risk persists; (3) no public domain benefit; (4) continued use required. \
         Modern issues: trademark squatting (5-year use requirement mitigates), domain name/social media use \
         counts as commercial use, NFTs/metaverse (virtual goods Class 9, use in metaverse = commercial use). \
         Compare: USA 15 USC §1058-1059 (10y renewable, but strict maintenance: years 5-6 declaration + \
         renewal declaration, incontestability after 5y), Germany MarkenG §47 (identical 10y renewable), \
         UK Trade Marks Act §42 (retained post-Brexit, 10y renewable), Japan Trademark Act §19 (10y renewable), \
         China Trademark Law §39 (10y renewable, 3y use requirement stricter than France's 5y)."
    )
}

/// Article L511-1 - Design protection requirements (novelty, individual character)
///
/// # French Text
/// "Peut être protégé à titre de dessin ou modèle l'apparence d'un produit, ou d'une partie
/// de produit, caractérisée en particulier par ses lignes, ses contours, ses couleurs, sa forme,
/// sa texture ou ses matériaux. Ces caractéristiques peuvent être celles du produit lui-même
/// ou de son ornementation. Seul peut être protégé le dessin ou modèle qui est nouveau et
/// présente un caractère propre."
///
/// # English Translation
/// "The appearance of a product, or part of a product, may be protected as a design,
/// characterized in particular by its lines, contours, colors, shape, texture or materials.
/// These characteristics may be those of the product itself or its ornamentation.
/// Only designs which are new and have individual character may be protected."
///
/// # Legal Commentary
///
/// Article L511-1 establishes **design protection requirements** in French law,
/// implementing EU Design Regulation 6/2002 and harmonizing with international
/// treaties (Hague Agreement). Design protects **aesthetic appearance**, not function.
///
/// ## Design Definition
///
/// ### Appearance Elements
/// **Visual features**:
/// - **Lines**: Contours, outlines, edges
/// - **Contours**: Shape boundaries
/// - **Colors**: Color schemes, combinations
/// - **Shape**: Three-dimensional form
/// - **Texture**: Surface qualities
/// - **Materials**: Visual appearance of materials
///
/// **Product vs. ornamentation**:
/// - Entire product design (iPhone shape)
/// - Decorative elements only (pattern on fabric)
///
/// ### Design vs. Other IP
/// **Design** (aesthetic appearance):
/// - Protection: Novelty + individual character
/// - Duration: Up to 25 years (5-year renewable periods)
/// - Example: Chair design, smartphone shape
///
/// **Patent** (technical function):
/// - Protection: Novelty + inventive step + industrial applicability
/// - Duration: 20 years (not renewable)
/// - Example: Mechanical invention
///
/// **Copyright** (artistic work):
/// - Protection: Originality (author's personality)
/// - Duration: Life + 70 years
/// - **Cumulative protection**: Design can also have copyright if original (Article L513-5)
///
/// ## Requirement 1: Novelty (Nouveauté)
///
/// ### Novelty Standard
/// **No identical design** made available to public before filing:
/// - **Worldwide disclosure**: Like patent novelty (absolute)
/// - **Identical or substantially identical**: Minor differences don't avoid novelty bar
/// - **Grace period**: 12 months from first disclosure by designer (unlike patent's zero grace period)
///
/// ### Prior Art
/// **Public availability**:
/// - Published designs, exhibited products
/// - Products sold in commerce
/// - Design shown at trade show
/// - **Not secret**: Internal documents, prototypes shown under NDA
///
/// **Specialized field**: Only designs reasonably available to specialists in sector count
/// - Example: Japanese furniture catalog = prior art for French furniture design
/// - But obscure academic dissertation in different field may not count
///
/// ### Grace Period (12 Months)
/// **Designer's own disclosure** does not destroy novelty if:
/// - Disclosure within 12 months before filing
/// - By designer or successor in title
/// - **Example**: Designer exhibits at Milan Furniture Fair in January 2024, files in December 2024 (OK)
///
/// ## Requirement 2: Individual Character (Caractère Propre)
///
/// ### Individual Character Test
/// **Overall impression** differs from prior designs:
/// - **Informed user standard**: Not expert, not layman, but knowledgeable user
/// - **Overall impression**: Holistic view, not feature-by-feature comparison
/// - **Degree of freedom**: More freedom in design = less individual character needed
///
/// ### Informed User
/// **Not skilled artisan** (patent standard):
/// - **Higher than layman**: Familiar with design sector
/// - **Lower than expert**: Not designer or engineer
/// - **Example**: For smartphone design, informed user = frequent smartphone buyer, not designer
///
/// ### Degree of Freedom
/// **Functional constraints** affect individual character:
/// - **High freedom** (jewelry): Small differences suffice for individual character
/// - **Low freedom** (screws): Must differ substantially due to functional constraints
/// - **Example**: Smartphone design constrained by screen size, buttons → less individual character needed
///
/// ## Exclusions from Protection
///
/// ### Article L511-2: Functionality Exclusion
/// **Features dictated by technical function** not protectable:
/// - Must-fit features (connectors, interoperability)
/// - Technical functionality features
/// - **Example**: USB connector shape = functional, not design
///
/// ### Article L511-3: Public Order Exclusion
/// **Designs contrary to public order or morality**:
/// - Offensive, obscene designs
/// - Symbols violating law (Nazi swastika)
///
/// ## International Comparison
///
/// ### United States (35 USC §171)
/// **Design patents**:
/// - **Novelty + non-obviousness**: Similar to France but higher bar
/// - **Ornamental**: Design must be non-functional (functionality bars patent)
/// - **Duration**: 15 years from grant (not renewable)
/// - **Examination**: Substantive examination (unlike EU's registration-only)
///
/// ### European Union (Regulation 6/2002)
/// **Community Design**:
/// - **Identical to France**: Novelty + individual character
/// - **Unregistered design**: 3 years' protection from first disclosure (no registration)
/// - **Registered design**: Up to 25 years (5-year renewable periods)
///
/// ### Germany (DesignG §2)
/// **Identical to France** (EU harmonized):
/// - Same novelty + individual character test
/// - Same 12-month grace period
///
/// ### United Kingdom (RDA 1949 §1B)
/// **Retained EU law post-Brexit**:
/// - Identical novelty + individual character
/// - UK designs separate from EU designs post-Brexit
///
/// ### Japan (Design Act §3)
/// **Similar requirements**:
/// - Novelty + creativity (similar to individual character)
/// - 6-month grace period (shorter than France's 12)
/// - No unregistered design protection (unlike EU)
///
/// ### China (Patent Law §23)
/// **Design patents**:
/// - Novelty + aesthetic appeal (similar to individual character)
/// - No grace period
/// - 15-year term (unlike France's 25)
///
/// ## Modern Issues
///
/// ### GUI and Icon Designs
/// **Graphical user interfaces**:
/// - Registrable as designs (EU practice since 2004)
/// - Examples: App icons, screen layouts
/// - Must show novelty + individual character
///
/// ### 3D Printing and Digital Designs
/// **File sharing**:
/// - Uploading 3D design file = making available (infringement risk)
/// - Personal 3D printing = private use (exception may apply)
///
/// ### NFT Designs
/// **Digital art as design**:
/// - NFT artwork can be protected as design
/// - Minting NFT may require design owner's permission
///
/// ## Examples
///
/// ```rust
/// use legalis_fr::intellectual_property::{Design, validate_design_novelty, validate_design_individual_character};
/// use chrono::NaiveDate;
///
/// // Example 1: Novel chair design with individual character
/// let chair_design = Design::builder()
///     .title("Modern Ergonomic Chair".to_string())
///     .creator("Philippe Starck".to_string())
///     .filing_date(NaiveDate::from_ymd_opt(2024, 1, 1).unwrap())
///     .novelty(true)  // Not identical to any prior chair design
///     .individual_character(true)  // Overall impression differs from prior designs
///     .build()
///     .unwrap();
///
/// // Chair design satisfies both requirements
/// assert!(validate_design_novelty(&chair_design).is_ok());
/// assert!(validate_design_individual_character(&chair_design).is_ok());
/// ```
pub fn article_l511_1() -> Statute {
    Statute::new(
        "cpi-l511-1",
        "CPI Article L511-1 - Design protection requirements (novelty + individual character)",
        Effect::new(
            EffectType::Grant,
            "Design protectable if novel and has individual character to informed user",
        )
        .with_parameter("requirement_1", "novelty")
        .with_parameter("requirement_2", "individual_character")
        .with_parameter("design_elements", "lines,contours,colors,shape,texture,materials")
        .with_parameter("novelty_standard", "no_identical_design_worldwide")
        .with_parameter("grace_period", "12_months_designer_disclosure")
        .with_parameter("individual_character_test", "overall_impression_informed_user")
        .with_parameter("functional_exclusion", "technical_features_not_protectable")
        .with_parameter("basis", "eu_regulation_6_2002_hague_agreement"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "novelty".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "individual_character".to_string(),
            value: "true".to_string(),
        }),
    ))
    .with_discretion(
        "Design protection: Aesthetic appearance (lines, contours, colors, shape, texture, materials) of \
         product/ornamentation. Two requirements: (1) NOVELTY - no identical/substantially identical design \
         made available worldwide before filing; 12-month grace period for designer's own disclosure (unlike \
         patent zero grace period); prior art = published designs, sold products, trade shows (not secret \
         prototypes); (2) INDIVIDUAL CHARACTER - overall impression differs to informed user (not expert, not \
         layman, but knowledgeable); degree of freedom: high freedom (jewelry) = small differences suffice, \
         low freedom (technical products) = substantial difference needed. Functional exclusion: must-fit, \
         technical function not protectable (Article L511-2). Cumulative copyright: if original, design also \
         copyrightable (Article L513-5, life+70y). Modern issues: GUI/icons registrable (app icons), 3D printing \
         (file sharing = infringement), NFT designs (digital art protectable). Compare: USA 35 USC §171 (design \
         patents, novelty+non-obviousness, 15y not renewable, substantive examination), EU Regulation 6/2002 \
         (identical, unregistered design 3y), Germany DesignG §2 (identical EU), UK RDA §1B (retained post-Brexit), \
         Japan Design Act §3 (novelty+creativity, 6-month grace), China Patent Law §23 (novelty+aesthetic, 15y)."
    )
}

/// Article L513-1 - Design duration (up to 25 years in 5-year periods)
///
/// # French Text
/// "La durée de protection du dessin ou modèle est de cinq ans à compter de la date
/// de dépôt de la demande d'enregistrement. Elle peut être prorogée par périodes de cinq ans
/// jusqu'à un maximum de vingt-cinq ans."
///
/// # English Translation
/// "The term of protection of a design is five years from the filing date of the registration
/// application. It may be renewed for periods of five years up to a maximum of twenty-five years."
///
/// # Legal Commentary
///
/// Article L513-1 establishes **25-year maximum protection** for registered designs,
/// renewable in **5-year periods**. This intermediate duration balances design investment
/// protection with public domain enrichment (longer than trademark renewal cycles but
/// shorter than copyright).
///
/// ## Duration Structure
///
/// ### Initial 5-Year Term
/// **Automatic upon registration**:
/// - Starts from **filing date** (not registration date)
/// - No renewal needed for first 5 years
/// - Registration fee covers initial term
///
/// ### Renewable Periods (5 Years Each)
/// **Up to 4 renewals**:
/// - 5 years (initial) + 5 + 5 + 5 + 5 = 25 years maximum
/// - Each renewal requires fee payment
/// - No substantive re-examination (like trademark renewal)
///
/// ### 25-Year Maximum
/// **Not indefinitely renewable** (unlike trademarks):
/// - After 25 years, design enters public domain
/// - Cannot extend beyond 25 years
/// - **Rationale**: Fashion cycles make 25 years sufficient for exploitation
///
/// ## Renewal Procedure
///
/// ### Timing
/// **Before expiry of current 5-year period**:
/// - Apply 6 months before expiry
/// - Grace period: 6 months after expiry (with surcharge)
/// - If not renewed: Design lapses, cannot be revived
///
/// ### Fees (INPI, 2024)
/// - **Years 1-5**: Included in registration fee (€50)
/// - **Years 6-10**: €52
/// - **Years 11-15**: €64
/// - **Years 16-20**: €78
/// - **Years 21-25**: €90
/// - **Total for 25 years**: ~€334
///
/// ## Cumulative Protection
///
/// ### Design + Copyright (Article L513-5)
/// **Dual protection possible**:
/// - Design protection: Up to 25 years (novelty + individual character)
/// - Copyright protection: Life + 70 years (originality)
/// - **No election required**: Both apply simultaneously
///
/// **Example**: Original furniture design
/// - Design right: 25 years max (protects shape from copying)
/// - Copyright: Life + 70 years (protects artistic originality)
/// - After 25 years: Design expires, but copyright continues
///
/// ### Design + Trademark
/// **Shape as trademark**:
/// - Product shape can be trademarked if distinctive (Article L711-1)
/// - Trademark: Indefinitely renewable (10-year periods)
/// - **Example**: Coca-Cola bottle shape (design expired, trademark continues)
///
/// **Limitation**: Cannot trademark functional shape (Article L711-2(f))
///
/// ## Unregistered Design (EU Only)
///
/// ### Community Unregistered Design
/// **3-year protection without registration**:
/// - Starts from first public disclosure in EU
/// - **No registration** required
/// - Limited protection: Against copying only (not independent creation)
///
/// **Not available in France alone** (EU-wide concept only)
///
/// ## International Comparison
///
/// ### United States (35 USC §173)
/// **Design patents**:
/// - **15 years** from grant (not 25 years)
/// - **Not renewable** (unlike France's 5-year renewals)
/// - Shorter term reflects patent classification (not design registration)
///
/// ### European Union (Regulation 6/2002 Art. 12)
/// **Registered Community Design**:
/// - **Identical to France**: 25 years max in 5-year periods
/// - **Unregistered Community Design**: 3 years from disclosure (no renewal)
///
/// ### Germany (DesignG §27)
/// **Identical to France** (EU harmonized):
/// - 25 years max in 5-year periods
///
/// ### United Kingdom (RDA 1949 §8)
/// **Retained EU law post-Brexit**:
/// - 25 years max in 5-year periods (unchanged)
/// - UK unregistered design: 15 years from creation (domestic concept)
///
/// ### Japan (Design Act §21)
/// **25 years** from registration (as of 2020 amendment):
/// - Previously 20 years (extended in 2020)
/// - Single term, no 5-year renewal periods
///
/// ### China (Patent Law §42)
/// **15 years** from grant:
/// - Shorter than France (15 vs. 25 years)
/// - Not renewable
/// - Treated as patent, not design registration
///
/// ## Historical Context
///
/// ### Term Evolution
/// - **1909**: 50 years (artistic property tradition)
/// - **1992**: 25 years in 5-year periods (CPI codification)
/// - **1998**: EU harmonization confirmed 25-year term
///
/// ### Rationale for 25 Years
/// **Fashion and product cycles**:
/// - Furniture: 10-15 year design lifespan
/// - Fashion: 1-3 year seasonal cycles (but classics last longer)
/// - Electronics: 2-5 year product cycles
/// - Automotive: 5-7 year model cycles
///
/// **25 years** exceeds most product cycles, providing ample protection
///
/// ## Modern Issues
///
/// ### Fast Fashion and Short Cycles
/// **Criticism**:
/// - Fashion designs obsolete within 1-3 years
/// - 25-year protection excessive for disposable fashion
/// - **Counter**: Classic designs (Hermès Kelly bag) remain valuable decades later
///
/// ### 3D Printing and Digital Sharing
/// **Enforcement challenges**:
/// - Personal 3D printing of protected designs (private use exception?)
/// - File sharing platforms (Thingiverse, MyMiniFactory)
/// - Cross-border enforcement difficult
///
/// ### Repair Clause Debate
/// **Spare parts exemption**:
/// - EU considering "repair clause" for car parts
/// - Allow generic spare parts (defeat design protection for aftermarket)
/// - **Current law**: No general repair exemption (design owner controls spares)
///
/// ## Examples
///
/// ```rust
/// use legalis_fr::intellectual_property::{Design, validate_design_duration};
/// use chrono::{NaiveDate, Datelike};
///
/// // Example 1: Design within 25-year term
/// let design = Design::builder()
///     .title("Iconic Chair Design".to_string())
///     .creator("Designer".to_string())
///     .filing_date(NaiveDate::from_ymd_opt(2010, 1, 1).unwrap())
///     .novelty(true)
///     .individual_character(true)
///     .protection_years(25)
///     .build()
///     .unwrap();
///
/// // In 2024, design still valid (14 years elapsed, 11 remaining)
/// let current = NaiveDate::from_ymd_opt(2024, 6, 1).unwrap();
/// assert!(!design.is_expired(current));
/// assert!(validate_design_duration(&design, current).is_ok());
///
/// // Expiry date is 2034 (2010 filing + 365*25 days)
/// let expiry = design.expiry_date();
/// assert_eq!(expiry.year(), 2034);
///
/// // Example 2: Design with shorter protection period (10 years chosen)
/// let short_design = Design::builder()
///     .title("Fashion Design".to_string())
///     .creator("Designer".to_string())
///     .filing_date(NaiveDate::from_ymd_opt(2015, 1, 1).unwrap())
///     .novelty(true)
///     .individual_character(true)
///     .protection_years(10)  // Only renew once (5+5 years)
///     .build()
///     .unwrap();
///
/// // Expires 2024-12-xx (2015 + 365*10 days), check in 2025 when expired
/// let future = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
/// assert!(short_design.is_expired(future));
/// ```
pub fn article_l513_1() -> Statute {
    Statute::new(
        "cpi-l513-1",
        "CPI Article L513-1 - Design duration (5 years renewable up to 25 years maximum)",
        Effect::new(
            EffectType::Grant,
            "Design protection lasts 5 years, renewable in 5-year periods up to 25 years max",
        )
        .with_parameter("initial_term", "5_years")
        .with_parameter("renewable_periods", "5_years_each")
        .with_parameter("maximum_term", "25_years")
        .with_parameter("renewals_count", "4_renewals_max")
        .with_parameter("cumulative_copyright", "article_l513_5_allows")
        .with_parameter("cumulative_trademark", "shape_mark_possible")
        .with_parameter("unregistered_design_eu", "3_years_no_registration")
        .with_parameter("basis", "eu_regulation_6_2002_art12"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::HasAttribute {
            key: "design_registration".to_string(),
        }),
        Box::new(Condition::HasAttribute {
            key: "filing_date".to_string(),
        }),
    ))
    .with_discretion(
        "Design duration: Initial 5 years from filing, renewable in 5-year periods up to 25 years maximum \
         (5 + 5 + 5 + 5 + 5 = 25y). Renewal: simple fee payment (€52-90), 6-month grace period. Not indefinitely \
         renewable (unlike trademarks, enters public domain after 25y). Cumulative protection: (1) Design+Copyright \
         (Article L513-5): if original, both apply (design 25y, copyright life+70y); (2) Design+Trademark: shape \
         mark possible if distinctive (indefinite via 10y renewals, e.g., Coca-Cola bottle). EU unregistered design: \
         3 years from disclosure without registration (copying protection only, not independent creation). Term \
         rationale: 25y exceeds most product cycles (fashion 1-3y, electronics 2-5y, automotive 5-7y, furniture \
         10-15y), classic designs (Hermès) valuable decades later. Modern issues: fast fashion (25y excessive?), \
         3D printing (file sharing enforcement), repair clause debate (spare parts exemption proposed). Compare: \
         USA 35 USC §173 (15y not renewable), EU Reg 6/2002 (identical 25y, unregistered 3y), Germany DesignG §27 \
         (identical 25y), UK RDA §8 (retained 25y post-Brexit), Japan Design Act §21 (25y since 2020, was 20y), \
         China Patent Law §42 (15y not renewable)."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_l711_1_structure() {
        let statute = article_l711_1();
        assert_eq!(statute.id, "cpi-l711-1");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("L711-1"));
        assert!(statute.title.contains("Trademark"));
    }

    #[test]
    fn test_article_l711_1_parameters() {
        let statute = article_l711_1();
        let params = &statute.effect.parameters;
        assert_eq!(params.get("requirement").unwrap(), "distinctiveness");
        assert!(params.get("sign_types").unwrap().contains("word"));
        assert!(params.get("nice_classification").unwrap().contains("45"));
    }

    #[test]
    fn test_article_l712_1_structure() {
        let statute = article_l712_1();
        assert_eq!(statute.id, "cpi-l712-1");
        assert!(statute.title.contains("L712-1"));
        assert!(statute.title.contains("duration"));
    }

    #[test]
    fn test_article_l712_1_parameters() {
        let statute = article_l712_1();
        let params = &statute.effect.parameters;
        assert_eq!(params.get("duration_years").unwrap(), "10");
        assert_eq!(params.get("renewable").unwrap(), "indefinitely");
        assert_eq!(
            params.get("use_requirement").unwrap(),
            "5_years_serious_use"
        );
    }

    #[test]
    fn test_article_l511_1_structure() {
        let statute = article_l511_1();
        assert_eq!(statute.id, "cpi-l511-1");
        assert!(statute.title.contains("L511-1"));
        assert!(statute.title.contains("Design"));
    }

    #[test]
    fn test_article_l511_1_parameters() {
        let statute = article_l511_1();
        let params = &statute.effect.parameters;
        assert_eq!(params.get("requirement_1").unwrap(), "novelty");
        assert_eq!(params.get("requirement_2").unwrap(), "individual_character");
        assert_eq!(
            params.get("grace_period").unwrap(),
            "12_months_designer_disclosure"
        );
    }

    #[test]
    fn test_article_l513_1_structure() {
        let statute = article_l513_1();
        assert_eq!(statute.id, "cpi-l513-1");
        assert!(statute.title.contains("L513-1"));
        assert!(statute.title.contains("duration"));
    }

    #[test]
    fn test_article_l513_1_parameters() {
        let statute = article_l513_1();
        let params = &statute.effect.parameters;
        assert_eq!(params.get("initial_term").unwrap(), "5_years");
        assert_eq!(params.get("maximum_term").unwrap(), "25_years");
        assert_eq!(params.get("renewals_count").unwrap(), "4_renewals_max");
    }

    #[test]
    fn test_all_trademark_design_articles_have_effect_type() {
        let articles = vec![
            article_l711_1(),
            article_l712_1(),
            article_l511_1(),
            article_l513_1(),
        ];
        for article in articles {
            assert!(matches!(article.effect.effect_type, EffectType::Grant));
        }
    }
}
