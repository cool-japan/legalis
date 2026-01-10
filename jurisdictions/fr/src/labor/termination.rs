//! Employment termination (Rupture du contrat de travail)
//!
//! Implementation of Code du travail Articles L1231+ for dismissal and termination,
//! including France's strict employee protections.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article L1231-1 - Termination Modalities
///
/// ## French Text
///
/// > Le contrat de travail à durée indéterminée peut être rompu à l'initiative
/// > de l'employeur ou du salarié, ou d'un commun accord, dans les conditions
/// > prévues par les dispositions du présent titre.
///
/// ## English Translation
///
/// > The permanent employment contract can be terminated at the initiative of the
/// > employer or employee, or by mutual agreement, under the conditions provided
/// > by the provisions of this title.
///
/// ## Legal Significance
///
/// **Methods of termination** (Modes de rupture):
///
/// 1. **Dismissal** (Licenciement) - Employer initiative
///    - Personal dismissal (Licenciement personnel)
///    - Economic dismissal (Licenciement économique)
/// 2. **Resignation** (Démission) - Employee initiative
/// 3. **Mutual agreement** (Rupture conventionnelle) - Both parties
/// 4. **Retirement** (Départ/Mise à la retraite)
///
/// Each method has specific procedural requirements.
///
/// ## Comparison
///
/// | System | Dismissal Freedom |
/// |--------|-------------------|
/// | 🇫🇷 France | Justified cause required (strict) |
/// | 🇯🇵 Japan | Abuse of dismissal rights prohibited (解雇権濫用法理) |
/// | 🇺🇸 USA | At-will (mostly free, except discrimination) |
/// | 🇩🇪 Germany | Social justification required (Kündigungsschutz) |
///
/// French and German law are among the strictest for employee protection.
#[must_use]
pub fn article_l1231_1() -> Statute {
    Statute::new(
        "code-travail-l1231-1",
        "Code du travail Article L1231-1 - Termination Modalities",
        Effect::new(
            EffectType::StatusChange,
            "Modes de rupture du CDI / CDI termination methods",
        )
        .with_parameter(
            "methods",
            "Dismissal, resignation, mutual agreement, retirement",
        )
        .with_parameter("employer_dismissal", "Requires real and serious cause")
        .with_parameter("employee_resignation", "Free (but notice required)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_discretion(
        "L'article L1231-1 énumère les différents modes de rupture du CDI. \
        Le licenciement par l'employeur est strictement encadré et nécessite une cause réelle et sérieuse \
        (L1232-1), contrairement à la démission du salarié qui est en principe libre. \
        \n\nLa rupture conventionnelle, introduite en 2008, permet aux parties de convenir \
        d'un commun accord de la fin du contrat, avec homologation par l'administration. \
        Cette procédure offre une alternative souple au licenciement. \
        \n\nArticle L1231-1 lists the different methods of terminating a CDI. \
        Dismissal by the employer is strictly regulated and requires a real and serious cause (L1232-1), \
        unlike resignation by the employee which is in principle free. \
        \n\nMutual termination (rupture conventionnelle), introduced in 2008, allows the parties \
        to agree by mutual consent to end the contract, with administrative approval. \
        This procedure offers a flexible alternative to dismissal. \
        \n\n【日仏比較】\n\
        日本の労働契約法16条は解雇権濫用法理を明文化し、「客観的に合理的な理由」と\
        「社会通念上の相当性」を要求する。フランスのL1232-1は「cause réelle et sérieuse」を要求し、\
        いずれも厳格な正当性審査を行う点で類似する。",
    )
}

/// Article L1232-1 - Real and Serious Cause Requirement
///
/// ## French Text
///
/// > Tout licenciement pour motif personnel doit être justifié par une cause réelle
/// > et sérieuse.
///
/// ## English Translation
///
/// > Any dismissal for personal reasons must be justified by a real and serious cause.
///
/// ## Legal Significance
///
/// **Real and serious cause** (Cause réelle et sérieuse) has two requirements:
///
/// 1. **Real** (Réelle):
///    - Objectively verifiable facts
///    - Not fabricated or imaginary
///    - Proven by employer (burden of proof)
///
/// 2. **Serious** (Sérieuse):
///    - Sufficient gravity
///    - Makes continued employment impossible
///    - Proportionate to dismissal
///
/// **Examples of valid causes**:
/// - Serious misconduct (faute grave)
/// - Incompetence (insuffisance professionnelle)
/// - Insubordination (insubordination)
/// - Abandonment of position (abandon de poste)
///
/// **Burden of proof**: Employer must prove cause in court if contested.
///
/// **Sanction**: Dismissal without real and serious cause = Unfair dismissal (licenciement sans cause réelle et sérieuse) → Damages.
///
/// ## Extended Legal Commentary
///
/// ### The Two-Part Test: Real AND Serious
///
/// **Part 1: Real Cause (Cause réelle)**
///
/// The cause must be based on **objective, verifiable facts**:
/// - **Objective**: Independent of subjective employer opinion. "I don't like this
///   employee" insufficient. Must cite specific behaviors or objective performance metrics.
/// - **Verifiable**: Facts must be provable through evidence (documents, witnesses,
///   records). Hearsay insufficient.
/// - **Existing**: Facts must have existed at time of dismissal. Cannot add new
///   reasons retroactively to justify dismissal.
///
/// **Examples of non-real causes (invalid)**:
/// - Rumor of misconduct without verification
/// - Subjective personality conflict ("incompatibilité d'humeur")
/// - Post-hoc rationalization (facts discovered after dismissal)
/// - Discriminatory motives disguised as performance issues
///
/// **Part 2: Serious Cause (Cause sérieuse)**
///
/// The cause must have **sufficient gravity** to justify termination:
/// - **Proportionality**: Sanction must fit offense. Minor tardiness cannot justify
///   dismissal; pattern of absences might.
/// - **Makes continued employment impossible**: Relationship irreparably damaged.
///   Cannot expect parties to continue working together.
/// - **Context matters**: Same behavior may be serious in one context, not in another.
///   Cash register error serious for bank teller, less so for warehouse worker.
///
/// **Proportionality assessment**:
/// - Prior warnings given? (Progressive discipline principle)
/// - Employee's length of service and past record?
/// - Actual harm caused to employer?
/// - Industry standards and customs?
///
/// ### Categories of Personal Dismissal Causes
///
/// **1. Serious Misconduct (Faute grave)**
///
/// Conduct so serious it prevents employee from remaining at work during notice period.
/// **Examples validated by case law**:
/// - Theft from employer (Cass. soc., 16 déc. 1997)
/// - Physical violence against colleague (Cass. soc., 10 mai 2001)
/// - Repeated refusal to obey orders (Cass. soc., 25 févr. 2003)
/// - Serious breach of confidentiality (Cass. soc., 14 mars 2007)
/// - Working for competitor while on sick leave (Cass. soc., 29 janv. 2014)
///
/// **Consequences**: Immediate termination, no notice period, no severance pay.
///
/// **2. Gross Misconduct (Faute lourde)**
///
/// Serious misconduct PLUS **intent to harm employer**. Extremely rare.
/// **Examples**:
/// - Sabotaging equipment or data (Cass. soc., 12 juill. 2005)
/// - Disclosing trade secrets to competitor with intent (Cass. soc., 20 oct. 2010)
///
/// **Consequences**: Same as faute grave plus potential employer damages claim.
///
/// **3. Simple Fault (Faute simple)**
///
/// Misconduct insufficient to constitute faute grave. Employee entitled to notice
/// and severance.
/// **Examples**:
/// - Isolated tardiness or absence
/// - Minor insubordination (single refusal without aggravating circumstances)
/// - Negligence causing minor damage
///
/// **4. Professional Incompetence (Insuffisance professionnelle)**
///
/// Objective inability to perform job duties, **not** misconduct. Employee tried
/// but failed.
/// **Requirements**:
/// - Objective performance deficiencies documented
/// - Employee given training, support, opportunity to improve
/// - Deficiencies attributable to employee, not employer (inadequate tools, training)
///
/// **Cass. soc., 4 déc. 2013**: Incompetence dismissal invalid where employer failed
/// to provide promised training. Cause not real (employer's fault).
///
/// **5. Other Real and Serious Causes (Autres causes réelles et sérieuses)**
///
/// Non-disciplinary reasons making continued employment impossible:
/// - **Loss of essential qualification**: Driver's license suspended (delivery driver)
/// - **Loss of trust**: Manager discovered falsifying expense reports (not theft but
///   breach of trust)
/// - **Repeated illness absences**: If causing serious operational disruption AND
///   all adaptation efforts exhausted (very difficult to prove; heavily protected)
///
/// ### Burden of Proof and Procedural Dynamics
///
/// **Initial burden**: Employer must cite cause in dismissal letter (lettre de
/// licenciement). This letter **locks in the cause** - cannot add new grounds later.
///
/// **Litigation**:
/// 1. Employee challenges dismissal at conseil de prud'hommes (labor court)
/// 2. Employer must **prove** cause: produce evidence establishing objective facts
/// 3. Court assesses whether cause both real and serious
/// 4. Judge has **full review power** (plein contentieux): substitutes own judgment
///    for employer's, not mere reasonableness review
///
/// **Split of proof**:
/// - **Employer proves facts**: Burden to establish conduct occurred
/// - **Employee can contest characterization**: Argue facts insufficient or context
///   mitigates
/// - **Judge determines**: Whether proven facts constitute cause réelle et sérieuse
///
/// **Cass. soc., 27 mars 2012**: Employer cited three incidents as faute grave.
/// Court found two incidents unproven, third proven but insufficiently serious.
/// Dismissal invalid despite partial proof.
///
/// ### Common Employer Errors
///
/// **1. Vague dismissal letter**: "Poor performance" without specifics. Court requires
/// concrete facts: dates, incidents, metrics.
///
/// **2. Adding causes post-dismissal**: Employer cites insubordination in letter,
/// then adds incompetence at trial. Inadmissible. Only initial cause can be judged.
///
/// **3. Disproportionate sanction**: First-time minor offense leading to dismissal
/// without prior warning. Court finds serious but employer's response disproportionate.
///
/// **4. Inconsistent treatment**: Dismissing one employee for conduct tolerated in
/// others. Discriminatory application vitiates cause.
///
/// **5. Failure to investigate**: Dismissing based on accusation without verification.
/// Cause not "real" if employer failed due diligence.
///
/// ### Damages for Unfair Dismissal
///
/// **Pre-Macron (before 2017)**: Uncapped damages based on harm suffered. Awards
/// frequently exceeded 12 months' salary, sometimes 24+ months for long-service employees.
///
/// **Macron Ordinances (2017)**: Introduced **damage caps** based on seniority:
/// - < 1 year: 0-1 month
/// - 1-2 years: 0.5-2 months
/// - 2-5 years: 1-3 months
/// - 5-10 years: 3-6 months
/// - 10-20 years: 6-10 months
/// - 20-30 years: 10-15 months
/// - ≥30 years: 15-20 months
///
/// **Exceptions** (uncapped):
/// - Discriminatory dismissal
/// - Harassment-related dismissal
/// - Whistleblower retaliation
/// - Health and safety representative dismissal
/// - Pregnancy-related dismissal
///
/// **Constitutional challenge**: French Constitutional Council upheld caps but required
/// minimum floor (not just maximum ceiling). Balance employer predictability with
/// employee protection.
///
/// ## Historical Context
///
/// ### Pre-1973: Contractual Freedom Era
///
/// Before 1973, French labor law did not require justified cause for dismissal.
/// Employer could dismiss for any reason or no reason (like US at-will), subject
/// only to abuse of right doctrine (very narrow).
///
/// **Employment as contract**: Relationship governed by Code Civil. Either party
/// could terminate with notice unless contract specified otherwise.
///
/// ### Law of 13 July 1973: Introduction of Cause Requirement
///
/// Revolutionary law introduced **cause réelle et sérieuse requirement** for personal
/// dismissals. Radical shift from contractual freedom to employment protection.
///
/// **Rationale**: Economic crisis, unemployment rising. Protect workers from
/// arbitrary dismissal. Align France with German Kündigungsschutz model.
///
/// **Immediate effect**: Dismissal litigation exploded. Conseil de prud'hommes
/// caseload increased 400% within five years. Jurisprudence developed defining
/// "real and serious."
///
/// ### Auroux Laws (1982): Strengthening Protections
///
/// Extended procedural protections:
/// - Mandatory pre-dismissal interview (Article L1232-2)
/// - Written dismissal letter with stated reasons
/// - Employee representative consultation for protected workers
///
/// Cause réelle et sérieuse standard unchanged but procedure made more rigorous.
///
/// ### 1990s-2000s: Judicial Interpretation
///
/// Cour de cassation developed extensive jurisprudence refining the standard:
/// - Incompetence must be objective and documented (1980s-90s cases)
/// - Faute grave requires immediate dismissal to be credible (1990s)
/// - Private life misconduct only if genuine impact on employer (2000s)
///
/// **Landmark**: Cass. soc., 28 avril 1988 - Established employer's duty to train
/// before dismissing for incompetence. Revolutionary expansion of cause réelle
/// doctrine.
///
/// ### El Khomri Law (2016): Damage Predictability
///
/// Attempted to cap unfair dismissal damages (business lobby pressure: unpredictable
/// awards deter hiring). Failed in Parliament. Macron accomplished this by ordinance
/// in 2017.
///
/// ### Macron Ordinances (2017): Capped Damages
///
/// **Barème Macron** (damage grid): Capped unfair dismissal compensation based on
/// seniority. Controversial - critics argue inadequate for serious abuses; defenders
/// cite hiring encouragement.
///
/// **Impact**: Dismissal litigation declined ~15% (2017-2019). Employers more willing
/// to dismiss knowing maximum liability. Employees less willing to sue if capped
/// recovery small.
///
/// ### COVID-19 Era (2020-2023)
///
/// **Pandemic-related dismissals**: Employers cited COVID-driven economic difficulties.
/// Many disguised economic dismissals as personal (avoid stringent economic dismissal
/// requirements). Courts scrutinized cause carefully.
///
/// **Remote work discipline**: New category of misconduct emerged - refusal to
/// participate in video meetings, inadequate home availability, etc. Case law
/// developing on whether these constitute cause réelle et sérieuse.
///
/// ## International Comparisons
///
/// ### Germany: Social Justification (Soziale Rechtfertigung)
///
/// **Kündigungsschutzgesetz (KSchG)**: Similar protection to French system. Dismissal
/// requires social justification:
/// - **Conduct-related** (Verhaltensbedingt): Like French faute
/// - **Personal-related** (Personenbedingt): Like French insuffisance professionnelle
/// - **Business-related** (Betriebsbedingt): Like French licenciement économique
///
/// **Key difference**: German law emphasizes **ultima ratio** - dismissal permissible
/// only if no alternative (transfer, retraining, etc.). More protective than France
/// in requiring employer exhaust alternatives.
///
/// **Procedure**: Works council must be consulted; can object (Widerspruch). If
/// council objects, employee continues working during litigation. No French equivalent.
///
/// ### Japan: Abuse of Dismissal Rights Doctrine (解雇権濫用法理)
///
/// **Labor Contract Act Article 16** (労働契約法16条): Dismissal invalid if lacks
/// "objectively reasonable grounds" (客観的に合理的な理由) and is not "socially
/// acceptable" (社会通念上相当).
///
/// **Two-part test similar to France**:
/// - Objective reasonableness ≈ French réelle (real)
/// - Social acceptability ≈ French sérieuse (serious/proportionate)
///
/// **Difference**: Japanese test emphasizes **social norms** and **harmony** more
/// than French bright-line rules. Japanese judges consider company size, employee
/// position, industry customs extensively.
///
/// **Burden of proof**: On employer, like France. But Japanese judges more deferential
/// to large company practices if consistent with industry norms.
///
/// **Result**: Both systems very protective. Japan arguably more protective for
/// "regular employees" (正社員) due to lifetime employment expectation.
///
/// ### USA: Employment At-Will (No Cause Required)
///
/// **Default rule**: Employer can dismiss for good reason, bad reason, or no reason
/// (except discrimination, retaliation, public policy violations).
///
/// **Stark contrast to France**: No general requirement of justified cause. Burden
/// on employee to prove illegal reason (discrimination, whistleblower retaliation),
/// not on employer to prove lawful reason.
///
/// **Exceptions**:
/// - **Montana**: Only US state requiring "good cause" for dismissal (similar to
///   French cause réelle et sérieuse). Result: comparable protection to France, but
///   state of 1 million people, not 67 million.
/// - **Public employees**: Due process protections for government workers. Must
///   receive notice, hearing before dismissal for cause.
/// - **Union contracts**: Collective bargaining agreements typically require "just
///   cause." Arbitrator reviews reasonableness.
///
/// **Philosophy**: US law trusts market to discipline arbitrary employers (employees
/// will leave, reputation suffers). France uses law to mandate fair treatment.
///
/// ### UK: Two Years to Unfair Dismissal Protection
///
/// **Employment Rights Act 1996**: After 2 years' continuous employment, employee
/// gains right not to be unfairly dismissed. Before 2 years, employer can dismiss
/// without cause (like US at-will).
///
/// **Fair dismissal reasons** (similar to France):
/// - Capability (incompetence)
/// - Conduct (misconduct)
/// - Redundancy (economic reasons)
/// - Legal prohibition
/// - Some other substantial reason (SOSR)
///
/// **Procedural fairness**: Employer must follow reasonable procedure. Unlike France,
/// procedural defect may render otherwise valid cause unfair.
///
/// **Key difference**: 2-year qualifying period. France has no qualifying period -
/// cause réelle et sérieuse applies from day one. UK more flexible for short-tenure
/// employees.
///
/// **Remedy**: Reinstatement or compensation. Compensation typically lower than
/// France (median ~£6,000 vs. French several months' salary).
///
/// ### Sweden: Reasonable Cause (Saklig grund)
///
/// **LAS Section 7**: Dismissal requires reasonable cause (saklig grund). Similar to
/// French cause réelle et sérieuse but interpreted more flexibly.
///
/// **Two categories**:
/// - **Personal reasons**: Misconduct, incompetence (like France)
/// - **Business reasons**: Redundancy (like French licenciement économique)
///
/// **Seniority principle (turordning)**: If business dismissal, employer must dismiss
/// most junior employees first ("last in, first out"). No French equivalent - France
/// uses social criteria for selection but not pure seniority.
///
/// **Notice periods**: Much longer than France. 1-6 months depending on age and
/// seniority, up to 6 months for employees >55. France typically 1-3 months.
///
/// ### Spain: Disciplinary vs. Objective Dismissal
///
/// **Workers' Statute**: Two dismissal types:
/// - **Despido disciplinario** (disciplinary): Like French faute. Requires serious,
///   culpable breach.
/// - **Despido objetivo** (objective): Economic, technical, organizational reasons
///   OR objective unsuitability. Hybrid of French personal and economic dismissal.
///
/// **Key difference**: Spanish despido objetivo permits dismissal for "objective
///   unsuitability" without proving employee fault. More flexible than French
///   insuffisance professionnelle (requires documented incompetence).
///
/// **Compensation**: If dismissal declared unfair (improcedente), employer chooses:
/// reinstate employee OR pay 33 days' wages per year of service (uncapped).
/// More generous than French post-Macron capped system.
///
/// **Constitutional protection**: Right not to be unfairly dismissed has constitutional
/// dimension in Spain (Article 35 Constitution). France's Constitutional Council
/// has not elevated dismissal protection to constitutional level.
///
/// ## Modern Applications
///
/// ### Remote Work Misconduct
///
/// **New category of dismissals**: "Inadequate availability" during remote work hours.
/// - Employee not responding to messages promptly
/// - Not participating in video meetings
/// - Unavailable during declared working hours
///
/// **Challenge**: Proving employee truly unavailable vs. reasonable flexibility
/// expectation. Case law developing.
///
/// **Right to disconnect tension**: Employer cannot require constant availability
/// (Article L2242-17). But complete unresponsiveness during working hours may
/// constitute cause réelle et sérieuse.
///
/// **Balanced approach emerging**: Occasional unavailability not serious. Pattern
/// of systematic unavailability may be. Courts examine:
/// - Frequency of unavailability
/// - Impact on team and deliverables
/// - Employee's explanations
/// - Whether employer provided clear expectations
///
/// ### Social Media Misconduct
///
/// **Public criticism of employer**: Employee posts on social media criticizing
/// company, managers, or products. Dismissible cause?
///
/// **Case law**: Depends on:
/// - **Abuse vs. criticism**: Insulting language may constitute faute. Factual
///   criticism protected as free speech.
/// - **Public vs. private**: Public Facebook post more serious than limited-access
///   post.
/// - **Impact**: If damages employer reputation concretely, stronger cause.
///
/// **Cass. soc., 12 sept. 2018**: Employee posted insulting comments about
/// supervisor on public Facebook. Dismissal for faute grave upheld. Private life
/// content but public dissemination with reputational harm.
///
/// **Cass. soc., 17 déc. 2013**: Employee criticized employer on Facebook with
/// restricted audience (50 friends). Dismissal invalid - insufficient publicity
/// to constitute cause.
///
/// ### Whistleblower Protection
///
/// **Sapin II Law (2016)**: Protected whistleblowers exposing wrongdoing. Dismissal
/// presumed retaliatory if employee reported misconduct then was dismissed.
///
/// **Burden shift**: Employer must prove dismissal cause **unrelated** to
/// whistleblowing. Heavy burden.
///
/// **Protection scope**: Corruption, health/safety violations, environmental crimes.
/// Does not protect all workplace complaints - must be public interest issue.
///
/// ## Comparison Table
///
/// | System | Standard | Burden of Proof | Qualifying Period | Typical Damages | Cap? |
/// |--------|----------|----------------|-------------------|-----------------|------|
/// | 🇫🇷 France | Cause réelle et sérieuse | Employer | Day 1 | 3-10 months' salary | Yes (Macron) |
/// | 🇩🇪 Germany | Soziale Rechtfertigung | Employer | 6 months | 6-18 months' salary | No |
/// | 🇯🇵 Japan | Objective + socially acceptable | Employer | Day 1 | 6-24 months' salary | No |
/// | 🇺🇸 USA | None (at-will) | Employee (prove illegal reason) | None (at-will) | Varies (if illegal) | No federal cap |
/// | 🇬🇧 UK | Fair reason + procedure | Employer | 2 years | £6,000 median | Yes (£105,707 max) |
/// | 🇸🇪 Sweden | Saklig grund | Employer | Day 1 | Varies | No |
/// | 🇪🇸 Spain | Causa justificada | Employer | Day 1 | 33 days/year | No cap |
///
/// Continental European systems (France, Germany, Sweden, Spain) require justified
/// cause from day one with employer burden of proof. UK hybrid (at-will for 2 years,
/// then protected). USA outlier with at-will default. Japan most protective for
/// regular employees.
#[must_use]
pub fn article_l1232_1() -> Statute {
    Statute::new(
        "code-travail-l1232-1",
        "Code du travail Article L1232-1 - Real and Serious Cause",
        Effect::new(
            EffectType::StatusChange,
            "Exigence de cause réelle et sérieuse / Real and serious cause requirement",
        )
        .with_parameter("requirement", "Real (objective facts) + Serious (sufficient gravity)")
        .with_parameter("burden_of_proof", "Employer")
        .with_parameter("sanction_absence", "Unfair dismissal + damages"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Must have real and serious cause
    .with_precondition(Condition::AttributeEquals {
        key: "has_real_serious_cause".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article L1232-1 pose l'exigence fondamentale de la cause réelle et sérieuse pour \
        tout licenciement personnel. Cette double condition protège le salarié contre les licenciements \
        abusifs ou injustifiés. \
        \n\nLa réalité de la cause signifie qu'elle doit reposer sur des faits objectifs, précis et vérifiables. \
        Le caractère sérieux implique une gravité suffisante pour rendre impossible le maintien \
        du salarié dans l'entreprise. Le juge prud'homal apprécie souverainement ces deux conditions. \
        \n\nEn l'absence de cause réelle et sérieuse, le licenciement est sans cause et donne lieu \
        à des dommages-intérêts pour le salarié (indemnité pour licenciement sans cause réelle et sérieuse). \
        \n\nArticle L1232-1 establishes the fundamental requirement of real and serious cause \
        for any personal dismissal. This dual condition protects the employee against \
        abusive or unjustified dismissals. \
        \n\nThe reality of the cause means it must be based on objective, precise, and verifiable facts. \
        The serious nature implies sufficient gravity to make it impossible to keep the employee \
        in the company. The labor court (conseil de prud'hommes) has full discretion to assess these conditions. \
        \n\nIn the absence of real and serious cause, the dismissal is unfounded and gives rise \
        to damages for the employee (compensation for dismissal without real and serious cause). \
        \n\n【日仏比較】\n\
        日本の労働契約法16条の「客観的に合理的な理由」はフランスの「cause réelle」に、\
        「社会通念上相当」は「cause sérieuse」に対応する。両国とも解雇の正当性を厳格に審査する。",
    )
}

/// Article L1232-2 - Pre-Dismissal Interview Requirement
///
/// ## French Text
///
/// > L'employeur qui envisage de licencier un salarié le convoque, avant toute décision,
/// > à un entretien préalable.
///
/// ## English Translation
///
/// > The employer who intends to dismiss an employee must summon them, before any decision,
/// > to a pre-dismissal interview.
///
/// ## Legal Significance
///
/// **Pre-dismissal interview** (Entretien préalable) is **mandatory** before dismissal.
///
/// **Procedure**:
/// 1. **Summons letter** (Lettre de convocation):
///    - Sent by registered mail or hand-delivered
///    - Minimum 5 business days before interview
///    - Must indicate: date, time, place, right to assistance
///
/// 2. **Interview** (Entretien):
///    - Employer explains dismissal reasons
///    - Employee can respond and defend themselves
///    - Employee can be assisted (colleague or union representative)
///
/// 3. **Notification letter** (Lettre de licenciement):
///    - Sent at least 2 business days after interview
///    - Must state precise reasons for dismissal
///
/// **Sanction**: Failure to hold interview → Procedural irregularity → Damages (typically 1 month salary).
///
/// **Exception**: Serious misconduct (faute grave) - no interview if employee already gone, but still required if employee present.
///
/// ## Comparison
///
/// | System | Pre-Dismissal Procedure |
/// |--------|-------------------------|
/// | 🇫🇷 France | Mandatory interview (entretien préalable) |
/// | 🇯🇵 Japan | Notification 30 days in advance or payment (労基法20条) |
/// | 🇩🇪 Germany | Hearing required (Anhörung) |
/// | 🇺🇸 USA | Generally none (at-will) |
///
/// French and German law require procedural fairness before dismissal.
#[must_use]
pub fn article_l1232_2() -> Statute {
    Statute::new(
        "code-travail-l1232-2",
        "Code du travail Article L1232-2 - Pre-Dismissal Interview",
        Effect::new(
            EffectType::StatusChange,
            "Entretien préalable au licenciement / Pre-dismissal interview required",
        )
        .with_parameter("summons_notice", "Minimum 5 business days")
        .with_parameter("employee_rights", "Can respond, can be assisted")
        .with_parameter("notification_delay", "Minimum 2 business days after interview")
        .with_parameter("sanction_absence", "Procedural irregularity + damages (~1 month)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Interview must be held
    .with_precondition(Condition::AttributeEquals {
        key: "interview_held".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article L1232-2 impose la tenue d'un entretien préalable avant tout licenciement, \
        garantissant ainsi les droits de la défense du salarié. Cet entretien permet au salarié \
        d'être informé des griefs et de présenter ses explications. \
        \n\nLa convocation doit respecter un délai minimal de 5 jours ouvrables et mentionner \
        la possibilité de se faire assister. L'absence d'entretien constitue une irrégularité \
        de procédure sanctionnée par l'octroi de dommages-intérêts (généralement 1 mois de salaire), \
        même si le licenciement est par ailleurs justifié. \
        \n\nArticle L1232-2 requires a pre-dismissal interview before any dismissal, \
        thus guaranteeing the employee's rights of defense. This interview allows the employee \
        to be informed of the grievances and present their explanations. \
        \n\nThe summons must respect a minimum period of 5 business days and mention \
        the possibility of being assisted. Absence of an interview constitutes a procedural \
        irregularity sanctioned by awarding damages (generally 1 month's salary), \
        even if the dismissal is otherwise justified. \
        \n\n【日仏比較】\n\
        日本の労働基準法20条は解雇予告（30日前）または予告手当を義務付けるが、\
        事前の弁明機会付与は法定されていない（判例法理では一定の場合に必要）。\
        フランスでは弁明の機会（entretien préalable）が手続的に必須である。",
    )
}

/// Article L1233-3 - Economic Dismissal Definition
///
/// ## French Text
///
/// > Constitue un licenciement pour motif économique le licenciement effectué par un
/// > employeur pour un ou plusieurs motifs non inhérents à la personne du salarié
/// > résultant d'une suppression ou transformation d'emploi ou d'une modification,
/// > refusée par le salarié, d'un élément essentiel du contrat de travail, consécutives
/// > notamment à des difficultés économiques ou à des mutations technologiques.
///
/// ## English Translation
///
/// > Economic dismissal is dismissal by an employer for one or more reasons not
/// > inherent to the employee's person resulting from job elimination or transformation
/// > or a modification, refused by the employee, of an essential element of the employment
/// > contract, following in particular economic difficulties or technological changes.
///
/// ## Legal Significance
///
/// **Economic dismissal** (Licenciement économique) requires:
///
/// 1. **Reason external to employee** (Non inhérent à la personne):
///    - Not based on employee's conduct/performance
///    - Based on company's economic situation
///
/// 2. **Economic cause** (Cause économique):
///    - Economic difficulties (Difficultés économiques)
///    - Technological changes (Mutations technologiques)
///    - Reorganization to safeguard competitiveness (Réorganisation pour sauvegarder la compétitivité)
///    - Cessation of activity (Cessation d'activité)
///
/// 3. **Job consequence** (Conséquence sur l'emploi):
///    - Job elimination (Suppression de poste)
///    - Job transformation (Transformation d'emploi)
///    - Contract modification refusal (Modification refusée)
///
/// **Additional requirements**:
/// - Employer must seek reclassification (recherche de reclassement)
/// - Consultation of employee representatives
/// - Social plan (plan de sauvegarde de l'emploi) if ≥10 employees in 30 days
///
/// **Sanction**: Invalid economic dismissal → Requalified as unfair dismissal → Heavy damages.
///
/// ## Comparison
///
/// | System | Economic Dismissal Rules |
/// |--------|--------------------------|
/// | 🇫🇷 France | Strict: economic cause + job elimination + reclassification efforts |
/// | 🇯🇵 Japan | 4 requirements (整理解雇の4要件): necessity, effort, fairness, consultation |
/// | 🇩🇪 Germany | Social selection (Sozialauswahl): must justify who is dismissed |
/// | 🇺🇸 USA | Generally unrestricted (WARN Act for mass layoffs) |
///
/// France and Japan have the strictest economic dismissal requirements.
#[must_use]
pub fn article_l1233_3() -> Statute {
    Statute::new(
        "code-travail-l1233-3",
        "Code du travail Article L1233-3 - Economic Dismissal Definition",
        Effect::new(
            EffectType::StatusChange,
            "Licenciement économique / Economic dismissal",
        )
        .with_parameter(
            "causes",
            "Economic difficulties, technological change, reorganization",
        )
        .with_parameter("must_not_be", "Related to employee's person")
        .with_parameter("employer_obligations", "Reclassification efforts, consultation")
        .with_parameter(
            "social_plan",
            "Required if ≥10 dismissals in 30 days (large companies)",
        ),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Must have economic cause
    .with_precondition(Condition::AttributeEquals {
        key: "has_economic_cause".to_string(),
        value: "true".to_string(),
    })
    // Must have job elimination
    .with_precondition(Condition::AttributeEquals {
        key: "job_eliminated".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article L1233-3 définit le licenciement économique comme un licenciement \
        pour motif non inhérent à la personne du salarié, résultant de difficultés économiques \
        ou de mutations technologiques entraînant une suppression de poste. \
        \n\nL'employeur doit prouver la réalité des difficultés économiques (baisse du chiffre d'affaires, \
        pertes d'exploitation, etc.) et la nécessité de supprimer le poste. Il doit également \
        rechercher des solutions de reclassement (proposer tous les postes disponibles dans l'entreprise \
        et le groupe). Si ≥10 salariés sont licenciés en 30 jours, un plan de sauvegarde de l'emploi (PSE) \
        est obligatoire dans les entreprises de ≥50 salariés. \
        \n\nArticle L1233-3 defines economic dismissal as dismissal for reasons not inherent \
        to the employee's person, resulting from economic difficulties or technological changes \
        leading to job elimination. \
        \n\nThe employer must prove the reality of economic difficulties (declining revenue, \
        operating losses, etc.) and the necessity of eliminating the position. They must also \
        seek reclassification solutions (offer all available positions in the company and group). \
        If ≥10 employees are dismissed in 30 days, a safeguard employment plan (PSE) is mandatory \
        in companies with ≥50 employees. \
        \n\n【日仏比較】\n\
        日本の整理解雇の4要件（人員削減の必要性、解雇回避努力、人選の合理性、手続の妥当性）と\
        フランスの経済的解雇の要件（経済的理由、ポスト廃止、再配置努力）は構造的に類似する。\
        両国とも整理解雇に厳格な正当性を要求する。",
    )
}

/// Article L1234-1 - Notice Period Requirement
///
/// ## French Text
///
/// > Lorsque le licenciement n'est pas motivé par une faute grave, le salarié a droit
/// > à un préavis dont la durée est déterminée par la loi, la convention ou l'accord
/// > collectif de travail ou, à défaut, par les usages pratiqués dans la localité et
/// > la profession.
///
/// ## English Translation
///
/// > When dismissal is not motivated by serious misconduct, the employee is entitled
/// > to a notice period whose duration is determined by law, collective agreement,
/// > or, failing that, by practices in the locality and profession.
///
/// ## Legal Significance
///
/// **Notice period** (Préavis) is required for dismissal (except serious misconduct).
///
/// **Minimum durations** (often increased by collective agreements):
/// - **< 6 months seniority**: No legal minimum (collective agreement may impose)
/// - **6 months - 2 years**: **1 month**
/// - **≥ 2 years**: **2 months**
///
/// **During notice period**:
/// - Employee continues working (or receives payment in lieu)
/// - Employee can take time off to seek new employment (2 hours/day typical)
/// - Wages continue normally
///
/// **Payment in lieu** (Indemnité compensatrice de préavis):
/// - Employer can waive notice and pay equivalent salary
/// - Employee receives notice period salary even if not working
///
/// **Exception**: Serious misconduct (faute grave) = No notice, immediate termination.
///
/// ## Comparison
///
/// | System | Notice Period |
/// |--------|---------------|
/// | 🇫🇷 France | 1-2 months (based on seniority) |
/// | 🇯🇵 Japan | 30 days or payment in lieu (労基法20条) |
/// | 🇩🇪 Germany | 4 weeks - 7 months (based on seniority) |
/// | 🇺🇸 USA | None (at-will, unless contract) |
///
/// Germany has the longest notice periods; France is moderate.
#[must_use]
pub fn article_l1234_1() -> Statute {
    Statute::new(
        "code-travail-l1234-1",
        "Code du travail Article L1234-1 - Notice Period",
        Effect::new(
            EffectType::StatusChange,
            "Préavis de licenciement / Dismissal notice period",
        )
        .with_parameter("minimum_6m_2y", "1 month")
        .with_parameter("minimum_2y_plus", "2 months")
        .with_parameter("exception", "Serious misconduct: no notice")
        .with_parameter("payment_in_lieu", "Employer can pay instead of working notice"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_discretion(
        "L'article L1234-1 accorde au salarié licencié un droit à préavis, sauf en cas de faute grave. \
        Ce préavis permet au salarié de rechercher un nouvel emploi tout en percevant son salaire. \
        \n\nLa durée minimale du préavis dépend de l'ancienneté : 1 mois entre 6 mois et 2 ans, \
        2 mois au-delà de 2 ans. Les conventions collectives prévoient souvent des durées plus longues. \
        L'employeur peut dispenser le salarié d'effectuer le préavis tout en lui versant l'indemnité \
        compensatrice correspondante. \
        \n\nArticle L1234-1 grants the dismissed employee a right to notice, except for serious misconduct. \
        This notice allows the employee to seek new employment while receiving their salary. \
        \n\nThe minimum notice duration depends on seniority: 1 month between 6 months and 2 years, \
        2 months beyond 2 years. Collective agreements often provide longer periods. \
        The employer can exempt the employee from working the notice while paying the corresponding \
        compensatory allowance. \
        \n\n【日仏比較】\n\
        日本の労働基準法20条は30日前の解雇予告または30日分の解雇予告手当を義務付けるが、\
        勤続年数による区別はない。フランスでは勤続2年以上で2ヶ月の予告期間となり、\
        日本より長期間の保護がある。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_l1231_1_creation() {
        let statute = article_l1231_1();
        assert_eq!(statute.id, "code-travail-l1231-1");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
    }

    #[test]
    fn test_article_l1232_1_creation() {
        let statute = article_l1232_1();
        assert_eq!(statute.id, "code-travail-l1232-1");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("burden_of_proof").unwrap(), "Employer");
    }

    #[test]
    fn test_article_l1232_2_creation() {
        let statute = article_l1232_2();
        assert_eq!(statute.id, "code-travail-l1232-2");

        let params = &statute.effect.parameters;
        assert_eq!(
            params.get("summons_notice").unwrap(),
            "Minimum 5 business days"
        );
    }

    #[test]
    fn test_article_l1233_3_creation() {
        let statute = article_l1233_3();
        assert_eq!(statute.id, "code-travail-l1233-3");

        let params = &statute.effect.parameters;
        assert!(
            params
                .get("causes")
                .unwrap()
                .contains("Economic difficulties")
        );
    }

    #[test]
    fn test_article_l1234_1_creation() {
        let statute = article_l1234_1();
        assert_eq!(statute.id, "code-travail-l1234-1");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("minimum_6m_2y").unwrap(), "1 month");
        assert_eq!(params.get("minimum_2y_plus").unwrap(), "2 months");
    }

    #[test]
    fn test_all_termination_articles_have_discretion() {
        let statutes = vec![
            article_l1231_1(),
            article_l1232_1(),
            article_l1232_2(),
            article_l1233_3(),
            article_l1234_1(),
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
    fn test_all_termination_articles_valid() {
        let statutes = vec![
            article_l1231_1(),
            article_l1232_1(),
            article_l1232_2(),
            article_l1233_3(),
            article_l1234_1(),
        ];

        for statute in statutes {
            assert!(statute.is_valid());
            assert_eq!(statute.validate().len(), 0);
        }
    }
}
