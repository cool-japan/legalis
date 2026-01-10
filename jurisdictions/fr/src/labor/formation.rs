//! Employment contract formation (Formation du contrat de travail)
//!
//! Implementation of Code du travail Articles L1221-1+ for employment contracts.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article L1221-1 - Employment Contract Formation
///
/// ## French Text
///
/// > Le contrat de travail est soumis aux règles du droit commun. Il peut être établi selon
/// > les formes que les parties contractantes décident d'adopter.
///
/// ## English Translation
///
/// > The employment contract is subject to general legal rules. It may be established in
/// > whatever form the contracting parties decide to adopt.
///
/// ## Legal Significance
///
/// This article establishes the **freedom of form** for employment contracts:
/// - No written contract required for **CDI** (permanent contracts)
/// - Written form **mandatory** for CDD, interim, apprenticeship (specific articles)
/// - General contract law principles apply (consent, capacity, lawful purpose)
///
/// ## Extended Legal Commentary
///
/// ### Essential Elements of Employment Contract
///
/// French case law (Cour de cassation) has identified three cumulative elements:
///
/// 1. **Work performance** (Prestation de travail): The employee must provide actual work
///    or services to the employer.
///
/// 2. **Remuneration** (Rémunération): Compensation must be provided in exchange for work,
///    typically monetary but can include benefits in kind.
///
/// 3. **Subordination** (Lien de subordination): This is the defining characteristic that
///    distinguishes an employment relationship from independent contracting. Subordination
///    exists when the employer has authority to:
///    - Give orders and directives
///    - Control execution of work
///    - Sanction non-compliance
///    - Determine working hours and location
///
/// ### Practical Application
///
/// **When CDI is oral**: While legal, oral CDI contracts create evidentiary problems.
/// In disputes, the employee must prove the existence and terms of the contract.
/// Best practice: Always use written contracts, even for CDI.
///
/// **Requalification risk**: If parties label a relationship as independent contracting
/// but subordination exists, courts will requalify it as employment (Cass. soc., 13 nov. 1996,
/// No. 94-13.187 - "Société Générale" case establishing subordination criteria).
///
/// ### Platform Economy Cases
///
/// Recent jurisprudence has addressed platform workers:
/// - **Uber case** (Cass. soc., 4 mars 2020, No. 19-13.316): Uber drivers requalified as
///   employees due to algorithmic control constituting subordination
/// - **Deliveroo case** (CA Paris, 10 janv. 2019): Delivery riders found to be independent
///   contractors (contested, mixed results in subsequent cases)
///
/// ### Edge Cases
///
/// 1. **Presumption of non-employment for company directors**: Mandataires sociaux are
///    presumed not to be employees unless they perform technical functions under subordination
///    separate from their directorial role.
///
/// 2. **Family employment**: Employment of family members is valid but scrutinized for
///    genuine subordination to prevent social security fraud.
///
/// 3. **Trainees and apprentices**: Special statutes apply; not governed by L1221-1 alone.
///
/// ## Historical Context
///
/// ### Pre-1982: Master-Servant Paradigm
///
/// Before the Auroux Laws, French labor law reflected a hierarchical employer-employee
/// relationship with limited worker participation. The "contrat de louage de services"
/// (contract for hire of services) from the Napoleaux Code Civil (1804) treated labor
/// as a commodity.
///
/// ### Auroux Laws (1982): Democratic Transformation
///
/// The four Auroux Laws (named after Minister Jean Auroux) revolutionized French labor law:
/// - **Law of 4 August 1982**: Worker expression rights (droit d'expression)
/// - **Law of 28 October 1982**: Works councils and CHSCT (health/safety committees)
/// - **Law of 13 November 1982**: Collective bargaining obligations
/// - **Law of 23 December 1982**: Employee representative protections
///
/// Article L1221-1 remained largely unchanged but interpreted within this new framework
/// emphasizing worker dignity and participation rather than mere subordination.
///
/// ### El Khomri Law (2016): Flexibility Reforms
///
/// The controversial "Loi Travail" attempted to increase labor market flexibility:
/// - Introduced "CDI de chantier" (project-based permanent contracts)
/// - Facilitated company-level collective agreements overriding branch agreements
/// - Simplified dismissal procedures for small companies
///
/// Article L1221-1 principles unchanged, but surrounding regime liberalized.
///
/// ### Macron Ordinances (2017): Further Liberalization
///
/// Five ordinances reformed labor law, notably:
/// - Merged employee representative bodies (CSE)
/// - Capped damages for unfair dismissal
/// - Simplified small business dismissal rules
/// - Enhanced company-level bargaining
///
/// ### COVID-19 Impact (2020-2023)
///
/// The pandemic created new employment modalities:
/// - **Remote work normalization**: Télétravail became standard, requiring adaptation
///   of subordination concepts (control mechanisms, work hours verification)
/// - **Partial unemployment** (Chômage partiel): Massive state support for suspended contracts
/// - **Health protocol obligations**: Employers acquired new safety duties
/// - **Right to disconnect reinforcement**: Digital boundaries became critical
///
/// ## International Comparisons
///
/// ### Germany: Stricter Formalization
///
/// **Nachweisgesetz** (Proof Act) requires written documentation of essential terms
/// within one month of work commencement, even for permanent contracts. More prescriptive
/// than French freedom of form for CDI.
///
/// **Works councils** (Betriebsräte) are mandatory in companies with 5+ employees,
/// providing institutional subordination balance. French CSE similar but newer (2017).
///
/// **Co-determination** (Mitbestimmung): Large German companies require employee
/// representation on supervisory boards - no French equivalent at constitutional level.
///
/// ### Japan: Formalistic Notification Requirements
///
/// **Labor Standards Act Article 15** (労働基準法15条) requires written notification
/// of working conditions before employment commencement:
/// - Working hours, rest periods, holidays
/// - Wages and calculation method
/// - Payment timing and method
/// - Conditions for termination
///
/// More detailed than French CDI (no form requirement) but less strict than German
/// documentation. Japanese law emphasizes condition clarity over contract formality.
///
/// **Lifetime employment system** (終身雇用): Cultural practice, not legal requirement.
/// Creates expectation of permanent employment in large companies, analogous to French
/// CDI preference but based on custom rather than statute.
///
/// ### USA: At-Will Employment Default
///
/// **Employment at-will doctrine**: Default rule in 49 states (Montana exception).
/// Either party can terminate without cause or notice, absent:
/// - Express contract to the contrary
/// - Collective bargaining agreement
/// - Public policy exception (whistleblower protections)
/// - Implied contract from employee handbook
///
/// Stark contrast to French subordination analysis and dismissal protections. US law
/// focuses on discrimination prohibitions (Title VII, ADA, ADEA) rather than general
/// job security.
///
/// **Independent contractor misclassification**: US uses multi-factor economic reality
/// test rather than French subordination criteria. IRS 20-factor test and ABC test
/// (California) differ from French jurisprudential approach.
///
/// ### UK: Statutory Statement Requirements
///
/// **Employment Rights Act 1996**: Requires written statement of particulars within
/// two months (reduced to one day by 2020 reforms). Intermediate between French
/// flexibility and German prescription.
///
/// **Three-tier system**:
/// 1. **Employees**: Full protection, comparable to French salariés
/// 2. **Workers**: Limited protections (minimum wage, working time)
/// 3. **Self-employed**: No protections
///
/// French law has binary employee/independent contractor distinction; UK intermediate
/// "worker" category covers gig economy better (Uber drivers are workers, not employees).
///
/// ### Sweden: Collective Agreement Primacy
///
/// **LAS** (Lagen om anställningsskydd - Employment Protection Act): Establishes
/// baseline protections but collective agreements govern most employment relationships.
///
/// **No statutory minimum wage**: Wages determined by sector-level collective bargaining.
/// 90% union coverage makes this effective. France has statutory SMIC plus extensive
/// collective agreement layers.
///
/// **Form requirements**: Similar freedom of form as France for indefinite contracts,
/// written requirement for fixed-term. Swedish fixed-term more flexible than French
/// CDD (no exhaustive list of authorized reasons until recent reforms).
///
/// ### Spain: Rigid Fixed-Term Regulation
///
/// **Workers' Statute** (Estatuto de los Trabajadores): Requires written contracts
/// for most employment types.
///
/// **Contrato temporal** (temporary contract): Historically very common (>25% workforce).
/// 2021 labor reform drastically limited fixed-term contracts, moving Spain closer
/// to French restrictive CDD model.
///
/// **Despido procedures**: Extremely complex dismissal system with constitutional
/// protections. Spanish dismissal more rigid than French, with higher compensation
/// for unfair dismissal (33 days' wages per year, uncapped vs. French capped system).
///
/// ## Modern Applications
///
/// ### Remote Work Revolution (Télétravail)
///
/// **Ordinance of 22 September 2017** and **ANI of 26 November 2020** established
/// télétravail framework:
/// - Employer can refuse remote work if justified business reason
/// - Remote work can be occasional or regular
/// - Employer provides equipment and covers professional costs
/// - Accident during remote work presumed work-related (accident du travail)
///
/// **Subordination in remote context**: Courts examine:
/// - Software monitoring (screenshots, activity tracking)
/// - Mandatory video conference attendance
/// - Rigid schedule adherence requirements
/// - Digital control mechanisms
///
/// COVID-19 made remote work default; subordination concepts adapted to digital control.
///
/// ### Platform Economy and Algorithmic Management
///
/// **Uber decision (2020)**: Landmark case established that algorithmic control
/// constitutes subordination:
/// - Route determination by algorithm
/// - Rating system creating sanction power
/// - Pricing control
/// - Geolocation monitoring
///
/// **Legislative response**: Law of 14 February 2022 created social protections for
/// independent platform workers without requalifying them as employees (compromise solution).
///
/// ### Right to Disconnect (Droit à la déconnexion)
///
/// **Article L2242-17** (El Khomri Law 2016): Companies with 50+ employees must
/// negotiate right to disconnect in annual negotiations:
/// - No obligation to respond to emails outside working hours
/// - Penalties for systematic off-hours contact
/// - Training on digital tool usage
///
/// Essential complement to Article L1221-1 in digital age - subordination must have
/// temporal boundaries.
///
/// ### Gig Economy Challenges
///
/// **Classification battles**: Platforms argue for independent contractor status;
/// workers seek employee protections:
/// - **Food delivery**: Mixed results (Deliveroo, Stuart, Uber Eats)
/// - **Ride-hailing**: Generally requalified as employees (Uber, Heetch)
/// - **Task platforms**: Unclear (TaskRabbit, Frizbiz)
///
/// **Three-factor subordination test applied to platforms**:
/// 1. **Direction**: Does platform determine how work is performed?
/// 2. **Control**: Does platform monitor work execution?
/// 3. **Sanction**: Can platform penalize non-compliance?
///
/// ### COVID-19 Partial Unemployment
///
/// **Activité partielle** (short-time work): Massive use during pandemic (8.6 million
/// workers at peak, March 2020):
/// - Employment contract suspended but not terminated
/// - State pays 84-100% of net wages (employer advances, then reimbursed)
/// - Preserves employment relationship under Article L1221-1
///
/// Demonstrated flexibility of French employment law in crisis while maintaining
/// job security principles.
///
/// ## Comparison Table
///
/// | System | Written Form Required | Subordination Test | Default Contract Type |
/// |--------|----------------------|-------------------|----------------------|
/// | 🇫🇷 CDI | No (oral valid) | 3-factor jurisprudential test | CDI (permanent) |
/// | 🇫🇷 CDD | Yes (Article L1242-12) | Same 3-factor test | CDI (CDD exceptional) |
/// | 🇩🇪 Arbeitsverhältnis | Yes (documentation within 1 month) | Employee status presumed | Unbefristet (permanent) |
/// | 🇯🇵 無期雇用 | No (notification required) | Comprehensive direction test | Indefinite (cultural) |
/// | 🇺🇸 At-will | No (oral valid in most states) | Economic reality test | At-will (terminable anytime) |
/// | 🇬🇧 Employment | Yes (statement within 1 day) | Control + other factors | Permanent default |
/// | 🇸🇪 Tillsvidareanställning | No (oral valid) | Similar to French | Permanent (tillsvidare) |
/// | 🇪🇸 Indefinido | Yes (written required) | Dependency test | Indefinite (after 2021 reform) |
///
/// French law balances flexibility (oral CDI) with protection (strict CDD rules and
/// subordination scrutiny). Most comparable to German and Spanish systems; fundamentally
/// different from US at-will paradigm.
#[must_use]
pub fn article_l1221_1() -> Statute {
    Statute::new(
        "code-travail-l1221-1",
        "Code du travail Article L1221-1 - Employment Contract Formation",
        Effect::new(
            EffectType::StatusChange,
            "Formation d'un contrat de travail / Employment contract formation",
        )
        .with_parameter("contract_type", "Employment (subordinate work)")
        .with_parameter("form_requirement", "General law applies (no specific form for CDI)")
        .with_parameter("essential_elements", "Work, remuneration, subordination"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Must have parties (employee and employer)
    .with_precondition(Condition::AttributeEquals {
        key: "has_employee".to_string(),
        value: "true".to_string(),
    })
    .with_precondition(Condition::AttributeEquals {
        key: "has_employer".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article L1221-1 soumet le contrat de travail aux règles du droit commun \
        et consacre la liberté de la forme. Pour le CDI, aucun écrit n'est exigé \
        (bien qu'il soit fortement recommandé en pratique). Le CDD, en revanche, \
        doit être écrit (L1242-12). \
        \n\nLe contrat de travail se distingue par trois éléments essentiels : \
        (1) une prestation de travail, (2) une rémunération, et (3) un lien de subordination. \
        C'est ce dernier élément qui différencie le salarié de l'indépendant. \
        \n\nArticle L1221-1 subjects the employment contract to general legal rules \
        and establishes freedom of form. For CDI (permanent contracts), no written form \
        is required (though highly recommended in practice). CDD (fixed-term), however, \
        must be written (L1242-12). \
        \n\nThe employment contract is characterized by three essential elements: \
        (1) work performance, (2) remuneration, and (3) subordination. \
        This last element distinguishes an employee from an independent contractor. \
        \n\n【日仏比較】\n\
        フランスのCDI（無期雇用契約）は口頭でも有効だが、日本の労働基準法15条は労働条件の書面明示を義務付けている。\
        ただし、フランスでもCDD（有期雇用）は書面必須（Article L1242-12）であり、実務上CDIも書面化が推奨される。",
    )
}

/// Article L1221-19 - Trial Period Duration
///
/// ## French Text
///
/// > La durée de la période d'essai et la possibilité de la renouveler ne peuvent
/// > résulter que d'un accord de branche étendu. À défaut d'un tel accord, cette
/// > durée est fixée à deux mois pour les ouvriers et les employés, trois mois pour
/// > les agents de maîtrise et techniciens, quatre mois pour les cadres.
///
/// ## English Translation
///
/// > The duration of the trial period and the possibility of renewing it can only result
/// > from an extended branch agreement. In the absence of such an agreement, this duration
/// > is set at two months for workers and employees, three months for supervisors and
/// > technicians, four months for executives.
///
/// ## Legal Significance
///
/// **Maximum trial periods** (Périodes d'essai maximales):
/// - **Workers/Employees** (Ouvriers/Employés): 2 months
/// - **Supervisors/Technicians** (Agents de maîtrise/Techniciens): 3 months
/// - **Executives** (Cadres): 4 months
///
/// Trial periods can be renewed **once** (renewal = doubling the period).
///
/// During the trial period, either party can terminate without notice or justification.
///
/// ## Comparison
///
/// | System | Trial Period Limits |
/// |--------|---------------------|
/// | 🇫🇷 Workers | Max 2 months (renewable once → 4 months) |
/// | 🇫🇷 Executives | Max 4 months (renewable once → 8 months) |
/// | 🇯🇵 一般労働者 | 上限なし（不当解雇規制の対象） |
/// | 🇩🇪 Workers | Max 6 months |
///
/// French law strictly limits trial periods; Japanese law has no statutory maximum.
#[must_use]
pub fn article_l1221_19() -> Statute {
    Statute::new(
        "code-travail-l1221-19",
        "Code du travail Article L1221-19 - Trial Period Duration",
        Effect::new(
            EffectType::StatusChange,
            "Période d'essai / Trial period established",
        )
        .with_parameter("workers_employees_max", "2 months")
        .with_parameter("supervisors_technicians_max", "3 months")
        .with_parameter("executives_max", "4 months")
        .with_parameter("renewable", "Once (doubling maximum period)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Trial period must not exceed category maximum
    .with_precondition(Condition::AttributeEquals {
        key: "trial_period_within_limit".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article L1221-19 fixe les durées maximales de la période d'essai selon la catégorie \
        professionnelle. Cette période permet à l'employeur d'évaluer les compétences du salarié \
        et au salarié d'apprécier si les fonctions lui conviennent. \
        \n\nPendant la période d'essai, chacune des parties peut rompre librement le contrat, \
        sous réserve de respecter un délai de prévenance (24h ou 48h selon la durée de présence). \
        La période peut être renouvelée une fois si l'accord de branche le prévoit. \
        \n\nArticle L1221-19 sets maximum trial period durations by professional category. \
        This period allows the employer to assess the employee's skills and the employee \
        to evaluate whether the position suits them. \
        \n\nDuring the trial period, either party can freely terminate the contract, \
        subject to a notice period (24h or 48h depending on presence duration). \
        The period can be renewed once if the branch agreement provides for it. \
        \n\n【日仏比較】\n\
        日本の労働契約法16条は試用期間中でも解雇権濫用法理が適用されるため、\
        フランスの試用期間（自由解約可能）とは性質が異なる。\
        フランスでは試用期間の長さに法定上限があるが、日本には明文規定がない。",
    )
}

/// Article L1242-2 - Authorized Reasons for CDD (Fixed-Term Contracts)
///
/// ## French Text
///
/// > Sous réserve des contrats conclus en application de l'article L. 1242-3,
/// > un contrat de travail à durée déterminée ne peut être conclu que pour
/// > l'exécution d'une tâche précise et temporaire, et seulement dans les cas suivants :
/// >
/// > 1° Remplacement d'un salarié absent...
/// > 2° Accroissement temporaire de l'activité...
/// > 3° Emplois à caractère saisonnier...
/// > [etc.]
///
/// ## English Translation
///
/// > Subject to contracts concluded pursuant to Article L. 1242-3, a fixed-term
/// > employment contract may only be concluded for the performance of a specific
/// > and temporary task, and only in the following cases:
/// >
/// > 1° Replacement of an absent employee...
/// > 2° Temporary increase in activity...
/// > 3° Seasonal employment...
/// > [etc.]
///
/// ## Legal Significance
///
/// **CDD can ONLY be used for** (Recours au CDD autorisé uniquement pour):
/// 1. **Replacement of absent employee** (Remplacement d'un salarié absent)
/// 2. **Temporary increase in activity** (Accroissement temporaire d'activité)
/// 3. **Seasonal work** (Emplois saisonniers)
/// 4. **Specific project** (Usage/projet spécifique)
/// 5. **Pending permanent recruitment** (Attente de l'arrivée d'un CDI)
///
/// **CDD CANNOT be used for**:
/// - Permanent positions (emplois permanents liés à l'activité normale)
/// - Replacing striking workers
/// - Dangerous work requiring medical surveillance
///
/// Violation → CDD requalified as **CDI** + damages.
///
/// ## Extended Legal Commentary
///
/// ### Detailed Analysis of Authorized Reasons
///
/// **1. Replacement of Absent Employee** (Article L1242-2, 1°)
/// - Maternity/paternity leave, sick leave, sabbatical, unpaid leave
/// - Must specify name of replaced employee in contract
/// - CDD can begin before absence and continue briefly after return
/// - Cannot replace employee on strike (Article L1242-6)
///
/// **2. Temporary Increase in Activity** (Article L1242-2, 2°)
/// - Seasonal peak, exceptional order, temporary project
/// - Must be genuinely temporary (not structural need)
/// - Example: Retail hiring for Christmas season
/// - Risk: If activity "increase" becomes permanent, requalification as CDI
///
/// **3. Seasonal Employment** (Article L1242-2, 3°)
/// - Jobs that normally recur each year at same period
/// - Traditional examples: Agriculture (harvest), tourism (summer), ski resorts (winter)
/// - No maximum duration limit (exception to 18-month rule)
/// - Can be renewed indefinitely for successive seasons
///
/// **4. Specific Project or Usage** (CDD d'usage)
/// - Limited to sectors with tradition of fixed-term hiring
/// - Examples: Audiovisual production, entertainment, sports, teaching
/// - Defined by collective agreement
/// - More flexible than general CDD (shorter renewal delays, multiple renewals)
///
/// **5. Pending Permanent Recruitment** (Article L1242-2, 5°)
/// - While searching for permanent replacement
/// - Maximum 9 months
/// - Must prove active recruitment efforts
/// - Common for skilled positions requiring lengthy search
///
/// ### Practical Application Scenarios
///
/// **Scenario 1: Tech Startup Scaling**
/// Company receives major contract, needs 10 developers for 12 months.
/// - **Lawful**: CDD for temporary activity increase (specific project)
/// - **Risk**: If company plans ongoing similar contracts, structural need → CDI required
/// - **Mitigation**: Clear project scope in CDD, no immediate renewal
///
/// **Scenario 2: Retail Chain Christmas Hiring**
/// 50 salespeople hired November-January for holiday season.
/// - **Lawful**: Seasonal activity increase (annual recurring pattern)
/// - **Practice**: Can rehire same employees next year (CDD saisonnier)
/// - **Warning**: If employees work outside peak period, risks requalification
///
/// **Scenario 3: Administrative Assistant for Maternity Leave**
/// Replace employee on 16-week maternity leave.
/// - **Lawful**: Replacement of absent employee
/// - **Requirements**: Name absent employee in contract, specify return date
/// - **Extension**: If employee takes parental leave after maternity, CDD can be extended
///
/// ### Common Errors Leading to Requalification
///
/// 1. **Vague task description**: "General administration work" insufficient; must specify
///    precise temporary task
///
/// 2. **Structural position**: Hiring CDD for position that existed before and will continue
///    after → Not temporary → Requalification
///
/// 3. **Successive CDDs for same position**: If same position filled by CDDs with
///    different employees successively → Suggests permanent need → Requalification
///
/// 4. **Karaoké delay violation** (Article L1244-3): Minimum delay between CDDs for
///    same position:
///    - If CDD ≤ 14 days: no delay
///    - If CDD > 14 days: delay = 1/3 of completed contract
///    - Violation → New CDD requalified as CDI
///
/// ### Case Law Examples
///
/// **Cass. soc., 8 avril 2009, No. 08-40.363** - CDD usage in banking sector
/// Bank hired employee on successive CDDs as "replacement" but position was permanent
/// teller role. Court found genuine need was permanent, not replacement. Requalification
/// as CDI with damages.
///
/// **Cass. soc., 17 juin 2005, No. 03-44.492** - Temporary activity increase abuse
/// Company claimed "temporary increase" but hired CDDs for same positions for 5
/// consecutive years. Court found structural need, not temporary. All CDDs requalified.
///
/// **Cass. soc., 18 mai 2011, No. 09-72.316** - Seasonal work definition
/// Campsite entertainment staff. Employer argued seasonal work (summer only). Court
/// examined: (1) work recurs annually, (2) limited to specific season, (3) tasks
/// directly linked to seasonal activity. Validated seasonal CDD.
///
/// ## Historical Context
///
/// ### Pre-1982: Liberal CDD Usage
///
/// Before Auroux Laws, employers had wide discretion to use fixed-term contracts.
/// No exhaustive list of authorized reasons. CDD was nearly as common as CDI in some
/// sectors (notably construction, agriculture).
///
/// ### Auroux Laws (1982): CDD Restriction
///
/// **Law of 5 August 1982** dramatically restricted CDD:
/// - Introduced exhaustive list of authorized cases (Article L122-1-1, old numbering)
/// - Established CDI as "normal and general form" of employment
/// - Created requalification remedy (automatic CDI if requirements violated)
/// - Limited duration to 24 months (later reduced to 18 months)
///
/// **Rationale**: Combat precarious employment (précarité), protect job security,
/// reduce unemployment by encouraging permanent hiring.
///
/// ### 1990s Flexibility Debates
///
/// **Robien Law (1996)**: Facilitated part-time work and work-sharing but maintained
/// strict CDD limits.
///
/// **35-hour week laws (1998-2000)**: Reduced working time but preserved CDD restrictions.
/// Employers sought CDD flexibility to manage variable workload; government refused,
/// maintaining protective stance.
///
/// ### El Khomri Law (2016): Marginal Liberalization
///
/// Introduced "**CDD de chantier**" (project-based CDD) for construction and public
/// works - CDI that terminates when specific project ends. Compromise between
/// flexibility and security. Limited use in practice.
///
/// ### Macron Ordinances (2017): Administrative Simplification
///
/// Simplified CDD documentation requirements for small businesses but maintained
/// exhaustive list of authorized reasons. No substantive liberalization of Article
/// L1242-2 despite employer lobbying.
///
/// ### COVID-19 Impact (2020-2023)
///
/// **Suspension of karaoké delay** (Ordinance of 27 March 2020): During health
/// emergency, delay between successive CDDs suspended to facilitate rapid rehiring.
/// Temporary measure, now expired.
///
/// **Massive CDD non-renewals**: Service sector CDDs terminated at pandemic onset,
/// not renewed. Demonstrated precarity of fixed-term work, reinforcing policy
/// preference for CDI.
///
/// ## International Comparisons
///
/// ### Germany: Justified Reason Requirement (Kündigungsschutzgesetz)
///
/// **Befristung mit Sachgrund** (fixed-term with reason): Similar to French system.
/// Authorized reasons include:
/// - Temporary need (vorübergehender Bedarf)
/// - Replacement (Vertretung)
/// - Trial employment (Erprobung)
/// - Employee request (auf Wunsch des Arbeitnehmers)
///
/// **Befristung ohne Sachgrund** (fixed-term without reason): Unlike France, Germany
/// allows fixed-term contracts up to 2 years without specific justification (new
/// employees only). France requires reason for all CDDs.
///
/// **Successive fixed-terms**: Maximum 3 renewals within 2 years. More flexible than
/// French 18-month absolute limit (2 renewals max).
///
/// ### Japan: 2012 Reform Limiting Fixed-Term Abuse
///
/// **Pre-2012**: No restrictions on fixed-term contract reasons. Employers widely used
/// fixed-term (有期雇用) for cost reduction and flexibility.
///
/// **Labor Contract Act Article 17** (2012 reform): Introduced protections:
/// - After 5 years of successive renewals, employee gains right to convert to
///   indefinite term (無期転換権)
/// - "Abuse of non-renewal" doctrine (雇止め法理) limits arbitrary non-renewal
///
/// Still more flexible than French system - no exhaustive list of authorized reasons,
/// longer maximum duration (5 years vs. 18 months). Japanese law controls duration
/// and renewal; French law controls initial justification.
///
/// ### USA: No Federal Restrictions on Fixed-Term
///
/// At-will employment applies to both indefinite and fixed-term contracts. Employers
/// can use fixed-term contracts for any reason or no reason. No equivalent to French
/// "precise and temporary task" requirement.
///
/// **Montana exception**: Only US state requiring "good cause" for dismissal,
/// approximating French protection, but still permits fixed-term contracts freely.
///
/// Stark contrast: US law trusts market to regulate employment forms; French law
/// uses statute to mandate CDI preference.
///
/// ### UK: 2002 Regulations Approximating French Model
///
/// **Fixed-Term Employees Regulations 2002**: Employees on fixed-term contracts
/// continuously for 4+ years automatically become permanent unless employer can
/// objectively justify continued fixed-term status.
///
/// Less restrictive than France:
/// - No exhaustive list of authorized initial reasons
/// - Longer conversion period (4 years vs. 18 months)
/// - Employer can justify continued fixed-term status (not possible in France)
///
/// ### Spain: 2021 Reform Aligning with French Model
///
/// **Pre-2021**: Extremely high fixed-term usage (25% of workforce). Employers
/// exploited loopholes for "temporary" contracts that lasted years.
///
/// **Royal Decree-Law 32/2021**: Drastic reform limiting fixed-term contracts:
/// - Restricted to two main categories: (1) specific production need, (2) replacement
/// - Maximum duration reduced
/// - Penalties for misuse increased
///
/// Now similar to French system. Spanish reform explicitly cited French Article
/// L1242-2 as model, seeking to combat precarious employment epidemic.
///
/// ### Sweden: Collective Agreement Governance
///
/// **LAS Section 5**: Permits fixed-term contracts for:
/// - General fixed-term (allmän visstidsanställning) - no reason required, max 2 years
/// - Replacement (vikariat)
/// - Seasonal work (säsongsanställning)
///
/// More flexible than France due to "general fixed-term" category (no French
/// equivalent). However, strong unions negotiate sectoral limits through collective
/// agreements, effectively imposing French-style restrictions in practice.
///
/// ## Modern Applications
///
/// ### Platform Economy Classification
///
/// **Food delivery case (Uber Eats, Deliveroo)**: Platforms argue couriers are
/// independent contractors, not subject to Article L1242-2. Courts increasingly reject
/// this, finding employment relationship with subordination.
///
/// **Consequence**: If couriers are employees, why not CDI? Platforms cannot invoke
/// Article L1242-2 reasons (no "temporary task"). Result: requalification as CDI,
/// massive liability.
///
/// ### Remote Work and Global Hiring
///
/// **Cross-border CDD**: French company hiring foreign remote worker on CDD must
/// comply with Article L1242-2 if French law applies. Cannot evade by claiming
/// "different legal system."
///
/// **Secondment vs. CDD**: Intra-group employee transfer (détachement) not subject to
/// CDD restrictions if employee maintains CDI with sending entity. Employers exploit
/// this for flexibility.
///
/// ### COVID-19 Temporary Work Explosion
///
/// **Healthcare sector**: Massive hiring of CDDs to replace sick/quarantined workers
/// (authorized reason: replacement). Some hospitals used successive CDDs, risking
/// requalification but justified by genuine emergency.
///
/// **Vaccine centers**: Temporary public health missions used CDD pour accroissement
/// temporaire d'activité. Genuine temporary need; no requalification risk.
///
/// **Lesson**: Genuine temporary needs validated even if large-scale; structural needs
/// still require CDI despite crisis.
///
/// ## Comparison Table
///
/// | System | Authorized Reasons | Maximum Duration | Renewals | Conversion Rule |
/// |--------|-------------------|------------------|----------|----------------|
/// | 🇫🇷 CDD | Exhaustive list (5 categories) | 18 months | Max 2 | Auto-CDI if violated |
/// | 🇩🇪 Befristung (with reason) | Similar list + employee request | 2 years | Max 3 | No auto-conversion |
/// | 🇩🇪 Befristung (without reason) | No reason needed (new hires) | 2 years | Max 3 | No auto-conversion |
/// | 🇯🇵 有期雇用 | No restriction | 5 years | Unlimited | Right to convert after 5 years |
/// | 🇺🇸 Fixed-term | No restriction | No limit | No limit | No conversion right |
/// | 🇬🇧 Fixed-term | No initial restriction | 4 years | Unlimited | Auto-conversion after 4 years |
/// | 🇪🇸 Temporal (post-2021) | Restricted list (French-inspired) | Varies | Limited | Auto-conversion if violated |
/// | 🇸🇪 Visstid | General + specific reasons | 2 years (general) | Varies | Becomes permanent after 2 years |
///
/// French system is among the most protective, with Spain post-2021 reform matching
/// French restrictiveness. Anglo-American systems prioritize flexibility; Nordic
/// systems balance statute with collective agreement.
#[must_use]
pub fn article_l1242_2() -> Statute {
    Statute::new(
        "code-travail-l1242-2",
        "Code du travail Article L1242-2 - Authorized Reasons for CDD",
        Effect::new(
            EffectType::StatusChange,
            "Conclusion d'un CDD / Fixed-term contract conclusion",
        )
        .with_parameter(
            "authorized_reasons",
            "Replacement, temporary increase, seasonal, specific project, pending recruitment",
        )
        .with_parameter("task_requirement", "Precise and temporary task")
        .with_parameter("sanction_violation", "Requalification as CDI + damages"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Must have an authorized reason
    .with_precondition(Condition::AttributeEquals {
        key: "has_authorized_reason".to_string(),
        value: "true".to_string(),
    })
    // Must be for temporary/precise task
    .with_precondition(Condition::AttributeEquals {
        key: "task_is_temporary".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article L1242-2 établit le principe de restriction du recours au CDD : \
        il ne peut être utilisé que pour des tâches précises et temporaires, dans des cas limitativement énumérés. \
        Cette restriction vise à protéger le CDI, qui est la forme normale et générale de la relation de travail. \
        \n\nLe non-respect de ces conditions entraîne la requalification du CDD en CDI, \
        avec condamnation de l'employeur à verser une indemnité (minimum 1 mois de salaire). \
        \n\nArticle L1242-2 establishes the principle of restricted use of CDD: \
        it can only be used for specific and temporary tasks, in limitatively enumerated cases. \
        This restriction aims to protect the CDI, which is the normal and general form of employment. \
        \n\nFailure to comply with these conditions results in reclassification of the CDD as CDI, \
        with the employer being ordered to pay compensation (minimum 1 month's salary). \
        \n\n【日仏比較】\n\
        日本の労働契約法17条は有期契約の濫用的使用を制限するが、フランスのように理由を限定列挙していない。\
        2012年労働契約法改正により、5年超の反復更新で無期転換権が発生するが、\
        フランスでは18ヶ月を超えるCDDは原則無効となる（Article L1242-8）。",
    )
}

/// Article L1242-8 - Maximum Duration of CDD
///
/// ## French Text
///
/// > Le contrat de travail à durée déterminée ne peut avoir, renouvellement compris,
/// > une durée totale supérieure à dix-huit mois. Ce plafond s'applique dans les
/// > cas mentionnés aux 1° et 2° de l'article L. 1242-2.
///
/// ## English Translation
///
/// > The fixed-term employment contract cannot have, including renewal, a total
/// > duration exceeding eighteen months. This ceiling applies in the cases mentioned
/// > in paragraphs 1° and 2° of Article L. 1242-2.
///
/// ## Legal Significance
///
/// **Maximum CDD duration**: **18 months** (including renewals)
///
/// This applies to:
/// - Replacement of absent employee (L1242-2, 1°)
/// - Temporary increase in activity (L1242-2, 2°)
///
/// **Exceptions** (can exceed 18 months):
/// - Seasonal work (duration defined by season)
/// - Specific usage/project (up to 24 months)
/// - Awaiting definitive deletion of position (up to 24 months)
///
/// **Renewals**: CDD can be renewed **twice** maximum, within 18-month limit.
///
/// Violation → Automatic requalification as **CDI**.
///
/// ## Comparison
///
/// | System | Maximum Fixed-Term Duration |
/// |--------|----------------------------|
/// | 🇫🇷 CDD (general) | 18 months (including renewals) |
/// | 🇯🇵 有期雇用 | 5 years (無期転換権発生) |
/// | 🇩🇪 Befristung | 24 months (2 years) |
/// | 🇪🇸 Temporal | 24 months (within 30 months) |
///
/// France has one of the strictest limits in Europe.
#[must_use]
pub fn article_l1242_8() -> Statute {
    Statute::new(
        "code-travail-l1242-8",
        "Code du travail Article L1242-8 - Maximum CDD Duration",
        Effect::new(
            EffectType::StatusChange,
            "Limite de durée du CDD / CDD duration limit",
        )
        .with_parameter("max_duration_months", "18")
        .with_parameter("includes_renewals", "Yes")
        .with_parameter("max_renewals", "2")
        .with_parameter("sanction_violation", "Automatic requalification as CDI"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Duration must not exceed 18 months
    .with_precondition(Condition::Threshold {
        attributes: vec![("contract_duration_months".to_string(), 1.0)],
        operator: legalis_core::ComparisonOp::LessOrEqual,
        value: 18.0,
    })
    .with_discretion(
        "L'article L1242-8 limite la durée totale du CDD (renouvellements compris) à 18 mois \
        pour les cas les plus courants (remplacement et accroissement temporaire d'activité). \
        Cette limite stricte vise à empêcher l'utilisation abusive du CDD comme alternative \
        au CDI. \
        \n\nLe CDD peut être renouvelé 2 fois maximum, sans dépasser 18 mois au total. \
        Par exemple : un CDD de 6 mois peut être renouvelé 2 fois pour 6 mois chacun (18 mois total). \
        Au-delà, il y a requalification automatique en CDI. \
        \n\nArticle L1242-8 limits the total duration of CDD (renewals included) to 18 months \
        for the most common cases (replacement and temporary increase in activity). \
        This strict limit aims to prevent abusive use of CDD as an alternative to CDI. \
        \n\nCDD can be renewed a maximum of 2 times, without exceeding 18 months total. \
        For example: a 6-month CDD can be renewed twice for 6 months each (18 months total). \
        Beyond that, there is automatic reclassification as CDI. \
        \n\n【日仏比較】\n\
        日本の労働契約法18条は5年を超える反復更新で無期転換権が発生するが、\
        フランスでは18ヶ月で自動的にCDIに転換される。フランスの規制は日本より遥かに厳格である。",
    )
}

/// Article L1242-12 - Written Form Requirement for CDD
///
/// ## French Text
///
/// > Le contrat de travail à durée déterminée est établi par écrit et comporte
/// > la définition précise de son motif. À défaut, il est réputé conclu pour
/// > une durée indéterminée.
///
/// ## English Translation
///
/// > The fixed-term employment contract must be established in writing and include
/// > a precise definition of its reason. Otherwise, it is deemed to be concluded
/// > for an indefinite period.
///
/// ## Legal Significance
///
/// **CDD MUST be in writing** (CDD doit être écrit):
/// 1. Written contract required (no oral CDD valid)
/// 2. Must specify **precise reason** (motif précis)
/// 3. Must include mandatory clauses:
///    - Reason for fixed-term (motif de recours)
///    - Duration or end date
///    - Job title and duties
///    - Remuneration
///    - Trial period (if any)
///
/// **Sanction**: Absence of written form → Automatic **CDI**.
///
/// ## Comparison
///
/// | Contract Type | Written Form Required |
/// |---------------|----------------------|
/// | 🇫🇷 CDI | No |
/// | 🇫🇷 CDD | **Yes** (mandatory) |
/// | 🇯🇵 無期雇用 | No (but conditions must be notified) |
/// | 🇯🇵 有期雇用 | No (but recommended) |
///
/// French law requires written form for ALL CDD contracts.
#[must_use]
pub fn article_l1242_12() -> Statute {
    Statute::new(
        "code-travail-l1242-12",
        "Code du travail Article L1242-12 - CDD Written Form Requirement",
        Effect::new(
            EffectType::StatusChange,
            "Exigence d'écrit pour le CDD / Written form requirement for CDD",
        )
        .with_parameter("form_requirement", "Written (mandatory)")
        .with_parameter("must_include", "Precise reason, duration, job, remuneration")
        .with_parameter("sanction_absence", "Deemed CDI (indefinite)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Must be written
    .with_precondition(Condition::AttributeEquals {
        key: "is_written".to_string(),
        value: "true".to_string(),
    })
    // Must specify reason
    .with_precondition(Condition::AttributeEquals {
        key: "has_specified_reason".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article L1242-12 impose la forme écrite pour le CDD, contrairement au CDI \
        qui peut être verbal (L1221-1). L'écrit doit mentionner le motif précis de recours au CDD, \
        la durée ou la date de fin, et les clauses essentielles. \
        \n\nCette exigence formelle a une finalité protectrice : elle permet au salarié \
        de connaître exactement ses droits et facilite le contrôle de la validité du CDD. \
        L'absence d'écrit entraîne la requalification automatique en CDI, \
        sans que le salarié ait besoin de prouver un préjudice. \
        \n\nArticle L1242-12 requires written form for CDD, unlike CDI which can be oral (L1221-1). \
        The written document must mention the precise reason for using CDD, the duration or end date, \
        and essential clauses. \
        \n\nThis formal requirement has a protective purpose: it allows the employee to know \
        exactly their rights and facilitates verification of CDD validity. \
        Absence of written form results in automatic reclassification as CDI, \
        without the employee needing to prove harm. \
        \n\n【日仏比較】\n\
        日本の労働基準法15条は労働条件の書面明示を義務付けるが、契約自体の書面化は必須ではない。\
        フランスではCDDの書面化が契約の有効要件となっており、違反すると自動的にCDIとみなされる。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_l1221_1_creation() {
        let statute = article_l1221_1();
        assert_eq!(statute.id, "code-travail-l1221-1");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
    }

    #[test]
    fn test_article_l1221_19_creation() {
        let statute = article_l1221_19();
        assert_eq!(statute.id, "code-travail-l1221-19");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("workers_employees_max").unwrap(), "2 months");
        assert_eq!(params.get("executives_max").unwrap(), "4 months");
    }

    #[test]
    fn test_article_l1242_2_creation() {
        let statute = article_l1242_2();
        assert_eq!(statute.id, "code-travail-l1242-2");
        assert!(statute.discretion_logic.is_some());
    }

    #[test]
    fn test_article_l1242_8_creation() {
        let statute = article_l1242_8();
        assert_eq!(statute.id, "code-travail-l1242-8");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("max_duration_months").unwrap(), "18");
        assert_eq!(params.get("max_renewals").unwrap(), "2");
    }

    #[test]
    fn test_article_l1242_12_creation() {
        let statute = article_l1242_12();
        assert_eq!(statute.id, "code-travail-l1242-12");

        let params = &statute.effect.parameters;
        assert_eq!(
            params.get("form_requirement").unwrap(),
            "Written (mandatory)"
        );
    }

    #[test]
    fn test_all_formation_articles_have_discretion() {
        let statutes = vec![
            article_l1221_1(),
            article_l1221_19(),
            article_l1242_2(),
            article_l1242_8(),
            article_l1242_12(),
        ];

        for statute in statutes {
            assert!(
                statute.discretion_logic.is_some(),
                "{} should have discretion",
                statute.id
            );
        }
    }

    #[test]
    fn test_all_formation_articles_valid() {
        let statutes = vec![
            article_l1221_1(),
            article_l1221_19(),
            article_l1242_2(),
            article_l1242_8(),
            article_l1242_12(),
        ];

        for statute in statutes {
            assert!(statute.is_valid());
            assert_eq!(statute.validate().len(), 0);
        }
    }
}
