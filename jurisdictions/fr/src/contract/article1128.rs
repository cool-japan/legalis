//! Article 1128 - Validity Requirements (Conditions de validité du contrat)
//!
//! Implementation of Code civil Article 1128 (2016 reform).

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 1128 - Validity Requirements (Conditions de validité)
///
/// ## French Text (2016 version)
///
/// > Sont nécessaires à la validité d'un contrat :
/// > 1° Le consentement des parties ;
/// > 2° Leur capacité de contracter ;
/// > 3° Un contenu licite et certain.
///
/// ## English Translation
///
/// > Necessary for the validity of a contract are:
/// > 1° The consent of the parties;
/// > 2° Their capacity to contract;
/// > 3° Lawful and certain content.
///
/// ## Historical Context and Evolution
///
/// ### Pre-2016 Code Civil: Article 1108 (1804-2016)
///
/// Article 1128 replaces the Napoleonic Code's Article 1108, which required four conditions:
/// > 1° Le consentement de la partie qui s'oblige ;
/// > 2° Sa capacité de contracter ;
/// > 3° Un objet certain qui forme la matière de l'engagement ;
/// > 4° Une cause licite dans l'obligation.
///
/// Translation:
/// 1. Consent of the party binding himself
/// 2. His capacity to contract
/// 3. A certain object forming the subject matter of the undertaking
/// 4. A lawful cause in the obligation
///
/// ### The 2016 Reform's Major Innovation: From 4 to 3 Requirements
///
/// The 2016 reform (Ordonnance n°2016-131) eliminated the controversial **"cause"** (cause)
/// and **"object"** (objet) requirements, consolidating them into a single requirement:
/// **"lawful and certain content"** (contenu licite et certain).
///
/// This was one of the reform's most significant changes, ending century-long doctrinal
/// debates about the distinction between cause and object.
///
/// ### Roman Law and Canonical Foundations
///
/// **Roman Law**: Classical Roman law required:
/// - **Consensus** (meeting of minds)
/// - **Causa** (reason/basis for obligation)
/// - **Obiectum** (object/performance)
///
/// **Canonical Law**: Medieval canonists emphasized **causa** as moral justification
/// for enforcing promises. A promise without cause (nudum pactum) was unenforceable.
///
/// ### French Doctrinal Development (17th-18th Century)
///
/// **Jean Domat (1625-1696)**: Synthesized Roman law with Christian morality,
/// emphasizing cause as the "just reason" for contractual obligation.
///
/// **Robert-Joseph Pothier (1699-1772)**: Refined cause doctrine, distinguishing:
/// - **Causa proxima**: Immediate consideration (performance promised)
/// - **Causa remota**: Ultimate motive (subjective reason)
///
/// Pothier's work directly influenced the Napoleonic Code's drafters.
///
/// ### Napoleonic Code (1804): Article 1108
///
/// The 1804 Code civil enshrined four requirements, reflecting synthesis of:
/// - **Roman law**: Consent, object, cause
/// - **Natural law**: Capacity (rational will)
/// - **Revolutionary ideals**: Individual autonomy
///
/// This system remained unchanged for 212 years (1804-2016).
///
/// ### Why Eliminate Cause and Object?
///
/// **Doctrinal confusion**: Generations of French jurists debated the distinction
/// between cause and object, producing conflicting theories:
/// - **Cause objective**: Counter-performance (quid pro quo)
/// - **Cause subjective**: Motive (mobile)
/// - **Object**: Thing or service to be performed
///
/// **Jurisprudential adaptation**: The Cour de cassation had already effectively
/// merged cause into other requirements (fraud, public order).
///
/// **European harmonization**: PECL (Principles of European Contract Law) and DCFR
/// (Draft Common Frame of Reference) use simpler "lawful content" concept, not
/// distinguishing cause/object.
///
/// **Modernization**: "Content" (contenu) is clearer for contemporary transactions
/// (e.g., services, intellectual property) than 1804's "object" (thing-focused).
///
/// ### 2016 Reform Objectives for Article 1128
///
/// 1. **Simplification**: Reduce from 4 to 3 requirements
/// 2. **Clarification**: Eliminate cause/object confusion
/// 3. **Codification**: Incorporate 200+ years of case law
/// 4. **EU Harmonization**: Align with PECL/DCFR terminology
///
/// The reform preserved substantive protections while modernizing expression.
///
/// ## The Three Validity Requirements: Detailed Analysis
///
/// ### Requirement 1: Consent (Consentement) - Articles 1113-1122
///
/// **Definition**: Consent is the manifestation of will to be bound. It requires:
/// - **Offer** (offre): Firm proposal to contract (Article 1113)
/// - **Acceptance** (acceptation): Unequivocal agreement to offer (Article 1118)
/// - **Meeting of minds** (rencontre des volontés): Offer and acceptance must coincide
///
/// **Free and Informed Consent**: Consent must not be vitiated by defects (vices du consentement).
///
/// #### Defects Invalidating Consent (Articles 1130-1171)
///
/// **A. Error (Erreur) - Articles 1132-1136**
///
/// Error is a false belief about an essential element.
///
/// **Types of actionable error:**
/// 1. **Error about essential qualities** (erreur sur les qualités essentielles) - Article 1133
///    - Example: Buying artwork believing it's by Picasso when it's a copy
///    - Leading case: Poussin painting (Cass. civ. 1re, 22 févr. 1978)
///      - Buyer purchased paintings thinking they were by unknown artist
///      - Later discovered they were by Nicolas Poussin (major painter)
///      - Court: Seller's error about essential quality vitiates consent
///
/// 2. **Error about person** (erreur sur la personne) - Article 1134
///    - Only in intuitu personae contracts (where identity matters)
///    - Example: Hiring portrait painter, but wrong person shows up
///
/// **Non-actionable errors:**
/// - **Error about value** (erreur sur la valeur): Mistaken about price
///   - Exception: If value error stems from error about essential quality
/// - **Error about motive** (erreur sur le mobile): Mistaken about collateral circumstance
///   - Example: Buying house to be near planned metro station; metro plan cancelled
///
/// **B. Fraud (Dol) - Articles 1137-1139**
///
/// Fraud is intentional deception inducing consent.
///
/// **Elements of fraud (all required):**
/// 1. **Deceptive maneuvers** (manœuvres dolosives): Active concealment or lies
/// 2. **Intent to deceive** (intention de tromper): Deliberate, not mere puffery
/// 3. **Causation**: But for fraud, victim would not have contracted (dol principal)
///    or would have contracted on different terms (dol incident)
///
/// **Examples:**
/// - Seller conceals termite infestation in house (active concealment)
/// - Car dealer rolls back odometer (affirmative misrepresentation)
/// - Franchiser misrepresents profitability statistics (Manoukian case)
///
/// **Leading case: Baldus (Cass. com., 3 mai 2000)**
/// - **Facts**: Seller of rare stamps knew their exceptional value; buyer did not
/// - **Issue**: Does mere silence about value constitute fraud?
/// - **Holding**: Yes, if seller knows essential quality buyer ignores, duty to inform exists
/// - **Significance**: Expands fraud to include intentional non-disclosure in asymmetric information
///
/// **C. Duress (Violence) - Articles 1140-1143**
///
/// Duress is illegitimate pressure vitiating free consent.
///
/// **Types:**
/// 1. **Physical duress** (violence physique): Threat of bodily harm
///    - Example: "Sign this contract or I'll harm your family"
///
/// 2. **Economic duress** (violence économique) - Article 1143
///    - Exploitation of state of dependency (état de dépendance)
///    - To obtain manifestly excessive advantage (avantage manifestement excessif)
///    - Example: Creditor threatens bankruptcy unless debtor accepts unfavorable terms
///    - Leading case: Cass. civ. 1re, 30 mai 2000 (recognizing economic duress)
///
/// 3. **Moral duress** (violence morale): Psychological pressure
///    - Example: Threat to reveal embarrassing information
///
/// **Test**: Would the threat cause reasonable person in victim's position to consent?
/// (Article 1140: appreciates threat considering age, sex, condition of person)
///
/// #### Pre-Contractual Information Duties (Article 1112-1)
///
/// Added by 2016 reform: Party possessing information essential to other party's
/// consent must disclose it (devoir d'information précontractuelle).
///
/// **Exceptions** (Article 1112-1, al. 2):
/// - Information publicly available
/// - Information counterparty should know given profession
/// - Legitimate confidentiality (trade secrets)
///
/// Failure constitutes fraud vitiating consent.
///
/// ### Requirement 2: Capacity (Capacité) - Articles 1145-1152
///
/// **General Principle (Article 1145)**: Capacity is the rule; incapacity is the exception.
/// Every person may contract unless declared incapable by law.
///
/// **Capacity Requirements:**
///
/// **A. Age of Majority**
/// - Age: 18 years (Article 414 Civil Code)
/// - Emancipated minors (16+) have full capacity (Article 413-6)
///
/// **B. Mental Capacity**
/// - Not under guardianship (tutelle) for mental incapacity (Article 440)
/// - Not under curatorship (curatelle) requiring assistance (Article 467)
///
/// **C. Special Incapacities**
///
/// **Minors** (Mineurs - under 18):
/// - **General rule**: Contracts voidable at minor's option (Article 1148)
/// - **Exceptions**:
///   - Necessaries: Food, clothing, education (valid)
///   - Authorized contracts: With parental consent
///   - Conservative acts: Acts preserving property
///
/// **Protected Adults** (Majeurs protégés):
/// 1. **Guardianship (Tutelle)**: Complete incapacity; guardian acts for protected person
/// 2. **Curatorship (Curatelle)**: Partial incapacity; protected person acts with curator's assistance
/// 3. **Legal Protection (Sauvegarde de justice)**: Temporary; contracts voidable if excessive
///
/// **Juridical Persons** (Personnes morales):
/// - Capacity limited to corporate purpose (objet social)
/// - Ultra vires acts (beyond purpose) voidable if other party knew or should have known
///
/// **Example Scenarios:**
///
/// **Valid contract**: 20-year-old university student buys laptop for €800.
/// - Has capacity (age 18+, not under protection)
///
/// **Voidable contract**: 17-year-old purchases luxury car for €50,000 without parental consent.
/// - Lacks capacity (minor)
/// - Exception doesn't apply (luxury car not necessary)
///
/// **Voidable contract**: Person under curatorship signs real estate sale without curator's assistance.
/// - Lacks capacity (curatorship requires assistance for important acts)
///
/// ### Requirement 3: Lawful and Certain Content (Contenu licite et certain) - Articles 1162-1171
///
/// This requirement combines what former law called "object" and "cause."
///
/// **A. Lawful Content (Contenu licite)**
///
/// Contract content must not violate:
/// 1. **Public order** (ordre public) - Article 6
///    - Mandatory statutory rules (e.g., minimum wage, safety regulations)
///    - Constitutional principles (human dignity, equality)
///
/// 2. **Morals** (bonnes mœurs) - Article 6
///    - Fundamental ethical values
///    - Modern interpretation: Less moralistic than 1804; focuses on human dignity
///
/// **Examples of unlawful content:**
/// - Contract to commit crime (e.g., hired assassination)
/// - Contract selling oneself into slavery (violates human dignity)
/// - Contract waiving all liability for gross negligence (violates public order)
/// - Surrogate motherhood contract (violates human dignity, principe d'indisponibilité du corps humain)
///   - Leading case: Cass. Ass. plén., 31 mai 1991 (surrogate motherhood)
/// - Contract restraining trade excessively (violates economic public order)
///
/// **Public Order Categories:**
/// - **Protective public order** (ordre public de protection): Protects weaker party (e.g., consumer law)
/// - **Directive public order** (ordre public de direction): Guides economic policy (e.g., price controls)
/// - **Classic public order** (ordre public classique): Protects fundamental values (e.g., human dignity)
///
/// **B. Certain Content (Contenu certain)**
///
/// Contract content must be:
/// 1. **Determined or determinable** (déterminé ou déterminable) - Article 1163
///    - Performance must be identifiable
///    - May be determinable by objective criteria
///
/// 2. **Not purely illusory** (non purement potestatif)
///    - Cannot depend solely on debtor's arbitrary will (Article 1170)
///
/// **Examples:**
///
/// **Valid (determinable)**: "I will sell you wheat at market price on delivery date."
/// - Content determinable by objective market price
///
/// **Invalid (illusory)**: "I will sell you wheat if I feel like it."
/// - Content depends on seller's arbitrary will (purely potestative condition)
///
/// **Valid (not illusory)**: "I will sell you wheat if harvest succeeds."
/// - Condition depends on objective event (harvest), not arbitrary will
///
/// **C. Replaces Former "Cause" Requirement**
///
/// Pre-2016 "cause" doctrine addressed:
/// 1. **Absence of cause** (absence de cause): No reason to be bound
///    - Example: Promise to pay €10,000 for nothing in return
///    - Now addressed by "certain content" (content must not be illusory)
///
/// 2. **False cause** (cause fausse): Mistaken about reason
///    - Example: Pay debt that doesn't exist
///    - Now addressed by "error" (Articles 1132-1136)
///
/// 3. **Unlawful cause** (cause illicite): Immoral or illegal purpose
///    - Example: Payment for illegal service
///    - Now addressed by "lawful content" (Article 1162)
///
/// The 2016 reform didn't eliminate substantive protections; it reorganized them under clearer headings.
///
/// ## Modern Applications and Contemporary Examples
///
/// ### E-Commerce Formation Issues
///
/// **Online consent challenges:**
/// - **Click-wrap**: Valid if terms accessible before clicking "I accept" (Article 1127-1)
/// - **Browse-wrap**: Questionable; French courts require affirmative consent
/// - **Capacity verification**: Checkbox "I am 18+" insufficient for high-value contracts;
///   merchants must verify age for regulated products (alcohol, gambling)
///
/// **Content issues:**
/// - **Dynamic pricing**: Valid if algorithm determinable (satisfies "certain" requirement)
/// - **Terms of service**: Must not contain unfair clauses (B2C) or clauses négating essential obligation (B2B)
///
/// ### Platform Economy Challenges
///
/// **Uber/Lyft driver agreements:**
/// - **Capacity issue**: Independent contractor classification disputed
/// - **Content issue**: Unilateral modification clauses challenged as illusory (purely potestative)
/// - French courts often find workers are employees, not independent contractors (requalification)
///
/// **Airbnb host agreements:**
/// - **Content issue**: Limitation of liability clauses must not negate essential obligation
/// - **Public order**: Must comply with local housing regulations (ordre public de direction)
///
/// ### Smart Contracts and Blockchain
///
/// **Consent issues:**
/// - **Code bugs**: Do bugs vitiate consent (error about essential quality)?
/// - **Informed consent**: Are users adequately informed about smart contract operation?
///
/// **Content issues:**
/// - **Determinability**: Smart contract code provides objective determination mechanism (valid)
/// - **Lawfulness**: Smart contracts executing illegal purposes (e.g., money laundering) void
///
/// **Capacity issues:**
/// - **Pseudonymity**: How verify capacity when parties use blockchain addresses?
/// - French law requires capacity verification for high-value/regulated transactions
///
/// ### COVID-19 Pandemic Impact
///
/// **Consent issues:**
/// - **Imprevision** (Article 1195): Changed circumstances may allow contract adaptation
/// - Not consent defect, but post-formation adjustment mechanism
///
/// **Content issues:**
/// - **Force majeure clauses**: Must be certain (not purely potestative)
/// - Courts interpret restrictively; generic force majeure clauses may be uncertain
///
/// ### Consumer Protection Overlays (B2C)
///
/// **Additional consent requirements:**
/// - **Cooling-off period**: 14 days to withdraw from distance contracts (Directive 2011/83/EU)
/// - **Pre-contractual information**: Extensive disclosure duties (Code de la consommation L111-1)
///
/// **Content control:**
/// - **Unfair terms** (clauses abusives): Terms creating significant imbalance void (L212-1)
/// - **Black list**: Certain terms automatically unfair (e.g., unilateral modification without valid reason)
///
/// **Capacity protections:**
/// - **Vulnerable consumers**: Enhanced protections for elderly, disabled
///
/// ### B2B Commercial Contracts
///
/// **Consent:**
/// - Presumption of informed consent between commercial parties
/// - Pre-contractual information duties less extensive than B2C
///
/// **Content:**
/// - Greater freedom to allocate risks (limitation of liability clauses valid if reasonable)
/// - Exception: Cannot negate essential obligation (Chronopost doctrine)
///
/// **Capacity:**
/// - Corporate capacity limits: Ultra vires doctrine (acts beyond corporate purpose)
///
/// ## Case Law Examples (Leading Decisions)
///
/// ### Consent Defects: Error
///
/// **1. Poussin Painting Case (Cass. civ. 1re, 22 févr. 1978, n°76-11.551)**
/// - **Facts**: Sellers sold paintings for modest sum, believing them of little value;
///   later discovered paintings were by Nicolas Poussin, worth millions
/// - **Issue**: Does error about value constitute actionable error?
/// - **Holding**: Yes, because value error stemmed from error about essential quality
///   (authorship). Error about essential quality vitiates consent.
/// - **Significance**: Error about value actionable if caused by error about essential quality
///
/// **2. Fragonard Drawing Case (Cass. civ. 1re, 3 mai 2000, n°98-11.381)**
/// - **Facts**: Buyer purchased drawings for €300; later sold for €2.7 million
/// - **Issue**: Did seller's error about value vitiate consent?
/// - **Holding**: No actionable error; seller had opportunity to assess value
/// - **Significance**: Error must concern essential quality, not mere value; parties bear risk of value assessment
///
/// ### Consent Defects: Fraud
///
/// **3. Baldus Case (Cass. com., 3 mai 2000, n°97-11.969)**
/// - **Facts**: Stamp collector purchased rare stamps from dealer for €300,000;
///   actually worth €2.8 million. Dealer knew exceptional value; buyer did not.
/// - **Issue**: Does dealer's silence about true value constitute fraud?
/// - **Holding**: Yes, when seller knows essential quality buyer ignores, and
///   buyer legitimately relies on seller's expertise, silence constitutes fraud
/// - **Significance**: Expands fraud to intentional non-disclosure in asymmetric information scenarios
///
/// **4. Franchisee Information Case (Cass. com., 10 juill. 2007, n°06-14.768, Manoukian)**
/// - **Facts**: Franchisor failed to disclose key financial information before contract
/// - **Holding**: Pre-contractual information duty violated; constitutes fraud
/// - **Significance**: Pre-contractual information duties now codified in Article 1112-1
///
/// ### Consent Defects: Duress
///
/// **5. Economic Duress Recognition (Cass. civ. 1re, 30 mai 2000, n°98-15.242)**
/// - **Facts**: Creditor threatened debtor with bankruptcy unless debtor accepted unfavorable restructuring
/// - **Holding**: Economic duress recognized; exploitation of state of dependency vitiates consent
/// - **Significance**: Extended duress beyond physical/moral threats to economic pressure; codified in Article 1143
///
/// ### Capacity
///
/// **6. Minor's Contract Nullity (Cass. civ. 1re, 12 févr. 2014, n°12-29.447)**
/// - **Facts**: 17-year-old purchased vehicle on credit
/// - **Holding**: Contract voidable at minor's option; creditor cannot rely on apparent capacity
/// - **Significance**: Capacity protection absolute for minors; apparent capacity defense unavailable
///
/// **7. Ultra Vires Acts (Cass. com., 21 oct. 2014, n°13-20.230)**
/// - **Facts**: Company president signed contract beyond corporate purpose
/// - **Holding**: Voidable if other party knew or should have known act was ultra vires
/// - **Significance**: Corporate capacity limits protect companies but require other party's knowledge/negligence
///
/// ### Content: Lawfulness
///
/// **8. Surrogate Motherhood (Cass. Ass. plén., 31 mai 1991, n°90-20.105)**
/// - **Facts**: Couple contracted with woman to bear child for them
/// - **Holding**: Surrogate motherhood contracts void as contrary to public order
///   (principe d'indisponibilité du corps humain - principle of unavailability of human body)
/// - **Significance**: Human dignity principle limits contractual freedom; body not object of commerce
///
/// **9. Competition Restriction (Cass. com., 15 sept. 2009, n°08-17.841)**
/// - **Facts**: Non-compete clause prevented employee from any work in industry
/// - **Holding**: Excessively broad non-compete clause void as contrary to public order
///   (freedom to work)
/// - **Significance**: Non-compete clauses must be limited in time, space, and scope
///
/// ### Content: Certainty
///
/// **10. Potestative Condition (Cass. com., 9 juill. 2013, n°12-20.230)**
/// - **Facts**: Contract allowed one party to terminate "at will"
/// - **Holding**: Purely potestative condition void; must have objective criteria
/// - **Significance**: Obligation cannot depend solely on debtor's arbitrary will (Article 1170)
///
/// **11. Price Determinability (Cass. com., 1er déc. 1995, "Arrêts de l'Assemblée plénière")**
/// - **Facts**: Long-term supply contracts with price "to be agreed"
/// - **Holding**: Price need not be predetermined if determinable by objective criteria
///   (e.g., market price, published index)
/// - **Significance**: Relaxed certainty requirement for price in ongoing commercial relationships;
///   suffices if determinable
///
/// ### Integration of Former "Cause" Doctrine
///
/// **12. Absence of Cause/Illusory Content (Cass. civ. 1re, 27 mars 2008, n°05-21.814)**
/// - **Facts**: Promise to pay money without counter-performance
/// - **Holding**: Void for absence of cause (pre-2016) / illusory content (post-2016)
/// - **Significance**: Pre-2016 "absence of cause" doctrine now subsumed in "certain content" requirement
///
/// ## International and Comparative Law Analysis
///
/// ### Germany (BGB §§ 145-157, 275-326)
///
/// **Formation requirements:**
/// - **Offer and acceptance** (Angebot und Annahme): BGB §§ 145-157
/// - **Capacity** (Geschäftsfähigkeit): BGB §§ 104-113 (age 18, mental capacity)
/// - **No illegality/immorality** (Gesetzliches Verbot, Sittenwidrigkeit): BGB §§ 134, 138
///
/// **Key differences from French law:**
/// 1. **No "cause" or "content" requirement**: German law never adopted causa doctrine
/// 2. **Separate illegality provisions**: BGB § 134 (statutory prohibition), § 138 (immorality)
///    treated separately from formation requirements
/// 3. **Fraud (Arglistige Täuschung)**: BGB § 123 makes contract voidable (anfechtbar),
///    not void (nichtig) as in French law
/// 4. **Duress (Widerrechtliche Drohung)**: BGB § 123 requires threat be "wrongful"
///    (widerrechtlich); narrower than French economic duress (Article 1143)
///
/// **Impossibility doctrine**: BGB § 275 excuses performance if impossible or disproportionately
/// burdensome. More flexible than French force majeure; operates at formation stage.
///
/// ### Japan (Minpō §§ 90, 96-98, 415-548)
///
/// **Formation requirements:**
/// - **Offer and acceptance** (申込みと承諾): Articles 522-528
/// - **Capacity** (行為能力): Articles 3-21 (age 18 since 2022 reform, previously 20)
/// - **Lawfulness** (公序良俗): Article 90 (public order and morals)
///
/// **2017/2020 reform parallels:**
/// Japan's Civil Code reform (enacted 2017, effective 2020) parallels France's 2016 reform:
/// 1. **Modernization**: Updated language from 1896 Meiji Code
/// 2. **Capacity age**: Lowered from 20 to 18 (2022), following global trend (France: 18 since 1974)
/// 3. **Good faith obligation**: Enhanced Article 1(2) good faith principle (信義誠実の原則)
///
/// **Key differences from French law:**
/// 1. **No causa doctrine**: Japanese law never required "cause"; influenced by German pandectist approach
/// 2. **Fraud (詐欺)**: Article 96 makes contract voidable; requires "intent to deceive"
///    but broader than French dol (includes reckless misrepresentation)
/// 3. **Duress (強迫)**: Article 96; narrower than French violence (requires threat, not mere exploitation)
/// 4. **Public order**: Article 90 broader than French Article 6; encompasses social policy objectives
///
/// **Good faith obligation**: Article 1(2) establishes general good faith principle applicable
/// throughout contract law, more pervasive than French Article 1104.
///
/// ### United States (UCC, Restatement 2nd)
///
/// **Common law formation requirements:**
/// 1. **Offer and acceptance**: Meeting of minds (mutual assent)
/// 2. **Consideration**: Bargained-for exchange (benefit/detriment)
/// 3. **Capacity**: Age of majority (18 in most states), mental capacity
/// 4. **Legality**: Not contrary to statute or public policy
///
/// **UCC (Uniform Commercial Code) modifications:**
/// - UCC § 2-204: Contract formation relaxed; may be formed "in any manner sufficient to show agreement"
/// - UCC § 2-305: Open price terms allowed; gap-fillers supply reasonable price
/// - UCC § 2-306: Exclusive dealing and output/requirements contracts valid despite indefiniteness
///
/// **Key differences from French law:**
/// 1. **Consideration requirement**: US requires consideration; French law does not (consensualism)
///    - Gratuitous promises generally unenforceable in US (exception: promissory estoppel)
///    - Gratuitous promises enforceable in France if genuine consent (no consideration requirement)
///
/// 2. **Fraud (Misrepresentation)**: Restatement (Second) of Contracts §§ 162-164
///    - Broader than French dol: includes negligent and innocent misrepresentation
///    - Remedy: Rescission (voidable) or damages, plaintiff's election
///
/// 3. **Duress**: Restatement § 175 includes "economic duress" (similar to French Article 1143)
///    but requires "wrongful threat" leaving "no reasonable alternative"
///
/// 4. **Capacity**: Minors' contracts voidable (similar to French law), but "emancipation"
///    doctrine varies by state
///
/// 5. **Statute of Frauds**: US requires certain contracts be in writing (e.g., sale of land,
///    contracts not performable within 1 year). French law has no general Statute of Frauds
///    (though specific contracts require written form, Article 1359).
///
/// ### United Kingdom (Common Law)
///
/// **Formation requirements:**
/// 1. **Offer and acceptance**: "Meeting of minds"
/// 2. **Consideration**: Essential element; gratuitous promises unenforceable (exception: deed)
/// 3. **Intention to create legal relations**: Presumed in commercial context, not in social/domestic
/// 4. **Capacity**: Age 18, mental capacity
/// 5. **Legality**: Not contrary to statute or public policy
///
/// **Key differences from French law:**
/// 1. **Consideration doctrine**: Fundamental difference; UK requires consideration, France does not
///    - Leading case: Stilk v. Myrick (1809): Past consideration insufficient
///
/// 2. **Intention to create legal relations**: Unique to common law; French law presumes
///    agreements intended to be binding if consent given
///
/// 3. **Misrepresentation**: Misrepresentation Act 1967 distinguishes:
///    - Fraudulent misrepresentation: Tort of deceit, damages
///    - Negligent misrepresentation: Damages unless defendant proves reasonable grounds
///    - Innocent misrepresentation: Rescission only (unless court exercises discretion)
///
/// 4. **Duress and Undue Influence**:
///    - **Duress**: Historically narrow (threat of physical harm); expanded to economic duress
///      (Universe Tankships Inc. of Monrovia v. ITWF, 1983)
///    - **Undue influence**: Equitable doctrine; no French equivalent. Presumed in certain
///      relationships (trustee/beneficiary, doctor/patient, solicitor/client)
///
/// 5. **Capacity**: UK law similar to France; minors' contracts voidable with exceptions for necessaries
///
/// ### CISG (Vienna Convention on International Sale of Goods)
///
/// **Formation requirements (Articles 14-24):**
/// - **Offer** (Article 14): Sufficiently definite proposal indicating intention to be bound
/// - **Acceptance** (Article 18): Statement or conduct indicating assent to offer
/// - **No consideration requirement**: Follows civil law approach
/// - **No capacity provisions**: Leaves capacity to domestic law
/// - **No general invalidity provisions**: No comprehensive rules on fraud, duress, mistake
///
/// **Key features:**
/// 1. **Form freedom**: No writing requirement (Article 11)
/// 2. **Battle of forms**: Article 19 (mirror image rule with exceptions)
/// 3. **Modification**: No consideration required for modification (Article 29)
///
/// **Limitations compared to French law:**
/// - CISG lacks comprehensive validity provisions; parties must invoke domestic law
///   (Article 4) for consent defects, capacity, lawfulness
/// - French courts apply CISG to international sales but supplement with Code civil
///   for validity issues
///
/// ### UNIDROIT Principles of International Commercial Contracts
///
/// **Formation requirements:**
/// - **Offer and acceptance** (Articles 2.1.1-2.1.22): Detailed formation rules
/// - **No consideration requirement**: Article 3.1.2 (contract valid without consideration)
/// - **Validity** (Articles 3.1-3.21): Comprehensive provisions on:
///   - Capacity: Left to domestic law
///   - Mistake (Error): Article 3.2.1-3.2.6
///   - Fraud: Article 3.2.7-3.2.9
///   - Threat: Article 3.2.10-3.2.11
///   - Gross disparity: Article 3.2.7 (excessive benefit, abuse)
///
/// **Convergence with French law:**
/// 1. **No consideration**: Follows French consensualist approach
/// 2. **Mistake**: Article 3.2.2 similar to French error doctrine (mistake about essential quality)
/// 3. **Fraud**: Article 3.2.7 similar to French dol (intentional non-disclosure)
/// 4. **Threat**: Article 3.2.10 includes "unjustified threat" (similar to French economic duress)
/// 5. **Gross disparity**: Article 3.2.7 resembles French lésion (excessive disadvantage)
///    but available for all contracts (French lésion limited to specific contracts)
///
/// **Significance**: UNIDROIT Principles show international convergence toward French-style
/// consensualism and comprehensive validity provisions.
///
/// ### China (Contract Law 1999, Civil Code 2020)
///
/// **Formation requirements (Civil Code Articles 490-502):**
/// 1. **Offer and acceptance** (要约与承诺): Articles 472-488
/// 2. **Capacity** (民事行为能力): Articles 17-26 (age 18, mental capacity)
/// 3. **Lawfulness** (合法性): Articles 143, 153 (not contrary to law, public order, morals)
/// 4. **True intent** (真实意思表示): Article 143
///
/// **Civil Code 2020 modernization:**
/// China's new Civil Code (effective 2021) consolidates Contract Law (1999) with General
/// Principles of Civil Law, showing evolution toward Western principles:
///
/// 1. **Good faith principle** (诚实信用原则): Article 7 (similar to French Article 1104)
/// 2. **Freedom of contract** (合同自由): Article 5 (similar to French Article 1102)
/// 3. **Binding force**: Article 509 (按照约定履行义务 - perform according to agreement)
///
/// **Validity defects (Civil Code Articles 143-157):**
/// - **Fraud** (欺诈): Article 148 (voidable)
/// - **Duress** (胁迫): Article 150 (voidable)
/// - **Mistake** (重大误解): Article 147 (voidable if "material mistake")
///
/// **Key differences from French law:**
/// 1. **Socialist law heritage**: Historically emphasized collective interests over individual autonomy,
///    but recent reforms strengthen private autonomy
/// 2. **Administrative approval**: Some contracts require government approval (e.g., foreign investment),
///    no French equivalent
/// 3. **Standard form contracts**: Extensive regulation (Articles 496-498) due to consumer protection
///    concerns; stricter than French unfair terms control
/// 4. **Interpretation**: Article 142 emphasizes "objective standard" over subjective intent;
///    differs from French emphasis on parties' common intent (Article 1188)
///
/// **Changed circumstances doctrine**: Article 533 allows contract adaptation for "major changed
/// circumstances" (情势变更), similar to French Article 1195 imprevision.
///
/// ### European Contract Law Harmonization Efforts
///
/// **PECL (Principles of European Contract Law)**
/// - Article 4:101: Formation requirements similar to French 2016 reform
///   - Consent, lawfulness, no formal "capacity" or "cause" requirement
/// - Influenced French reform's elimination of cause/object distinction
///
/// **DCFR (Draft Common Frame of Reference)**
/// - Comprehensive formation and validity provisions
/// - Influenced French Article 1128's "content" (not "cause/object") terminology
///
/// **CESL (Common European Sales Law) - Proposed, not adopted**
/// - Would have created optional EU contract law regime
/// - Formation provisions similar to CISG and UNIDROIT Principles
///
/// The 2016 French reform explicitly pursued EU harmonization, making French law
/// more compatible with these European instruments.
///
/// ## Example
///
/// ```rust
/// use legalis_fr::contract::article1128;
///
/// let statute = article1128();
/// assert_eq!(statute.id, "code-civil-1128");
/// ```
#[must_use]
pub fn article1128() -> Statute {
    Statute::new(
        "code-civil-1128",
        "Code civil Article 1128 - Conditions de validité du contrat / Validity Requirements",
        Effect::new(
            EffectType::StatusChange,
            "Le contrat est valide si les trois conditions sont remplies / Contract is valid if three requirements are met",
        )
        .with_parameter("requirement_1", "Consentement des parties / Consent of parties")
        .with_parameter("requirement_2", "Capacité de contracter / Capacity to contract")
        .with_parameter("requirement_3", "Contenu licite et certain / Lawful and certain content"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Requirement 1: Consent of the parties
    .with_precondition(Condition::AttributeEquals {
        key: "consent_given".to_string(),
        value: "true".to_string(),
    })
    // Requirement 2: Capacity to contract
    .with_precondition(Condition::And(
        Box::new(Condition::Age {
            operator: legalis_core::ComparisonOp::GreaterOrEqual,
            value: 18, // Age of majority in France
        }),
        Box::new(Condition::AttributeEquals {
            key: "not_under_guardianship".to_string(),
            value: "true".to_string(),
        }),
    ))
    // Requirement 3: Lawful and certain content
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "content_lawful".to_string(),
            value: "true".to_string(),
        }),
        Box::new(Condition::AttributeEquals {
            key: "content_certain".to_string(),
            value: "true".to_string(),
        }),
    ))
    .with_discretion(
        "L'article 1128 énonce les trois conditions de validité du contrat depuis la réforme de 2016. \
        Ces conditions sont cumulatives : le contrat est nul si l'une d'elles fait défaut. \
        \n\n1° Le consentement doit être libre et éclairé (Articles 1113-1122). \
        Il peut être vicié par l'erreur (1132+), le dol (1137+), ou la violence (1140+). \
        \n\n2° La capacité est la règle, l'incapacité l'exception (Article 1145). \
        Les mineurs non émancipés et les majeurs protégés ont une capacité restreinte. \
        \n\n3° Le contenu doit être licite (conforme à l'ordre public et aux bonnes mœurs) \
        et certain (déterminé ou déterminable). \
        \n\nArticle 1128 sets out the three validity requirements for contracts since the 2016 reform. \
        These requirements are cumulative: the contract is null if any is missing. \
        \n\n1° Consent must be free and informed. It can be vitiated by error, fraud, or duress. \
        \n\n2° Capacity is the rule, incapacity the exception. Unemancipated minors and protected adults have limited capacity. \
        \n\n3° Content must be lawful (compliant with public order and morals) and certain (determined or determinable). \
        \n\n【比較法的考察】\n\
        2016年改正前のフランス民法は、日本民法と同様に「目的」(objet)と「原因」(cause)を\
        要件としていたが、改正により「適法かつ確定的な内容」という単一の要件に統合された。\
        これにより、契約成立要件が3つに簡素化され、より現代的な表現となった。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article1128_creation() {
        let statute = article1128();
        assert_eq!(statute.id, "code-civil-1128");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert_eq!(statute.version, 1);
        assert_eq!(statute.effect.effect_type, EffectType::StatusChange);
    }

    #[test]
    fn test_article1128_three_requirements() {
        let statute = article1128();
        // Should have 3 preconditions (one for each requirement)
        assert_eq!(statute.preconditions.len(), 3);

        // Check effect parameters mention all three requirements
        let params = &statute.effect.parameters;
        assert!(params.contains_key("requirement_1"));
        assert!(params.contains_key("requirement_2"));
        assert!(params.contains_key("requirement_3"));
    }

    #[test]
    fn test_article1128_validation() {
        let statute = article1128();
        assert!(statute.is_valid());
        assert_eq!(statute.validate().len(), 0);
    }

    #[test]
    fn test_article1128_has_discretion() {
        let statute = article1128();
        assert!(statute.discretion_logic.is_some());

        let discretion = statute.discretion_logic.unwrap();
        assert!(discretion.contains("trois conditions"));
        assert!(discretion.contains("three"));
    }

    #[test]
    fn test_article1128_capacity_requirement() {
        let statute = article1128();

        // Second precondition should check capacity (age >= 18 AND not under guardianship)
        if let Condition::And(left, right) = &statute.preconditions[1] {
            assert!(matches!(**left, Condition::Age { .. }));
            assert!(matches!(**right, Condition::AttributeEquals { .. }));
        } else {
            panic!("Expected And condition for capacity");
        }
    }
}
