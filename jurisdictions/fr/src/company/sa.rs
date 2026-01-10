//! SA (Société Anonyme) articles - French Public Company Law
//!
//! Implementation of Code de commerce Articles L225-1+ for SA companies.
//!
//! # Historical Context of the Société Anonyme
//!
//! The SA is France's flagship corporate form, with roots in the Napoleonic Commercial Code of 1807.
//! The modern SA emerged from the liberalization of 1867, which introduced limited liability for all
//! shareholders and abolished prior government authorization requirements. This reform catalyzed
//! industrial development in the Second Empire.
//!
//! ## Key Legislative Milestones
//!
//! - **1807**: Napoleonic Commercial Code establishes early corporate forms
//! - **1867**: SA liberalization - limited liability expansion, end of authorization requirement
//! - **1966**: Major company law reform (Loi sur les sociétés commerciales) - modern SA structure
//! - **1994**: SAS creation provides flexible alternative to rigid SA
//! - **2001**: NRE Law (Nouvelles Régulations Économiques) - enhanced corporate governance
//! - **2011**: Copé-Zimmermann Law - 40% gender quota for SA/SCA boards (phased in by 2017)
//! - **2019**: PACTE Law (Plan d'Action pour la Croissance et la Transformation des Entreprises)
//!   - Introduction of "raison d'être" (corporate purpose beyond profit)
//!   - Stakeholder capitalism provisions (Article 1833 Code civil amended)
//!
//! ## EU Harmonization Influence
//!
//! French SA law incorporates numerous EU directives on company law:
//! - First Company Law Directive (1968) - disclosure and validity
//! - Second Directive (1976/2012) - capital maintenance
//! - Fourth Directive (1978) - annual accounts
//! - Shareholder Rights Directive (2007/2017) - proxy voting, information rights
//! - Company Law Package (2017-2019) - cross-border conversions, digitalization
//!
//! The SA's €37,000 capital requirement reflects Second Directive minimums, though France
//! exceeds the €25,000 EU floor to maintain SA prestige.
//!
//! # SA vs. SARL vs. SAS: Strategic Choice
//!
//! The SA competes with two simpler forms introduced to democratize entrepreneurship:
//!
//! | Feature | SA (1867) | SARL (1925) | SAS (1994) |
//! |---------|-----------|-------------|------------|
//! | Minimum capital | €37,000 | €1 (since 2003) | €1 (since 1999) |
//! | Partners/shareholders | No limit | Max 100 | No limit |
//! | Board requirement | Mandatory (3-18) | Optional (gérant) | Optional (président) |
//! | Public offering | Yes | No | No (SAS-listed rare) |
//! | Governance flexibility | Rigid (Code) | Moderate | Very high (statuts) |
//! | Use case | Large corps, IPOs | SMEs, family firms | Startups, PE/VC |
//!
//! **Why SA persists despite SAS flexibility:**
//! 1. Public market access (Euronext Paris requires SA/SCA)
//! 2. International recognition (German AG, UK PLC equivalents)
//! 3. Investor confidence from standardized governance
//! 4. Certain regulated industries prefer SA (banking, insurance)
//!
//! The SAS has eclipsed SA for new formations (70%+ of incorporations), but SA dominates
//! CAC 40 listings (38/40 companies) and remains the form for large-scale enterprises.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article L225-1 - SA Definition and Minimum Capital
///
/// ## French Text
///
/// > La société anonyme est la société dans laquelle les actionnaires ne sont responsables
/// > des dettes sociales qu'à concurrence de leurs apports et dont les droits des actionnaires
/// > sont représentés par des actions.
/// >
/// > Le capital minimum de la société anonyme est fixé à 37 000 euros.
///
/// ## English Translation
///
/// > The société anonyme (SA) is a company in which shareholders are liable for company debts
/// > only up to the amount of their contributions and whose shareholders' rights are
/// > represented by shares.
/// >
/// > The minimum capital of the SA is set at €37,000.
///
/// ## Legal Significance
///
/// This article establishes the foundational elements of the SA:
///
/// 1. **Limited liability (responsabilité limitée)**: Shareholders risk only their capital contributions,
///    never personal assets. This principle, established in 1867, enabled capital formation for railroads,
///    steel mills, and industrial ventures by protecting bourgeois investors.
///
/// 2. **Minimum capital of €37,000**: Much higher than SARL/SAS (€1 since 2003/1999 reforms). This
///    requirement serves multiple functions:
///    - Creditor protection (capital buffer against insolvency)
///    - Seriousness filter (prevents frivolous SA formations)
///    - Public market readiness (€37K seen as floor for companies seeking IPO)
///    - EU compliance (Second Company Law Directive requires €25K minimum for public companies)
///
/// 3. **Share representation (actions)**: SA rights are embodied in shares (actions), not parts sociales
///    (SARL). Actions are freely transferable unless statuts restrict (Article L228-23), enabling
///    liquidity and stock exchange listing.
///
/// ## Historical Evolution of Capital Requirements
///
/// - **Pre-1867**: SA formation required government authorization; no fixed minimum but high barriers
/// - **1867**: Liberalization with 25 million francs minimum (for larger SAs)
/// - **1966 Reform**: 250,000 francs minimum (public offering) / 100,000 francs (private SA)
/// - **1983**: 250,000 francs (~€38K) for public / 100,000 francs for private
/// - **2002**: €37,000 (conversion from 250,000 francs), no public/private distinction
/// - **2009**: Ordonnance maintains €37,000 despite crisis-era calls for reduction
///
/// The capital has remained stable at €37,000 despite inflation, reflecting policy choice to
/// preserve SA as premium corporate form. Critics argue this discourages entrepreneurship;
/// defenders cite creditor protection and SA's suitability for established businesses.
///
/// ## Comparative Analysis: Capital Requirements Across Jurisdictions
///
/// | Jurisdiction | Company Type | Minimum Capital | Notes |
/// |--------------|--------------|-----------------|-------|
/// | **France** | SA | €37,000 | Highest in EU for stock companies |
/// | France | SARL | €1 | Abolished €7,500 minimum in 2003 |
/// | France | SAS | €1 | Flexible since 1999 |
/// | **Germany** | AG (Aktiengesellschaft) | €50,000 | Similar prestige role as SA |
/// | Germany | GmbH | €25,000 | Half must be paid up initially |
/// | **Japan** | KK (株式会社) | ¥1 | Abolished ¥10M minimum in 2006 |
/// | Japan | GK (合同会社) | ¥1 | LLC equivalent, low use |
/// | **USA** | Delaware Corp | $0 | No capital requirement (par value nominal) |
/// | **UK** | PLC (Public Limited Co) | £50,000 (~€57K) | EU directive compliance |
/// | UK | Ltd (Private Limited) | £1 | Most common form |
/// | **Netherlands** | NV (Naamloze Vennootschap) | €45,000 | Public company form |
/// | **Switzerland** | AG | CHF 100,000 (~€110K) | 20% must be paid up |
/// | **China** | Company (公司) | CNY 0 | Abolished minimums in 2013 |
///
/// **Key Insight**: France's €37,000 requirement is moderate globally but high within EU.
/// This reflects French preference for capital maintenance over Anglo-American flexibility.
///
/// ## Modern Applications and Policy Debates
///
/// ### ESG and Stakeholder Governance (PACTE 2019)
///
/// The PACTE Law amended Article 1833 of the Code civil to require companies to consider
/// social and environmental impacts, not just shareholder profit maximization. SAs can now
/// adopt a "raison d'être" (purpose) in their statuts, committing to specific ESG goals.
///
/// Example: Danone SA adopted "Entreprise à Mission" status, embedding health/environment
/// goals in governance.
///
/// ### Digital Transformation
///
/// - **Dematerialized shares**: Article L228-1 allows fully electronic shares (no paper certificates)
/// - **Remote general meetings**: COVID-19 accelerated virtual AGM adoption (Ordonnance 2020-321)
/// - **Blockchain experiments**: Some SAs exploring tokenized shares (regulatory uncertainty remains)
///
/// ### Venture Capital Considerations
///
/// VCs overwhelmingly prefer SAS (95%+ of French tech startups) due to:
/// - Lower €1 capital requirement
/// - Flexible governance (no board requirement)
/// - Easier liquidation preferences and anti-dilution in statuts
///
/// SA remains rare in startup ecosystem, reserved for later-stage companies nearing IPO.
///
/// ### Recent Case Law
///
/// - **Metaleurop (Cass. crim., 2005)**: SA directors liable for environmental damages despite
///   limited liability—personal fault exception
/// - **Kerviel/Société Générale (2016)**: Trader's unauthorized €4.9B loss; court held SA liable
///   but reduced damages due to governance failures
/// - **Renault-Nissan (2019)**: Cross-border governance disputes highlight SA board independence issues
///
/// ## Practical Formation Steps
///
/// 1. **Draft statuts** (articles of incorporation) with notary or legal counsel
/// 2. **Deposit €37,000** in blocked bank account (released upon registration)
/// 3. **Appoint board** (3-18 directors per Article L225-17)
/// 4. **File with RCS** (Registre du Commerce et des Sociétés) at local commercial court
/// 5. **Publish formation notice** in official legal gazette (JAL - Journal d'Annonces Légales)
/// 6. **Obtain SIREN/SIRET** from INSEE (national statistical office)
///
/// Average formation time: 1-2 weeks. Costs: €500-2,000 (legal fees + filing fees).
#[must_use]
pub fn article_l225_1() -> Statute {
    Statute::new(
        "code-commerce-l225-1",
        "Code de commerce Article L225-1 - SA Definition / Minimum Capital",
        Effect::new(
            EffectType::StatusChange,
            "Création d'une société anonyme avec capital minimum / Creation of SA with minimum capital",
        )
        .with_parameter("company_type", "Société Anonyme (SA)")
        .with_parameter("liability", "Limited to contributions (responsabilité limitée)")
        .with_parameter("minimum_capital", "€37,000")
        .with_parameter("share_type", "Actions (shares)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Minimum capital requirement: €37,000
    .with_precondition(Condition::Income {
        operator: legalis_core::ComparisonOp::GreaterOrEqual,
        value: 37_000, // Using Income condition for capital amount
    })
    // Must have shareholders
    .with_precondition(Condition::AttributeEquals {
        key: "has_shareholders".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article L225-1 définit la société anonyme et fixe son capital minimum à 37 000 €. \
        Ce montant, bien supérieur à celui de la SARL (1 €), reflète la vocation de la SA \
        à accueillir de grandes entreprises et à faire appel public à l'épargne. \
        \n\nLa responsabilité limitée des actionnaires est un principe fondamental : \
        ils ne risquent que leur apport, jamais leurs biens personnels. \
        \n\nArticle L225-1 defines the SA and sets its minimum capital at €37,000. \
        This amount, much higher than the SARL (€1), reflects the SA's purpose \
        to accommodate large enterprises and make public offerings. \
        \n\nLimited liability of shareholders is a fundamental principle: \
        they risk only their contribution, never their personal assets. \
        \n\n【日仏比較】\n\
        日本の株式会社の最低資本金は2006年会社法改正により撤廃され（旧1,000万円から1円に）、\
        フランスのSAと同様、株主は出資額の範囲でのみ責任を負う。\
        ただし、フランスのSAは依然として€37,000の最低資本金を要求しており、\
        これは大企業向けの法形態としての性格を維持するためである。",
    )
}

/// Article L225-17 - Board of Directors Composition
///
/// ## French Text
///
/// > Le conseil d'administration est composé de trois membres au moins et de dix-huit au plus.
///
/// ## English Translation
///
/// > The board of directors is composed of at least three members and at most eighteen.
///
/// ## Legal Significance
///
/// Article L225-17 mandates a collegial board structure for all SAs, establishing precise size limits:
///
/// - **Minimum 3 directors**: Ensures collegial deliberation and prevents concentration of power.
///   This contrasts with SARL (single gérant permitted) and reflects French preference for
///   collective decision-making derived from Roman law traditions of collegial magistracies.
///
/// - **Maximum 18 directors**: Prevents boards from becoming unwieldy and inefficient. The 18-member
///   cap (raised from 12 in 1966 reform, then to 15, then 18 in 2001 NRE Law) balances representation
///   (employee directors, diverse expertise) against efficiency concerns.
///
/// - **Legal personality permitted**: Directors can be individuals (personnes physiques) or legal
///   entities (personnes morales), though the latter must appoint a permanent representative.
///   This allows corporate shareholders (especially in groups) to have direct board seats.
///
/// - **Mandatory for SA**: Unlike SAS (no board requirement, just président), SA must have
///   conseil d'administration. This rigidity provides standardization for investors but limits
///   governance flexibility.
///
/// ## Historical Development of Board Size Limits
///
/// - **1867 Law**: No fixed minimum/maximum; practice varied widely
/// - **1966 Reform**: 3-12 directors mandated (codifying best practices)
/// - **2001 NRE Law**: Maximum raised to 18 to accommodate:
///   - Employee-elected directors (up to 5 under certain conditions)
///   - Gender diversity initiatives (pre-quota era)
///   - Stakeholder representation (environmentalists, consumer advocates in some SAs)
/// - **2011 Copé-Zimmermann**: Gender quota (40% women) drove some boards to 18 to comply
///   without removing existing directors
///
/// ## Comparative Analysis: Board Structures Globally
///
/// | Jurisdiction | Board Type | Minimum | Maximum | Key Features |
/// |--------------|------------|---------|---------|--------------|
/// | **France SA** | One-tier (conseil) | 3 | 18 | Mandatory; can opt for two-tier (directoire + conseil de surveillance) |
/// | **Germany AG** | Two-tier | Varies | Varies by size | Aufsichtsrat (supervisory, 3-21) + Vorstand (management, 1+) |
/// | **Japan KK** | Kansayaku or committee | 3 | No limit | With kansayaku: 3+ directors + 3+ auditors |
/// | **USA Delaware** | One-tier | 1 | No limit | Extreme flexibility; most large corps have 9-15 |
/// | **UK PLC** | One-tier | 2 | No limit | Companies Act requires 2+ directors |
/// | **Netherlands NV** | One-tier or two-tier | 1 | No limit | Structuurregime requires two-tier if large |
/// | **Switzerland AG** | One-tier (Verwaltungsrat) | 1 | No limit | Small boards common (3-7) |
/// | **China Company** | Board or executive director | 3+ (if board) | 13 | Party committees in SOEs effectively co-govern |
///
/// **Key Insight**: France's 3-18 range is restrictive compared to common law jurisdictions but
/// less so than Germany's rigid two-tier system. The fixed maximum reflects civil law preference
/// for bright-line rules over Anglo-American principles-based governance.
///
/// ## Modern Applications: Gender Diversity and Quotas
///
/// ### Copé-Zimmermann Law (2011)
///
/// Article L225-17 must be read alongside Article L225-18-1 (gender quota):
/// - Boards with 8+ members: 40% minimum of each sex (phased: 20% by 2014, 40% by 2017)
/// - Non-compliance: Appointments void, director fees suspended
/// - **Impact**: France went from 10% women directors (2009) to 45% (2022), highest in EU
///
/// **Case Study: CAC 40 Compliance**
/// - 2010: Average 12% women
/// - 2017: 42% (quota reached)
/// - 2024: 46% (exceeds quota)
///
/// **Unintended Consequences**:
/// - "Golden skirts" phenomenon: Small pool of qualified women serving on many boards
/// - Fewer women in executive roles (CEO, CFO) than board seats
/// - Pressure to expand boards to 18 to add women without removing men (temporary transitional tactic)
///
/// ### Independent Directors
///
/// While not required by Code de commerce, soft law (AFEP-MEDEF Code, 2008 revised) recommends:
/// - Listed SAs: 50% independent directors
/// - Controlled companies: 33% independent
///
/// **Independence criteria** (AFEP-MEDEF):
/// - No employment by company in past 5 years
/// - No significant business relationships
/// - Not representing major shareholder (>10%)
/// - Less than 12 years tenure
///
/// **Leading Cases**:
/// - **Vivendi (2002-2012)**: Shareholder lawsuits alleged lack of independence enabled
///   management misconduct (Messier scandal). Courts held directors liable for insufficient oversight.
/// - **Renault-Nissan (2019-2020)**: Alliance governance disputes highlighted need for genuinely
///   independent directors to mediate shareholder conflicts.
///
/// ## Board Governance Options: Monist vs. Dualist
///
/// Article L225-17 establishes the default **monist system** (one-tier board). However, SAs can
/// opt for **dualist system** (Articles L225-57+):
///
/// | Feature | Monist (conseil d'administration) | Dualist (directoire + conseil de surveillance) |
/// |---------|-----------------------------------|------------------------------------------------|
/// | Structure | Single board (3-18) | Management board (1-5) + supervisory board (3-18) |
/// | Power | Appoints PDG/CEO; strategic decisions | Directoire manages; conseil supervises only |
/// | Separation | Chairman can be CEO (PDG) | Strict separation (directoire cannot sit on conseil) |
/// | Flexibility | More flexible (président + DG option) | Rigid roles prevent conflicts of interest |
/// | Prevalence | 90% of French SAs | 10% (e.g., Airbus, Michelin) |
///
/// **When to choose dualist**:
/// - Family firms wanting to separate ownership (conseil) from management (directoire)
/// - Companies with labor co-determination (easier to add employee reps to conseil)
/// - Preventing CEO dominance (strict separation ensures oversight)
///
/// ## Practical Considerations
///
/// ### Board Composition Best Practices
///
/// 1. **Skills matrix**: Map expertise (finance, tech, legal, international) against strategic needs
/// 2. **Diversity**: Beyond gender (age, nationality, professional background)
/// 3. **Committees**: Audit (mandatory if listed), remuneration, nomination, CSR
/// 4. **Evaluation**: Annual board self-assessment (AFEP-MEDEF recommendation)
///
/// ### Director Appointment Process
///
/// 1. **Initial directors** (3-18): Named in statuts or appointed by founders pre-formation
/// 2. **Subsequent directors**: Elected by ordinary general meeting (AGO)
/// 3. **Employee directors**: If 1,000+ employees, 1-2 employee-elected directors (Loi Rebsamen 2015)
/// 4. **Term**: Max 6 years (Article L225-18), renewable
///
/// ### Removal
///
/// - **Ad nutum** (at will): AGO can remove any director without cause (Article L225-18 paragraph 3)
/// - **Compensation**: No severance required unless employment contract (rare for directors)
/// - This contrasts with directoire (dualist system), which has fixed terms
///
/// ## Recent Developments: ESG and Stakeholder Boards
///
/// ### PACTE Law (2019) - Stakeholder Representation
///
/// While not changing Article L225-17 numerically, PACTE shifted board duties:
/// - Directors must consider social/environmental impacts (Article 1833 Code civil amended)
/// - "Entreprise à mission" SAs must have mission committee (outside board or as board subcommittee)
/// - Some SAs (Danone, Veolia) added sustainability experts to boards
///
/// ### Duty of Vigilance (Loi 2017)
///
/// SAs with 5,000+ employees in France (or 10,000+ worldwide) must establish vigilance plan:
/// - Board oversight of supply chain human rights/environmental risks
/// - Potential director liability for failures (untested in courts as of 2024)
///
/// ### COVID-19 Adaptations
///
/// - **Ordonnance 2020-321**: Allowed fully remote board meetings (previously required physical quorum)
/// - Many SAs made remote meetings permanent in statuts
/// - Debate ongoing: Does remote governance reduce board effectiveness?
#[must_use]
pub fn article_l225_17() -> Statute {
    Statute::new(
        "code-commerce-l225-17",
        "Code de commerce Article L225-17 - Board Composition",
        Effect::new(
            EffectType::StatusChange,
            "Constitution d'un conseil d'administration / Constitution of board of directors",
        )
        .with_parameter("minimum_directors", "3")
        .with_parameter("maximum_directors", "18"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Minimum 3 directors
    .with_precondition(Condition::Threshold {
        attributes: vec![("director_count".to_string(), 1.0)],
        operator: legalis_core::ComparisonOp::GreaterOrEqual,
        value: 3.0,
    })
    // Maximum 18 directors
    .with_precondition(Condition::Threshold {
        attributes: vec![("director_count".to_string(), 1.0)],
        operator: legalis_core::ComparisonOp::LessOrEqual,
        value: 18.0,
    })
    .with_discretion(
        "L'article L225-17 impose au conseil d'administration de la SA une composition \
        entre 3 et 18 membres. Le minimum de 3 assure une délibération collégiale, \
        tandis que le maximum de 18 évite les conseils pléthoriques. \
        \n\nCette composition contraste avec celle de la SARL (gérant unique possible) \
        et de la SAS (liberté statutaire totale). \
        \n\nArticle L225-17 requires the SA board to have between 3 and 18 members. \
        The minimum of 3 ensures collegial deliberation, while the maximum of 18 \
        prevents unwieldy boards. \
        \n\nThis composition contrasts with the SARL (single manager possible) \
        and SAS (total statutory freedom). \
        \n\n【日本法との比較】\n\
        日本の取締役会設置会社も取締役3名以上が必要（会社法331条5項）。\
        ただし、最大人数の制限はない。フランスのSAが18名を上限とするのは、\
        効率的な意思決定を確保するためである。",
    )
}

/// Article L225-18 - Director Term Length
///
/// ## French Text
///
/// > La durée des fonctions des administrateurs ne peut excéder six ans en cas de
/// > nomination par l'assemblée générale et trois ans en cas de nomination par les statuts.
///
/// ## English Translation
///
/// > The term of office of directors cannot exceed six years if appointed by the
/// > general meeting and three years if appointed by the articles of incorporation.
///
/// ## Legal Significance
///
/// Article L225-18 limits director terms to ensure periodic shareholder accountability:
///
/// - **6-year maximum** (AGO appointment): Most common for directors elected at general meetings.
///   The 6-year term balances stability (director expertise accumulation) against democratic
///   accountability (regular shareholder review).
///
/// - **3-year maximum** (statuts appointment): Applies only to initial directors named in articles
///   of incorporation at formation. Shorter term reflects founders' directors' provisional nature—
///   shareholders should ratify governance within 3 years of incorporation.
///
/// - **Renewable without limit**: Directors can be re-elected indefinitely. While AFEP-MEDEF
///   soft law recommends limiting tenure to 12 years for independence, the Code imposes no
///   term limits. Some long-serving directors (20+ years) raise independence concerns.
///
/// - **Removal ad nutum** (at will): Article L225-18 paragraph 3 allows general meeting to
///   remove any director without cause at any time. No severance required (unless separate
///   employment contract, which is rare and heavily regulated).
///
/// ## Historical Context: Term Length Reforms
///
/// - **1966 Law**: Introduced 6-year maximum (previously no limit; some directors served decades)
/// - **1966-2001**: Some debate on reducing to 4 years (US-style) for more frequent accountability
/// - **2001 NRE Law**: Maintained 6 years but encouraged staggered terms (1/3 renewed every 2 years)
///   to ensure board continuity while allowing regular renewal
/// - **2019 PACTE**: No change to term length but increased stakeholder accountability requirements
///
/// The 6-year term is longer than many jurisdictions (US: typically 1-3 years), reflecting
/// French preference for stability and long-term strategic focus over short-term shareholder pressure.
///
/// ## Comparative Analysis: Director Terms Globally
///
/// | Jurisdiction | Standard Term | Limits on Renewal | Removal Process |
/// |--------------|---------------|-------------------|-----------------|
/// | **France SA** | 6 years (3 if in statuts) | None | Ad nutum (at will) by AGO |
/// | **Germany AG** | 5 years (supervisory) | None | Cause required (for Aufsichsrat) |
/// | **Japan KK** | 2 years | None | AGM resolution (majority) |
/// | **USA Delaware** | 1-3 years (typically staggered) | Varies by charter | Majority vote (unless classified board) |
/// | **UK PLC** | No fixed term (annual re-election) | Recommendations (9-12 years) | Simple resolution |
/// | **Netherlands NV** | 4 years | None (soft law: 12 years) | AGM or supervisory board |
/// | **Switzerland AG** | Varies (1-4 years typical) | None | AGM resolution |
/// | **China Company** | 3 years | None | Board/shareholder resolution |
///
/// **Key Insight**: France's 6-year term is among the longest globally, reflecting civil law
/// emphasis on stability over Anglo-American focus on annual accountability. UK's annual
/// re-election (introduced post-2008 crisis) represents opposite extreme.
///
/// ## Staggered Terms (Renouvellement échelonné)
///
/// While not required by Code, AFEP-MEDEF and most listed SAs use staggered terms:
///
/// - **Example**: 18-member board divided into 3 classes of 6 directors each
/// - **Rotation**: One class (6 directors) up for renewal every 2 years
/// - **Advantages**:
///   - Board continuity (never lose all directors at once)
///   - Gradual knowledge transfer to new directors
///   - Reduced proxy fight vulnerability (hostile activist can only replace 1/3 at a time)
/// - **Disadvantages**:
///   - Slower board renewal (poor-performing directors stay longer)
///   - Entrenchment risk (management-friendly boards self-perpetuate)
///
/// **US Comparison**: Delaware allows classified boards (3-year staggered terms), but
/// institutional investors (ISS, Glass Lewis) oppose them as anti-takeover devices.
/// France embraces staggering without controversy, reflecting different activism culture.
///
/// ## Director Duties and Liability During Term
///
/// French directors owe two primary duties:
///
/// ### 1. Duty of Care (Devoir de diligence)
///
/// - Attend board meetings (75%+ attendance expected)
/// - Inform oneself about company affairs
/// - Exercise independent judgment
/// - Participate in committee work
///
/// **Breach**: Gross negligence (faute lourde) or repeated absence can trigger liability.
/// Directors personally liable for damages to company or third parties (Article L225-251).
///
/// **Leading Cases**:
/// - **Kerviel/Société Générale (2016)**: Board held liable for insufficient oversight
///   of trading desk. Directors fined for failure to supervise management adequately.
/// - **Metaleurop (Cass. crim., 2005)**: Environmental damages. Directors personally
///   liable despite corporate veil when personal fault (faute personnelle détachable).
/// - **AZF Toulouse Explosion (2001)**: Criminal charges against directors for
///   industrial accident. Some convicted for involuntary manslaughter.
///
/// ### 2. Duty of Loyalty (Devoir de loyauté)
///
/// - Act in company's interest, not personal or shareholder interests
/// - Disclose conflicts of interest (Article L225-38: related-party transactions)
/// - No corporate opportunity usurpation
/// - Confidentiality obligations
///
/// **Breach**: Self-dealing, insider trading, competing with company.
///
/// **Leading Cases**:
/// - **Vilgrain (Cass. com., 1999)**: Director diverted corporate opportunity to
///   personal business. Court held director liable for profits made.
/// - **Rozenblum Doctrine (Cass. crim., 1985)**: Related-party transactions in
///   corporate groups permitted if:
///   (a) Group interest exists
///   (b) Transactions balanced (reciprocal advantages)
///   (c) No excessive burden on transferor company
///   Applied in many intra-group financing cases.
///
/// ## Removal Procedures: Ad Nutum and Its Limits
///
/// Article L225-18 allows **ad nutum** (at will) removal, but with nuances:
///
/// ### Standard Removal (AGO)
///
/// 1. **Convene AGO**: Requires agenda item proposal (by board, shareholder with >5%)
/// 2. **Vote**: Ordinary resolution (>50% of shares voting; 20% quorum first call)
/// 3. **Effect**: Immediate removal; no severance (unless employment contract)
/// 4. **No cause required**: Unlike many jurisdictions, French law allows causeless removal
///
/// ### Exceptions and Complications
///
/// - **Employee directors**: If removed, replacement must be employee-elected (cannot be
///   filled by AGO). Reflects stakeholder governance protections.
/// - **Legal entity directors**: Removal of company-director requires revoking its
///   permanent representative, not just individual.
/// - **Contractual protections**: While rare, some directors negotiate termination fees
///   in side agreements (controversial; AMF scrutinizes these for conflicts).
///
/// ### Comparison: Germany's "Good Cause" Requirement
///
/// German Aufsichtsrat members can only be removed for "wichtiger Grund" (important cause):
/// - Gross breach of duty
/// - Inability to perform duties
/// - Loss of shareholder confidence (requires court approval)
///
/// French ad nutum rule gives shareholders far more power, but critics argue it
/// encourages short-termism (directors fear removal for unpopular long-term decisions).
///
/// ## Modern Applications: Shareholder Activism
///
/// ### Proxy Fights and Director Replacement
///
/// France has seen rising shareholder activism since 2000s:
///
/// - **2005-2010**: Hedge fund activism (TCI vs. ABN AMRO, Third Point vs. Sotheby's)
/// - **2015**: US activist Elliott targets Pernod Ricard (fails to replace directors)
/// - **2020**: ESG activists target Total SA (now TotalEnergies) over climate strategy;
///   successfully elect one environmental expert director
///
/// **Why activism rarer in France than US**:
/// 1. Concentrated ownership (founding families, state stakes in CAC 40)
/// 2. Double voting rights (actions à droit de vote double) for long-term holders
/// 3. Cultural resistance to hostile campaigns
/// 4. Strong management-labor alliances (employee directors often side with management)
///
/// ### ESG-Driven Director Changes
///
/// Post-PACTE (2019), some SAs face pressure to add ESG expertise:
/// - **Climate**: Boards adding environmental scientists (TotalEnergies, EDF)
/// - **Social**: Human rights experts (especially for duty of vigilance compliance)
/// - **Governance**: Diversity beyond gender (age, nationality, professional background)
///
/// ## Practical Guidance: Term Management Strategies
///
/// ### For Companies
///
/// 1. **Implement staggered terms**: 1/3 renewal every 2 years (continuity + accountability)
/// 2. **Publish term limits**: Adopt 12-year independence guideline (AFEP-MEDEF compliant)
/// 3. **Board evaluation**: Annual assessment; non-renewal if underperforming
/// 4. **Succession planning**: Identify replacements 1-2 years before term expiration
///
/// ### For Directors
///
/// 1. **Commitment**: Ensure availability for 6-year term (including committee work)
/// 2. **D&O insurance**: Verify adequate liability coverage (directors personally liable)
/// 3. **Exit planning**: Understand removal risk (ad nutum) and have transition plan
/// 4. **Continuous education**: Stay updated on governance trends (ESG, digital, etc.)
///
/// ### For Shareholders
///
/// 1. **Monitor terms**: Track directors approaching 12 years (independence concerns)
/// 2. **Voting discipline**: Vote against poorly performing directors at renewal
/// 3. **Proxy advisors**: Consult ISS, Glass Lewis recommendations (though France-specific
///    context often requires overriding US-centric guidelines)
/// 4. **Engagement**: Dialogue with nomination committee before proxy battles (stewardship approach)
#[must_use]
pub fn article_l225_18() -> Statute {
    Statute::new(
        "code-commerce-l225-18",
        "Code de commerce Article L225-18 - Director Term Length",
        Effect::new(
            EffectType::StatusChange,
            "Nomination d'administrateur / Director appointment",
        )
        .with_parameter("max_term_by_meeting", "6 years")
        .with_parameter("max_term_by_statuts", "3 years"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Term must not exceed 6 years
    .with_precondition(Condition::Threshold {
        attributes: vec![("term_years".to_string(), 1.0)],
        operator: legalis_core::ComparisonOp::LessOrEqual,
        value: 6.0,
    })
    .with_discretion(
        "L'article L225-18 limite la durée du mandat des administrateurs à 6 ans \
        (nomination par l'AG) ou 3 ans (nomination statutaire). Cette limitation \
        assure un contrôle périodique des actionnaires sur la gouvernance. \
        \n\nLes mandats sont renouvelables, permettant la stabilité tout en préservant \
        le contrôle démocratique. \
        \n\nArticle L225-18 limits directors' terms to 6 years (general meeting appointment) \
        or 3 years (statutory appointment). This ensures periodic shareholder oversight. \
        \n\nTerms are renewable, allowing stability while preserving democratic control.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_l225_1_creation() {
        let statute = article_l225_1();
        assert_eq!(statute.id, "code-commerce-l225-1");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
    }

    #[test]
    fn test_article_l225_1_capital_requirement() {
        let statute = article_l225_1();

        let params = &statute.effect.parameters;
        assert_eq!(params.get("minimum_capital").unwrap(), "€37,000");
    }

    #[test]
    fn test_article_l225_17_creation() {
        let statute = article_l225_17();
        assert_eq!(statute.id, "code-commerce-l225-17");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("minimum_directors").unwrap(), "3");
        assert_eq!(params.get("maximum_directors").unwrap(), "18");
    }

    #[test]
    fn test_article_l225_18_creation() {
        let statute = article_l225_18();
        assert_eq!(statute.id, "code-commerce-l225-18");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("max_term_by_meeting").unwrap(), "6 years");
    }

    #[test]
    fn test_all_sa_articles_have_discretion() {
        let statutes = vec![article_l225_1(), article_l225_17(), article_l225_18()];

        for statute in statutes {
            assert!(
                statute.discretion_logic.is_some(),
                "{} should have discretion",
                statute.id
            );
        }
    }

    #[test]
    fn test_all_sa_articles_valid() {
        let statutes = vec![article_l225_1(), article_l225_17(), article_l225_18()];

        for statute in statutes {
            assert!(statute.is_valid());
            assert_eq!(statute.validate().len(), 0);
        }
    }
}
