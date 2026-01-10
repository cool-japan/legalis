//! Working hours regulation (Durée du travail)
//!
//! Implementation of Code du travail Articles L3121+ for working time, including
//! the famous 35-hour work week.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article L3121-27 - Legal Working Duration (35-hour week)
///
/// ## French Text
///
/// > La durée légale du travail effectif des salariés à temps complet est fixée à
/// > trente-cinq heures par semaine.
///
/// ## English Translation
///
/// > The legal duration of actual work for full-time employees is set at thirty-five
/// > hours per week.
///
/// ## Legal Significance
///
/// The **35-hour work week** (semaine de 35 heures) is one of France's most famous
/// labor law provisions, enacted in 2000 (Loi Aubry).
///
/// **Key points**:
/// - **Legal duration**: 35 hours/week (not maximum, but threshold for overtime)
/// - **Overtime**: Hours beyond 35 are considered overtime (heures supplémentaires)
/// - **Overtime premium**: +25% for first 8 hours, +50% beyond (Article L3121-33)
/// - **Collective agreements**: Can modify application but not eliminate protection
///
/// **Not a maximum**: Employees can work more, but overtime rules apply.
///
/// ## Extended Legal Commentary
///
/// ### Practical Implementation Methods
///
/// Employers implement the 35-hour week through various modalities:
///
/// **1. Weekly Reduction**: Actual 35-hour work week (7 hours/day for 5 days).
///
/// **2. RTT (Réduction du Temps de Travail)**: Work longer weeks (e.g., 39 hours)
/// with compensatory rest days (jours de RTT). Calculation:
/// - Annual working hours target: 35h × 52 weeks = 1,820 hours
/// - Actual weekly hours: 39h generates 4h surplus per week
/// - Result: 4h × 52 weeks = 208 hours surplus / 7h per day ≈ 30 RTT days per year
///
/// **3. Modulation/Annualization**: Average 35 hours over year, varying by season.
/// Busy periods > 35h offset by calm periods < 35h. Requires collective agreement.
///
/// **4. Forfait jours** (Day-based package): Executives and autonomous employees work
/// fixed number of days per year (typically 218 days) rather than tracking hours.
/// Must not exceed maximum working time; requires individual agreement.
///
/// ### "Actual Work" Definition (Travail effectif)
///
/// Article L3121-1 defines actual work time as time during which employee is at
/// employer's disposal and must comply with directives without freely conducting
/// personal activities.
///
/// **Included**:
/// - Time at workstation
/// - Business meetings, training
/// - Business travel time beyond normal commute
/// - Mandatory dressing/undressing time (protective equipment)
///
/// **Excluded**:
/// - Commuting time (home to work)
/// - Meal breaks (unless employee cannot freely leave workplace)
/// - On-call time at home (astreinte) - compensated differently
///
/// **Case law**: Cass. soc., 31 mai 2006, No. 04-43.527 - Security guard required
/// to remain on premises during breaks. Breaks counted as working time.
///
/// ### Edge Cases and Exceptions
///
/// **Forfait jours abuse**: Despite not tracking hours, day-based packages must
/// respect maximum working time. Employer must monitor workload.
/// **Cass. soc., 29 juin 2011, No. 09-71.107**: Forfait jours invalid if no
/// safeguards preventing excessive working time. Employee paid overtime retroactively.
///
/// **Senior executives** (Cadres dirigeants): Completely exempt from working time
/// rules. Criteria: Significant responsibilities, high autonomy, remuneration in
/// top company levels. Very narrow category (perhaps 1% of cadres).
///
/// **Partial remote work**: Employee working 3 days office, 2 days home. How track
/// 35-hour week? Employer must trust-based management or monitoring software
/// (controversial, privacy concerns).
///
/// ## Historical Context: The 35-Hour Week Saga
///
/// ### Pre-Aubry: 39-Hour Standard (1982-1998)
///
/// **Auroux Laws (1982)**: Reduced legal working time from 40 to 39 hours.
/// Fifth week of paid vacation added. Major advance but still above European standards.
///
/// **Unemployment crisis (1990s)**: France suffered persistent high unemployment
/// (>10%). Socialist Party proposed radical solution: reduce working time to
/// "share" available work (partage du travail).
///
/// ### Aubry I Law (13 June 1998): Voluntary Phase
///
/// **Minister Martine Aubry** introduced progressive 35-hour week:
/// - Voluntary reduction starting 1998
/// - Financial incentives for early adopters (social charge reductions)
/// - Target: January 2000 (>20 employees), January 2002 (<20 employees)
///
/// **Rationale**: Job sharing would reduce unemployment. If all companies worked
/// 35h instead of 39h, roughly 10% more hiring needed (simplistic calculation).
///
/// **Employer reaction**: Mixed. Large companies saw opportunity for reorganization
/// and wage moderation. Small businesses protested complexity and cost.
///
/// ### Aubry II Law (19 January 2000): Mandatory Implementation
///
/// Made 35-hour week mandatory:
/// - 35h = new legal duration (overtime threshold)
/// - Previous 39h contracts: 4 hours automatically became overtime (paid premium)
/// - Massive wave of collective bargaining to adapt company organizations
///
/// **Implementation chaos**: Every company renegotiated working arrangements.
/// Some sectors adapted smoothly (office work, manufacturing); others struggled
/// (healthcare, public services, small retail).
///
/// **RTT explosion**: Most companies chose 39-hour weeks with RTT days rather than
/// true 35-hour weeks. Maintained productivity while appearing compliant.
///
/// ### Economic Debate: Did it Work?
///
/// **Proponents argue**: Created 350,000 jobs (official government estimate).
/// Improved work-life balance. Proved working time reduction feasible in modern economy.
///
/// **Critics argue**: Jobs created would have materialized anyway (economic growth).
/// Increased labor costs. Hurt competitiveness. Complicated administration.
///
/// **Academic consensus**: Modest positive employment effect (50,000-150,000 jobs)
/// offset by reduced competitiveness. Primary benefit was social (leisure time)
/// rather than economic.
///
/// ### Right-Wing Backlash (2002-2012)
///
/// **Fillon Law (2003)**: Increased overtime quota, facilitated derogations through
/// company agreements. Symbolic rollback without eliminating 35h.
///
/// **Sarkozy campaign (2007)**: "Travailler plus pour gagner plus" (Work more to
/// earn more). Promoted overtime as solution to purchasing power.
///
/// **TEPA Law (2007)**: Made overtime tax-exempt and social charge-exempt.
/// Dramatically increased overtime incentive. De facto 35h elimination for willing workers.
///
/// ### Hollande Era (2012-2017): Defensive Consolidation
///
/// Socialist government preserved 35h principle but allowed flexibility:
/// - Company agreements can override branch agreements on working time
/// - Maintained 35h as legal duration and overtime threshold
/// - Emphasized work-life balance rationale over job creation
///
/// ### Macron Reforms (2017-): Flexible 35-Hour Week
///
/// **Ordinances of 2017**: Further empowered company-level bargaining on working
/// time organization. 35h remains legal duration but companies can modulate extensively.
///
/// **Current status**: 35h is symbolic standard. Most employees work more (average
/// 36.6h) with overtime or RTT. But 35h baseline protects against excessive hours
/// and ensures overtime compensation.
///
/// ### COVID-19 Impact (2020-2023)
///
/// **Partial unemployment (chômage partiel)**: Millions worked reduced hours during
/// pandemic. 35h became aspirational goal; many worked 0-20 hours with state support.
///
/// **Remote work**: Blurred boundaries of "working time." At home, distinguishing
/// work from personal time difficult. Reinforced importance of right to disconnect.
///
/// **Post-pandemic**: Return to office debates centered on hybrid models. 35h
/// framework adapted to 3 office days + 2 remote days standard.
///
/// ## International Comparisons
///
/// ### Germany: Sector-Specific Through Collective Bargaining
///
/// **No statutory weekly hours**: Unlike France, Germany has no legal working week
/// duration. Daily maximum (10h after breaks) and rest requirements only.
///
/// **Collective agreements set hours**: Typically 35-40 hours depending on sector:
/// - Metalworking (IG Metall): 35 hours (since 1995)
/// - Chemical industry: 37.5 hours
/// - Retail: 37-40 hours
/// - Public sector: 39-40 hours
///
/// **Kurzarbeit** (short-time work): Like French chômage partiel, flexible reduction
/// with state support. Extensively used in 2008 crisis and COVID-19. More established
/// than French system.
///
/// **Comparison**: German system achieves similar results (low average hours) through
/// negotiation rather than statute. More flexible, sector-specific.
///
/// ### Japan: 40-Hour Week with Notorious Overtime Culture
///
/// **Labor Standards Act Article 32** (労働基準法32条): 40 hours/week, 8 hours/day
/// statutory maximum (since 1988, reduced from 48h). Higher than French 35h.
///
/// **36 Agreement loophole** (サブロク協定): Article 36 permits overtime through
/// labor-management agreement. Traditional limits: 45h/month, 360h/year.
///
/// **2019 Work Style Reform**: Introduced hard cap on overtime (100h/month maximum
/// including 36 agreement) following karoshi (death from overwork) scandals. Still
/// permits 100h/month - unthinkable in France (max 48h/week).
///
/// **Cultural reality**: Many Japanese workers exceed legal limits through unpaid
/// overtime (service zangyō). Labor inspectors underresourced. Work-life balance
/// rhetoric without enforcement.
///
/// **Comparison**: Japan has higher statutory limit (40h vs. 35h) and far higher
/// actual hours due to weak enforcement and cultural pressure. French system more
/// protective in practice.
///
/// ### USA: 40-Hour FLSA Overtime Threshold
///
/// **Fair Labor Standards Act (1938)**: 40-hour week for overtime purposes. Hours
/// beyond 40 paid at 1.5x (time-and-a-half). Unchanged since 1940.
///
/// **Key difference**: FLSA sets overtime threshold, not "legal duration." No
/// work-sharing philosophy; purely wage protection.
///
/// **Exempt employees**: Executives, administrators, professionals, outside salespeople
/// exempt from overtime. Very broad exemption (roughly 50% of workforce). French
/// exemptions much narrower (only cadres dirigeants fully exempt).
///
/// **Actual hours**: US workers average 38.7 hours (full-time), higher than France
/// (36.6h) but lower than perception. Many salaried employees work far beyond 40h
/// without overtime (exempt status).
///
/// **Comparison**: US prioritizes flexibility over work-life balance. No concept
/// of legal working time as social goal. Purely economic regulation.
///
/// ### UK: 48-Hour Maximum Under EU Working Time Directive
///
/// **Working Time Regulations 1998**: Implemented EU directive mandating maximum
/// 48-hour week (averaged over 17 weeks). Much higher than French 35h.
///
/// **Individual opt-out**: Unlike France, UK permits employees to waive 48-hour limit.
/// Controversial - unions argue coercive; employers argue flexibility. France
/// forbids opt-out.
///
/// **Post-Brexit**: UK retains 48-hour limit but debate over relaxation. May diverge
/// from EU standards. France unlikely to change 35h absent political earthquake.
///
/// **Comparison**: UK sets maximum (48h), France sets overtime threshold (35h) and
/// maximum (48h). UK more flexible (opt-out possible); France more protective.
///
/// ### Sweden: Collective Agreement Governance
///
/// **Working Time Act** (Arbetstidslagen): Maximum 40h/week statutory, but collective
/// agreements universally set lower hours (typically 37-40h depending on sector).
///
/// **High union density** (90% collective agreement coverage) means statutory
/// minimums rarely relevant. Negotiated norms govern.
///
/// **Six-hour day experiments**: Some Swedish municipalities and companies tested
/// 30-hour weeks (6h days × 5 days). Results mixed - improved well-being but costly.
/// Not widely adopted. France's 35h more sustainable compromise.
///
/// **Comparison**: Sweden achieves low hours through negotiation (like Germany).
/// Statutory 40h but actual ~37h average. Close to French 35h in practice but
/// different mechanism (collective vs. statutory).
///
/// ### Spain: 40-Hour Week, Recently Debating 37.5 Hours
///
/// **Workers' Statute**: 40-hour week statutory maximum since 1980. Higher than
/// French 35h.
///
/// **2023 debate**: Left-wing parties proposed 37.5-hour week (compromise between
/// 40h status quo and French 35h). Business opposition stalled reform. May eventually
/// pass but politically difficult.
///
/// **Actual hours**: Spanish full-time workers average 37.8 hours, close to French
/// 36.6h despite higher statutory limit. Collective agreements often set lower hours.
///
/// **Comparison**: Spain considering French-inspired reduction. Demonstrates France's
/// 35h as model for other European countries, not just curiosity.
///
/// ## Modern Applications
///
/// ### Digital Era: Tracking Remote Work Hours
///
/// **Challenge**: How enforce 35-hour week when employees work from home?
///
/// **Solutions attempted**:
/// 1. **Honor system**: Trust employees to work 35h. Risks abuse both ways (overwork
///    and underwork).
/// 2. **Time-tracking software**: Monitors activity, screenshots. Raises privacy
///    concerns; employees feel surveilled. Several court challenges.
/// 3. **Output-based management**: Measure deliverables rather than hours. Difficult
///    to calibrate; risks hidden overwork.
///
/// **Right to disconnect** (Article L2242-17): Essential complement to 35h in remote
/// work. Prevents "always-on" culture from nullifying working time protections.
///
/// **Case law emerging**: Cour de cassation will likely develop jurisprudence on
/// remote work hour verification over next decade. Open question.
///
/// ### Four-Day Week Experiments
///
/// Some French companies experimenting with 32-hour, 4-day weeks:
/// - **LDLC** (tech retailer): 4-day week since 2021, maintains full salary
/// - **Welcome to the Jungle** (startup): Tests 4 days for voluntary participants
///
/// **Legal framework**: Within 35h structure, just different organization (4 × 8.75h
/// days instead of 5 × 7h days). No statutory change needed.
///
/// **Results**: Improved employee satisfaction, similar productivity. Too early for
/// definitive conclusions. May become significant trend if proven sustainable.
///
/// **Comparison to 35h adoption**: If 4-day week spreads organically through company
/// choice (like current experiments), mirrors 1990s 35h early adopters. Could
/// pressure future statutory reduction to 32h. Cycle may repeat.
///
/// ### Gig Economy Challenges
///
/// **Platform workers**: If requalified as employees, subject to 35-hour protections?
///
/// **Uber driver case**: After 2020 requalification as employee, unclear how
/// calculate "working time." Time logged into app? Time with passenger only?
///
/// **Doctrinal puzzle**: 35h designed for factory/office model. Gig work's
/// intermittent nature (15 minutes here, 30 minutes there) challenges binary
/// work/non-work distinction.
///
/// **Proposed solution**: Platform-specific working time rules. Debate ongoing.
///
/// ### COVID-19 Partial Unemployment Normalization
///
/// **Chômage partiel**: State-subsidized reduced hours. During pandemic, millions
/// worked 20-30 hours with state topping up wages.
///
/// **Post-COVID**: Mechanism retained as permanent crisis tool. Employers can reduce
/// to 0-35 hours in economic downturns without layoffs.
///
/// **Integration with 35h framework**: Chômage partiel reduces below 35h threshold.
/// Flexible baseline: state subsidizes gap between actual hours and livable wage.
/// Preserves employment while adapting to reality.
///
/// **Lesson**: 35h is floor, not rigid mandate. In crisis, can go below with support.
/// In growth, can exceed with overtime. Flexible framework, not straitjacket.
///
/// ## Comparison Table
///
/// | System | Legal/Statutory Hours | Overtime Threshold | Maximum Hours | Average Actual (FT) | Mechanism |
/// |--------|----------------------|-------------------|---------------|-------------------|-----------|
/// | 🇫🇷 France | 35h/week | 35h | 48h/week, 44h avg | 36.6h | Statute (mandatory) |
/// | 🇩🇪 Germany | None (daily max only) | By agreement (35-40h) | 10h/day, 48h/week avg | 34.7h | Collective agreement |
/// | 🇯🇵 Japan | 40h/week, 8h/day | 40h (36 agreement) | 100h/month (2019 cap) | 38.8h | Statute + agreement |
/// | 🇺🇸 USA | None | 40h (FLSA overtime) | None (many exempt) | 38.7h | Federal statute (FLSA) |
/// | 🇬🇧 UK | None | Varies | 48h/week avg (opt-out OK) | 36.4h | EU directive (post-Brexit) |
/// | 🇸🇪 Sweden | 40h/week | By agreement (37-40h) | 40h/week | 37.2h | Statute + agreement |
/// | 🇪🇸 Spain | 40h/week | 40h | 40h/week | 37.8h | Statute (37.5h debated) |
///
/// France has shortest statutory working week among major economies. Germany achieves
/// similar actual hours through sectoral negotiation. Japan has longest statutory
/// hours and weak enforcement. US lacks working time philosophy, focuses on wage
/// protection. European convergence around 35-40h zone; France at protective end.
#[must_use]
pub fn article_l3121_27() -> Statute {
    Statute::new(
        "code-travail-l3121-27",
        "Code du travail Article L3121-27 - Legal Working Duration (35h)",
        Effect::new(
            EffectType::StatusChange,
            "Durée légale du travail établie / Legal working duration established",
        )
        .with_parameter("legal_weekly_hours", "35")
        .with_parameter("purpose", "Overtime threshold, not maximum")
        .with_parameter("enacted", "2000 (Loi Aubry)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_discretion(
        "L'article L3121-27 fixe la durée légale du travail à 35 heures par semaine, \
        issue de la loi Aubry de 2000. Cette durée n'est pas un maximum mais un seuil \
        déclenchant le régime des heures supplémentaires. \
        \n\nL'objectif historique était de réduire le chômage en partageant le travail, \
        bien que les effets économiques aient été débattus. En pratique, de nombreux salariés \
        travaillent plus de 35 heures, mais bénéficient de majorations pour les heures supplémentaires. \
        \n\nArticle L3121-27 sets the legal working duration at 35 hours per week, \
        resulting from the Aubry law of 2000. This duration is not a maximum but a threshold \
        triggering overtime rules. \
        \n\nThe historical objective was to reduce unemployment by work-sharing, \
        though the economic effects have been debated. In practice, many employees \
        work more than 35 hours but receive overtime premiums. \
        \n\n【日仏比較】\n\
        日本の労働基準法32条は週40時間・1日8時間を法定労働時間とするが、\
        フランスは週35時間と短い。ただし、日本の36協定による時間外労働の上限規制（月45時間）と異なり、\
        フランスでは週48時間の絶対的上限がある（Article L3121-20）。",
    )
}

/// Article L3121-18 - Maximum Daily Working Duration
///
/// ## French Text
///
/// > La durée quotidienne du travail effectif par salarié ne peut excéder dix heures.
///
/// ## English Translation
///
/// > The daily duration of actual work per employee cannot exceed ten hours.
///
/// ## Legal Significance
///
/// **Maximum daily hours**: **10 hours** (absolute limit)
///
/// **Exceptions** (limited, require agreement):
/// - Increased activity periods (with collective agreement)
/// - Urgent work (exceptional circumstances)
/// - Maximum 12 hours in exceptional cases
///
/// **Enforcement**:
/// - Labor inspectors can enforce (inspection du travail)
/// - Violation = criminal penalty + civil damages
///
/// ## Comparison
///
/// | System | Maximum Daily Hours |
/// |--------|---------------------|
/// | 🇫🇷 France | 10 hours (strict) |
/// | 🇯🇵 Japan | 8 hours (法定) + 36協定で延長可 |
/// | 🇪🇺 Directive | 13 hours (including breaks) |
/// | 🇩🇪 Germany | 10 hours (ArbZG) |
///
/// French and German law have similar strict daily limits.
#[must_use]
pub fn article_l3121_18() -> Statute {
    Statute::new(
        "code-travail-l3121-18",
        "Code du travail Article L3121-18 - Maximum Daily Duration",
        Effect::new(
            EffectType::StatusChange,
            "Durée quotidienne maximale / Maximum daily duration",
        )
        .with_parameter("max_daily_hours", "10")
        .with_parameter("exceptions", "Up to 12h with agreement (rare)")
        .with_parameter("enforcement", "Labor inspection + criminal penalties"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Daily hours must not exceed 10
    .with_precondition(Condition::Threshold {
        attributes: vec![("daily_hours".to_string(), 1.0)],
        operator: legalis_core::ComparisonOp::LessOrEqual,
        value: 10.0,
    })
    .with_discretion(
        "L'article L3121-18 limite la durée quotidienne du travail à 10 heures maximum, \
        afin de protéger la santé et la sécurité des travailleurs. Cette limite est stricte \
        et ne peut être dépassée que dans des circonstances exceptionnelles (urgence, accord collectif). \
        \n\nLe dépassement de cette limite constitue un délit pénal sanctionné par l'inspection du travail. \
        Le salarié peut également réclamer des dommages-intérêts pour le préjudice subi. \
        \n\nArticle L3121-18 limits daily working duration to a maximum of 10 hours, \
        to protect workers' health and safety. This limit is strict and can only be exceeded \
        in exceptional circumstances (urgency, collective agreement). \
        \n\nExceeding this limit constitutes a criminal offense sanctioned by labor inspection. \
        The employee can also claim damages for harm suffered. \
        \n\n【日仏比較】\n\
        日本の労働基準法32条は1日8時間を法定労働時間とするが、36協定により延長可能。\
        フランスでは10時間が絶対的上限に近く、例外は極めて限定的である。",
    )
}

/// Article L3121-20 - Maximum Weekly Working Duration
///
/// ## French Text
///
/// > Au cours d'une même semaine, la durée maximale hebdomadaire de travail est de
/// > quarante-huit heures.
///
/// ## English Translation
///
/// > During the same week, the maximum weekly working duration is forty-eight hours.
///
/// ## Legal Significance
///
/// **Maximum weekly hours**: **48 hours** (absolute weekly limit)
///
/// **Additional limit**:
/// - **Average over 12 weeks**: 44 hours maximum (can be extended to 46h by agreement)
///
/// **Relationship to 35-hour week**:
/// - Legal duration: 35 hours (overtime threshold)
/// - Maximum duration: 48 hours (absolute limit)
/// - Range 35-48h: Overtime with premiums required
///
/// ## Example
///
/// Week 1: 48 hours (max)
/// Week 2: 40 hours
/// → Average over 2 weeks: 44 hours ✓
///
/// ## Comparison
///
/// | System | Maximum Weekly Hours |
/// |--------|----------------------|
/// | 🇫🇷 France | 48h (absolute), 44h average over 12 weeks |
/// | 🇯🇵 Japan | 40h (法定) + 36協定で月45時間まで |
/// | 🇪🇺 Directive | 48h average over 4 months |
/// | 🇩🇪 Germany | 48h average over 6 months |
///
/// France has strict weekly limits aligned with EU law.
#[must_use]
pub fn article_l3121_20() -> Statute {
    Statute::new(
        "code-travail-l3121-20",
        "Code du travail Article L3121-20 - Maximum Weekly Duration",
        Effect::new(
            EffectType::StatusChange,
            "Durée hebdomadaire maximale / Maximum weekly duration",
        )
        .with_parameter("max_weekly_hours", "48")
        .with_parameter("average_over_12_weeks", "44 hours (or 46h with agreement)")
        .with_parameter("relationship_to_legal", "Legal 35h → Overtime threshold"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Weekly hours must not exceed 48
    .with_precondition(Condition::Threshold {
        attributes: vec![("weekly_hours".to_string(), 1.0)],
        operator: legalis_core::ComparisonOp::LessOrEqual,
        value: 48.0,
    })
    .with_discretion(
        "L'article L3121-20 fixe la durée maximale hebdomadaire à 48 heures, en conformité \
        avec la directive européenne sur le temps de travail. Cette limite absolue s'applique \
        à chaque semaine civile. \
        \n\nEn outre, la durée moyenne ne peut excéder 44 heures sur une période de 12 semaines \
        consécutives (46 heures avec accord collectif). Ainsi, même si une semaine peut atteindre \
        48 heures, la moyenne doit rester inférieure à 44h. \
        \n\nArticle L3121-20 sets the maximum weekly duration at 48 hours, in compliance \
        with the European working time directive. This absolute limit applies to each calendar week. \
        \n\nIn addition, the average duration cannot exceed 44 hours over a 12-week period \
        (46 hours with collective agreement). Thus, even if a week can reach 48 hours, \
        the average must remain below 44h. \
        \n\n【日仏比較】\n\
        日本の36協定による時間外労働の上限は月45時間（年360時間）だが、\
        特別条項で月100時間未満まで延長可能（2019年働き方改革）。\
        フランスでは週48時間（月約192-208時間）が絶対上限であり、日本より緩やかに見えるが、\
        12週平均44時間の規制により実効的な制限がある。",
    )
}

/// Article L3121-33 - Overtime Premium Rates
///
/// ## French Text
///
/// > Chacune des heures supplémentaires accomplies donne lieu à une majoration de
/// > salaire de 25 % pour les huit premières heures supplémentaires, les heures
/// > suivantes donnant lieu à une majoration de 50 %.
///
/// ## English Translation
///
/// > Each overtime hour worked gives rise to a wage premium of 25% for the first
/// > eight overtime hours, subsequent hours giving rise to a 50% premium.
///
/// ## Legal Significance
///
/// **Overtime premium rates** (Taux de majoration des heures supplémentaires):
///
/// | Overtime Hours | Premium Rate |
/// |----------------|--------------|
/// | Hours 36-43 (first 8 overtime) | **+25%** |
/// | Hours 44+ (beyond 8 overtime) | **+50%** |
///
/// **Example calculation**:
/// - Base rate: €15/hour
/// - Week: 40 hours (35 regular + 5 overtime)
/// - Pay: (35 × €15) + (5 × €15 × 1.25) = €525 + €93.75 = €618.75
///
/// **Collective agreements**: Can set higher rates (never lower).
///
/// **Compensatory rest**: Can partially replace premium with rest time (repos compensateur).
///
/// ## Comparison
///
/// | System | Overtime Premium |
/// |--------|------------------|
/// | 🇫🇷 France | +25% (first 8h), +50% (beyond) |
/// | 🇯🇵 Japan | +25% (weekday), +35% (late night) |
/// | 🇺🇸 USA | +50% (1.5× rate for hours > 40) |
/// | 🇩🇪 Germany | Varies by collective agreement |
///
/// French overtime premium is moderate compared to the US.
#[must_use]
pub fn article_l3121_33() -> Statute {
    Statute::new(
        "code-travail-l3121-33",
        "Code du travail Article L3121-33 - Overtime Premium Rates",
        Effect::new(
            EffectType::StatusChange,
            "Majoration des heures supplémentaires / Overtime premium established",
        )
        .with_parameter("first_8_hours_premium", "25%")
        .with_parameter("beyond_8_hours_premium", "50%")
        .with_parameter("can_be_replaced_by", "Compensatory rest (partial)"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_discretion(
        "L'article L3121-33 établit les taux de majoration pour les heures supplémentaires : \
        +25% pour les 8 premières heures au-delà de 35h, puis +50% au-delà. \
        Ces taux sont des minimums légaux ; les conventions collectives peuvent prévoir des taux supérieurs. \
        \n\nLes heures supplémentaires peuvent être compensées en tout ou partie par du repos \
        compensateur de remplacement (RCR), à condition d'un accord collectif. \
        Le contingent annuel d'heures supplémentaires est généralement de 220 heures. \
        \n\nArticle L3121-33 establishes overtime premium rates: +25% for the first 8 hours \
        beyond 35h, then +50% beyond. These rates are legal minimums; collective agreements \
        can provide higher rates. \
        \n\nOvertime hours can be compensated in whole or in part by compensatory rest (RCR), \
        subject to a collective agreement. The annual overtime quota is generally 220 hours. \
        \n\n【日仏比較】\n\
        日本の労働基準法37条は時間外労働に25%以上の割増賃金を要求し、\
        月60時間超の場合は50%以上（2023年中小企業も適用）。\
        フランスでは週35時間超から25%割増が始まり、8時間の時間外労働後は50%となる。",
    )
}

/// Article L3121-34 - Annual Overtime Hours Quota
///
/// ## French Text
///
/// > Le contingent annuel d'heures supplémentaires est défini par une convention ou
/// > un accord collectif d'entreprise ou d'établissement ou, à défaut, par une convention
/// > ou un accord de branche. À défaut de convention ou d'accord, ce contingent est fixé
/// > à deux cent vingt heures par salarié.
///
/// ## English Translation
///
/// > The annual quota of overtime hours is defined by a company or establishment
/// > collective agreement or, failing that, by a branch agreement. In the absence of
/// > an agreement, this quota is set at two hundred and twenty hours per employee.
///
/// ## Legal Significance
///
/// **Annual overtime quota** (Contingent annuel): **220 hours** (default)
///
/// **Purpose**:
/// - Limits excessive overtime
/// - Beyond quota: Requires labor inspector authorization or additional compensatory rest
///
/// **Collective agreements** can modify (increase or decrease).
///
/// **Calculation**:
/// - Per employee per year
/// - Only overtime hours beyond 35h count
/// - Example: 40h/week for 52 weeks = 260 overtime hours → Exceeds quota!
///
/// ## Comparison
///
/// | System | Annual Overtime Limit |
/// |--------|----------------------|
/// | 🇫🇷 France | 220h default (modifiable by agreement) |
/// | 🇯🇵 Japan | 360h (年間上限、2019年改革) |
/// | 🇩🇪 Germany | No statutory annual limit |
///
/// France's default is stricter than Japan's reformed limit.
#[must_use]
pub fn article_l3121_34() -> Statute {
    Statute::new(
        "code-travail-l3121-34",
        "Code du travail Article L3121-34 - Annual Overtime Quota",
        Effect::new(
            EffectType::StatusChange,
            "Contingent annuel d'heures supplémentaires / Annual overtime quota",
        )
        .with_parameter("default_annual_quota", "220 hours")
        .with_parameter("modifiable_by", "Collective agreement")
        .with_parameter("beyond_quota", "Requires authorization or extra rest"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_discretion(
        "L'article L3121-34 fixe le contingent annuel d'heures supplémentaires à 220 heures \
        en l'absence d'accord collectif. Ce contingent limite l'usage abusif des heures supplémentaires \
        et protège la santé des salariés. \
        \n\nAu-delà du contingent, l'employeur doit obtenir l'autorisation de l'inspecteur du travail \
        ou accorder une contrepartie obligatoire en repos (COR). Les heures hors contingent \
        donnent également droit à des garanties supplémentaires. \
        \n\nArticle L3121-34 sets the annual overtime quota at 220 hours in the absence \
        of a collective agreement. This quota limits abusive use of overtime and protects \
        employee health. \
        \n\nBeyond the quota, the employer must obtain authorization from the labor inspector \
        or grant mandatory compensatory rest (COR). Hours beyond the quota also give rise \
        to additional guarantees. \
        \n\n【日仏比較】\n\
        日本の2019年働き方改革により、時間外労働の上限は年360時間（特別条項で年720時間）。\
        フランスのデフォルト220時間はこれより厳格だが、労使協定で変更可能である点は日本の36協定に類似。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article_l3121_27_creation() {
        let statute = article_l3121_27();
        assert_eq!(statute.id, "code-travail-l3121-27");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));

        let params = &statute.effect.parameters;
        assert_eq!(params.get("legal_weekly_hours").unwrap(), "35");
    }

    #[test]
    fn test_article_l3121_18_creation() {
        let statute = article_l3121_18();
        assert_eq!(statute.id, "code-travail-l3121-18");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("max_daily_hours").unwrap(), "10");
    }

    #[test]
    fn test_article_l3121_20_creation() {
        let statute = article_l3121_20();
        assert_eq!(statute.id, "code-travail-l3121-20");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("max_weekly_hours").unwrap(), "48");
    }

    #[test]
    fn test_article_l3121_33_creation() {
        let statute = article_l3121_33();
        assert_eq!(statute.id, "code-travail-l3121-33");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("first_8_hours_premium").unwrap(), "25%");
        assert_eq!(params.get("beyond_8_hours_premium").unwrap(), "50%");
    }

    #[test]
    fn test_article_l3121_34_creation() {
        let statute = article_l3121_34();
        assert_eq!(statute.id, "code-travail-l3121-34");

        let params = &statute.effect.parameters;
        assert_eq!(params.get("default_annual_quota").unwrap(), "220 hours");
    }

    #[test]
    fn test_all_working_hours_articles_have_discretion() {
        let statutes = vec![
            article_l3121_27(),
            article_l3121_18(),
            article_l3121_20(),
            article_l3121_33(),
            article_l3121_34(),
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
    fn test_all_working_hours_articles_valid() {
        let statutes = vec![
            article_l3121_27(),
            article_l3121_18(),
            article_l3121_20(),
            article_l3121_33(),
            article_l3121_34(),
        ];

        for statute in statutes {
            assert!(statute.is_valid());
            assert_eq!(statute.validate().len(), 0);
        }
    }
}
