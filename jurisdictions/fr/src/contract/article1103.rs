//! Article 1103 - Binding Force of Contracts (Force obligatoire du contrat)
//!
//! Implementation of Code civil Article 1103 (2016 reform).

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 1103 - Binding Force of Contracts (Force obligatoire du contrat)
///
/// ## French Text (2016 version)
///
/// > Les contrats légalement formés tiennent lieu de loi à ceux qui les ont faits.
///
/// ## English Translation
///
/// > Contracts legally formed have the force of law for those who made them.
///
/// ## Historical Context and Evolution
///
/// ### Pre-2016 Code Civil: Article 1134 (1804-2016)
///
/// Article 1103 restates the principle from the Napoleonic Code's Article 1134:
/// > "Les conventions légalement formées tiennent lieu de loi à ceux qui les ont faites."
///
/// The 2016 reform (Ordonnance n°2016-131 du 10 février 2016) modernized the language:
/// - Changed "conventions" → "contrats" (more precise terminology)
/// - Simplified phrasing while preserving the fundamental principle
/// - Relocated from Article 1134 to Article 1103 in the restructured code
///
/// ### Roman Law Foundations
///
/// The binding force principle derives from Roman law's **pacta sunt servanda**
/// ("agreements must be kept"), found in Justinian's Digest. French jurists
/// Jean Domat (1625-1696) and Robert-Joseph Pothier (1699-1772) developed
/// this into the modern principle that contracts have quasi-legislative force.
///
/// ### Napoleonic Code (1804)
///
/// The 1804 Code civil elevated contractual autonomy to unprecedented heights.
/// Article 1134's formulation "tiennent lieu de loi" (have the force of law)
/// reflected post-Revolutionary emphasis on individual liberty and reduced
/// state paternalism. This principle became the cornerstone of French contract law.
///
/// ### 2016 Reform Objectives
///
/// Ordonnance n°2016-131 pursued three goals:
/// 1. **Modernization**: Update language to reflect 200+ years of jurisprudence
/// 2. **Clarification**: Codify Cour de cassation's interpretations
/// 3. **EU Harmonization**: Align with European contract law principles (PECL, DCFR)
///
/// Article 1103 exemplifies continuity: the 2016 reform preserved Napoleonic
/// principles while modernizing expression. The binding force remains absolute,
/// subject only to statutory exceptions.
///
/// ## Legal Significance and Analysis
///
/// This article establishes the **binding force of contracts** (force obligatoire),
/// one of the three pillars of French contract law:
///
/// 1. **Freedom of contract** (liberté contractuelle) - Article 1102
/// 2. **Binding force** (force obligatoire) - Article 1103
/// 3. **Good faith** (bonne foi) - Article 1104
///
/// ### Key Principles:
///
/// **1. Quasi-Legislative Force ("tiennent lieu de loi")**
///
/// Contracts bind the parties with the same force as statutory law. This means:
/// - Parties must perform their obligations (performance is the rule)
/// - Courts cannot modify contract terms (principle of immutability)
/// - Third parties must respect the contractual relationship
///
/// Example: A sale contract for €100,000 creates obligations as binding as
/// statutory obligations. The seller must deliver; the buyer must pay. Neither
/// can unilaterally reduce the price or change delivery terms.
///
/// **2. Principle of Relativity (Article 1199)**
///
/// Only parties to the contract are bound. Third parties cannot be obligated
/// without their consent, though contracts may create rights for third-party
/// beneficiaries (stipulation pour autrui - Articles 1205-1209).
///
/// Example: If Company A contracts with Company B to supply goods, Company C
/// cannot be compelled to fulfill A's obligations, even if C later acquires A.
///
/// **3. Irrevocability and Exceptions**
///
/// Contracts cannot be revoked unilaterally, except:
/// - **Mutual agreement** (mutus dissensus) - Article 1193: Both parties agree to terminate
/// - **Statutory causes** - Article 1193: Law authorizes unilateral termination
///   (e.g., consumer protection laws, labor law protective dismissal)
/// - **Judicial termination** (résolution judiciaire) - Article 1224: Court orders
///   termination for serious breach (inexécution grave)
/// - **Unilateral termination** - Article 1226: In cases of serious breach with
///   formal notice (mise en demeure)
///
/// ### Formation Requirements and Defects
///
/// For Article 1103 to apply, the contract must be "legally formed" (légalement formé),
/// meaning it satisfies Article 1128's three requirements:
///
/// **1. Consent (Consentement) - Articles 1113-1122**
/// - Offer and acceptance must meet (rencontre des volontés)
/// - Consent must be free and informed
///
/// **Defects invalidating consent:**
/// - **Error (Erreur)** - Articles 1132-1136: Mistake about essential quality
///   - Example: Buying a "Stradivarius" violin that's actually a copy
///   - Leading case: Poussin painting case (Cass. civ. 1re, 22 févr. 1978)
/// - **Fraud (Dol)** - Articles 1137-1139: Intentional deception
///   - Example: Seller conceals structural defects in building
///   - Leading case: Baldus case (Cass. com., 3 mai 2000)
/// - **Duress (Violence)** - Articles 1140-1143: Threat or coercion
///   - Example: Contract signed under death threat
///   - Economic duress recognized (Cass. civ. 1re, 30 mai 2000)
///
/// **2. Capacity (Capacité) - Articles 1145-1152**
/// - Age of majority: 18 years (Article 414)
/// - Not under guardianship (tutelle) or curatorship (curatelle)
///
/// **3. Lawful and Certain Content - Articles 1162-1171**
/// - Not contrary to public order (ordre public) or morals (bonnes mœurs)
/// - Determinable and not purely illusory
///
/// ### Breach Remedies Hierarchy
///
/// When binding obligations are breached, Article 1217 provides five remedies:
/// 1. **Exception of non-performance** (exception d'inexécution) - Article 1219
/// 2. **Specific performance** (exécution forcée) - Articles 1221-1222
/// 3. **Price reduction** (réduction du prix) - Article 1223
/// 4. **Termination** (résolution) - Articles 1224-1230
/// 5. **Damages** (dommages-intérêts) - Articles 1231-1231-7
///
/// ## Modern Applications and Contemporary Examples
///
/// ### E-Commerce and Digital Contracts
///
/// **Click-wrap agreements**: French courts recognize binding force of online
/// contracts where user clicks "I accept," provided terms were accessible
/// (Cass. civ. 1re, 5 mars 2015, Leboncoin.fr).
///
/// **Browse-wrap agreements**: Less certain. Court of Cassation requires
/// affirmative acceptance; mere website use insufficient (TGI Paris, 2013).
///
/// ### Platform Economy Contracts
///
/// **Uber/Airbnb contracts**: Binding force applies, but courts scrutinize
/// unfair terms under consumer protection law (Code de la consommation L212-1).
/// Platform's general conditions bind users but subject to fairness review.
///
/// **Smart contracts and blockchain**: French law recognizes electronic contracts
/// (Ordonnance 2016-131, Article 1174). Smart contracts on blockchain have
/// binding force if they meet Article 1128 requirements. However, "Code is Law"
/// debate persists: should bugs in code vitiate consent (error) or must parties
/// bear consequences?
///
/// Example: DAO hack (2016) raised question whether code bug constitutes error
/// vitiating consent. French law likely would find error (Article 1132) if bug
/// concerns essential quality.
///
/// ### COVID-19 and Force Majeure
///
/// **Pandemic impact**: COVID-19 lockdowns raised force majeure claims (Article 1218).
/// Courts distinguish:
/// - **Force majeure**: Government prohibition preventing performance
///   - Example: Restaurant cannot perform catering contract during lockdown
///   - Effect: Suspension or termination, no damages
/// - **Imprevision** (unforeseeability): Economic hardship (Article 1195)
///   - Example: Supplier's costs triple due to supply chain disruption
///   - Effect: Renegotiation duty; judicial adaptation if parties fail
///
/// Leading case: Cass. civ. 1re, 25 nov. 2020 (COVID-19 rental contracts)
/// - Rent obligations remain binding despite economic hardship
/// - Only total impossibility constitutes force majeure
///
/// ### Consumer Protection Overlays
///
/// **B2C distinction**: While Article 1103 applies to all contracts, consumer
/// contracts (B2C) have additional protections under Code de la consommation:
/// - Right of withdrawal (droit de rétractation) - 14 days for distance contracts
/// - Unfair terms control (clauses abusives) - Articles L212-1 to L212-3
/// - Pre-contractual information duties - Article L111-1
///
/// **B2B contracts**: Full binding force with limited judicial intervention.
/// Courts presume commercial parties have equal bargaining power.
///
/// ## Case Law Examples (Leading Decisions)
///
/// ### Contract Formation and Binding Force
///
/// **1. Chronopost (Cass. com., 22 oct. 1996, n°93-18.632)**
/// - **Facts**: Courier company failed to deliver time-sensitive documents
/// - **Issue**: Limitation of liability clause excluding consequential damages
/// - **Holding**: Clause contradicts essential obligation (obligation essentielle),
///   rendering it void
/// - **Significance**: Binding force has limits when clause negates contractual purpose
///
/// **2. Champagne bottles case (Cass. civ. 3e, 28 mai 2008)**
/// - **Facts**: Buyer claimed error about quantity of champagne bottles
/// - **Holding**: No error; contract terms were clear and binding
/// - **Significance**: Courts strictly enforce clear contract terms under Article 1103
///
/// ### Good Faith and Binding Force
///
/// **3. Manoukian (Cass. com., 10 juill. 2007, n°06-14.768)**
/// - **Facts**: Franchisee claimed franchisor withheld pre-contractual information
/// - **Holding**: Pre-contractual good faith duty violated (now Article 1112)
/// - **Significance**: Good faith (Article 1104) complements binding force
///
/// **4. Huard (Cass. com., 3 nov. 1992)**
/// - **Facts**: Distributor terminated long-term relationship abruptly
/// - **Holding**: Abrupt termination violates good faith; reasonable notice required
/// - **Significance**: Binding force includes implicit duty to terminate reasonably
///
/// ### Unforeseeability and Contract Adaptation
///
/// **5. Canal de Craponne (Cass. civ., 6 mars 1876)**
/// - **Facts**: Inflation made 1560 contract price absurdly low
/// - **Holding**: Courts cannot revise contracts for changed circumstances
///   (refusal of imprevision doctrine)
/// - **Significance**: Historic rejection of unforeseeability doctrine
/// - **Note**: Overruled by 2016 reform's Article 1195 allowing imprevision
///
/// **6. Post-2016 imprevision cases (various, 2017-present)**
/// - **Facts**: Parties seek contract adaptation under Article 1195
/// - **Holding**: Courts require genuine unforeseeability and excessive onerousness
/// - **Significance**: Article 1195 creates exception to binding force for
///   unforeseeable changed circumstances
///
/// ## International and Comparative Law Analysis
///
/// ### Germany (BGB §§ 241, 275-326)
///
/// **Binding force**: BGB § 241 establishes obligation to perform (Pflicht zur Leistung).
/// German law uses "pacta sunt servanda" terminology rather than "force of law."
///
/// **Key differences**:
/// - German law allows judicial contract adaptation for changed circumstances
///   (Störung der Geschäftsgrundlage, § 313 BGB) more readily than French pre-2016 law
/// - BGB § 242 good faith principle (Treu und Glauben) is more pervasive than
///   French Article 1104
///
/// **Impossibility doctrine**: § 275 BGB excuses performance when impossible or
/// disproportionately burdensome. More flexible than French force majeure (Article 1218).
///
/// ### Japan (Minpō §§ 414-548)
///
/// **Binding force**: Article 414 provides compulsory performance (履行の強制).
/// Japanese law conceptually similar to French approach.
///
/// **2017 reform parallels**: Japan's 2017 Civil Code reform (effective 2020)
/// parallels France's 2016 reform:
/// - Modernized language (200+ years after 1896 Minpō)
/// - Clarified performance obligations
/// - Introduced unforeseeability doctrine (Article 543-2, similar to French Article 1195)
///
/// **Good faith obligation**: Article 1(2) establishes general good faith principle
/// (信義誠実の原則), similar to French Article 1104.
///
/// ### United States (UCC, Restatement 2d)
///
/// **Common law approach**: No equivalent to "force of law" language. Contracts
/// create obligations enforceable through remedies.
///
/// **UCC § 2-609**: Anticipatory repudiation allows suspension of performance
/// when reasonable grounds for insecurity arise. More flexible than French
/// exception d'inexécution (Article 1219).
///
/// **Restatement (Second) of Contracts § 175**: Duress and undue influence more
/// expansively defined than French violence (Articles 1140-1143).
///
/// **Key difference**: Common law prefers damages over specific performance;
/// French law prefers specific performance (Article 1221) reflecting binding force.
///
/// ### United Kingdom (Common Law)
///
/// **Consideration doctrine**: UK requires consideration (benefit/detriment) for
/// contract formation. French law requires no consideration (consensualism sufficient).
///
/// **Privity doctrine**: Only parties to contract can sue or be sued. Similar to
/// French relativity principle (Article 1199), but UK law has statutory exceptions
/// (Contracts (Rights of Third Parties) Act 1999).
///
/// **Remoteness of damages**: Hadley v. Baxendale (1854) foreseeability test
/// similar to French Article 1231-3 foreseeability limit.
///
/// ### CISG (Vienna Convention on International Sale of Goods)
///
/// **Binding force**: Article 7 requires observance of good faith in international
/// trade. No "force of law" language, but Articles 45-52 establish binding obligations
/// for sellers and buyers.
///
/// **Fundamental breach**: Article 25 requires "fundamental breach" for avoidance,
/// similar to French "inexécution grave" (Article 1224).
///
/// **Adaptation**: CISG lacks unforeseeability doctrine; hardship not grounds for
/// adaptation. Less flexible than French Article 1195.
///
/// ### UNIDROIT Principles of International Commercial Contracts
///
/// **Binding force**: Article 1.3 establishes binding character of contracts.
///
/// **Hardship**: Article 6.2.2 provides hardship doctrine allowing renegotiation
/// or judicial adaptation when:
/// - Events fundamentally alter equilibrium
/// - Events beyond affected party's control
/// - Risk not assumed by affected party
///
/// This closely resembles French Article 1195 imprevision, suggesting international
/// convergence toward French approach.
///
/// ### China (Contract Law 1999, Civil Code 2020)
///
/// **Binding force**: Article 8 of Contract Law (now Civil Code Article 509)
/// requires performance according to agreement (按照约定履行义务).
///
/// **Changed circumstances**: Article 533 allows contract adaptation for major
/// changed circumstances (情势变更), similar to French Article 1195.
///
/// **Good faith**: Article 7 (now Article 509) requires good faith performance
/// (诚实信用原则), paralleling French Article 1104.
///
/// **Socialist law influence**: Chinese law historically emphasized collective
/// interests over individual autonomy, but 1999-2020 reforms moved toward
/// Western contract principles, including stronger binding force.
///
/// ## Example
///
/// ```rust
/// use legalis_fr::contract::article1103;
///
/// let statute = article1103();
/// println!("{}", statute);
/// // => STATUTE code-civil-1103: "Code civil Article 1103"
/// ```
#[must_use]
pub fn article1103() -> Statute {
    Statute::new(
        "code-civil-1103",
        "Code civil Article 1103 - Force obligatoire du contrat / Binding Force of Contracts",
        Effect::new(
            EffectType::Obligation,
            "Les contrats ont force obligatoire (Contracts have binding force)",
        )
        .with_parameter(
            "bound_parties",
            "ceux qui les ont faits (those who made them)",
        )
        .with_parameter(
            "legal_status",
            "tiennent lieu de loi (have the force of law)",
        )
        .with_parameter(
            "revocation",
            "ne peuvent être révoqués que par accord mutuel ou pour causes autorisées par la loi",
        ),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Precondition: Contract must be legally formed
    .with_precondition(Condition::AttributeEquals {
        key: "contract_legally_formed".to_string(),
        value: "true".to_string(),
    })
    // The contract must have been validly formed (Article 1128 requirements)
    .with_precondition(Condition::And(
        Box::new(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "consent_given".to_string(), // Consentement
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "has_capacity".to_string(), // Capacité
                value: "true".to_string(),
            }),
        )),
        Box::new(Condition::AttributeEquals {
            key: "lawful_content".to_string(), // Contenu licite et certain
            value: "true".to_string(),
        }),
    ))
    .with_discretion(
        "L'article 1103 consacre le principe de la force obligatoire du contrat. \
        Les contrats légalement formés s'imposent aux parties comme une loi. \
        Ils ne peuvent être révoqués que par le consentement mutuel (Article 1193) \
        ou pour les causes que la loi autorise (force majeure, résolution judiciaire, etc.). \
        \n\nArticle 1103 establishes the binding force of contracts principle. \
        Legally formed contracts bind parties like a law. They can only be revoked \
        by mutual consent (Article 1193) or for causes authorized by law \
        (force majeure, judicial termination, etc.). \
        \n\n【比較法的考察】\n\
        フランス法の「契約は法の地位を占める」(tiennent lieu de loi)という表現は、\
        契約の拘束力を最も強く表現したものである。日本民法には同等の明文規定はないが、\
        契約の拘束力は当然の原則として認められている（民法414条の強制履行等）。\
        フランス法は契約自由の原則と拘束力の原則を明確に宣言している点が特徴的である。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article1103_creation() {
        let statute = article1103();
        assert_eq!(statute.id, "code-civil-1103");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert_eq!(statute.version, 1);
        assert_eq!(statute.effect.effect_type, EffectType::Obligation);
    }

    #[test]
    fn test_article1103_preconditions() {
        let statute = article1103();
        // Should have 2 main preconditions
        assert_eq!(statute.preconditions.len(), 2);

        // First: contract_legally_formed
        assert!(matches!(
            statute.preconditions[0],
            Condition::AttributeEquals { .. }
        ));

        // Second: Article 1128 requirements (AND of consent, capacity, lawful content)
        assert!(matches!(statute.preconditions[1], Condition::And(_, _)));
    }

    #[test]
    fn test_article1103_validation() {
        let statute = article1103();
        assert!(statute.is_valid());
        assert_eq!(statute.validate().len(), 0);
    }

    #[test]
    fn test_article1103_has_discretion() {
        let statute = article1103();
        assert!(statute.discretion_logic.is_some());

        let discretion = statute.discretion_logic.unwrap();
        assert!(discretion.contains("force obligatoire"));
        assert!(discretion.contains("binding force"));
    }

    #[test]
    fn test_article1103_effect_parameters() {
        let statute = article1103();

        let params = &statute.effect.parameters;
        assert!(params.contains_key("bound_parties"));
        assert!(params.contains_key("legal_status"));
        assert!(params.contains_key("revocation"));

        assert_eq!(
            params.get("legal_status").unwrap(),
            "tiennent lieu de loi (have the force of law)"
        );
    }

    #[test]
    fn test_article1103_display() {
        let statute = article1103();
        let display = format!("{}", statute);

        assert!(display.contains("code-civil-1103"));
        assert!(display.contains("Article 1103"));
        assert!(display.contains("FR"));
        assert!(display.contains("OBLIGATION"));
    }
}
