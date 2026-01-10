//! French evidence law - Burden of proof articles (Articles 1353-1355)
//!
//! This module implements the fundamental principles of burden of proof
//! in French civil law.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 1353 - Burden of proof principle (Charge de la preuve)
///
/// **Original French** (Code civil Article 1353):
/// > "Celui qui réclame l'exécution d'une obligation doit la prouver.
/// > Réciproquement, celui qui se prétend libéré doit justifier le paiement
/// > ou le fait qui a produit l'extinction de son obligation."
///
/// **English Translation**:
/// > "The person who claims performance of an obligation must prove it.
/// > Conversely, the person who claims to be released must justify payment
/// > or the fact that extinguished their obligation."
///
/// ## Legal Commentary
///
/// Article 1353 codifies the fundamental principle "actori incumbit probatio"
/// (the burden is on the claimant) from Roman law. This principle has two parts:
///
/// 1. **Claimant's burden**: Must prove the existence of the obligation
/// 2. **Defendant's burden**: Must prove payment or extinction of obligation
///
/// ## Historical Context
///
/// This article derives from the original 1804 Napoleonic Code (old Article 1315).
/// It was renumbered in the 2016 reform (Ordonnance n°2016-131) but the substance
/// remained unchanged, preserving two centuries of jurisprudence.
///
/// ## International Comparison
///
/// - **Germany** (ZPO §286): Similar burden allocation with judge's free evaluation
/// - **Japan** (Minpō §415): Creditor must prove breach; debtor proves performance
/// - **Common Law**: "He who asserts must prove" (Woolmington principle in criminal;
///   preponderance standard in civil)
/// - **Switzerland** (CC Art. 8): Burden on the party deriving rights from the fact
///
/// ## Modern Applications
///
/// - **Contract disputes**: Plaintiff proves contract + breach; defendant proves payment
/// - **Tort claims**: Plaintiff proves fault + damage + causation
/// - **Electronic commerce**: Seller must prove delivery; buyer proves non-receipt
/// - **Medical malpractice**: Patient proves duty + breach + harm; doctor proves compliance
///
/// ## Parameters
///
/// - `claim_type`: Type of claim (contractual, tortious, etc.)
/// - `claimant_facts`: Facts claimant must prove
/// - `defendant_facts`: Facts defendant must prove
/// - `standard_of_proof`: Level required (civil: preponderance)
/// - `evidence_available`: Whether evidence exists
pub fn article1353() -> Statute {
    Statute::new(
        "code-civil-1353",
        "Article 1353 - Burden of proof principle (actori incumbit probatio)",
        Effect::new(
            EffectType::Obligation,
            "Claimant must prove obligation; defendant must prove payment or extinction",
        )
        .with_parameter("claimant_must_prove", "existence_of_obligation")
        .with_parameter("defendant_must_prove", "payment_or_extinction")
        .with_parameter("standard_of_proof", "preponderance")
        .with_parameter("burden_allocation", "actori_incumbit_probatio")
        .with_parameter("roman_law_origin", "actor_sequitur_forum_rei"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "proceeding_type".to_string(),
            value: "civil".to_string(),
        }),
        Box::new(Condition::Or(
            Box::new(Condition::HasAttribute {
                key: "obligation_claimed".to_string(),
            }),
            Box::new(Condition::HasAttribute {
                key: "payment_defense".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Burden of proof allocation under Article 1353.

**Claimant's Burden**:
- Existence of obligation (contract, tort, unjust enrichment)
- Breach of obligation
- Damages suffered
- Causal link between breach and damages

**Defendant's Burden**:
- Payment made (receipt, bank transfer, witness)
- Performance completed (delivery, service rendered)
- Extinction of obligation (novation, remission, prescription)
- Valid defense (force majeure, impossibility)

**Standard of Proof**:
- Civil cases: Preponderance of evidence (more likely than not)
- Administrative: Same standard
- Criminal: Beyond reasonable doubt (not governed by Article 1353)

**Burden Shifts**:
- After claimant establishes prima facie case, burden shifts to defendant
- Defendant must prove affirmative defenses
- No shift if claimant fails initial burden

**Historical Notes**:
- Originally Article 1315 (1804 Code)
- Renumbered in 2016 reform
- Jurisprudence from 1804-2016 remains valid

**Comparative Law**:
Germany requires 'full conviction' (volle Überzeugung) but judges have
wide discretion. Japan follows similar burden allocation but with more
formalistic requirements. Common law uses 'balance of probabilities' in civil,
'beyond reasonable doubt' in criminal.

**Modern Developments**:
- Electronic evidence: Claimant must prove authenticity (Articles 1366-1378)
- Consumer contracts: Burden may shift to professional (Code de la consommation)
- Environmental law: Polluter bears burden (precautionary principle)
- Medical: Duty of information presumes lack of consent unless proven",
    )
}

/// Article 1354 - Legal presumptions (Présomptions légales)
///
/// **Original French** (Code civil Article 1354):
/// > "Les présomptions légales dispensent de toute preuve celui au profit duquel
/// > elles existent, sauf si elles sont mixtes ou simples, auquel cas elles
/// > peuvent être combattues par la preuve contraire."
///
/// **English Translation**:
/// > "Legal presumptions dispense with proof for the person in whose favor they exist,
/// > except if they are mixed or simple, in which case they can be rebutted by
/// > counter-evidence."
///
/// ## Legal Commentary
///
/// Article 1354 establishes three types of legal presumptions:
///
/// 1. **Irrebuttable** (présomption irréfragable): Cannot be rebutted
/// 2. **Mixed** (présomption mixte): Rebuttable only by specific evidence
/// 3. **Simple** (présomption simple): Fully rebuttable by any counter-evidence
///
/// ## Key Presumptions in French Law
///
/// **Irrebuttable**:
/// - Legitimacy after 300 days from divorce (Article 311 CC)
/// - Child's interest in parental authority decisions
/// - Validity of official acts properly executed
///
/// **Mixed**:
/// - Receipt acknowledgment presumes payment (rebuttable by fraud/duress proof)
/// - Possession presumes ownership for movables (Article 2276)
///
/// **Simple**:
/// - Good faith (Article 2274)
/// - Solidary obligation among co-debtors in commercial matters
/// - Fault from thing under one's custody (Article 1242 old Article 1384)
///
/// ## International Comparison
///
/// - **Germany** (BGB §292): Statutory presumptions have similar rebuttability rules
/// - **Japan** (Minpō): Fewer statutory presumptions, more judge-created ones
/// - **Common Law**: "Rebuttable presumptions" vs. "conclusive presumptions"
/// - **Italy** (CC Art. 2697): Similar tripartite classification
///
/// ## Modern Applications
///
/// - **Digital signatures**: Presumed authentic if meeting Article 1367 requirements
/// - **Registered mail**: Presumed received (simple presumption)
/// - **Professional capacity**: Merchants presumed acting in commerce
/// - **Child welfare**: Best interest of child (irrebuttable in practice)
///
/// ## Parameters
///
/// - `presumption_type`: Irrebuttable, mixed, or simple
/// - `presumed_fact`: The fact presumed by law
/// - `basis_fact`: The fact that triggers presumption
/// - `rebuttal_allowed`: Whether counter-evidence admissible
/// - `rebuttal_means`: Methods to rebut (if allowed)
pub fn article1354() -> Statute {
    Statute::new(
        "code-civil-1354",
        "Article 1354 - Legal presumptions (irrebuttable, mixed, simple)",
        Effect::new(
            EffectType::Grant,
            "Legal presumption dispenses beneficiary from proof unless rebuttable",
        )
        .with_parameter("presumption_types", "irrebuttable,mixed,simple")
        .with_parameter("irrebuttable_effect", "no_counter_evidence_allowed")
        .with_parameter("mixed_effect", "specific_rebuttal_means_only")
        .with_parameter("simple_effect", "fully_rebuttable")
        .with_parameter("burden_shift", "opponent_must_rebut"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::HasAttribute {
            key: "legal_presumption".to_string(),
        }),
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "presumption_category".to_string(),
                value: "statutory".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "presumption_category".to_string(),
                value: "judicial".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Legal presumptions under Article 1354.

**Types of Presumptions**:

1. **Irrebuttable (Présomption irréfragable)**:
   - No counter-evidence permitted
   - Examples:
     * Legitimacy (Article 311): Child born during marriage or within 300 days
       after dissolution is presumed child of husband
     * Official acts: Public documents presumed authentic
     * Best interest of child: Irrebuttable in custody decisions
   - Effect: Conclusive proof, no rebuttal allowed
   - Rare in modern law due to human rights concerns

2. **Mixed (Présomption mixte)**:
   - Rebuttable only by specific means (writing, confession, oath)
   - Examples:
     * Receipt (Article 1353 old): Written receipt presumes payment,
       rebuttable only by proof of fraud/duress/error
     * Possession of movables (Article 2276): 'En fait de meubles, possession
       vaut titre' - rebuttable by proof of theft/loss
   - Effect: Strong presumption requiring formal counter-proof
   - Common in commercial law

3. **Simple (Présomption simple)**:
   - Fully rebuttable by any admissible evidence
   - Examples:
     * Good faith (Article 2274): Possessor presumed in good faith
     * Fault from thing (Article 1242 §1): Custodian presumed at fault
     * Solidary obligation: Co-debtors in commercial matters
     * Paternity: Man who recognizes child presumed father
   - Effect: Shifts burden to opponent but allows full rebuttal
   - Most common type in civil law

**Procedural Effects**:
- Beneficiary need not prove presumed fact
- Opponent bears burden of rebuttal
- Judge evaluates rebuttal evidence freely (Article 1355 old, now CPC 9)
- Irrebuttable presumptions are substantive rules, not evidence rules

**Constitutional Limits**:
- ECHR Article 6: Right to fair trial limits irrebuttable presumptions
- Constitutional Council: Must allow some rebuttal for human rights
- Trend: Converting irrebuttable to mixed/simple presumptions

**International Comparison**:
Germany has similar system but with stricter formalities for rebuttal.
Common law distinguishes 'rebuttable' vs. 'irrebuttable' without intermediate
category. Japan has fewer statutory presumptions, relies more on judicial fact-finding.

**Modern Developments**:
- Electronic presumptions (Article 1367): Digital signature presumed reliable
- Consumer law: Professional's fault presumed in defective products
- Environmental: Polluter presumed liable (reversal of burden)
- Medical: Failure to inform presumes lack of consent",
    )
}

/// Article 1355 - Res judicata (Autorité de la chose jugée)
///
/// **Original French** (Code civil Article 1355):
/// > "L'autorité de la chose jugée n'a lieu qu'à l'égard de ce qui a fait l'objet
/// > du jugement. Il faut que la chose demandée soit la même ; que la demande soit
/// > fondée sur la même cause ; que la demande soit entre les mêmes parties, et formée
/// > par elles et contre elles en la même qualité."
///
/// **English Translation**:
/// > "The authority of res judicata only applies to what was the subject of the judgment.
/// > The thing claimed must be the same; the claim must be based on the same cause;
/// > the claim must be between the same parties, and made by them and against them
/// > in the same capacity."
///
/// ## Legal Commentary
///
/// Article 1355 establishes the doctrine of *res judicata* (chose jugée), which
/// prevents re-litigation of matters already decided. Requires triple identity:
///
/// 1. **Same object** (même objet): Same relief sought
/// 2. **Same cause** (même cause): Same legal basis
/// 3. **Same parties** (mêmes parties): Same litigants in same capacity
///
/// ## Historical Context
///
/// Derives from Roman law maxim "res judicata pro veritate accipitur" (a matter
/// adjudged is accepted as true). Originally Article 1351 (1804 Code), renumbered
/// in 2016 reform.
///
/// ## Triple Identity Test
///
/// **Same Object**:
/// - Same specific relief (damages, performance, declaration)
/// - Not merely same type of claim
/// - Example: Damages for 2024 breach ≠ damages for 2025 breach
///
/// **Same Cause**:
/// - Same legal basis (contract vs. tort different)
/// - Same factual grounds
/// - Example: Article 1240 tort ≠ Article 1241 negligence (different articles)
///
/// **Same Parties in Same Capacity**:
/// - Identical parties (privity required)
/// - Same legal capacity (heir ≠ original party; agent ≠ principal)
/// - Example: Claim vs. seller ≠ claim vs. manufacturer
///
/// ## International Comparison
///
/// - **Germany** (ZPO §322): Rechtskraft doctrine, similar triple identity
/// - **Japan** (Minji Soshō Hō §114): 既判力 (kihannryoku), narrower scope
/// - **Common Law**: "Cause of action estoppel" + "issue estoppel"
/// - **USA**: "Claim preclusion" + "issue preclusion" (broader than France)
///
/// ## Exceptions and Limits
///
/// - Fraud or forgery in original judgment
/// - New facts discovered after judgment
/// - Judgment contrary to public order
/// - International judgments (recognition issues)
///
/// ## Modern Applications
///
/// - **Serial litigation**: Prevents vexatious re-filing
/// - **Preclusive effect**: Earlier judgment precludes inconsistent later claim
/// - **Collateral attack**: Cannot challenge judgment in new proceeding
/// - **Enforcement**: Res judicata required for execution (exécution forcée)
///
/// ## Parameters
///
/// - `previous_judgment`: Citation to earlier judgment
/// - `same_object`: Whether relief sought is identical
/// - `same_cause`: Whether legal/factual basis is same
/// - `same_parties`: Whether parties match in same capacity
/// - `final_judgment`: Whether earlier judgment is final (définitif)
pub fn article1355() -> Statute {
    Statute::new(
        "code-civil-1355",
        "Article 1355 - Res judicata (chose jugée) - triple identity requirement",
        Effect::new(
            EffectType::Prohibition,
            "Final judgment bars re-litigation if same object, cause, and parties",
        )
        .with_parameter("triple_identity", "object,cause,parties")
        .with_parameter("res_judicata_effect", "claim_preclusion")
        .with_parameter("finality_required", "yes")
        .with_parameter("exceptions", "fraud,new_facts,public_order")
        .with_parameter("latin_maxim", "res_judicata_pro_veritate_accipitur"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::And(
            Box::new(Condition::HasAttribute {
                key: "previous_judgment".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "judgment_finality".to_string(),
                value: "final".to_string(),
            }),
        )),
        Box::new(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "same_object".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::And(
                Box::new(Condition::AttributeEquals {
                    key: "same_cause".to_string(),
                    value: "true".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "same_parties".to_string(),
                    value: "true".to_string(),
                }),
            )),
        )),
    ))
    .with_discretion(
        "Res judicata (chose jugée) under Article 1355.

**Triple Identity Requirement**:

1. **Same Object (Même objet)**:
   - Same specific relief sought (not merely same type)
   - Examples:
     * ✓ Same: Damages for 2024 breach (both claims)
     * ✗ Different: Performance vs. damages
     * ✗ Different: Damages for 2024 vs. 2025
   - Test: Would granting second claim conflict with first judgment?

2. **Same Cause (Même cause)**:
   - Same legal basis + same factual grounds
   - Examples:
     * ✓ Same: Contract breach Article 1231 (both)
     * ✗ Different: Article 1231 breach vs. Article 1240 tort
     * ✗ Different: 2024 facts vs. 2025 facts
   - Both law and facts must match

3. **Same Parties in Same Capacity (Mêmes parties en même qualité)**:
   - Identical parties (not merely related)
   - Same legal capacity (not successor/representative)
   - Examples:
     * ✓ Same: A vs. B (both claims)
     * ✗ Different: A vs. B vs. A's heir vs. B
     * ✗ Different: A individually vs. A as company director
   - Privity required; third parties not bound

**Procedural Effects**:
- **Claim preclusion**: Cannot re-litigate same claim
- **Issue preclusion**: Decided issues may preclude in later case (more limited)
- **Defense**: Defendant raises res judicata as affirmative defense
- **Timing**: Judge may raise sua sponte (d'office) if obvious

**Finality Requirement**:
- Judgment must be final (définitif): No further appeal possible
- Provisional judgments (référé) lack res judicata effect
- Interlocutory orders generally not final
- Cassation remand re-opens res judicata

**Exceptions**:
1. **Fraud (dol)**: Original judgment obtained by fraud/forgery
2. **New facts**: Material facts discovered after judgment (requires revision)
3. **Public order**: Judgment violates ordre public (rare)
4. **Lack of jurisdiction**: Original court lacked jurisdiction (void)

**Scope - What is Precluded**:
- **Préclusively decided**: Direct holding (dispositif)
- **Not precluded**: Dicta, reasoning (motifs) unless necessary to holding
- **Implicit holdings**: May have res judicata if logically necessary

**International Dimension**:
- Foreign judgments: Recognition required before res judicata applies (Brussels Ia)
- Arbitral awards: Same res judicata effect as court judgments (Article 1484 CPC)

**Comparative Notes**:
USA 'claim preclusion' is broader (bars all claims that could have been brought).
France requires actual litigation of same claim. Germany similar to France but
with Rechtskraft extending to 'implicit co-decided' issues. Common law 'issue estoppel'
is broader than French chose jugée.

**Modern Trends**:
- EU: Brussels Ia automatic recognition expands transnational res judicata
- Arbitration: ICSID awards have res judicata even if set aside domestically
- Class actions: Judgment binds class members (new in France since 2014)
- Online dispute resolution: Questions about res judicata effect of ODR",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article1353_structure() {
        let statute = article1353();
        assert_eq!(statute.id, "code-civil-1353");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("Burden of proof"));
    }

    #[test]
    fn test_article1353_parameters() {
        let statute = article1353();
        let params = &statute.effect.parameters;
        assert!(params.contains_key("claimant_must_prove"));
        assert!(params.contains_key("defendant_must_prove"));
        assert_eq!(
            params.get("standard_of_proof").map(|s| s.as_str()),
            Some("preponderance")
        );
    }

    #[test]
    fn test_article1354_structure() {
        let statute = article1354();
        assert_eq!(statute.id, "code-civil-1354");
        assert!(statute.title.contains("presumptions"));
    }

    #[test]
    fn test_article1354_parameters() {
        let statute = article1354();
        let params = &statute.effect.parameters;
        assert!(params.contains_key("presumption_types"));
        assert_eq!(
            params.get("presumption_types").map(|s| s.as_str()),
            Some("irrebuttable,mixed,simple")
        );
    }

    #[test]
    fn test_article1355_structure() {
        let statute = article1355();
        assert_eq!(statute.id, "code-civil-1355");
        assert!(statute.title.contains("Res judicata"));
    }

    #[test]
    fn test_article1355_parameters() {
        let statute = article1355();
        let params = &statute.effect.parameters;
        assert!(params.contains_key("triple_identity"));
        assert_eq!(
            params.get("triple_identity").map(|s| s.as_str()),
            Some("object,cause,parties")
        );
    }

    #[test]
    fn test_article1353_precondition() {
        let statute = article1353();
        assert!(!statute.preconditions.is_empty());
    }

    #[test]
    fn test_article1354_precondition() {
        let statute = article1354();
        assert!(!statute.preconditions.is_empty());
    }

    #[test]
    fn test_article1355_precondition() {
        let statute = article1355();
        assert!(!statute.preconditions.is_empty());
    }

    #[test]
    fn test_article1353_discretion() {
        let statute = article1353();
        assert!(statute.has_discretion());
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("Claimant's Burden")
        );
    }

    #[test]
    fn test_article1354_discretion() {
        let statute = article1354();
        assert!(statute.has_discretion());
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("Irrebuttable")
        );
    }

    #[test]
    fn test_article1355_discretion() {
        let statute = article1355();
        assert!(statute.has_discretion());
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("Triple Identity")
        );
    }
}
