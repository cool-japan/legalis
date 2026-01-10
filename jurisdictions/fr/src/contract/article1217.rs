//! Article 1217 - Breach Remedies (Sanctions de l'inexécution)
//!
//! Implementation of Code civil Article 1217 (2016 reform).

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 1217 - Breach Remedies (Sanctions de l'inexécution)
///
/// ## French Text (2016 version)
///
/// > La partie envers laquelle l'engagement n'a pas été exécuté, ou l'a été imparfaitement,
/// > peut :
/// >
/// > – refuser d'exécuter ou suspendre l'exécution de sa propre obligation ;
/// > – poursuivre l'exécution forcée en nature de l'obligation ;
/// > – obtenir une réduction du prix ;
/// > – provoquer la résolution du contrat ;
/// > – demander réparation des conséquences de l'inexécution.
/// >
/// > Les sanctions qui ne sont pas incompatibles peuvent être cumulées ; des dommages et intérêts
/// > peuvent toujours s'y ajouter.
///
/// ## English Translation
///
/// > The party to whom the obligation has not been performed, or has been performed imperfectly,
/// > may:
/// >
/// > – refuse to perform or suspend performance of its own obligation;
/// > – pursue specific performance of the obligation;
/// > – obtain a price reduction;
/// > – cause termination of the contract;
/// > – seek reparation for consequences of non-performance.
/// >
/// > Sanctions that are not incompatible may be cumulated; damages may always be added.
///
/// ## Historical Context and Evolution
///
/// ### Pre-2016 Code Civil: Scattered Remedies
///
/// Before 2016, French contract law lacked a comprehensive provision listing all available
/// remedies for breach. Remedies were scattered across numerous articles:
/// - Former Articles 1142-1144 (specific performance)
/// - Former Article 1184 (judicial termination)
/// - Former Articles 1146-1147 (damages)
/// - Exception of non-performance (jurisprudential creation, not codified)
/// - Price reduction (not explicitly codified for general contracts)
///
/// ### 2016 Reform Innovation: Comprehensive Remedies Menu
///
/// Article 1217 represents one of the 2016 reform's major innovations. For the first time,
/// French law provides a single provision systematically listing all available remedies
/// for contract breach. This addresses criticism that French contract law was fragmented
/// and difficult to navigate.
///
/// **Reform objectives for Article 1217:**
/// 1. **Systematic organization**: Consolidate scattered remedies into single provision
/// 2. **Codification**: Enshrine jurisprudential remedies (exception d'inexécution, price reduction)
/// 3. **Flexibility**: Allow creditor to choose appropriate remedy
/// 4. **Cumulation**: Expressly authorize combining compatible remedies
/// 5. **EU harmonization**: Align with PECL/DCFR hierarchical remedies structure
///
/// ### Roman Law Foundations: Actio and Exceptio
///
/// **Roman law distinguished:**
/// - **Actio** (action): Offensive remedies seeking performance or compensation
///   - Actio ex contractu: Action for contract breach
/// - **Exceptio** (exception): Defensive remedies suspending performance
///   - Exceptio non adimpleti contractus: Exception of non-performed contract
///
/// French Article 1217 preserves this Roman law distinction between offensive remedies
/// (specific performance, termination, damages) and defensive remedy (exception d'inexécution).
///
/// ### Domat and Pothier: Development of Remedies Doctrine
///
/// **Jean Domat (1625-1696)**: Emphasized specific performance (exécution en nature) as
/// primary remedy, reflecting civil law's preference for actual performance over monetary
/// substitutes. Damages seen as fallback when specific performance impossible.
///
/// **Robert-Joseph Pothier (1699-1772)**: Developed systematic treatment of remedies,
/// including:
/// - **Résolution** (termination): For synallagmatic contracts when one party breaches
/// - **Dommages-intérêts** (damages): Compensation for harm caused by breach
/// - **Exception non adimpleti contractus**: Suspension of performance pending other party's performance
///
/// Pothier's framework directly influenced the Napoleonic Code and remains visible in
/// Article 1217's structure.
///
/// ### Napoleonic Code (1804): Remedies Structure
///
/// The 1804 Code civil contained remedies provisions but lacked systematic organization:
/// - **Articles 1142-1144**: Specific performance (obligations to do/not to do)
/// - **Article 1184**: Judicial termination (résolution judiciaire) for synallagmatic contracts
/// - **Articles 1146-1147**: Contractual damages
/// - **No codified exception d'inexécution**: Courts developed this remedy jurisprudentially
/// - **No codified price reduction**: Existed for sale contracts (vices cachés) but not generally
///
/// ### Cour de Cassation Jurisprudence (1804-2016)
///
/// **Key developments:**
///
/// **1. Exception d'inexécution (19th century onwards)**
/// - Courts recognized right to suspend performance when other party breaches
/// - Required proportionality between breach and suspension
/// - Based on synallagmatic contract interdependence
/// - Leading case: Cass. civ., 11 mai 1808
///
/// **2. Unilateral termination (20th-21st century)**
/// - Traditionally, termination required judicial authorization (Article 1184)
/// - Courts gradually recognized unilateral termination for serious breach with formal notice
/// - Leading case: Cass. civ. 1re, 13 oct. 1998 (recognizing unilateral termination)
/// - Codified in 2016 Article 1226
///
/// **3. Price reduction (21st century)**
/// - Courts applied price reduction to general contracts, not just sale contracts
/// - Seen as proportionate remedy for partial performance
/// - 2016 reform codified in Article 1223
///
/// ### 2016 Reform Objectives for Remedies
///
/// 1. **Accessibility**: Make French contract law easier to understand and apply
/// 2. **Completeness**: Provide comprehensive remedies menu in single provision
/// 3. **Flexibility**: Creditor chooses remedy appropriate to circumstances
/// 4. **Efficiency**: Unilateral remedies (exception, price reduction, termination) reduce litigation costs
/// 5. **EU Harmonization**: Align with PECL (Article 8:101), DCFR (III.-3:101), CISG (Article 45)
///
/// ## The Five Remedies: Detailed Analysis
///
/// ### Remedy 1: Exception of Non-Performance (Exception d'inexécution) - Article 1219
///
/// **Definition**: Right to refuse or suspend own performance when other party fails to perform.
///
/// **Historical basis**: Roman law exceptio non adimpleti contractus. Recognized by French
/// courts since 1808 but not codified until 2016.
///
/// **Requirements:**
/// 1. **Synallagmatic contract**: Mutual obligations (Article 1219, al. 1)
/// 2. **Non-performance by other party**: Total or partial breach
/// 3. **Proportionality**: Suspension must be proportionate to breach (Article 1219, al. 2)
/// 4. **Good faith**: Cannot be abusive exercise (Article 1104)
///
/// **Procedure:**
/// - **Self-help remedy**: No court authorization needed
/// - **Notification**: Should notify other party of suspension
/// - **Reversibility**: Performance resumes if other party performs
///
/// **Example scenarios:**
///
/// **Valid suspension**: Buyer refuses to pay purchase price when seller fails to deliver goods.
/// - Proportionate: price corresponds to goods
///
/// **Invalid suspension**: Buyer refuses to pay €100,000 because seller delayed delivery by 1 day
/// causing €100 loss.
/// - Disproportionate: suspension (€100,000) exceeds breach significance
///
/// **Strategy**: Exception d'inexécution used as leverage to compel performance. Less drastic
/// than termination; preserves contractual relationship.
///
/// ### Remedy 2: Specific Performance (Exécution forcée en nature) - Articles 1221-1222
///
/// **Definition**: Compel debtor to perform obligation in kind (actual performance, not damages).
///
/// **Historical principle**: Civil law systems prefer specific performance over damages,
/// reflecting obligation's binding force (Article 1103). Debtor must perform what promised,
/// not substitute with money.
///
/// **Article 1221 - General Right:**
/// > Le créancier d'une obligation peut [...] en poursuivre l'exécution en nature
/// > sauf s'il est impossible ou s'il existe une disproportion manifeste entre son coût
/// > pour le débiteur et son intérêt pour le créancier.
///
/// (Creditor may pursue specific performance unless impossible or manifestly disproportionate
/// cost for debtor compared to creditor's interest.)
///
/// **Exceptions (Article 1221):**
/// 1. **Impossibility** (impossibilité): Physical, legal, or practical impossibility
///    - Example: Building destroyed by fire; reconstruction would require new contract
///
/// 2. **Manifest disproportion** (disproportion manifeste): Cost for debtor >> benefit for creditor
///    - Example: €500,000 to repair defect providing €10,000 benefit
///    - Court balances costs and benefits
///
/// **Article 1222 - Obligations to Do (Obligations de faire):**
/// > Après mise en demeure, le créancier peut aussi demander que l'obligation soit exécutée
/// > aux dépens du débiteur.
///
/// (After formal notice, creditor may have obligation performed at debtor's expense.)
///
/// **Remplacement** (replacement performance):
/// - Creditor hires third party to perform
/// - Debtor pays cost
/// - Example: Contractor fails to complete construction; client hires another contractor;
///   original contractor pays
///
/// **Personal obligations exception**: Obligations intuitu personae (personal services
/// requiring specific person's performance) cannot be specifically enforced.
/// - Example: Cannot compel artist to paint contracted portrait
/// - Remedy: Damages only
///
/// **Specific performance mechanisms:**
/// - **Astreinte** (periodic penalty): Court orders debtor pay penalty for each day of non-compliance
/// - **Execution by third party**: Court authorizes creditor to hire replacement at debtor's expense
/// - **In rem execution**: Court orders transfer of property (e.g., land)
///
/// ### Remedy 3: Price Reduction (Réduction du prix) - Article 1223
///
/// **Definition**: Unilateral right to reduce purchase price proportionally to defect.
///
/// **Innovation**: Article 1223 is a major 2016 reform innovation. Previously, price
/// reduction existed for sale contracts (garantie des vices cachés, Article 1644 old)
/// but not for contracts generally.
///
/// **Article 1223:**
/// > En cas d'exécution imparfaite, le créancier peut [...] accepter une exécution
/// > imparfaite et solliciter une réduction proportionnelle du prix.
///
/// (In case of imperfect performance, creditor may accept imperfect performance and
/// obtain proportionate price reduction.)
///
/// **Requirements:**
/// 1. **Imperfect performance** (exécution imparfaite): Not total breach, but defective
/// 2. **Acceptance**: Creditor accepts defective performance (doesn't reject)
/// 3. **Proportionality**: Reduction proportionate to defect's impact on value
///
/// **Procedure:**
/// - **Unilateral**: Creditor notifies debtor of reduction (no court needed)
/// - **Good faith**: Reduction must reflect actual diminution in value
/// - **Dispute**: If debtor contests, creditor must prove proportionality in court
///
/// **Calculation methodology:**
/// - **Proportional method**: (Defect value / Full performance value) × Price
/// - **Market comparison**: Compare defective good's market value to conforming good's value
///
/// **Example scenarios:**
///
/// **Example 1 - Construction defect:**
/// - Contract: Build house for €200,000
/// - Defect: Roof improperly installed, reducing house value by 10%
/// - Price reduction: €20,000 (10% of €200,000)
///
/// **Example 2 - Machinery performance:**
/// - Contract: Machine produces 100 units/hour for €500,000
/// - Defect: Machine produces only 80 units/hour (20% shortfall)
/// - Price reduction: €100,000 (20% of €500,000)
///
/// **Strategic advantages:**
/// - Preserves contract (doesn't terminate)
/// - Avoids litigation costs (unilateral)
/// - Faster than judicial remedies
/// - Suitable for partial defects where termination disproportionate
///
/// ### Remedy 4: Termination (Résolution) - Articles 1224-1230
///
/// **Definition**: Retroactive dissolution of contract for serious breach.
///
/// **Historical evolution:**
/// - **1804 Article 1184**: Judicial termination only (court authorization required)
/// - **20th century jurisprudence**: Courts began recognizing unilateral termination for serious breach
/// - **2016 Article 1224**: Codifies multiple termination modes
///
/// **Article 1224 - Three Termination Modes:**
/// > La résolution résulte soit de l'application d'une clause résolutoire soit, en cas
/// > d'inexécution suffisamment grave, d'une notification du créancier au débiteur ou d'une
/// > décision de justice.
///
/// (Termination results from: 1) termination clause, 2) creditor's notification for
/// sufficiently serious breach, or 3) judicial decision.)
///
/// **Mode 1: Termination Clause (Clause résolutoire) - Article 1225**
/// - Parties stipulate contract terminates automatically upon specified breach
/// - **Procedure**: Formal notice (mise en demeure) required; termination if not remedied
/// - **Effect**: Automatic upon expiration of cure period
/// - **Advantage**: Avoids judicial intervention
///
/// **Mode 2: Unilateral Termination (Résolution unilatérale) - Article 1226**
/// - **Requirements**:
///   - **Serious breach** (inexécution suffisamment grave): Material, substantial failure
///   - **Formal notice** (mise en demeure): Demand performance within reasonable time
///   - **Non-remedy**: Debtor fails to cure within notice period
/// - **Procedure**: Creditor notifies debtor of termination
/// - **Risk**: If court finds breach not serious enough, creditor liable for wrongful termination
///
/// **Mode 3: Judicial Termination (Résolution judiciaire) - Article 1227**
/// - Creditor petitions court for termination
/// - **Advantages**: Court confirms breach sufficiently serious; no wrongful termination risk
/// - **Disadvantages**: Slower, more expensive than unilateral termination
/// - **Judicial discretion**: Court may refuse termination if breach insufficient, grant damages instead
///
/// **"Sufficiently Serious Breach" (Inexécution suffisamment grave) Standard:**
///
/// Courts consider:
/// 1. **Importance of breached obligation**: Essential vs. accessory
/// 2. **Extent of non-performance**: Total vs. partial
/// 3. **Impact on creditor**: Frustrates contract's purpose?
/// 4. **Debtor's behavior**: Intentional, negligent, or excusable?
///
/// **Examples:**
/// - **Serious**: Seller delivers 50% of contracted goods
/// - **Serious**: Landlord fails to provide habitable premises (essential obligation)
/// - **Not serious**: Seller delays delivery by 2 days, causing minimal loss
/// - **Not serious**: Minor cosmetic defects not affecting functionality
///
/// **Effects of Termination (Articles 1229-1230):**
///
/// **Article 1229 - Restitution:**
/// > Les restitutions ont lieu dans l'état où les choses se trouvent.
///
/// - **Retroactive effect**: Contract deemed never existed (ex tunc)
/// - **Restitution**: Parties return performances received
/// - **Depreciation**: Party retaining thing compensates for diminution in value
///
/// **Exceptions to restitution:**
/// - **Executed contracts**: Employment, rental (time cannot be returned); only prospective termination
/// - **Consumption**: Consumed goods valued and compensated
///
/// **Article 1230 - Cumulation with damages:**
/// Termination may be cumulated with damages for harm caused by breach.
///
/// ### Remedy 5: Damages (Dommages-intérêts) - Articles 1231-1231-7
///
/// **Definition**: Monetary compensation for harm caused by breach.
///
/// **Universal remedy**: Damages can always be added to other remedies (Article 1217, al. 2).
///
/// **Detailed analysis**: See Article 1231 documentation for comprehensive treatment.
///
/// **Key principles:**
/// - **Compensatory nature** (fonction réparatrice): Restore creditor to position if contract performed
/// - **Full compensation** (réparation intégrale): Actual loss + lost profits
/// - **Foreseeability limit** (Article 1231-3): Only foreseeable damages recoverable (except fraud/gross fault)
/// - **Duty to mitigate** (obligation de minimiser): Creditor must take reasonable steps to limit loss
///
/// **Calculation methodology:**
/// 1. **Damnum emergens** (perte éprouvée): Actual loss suffered
/// 2. **Lucrum cessans** (gain manqué): Lost profits/opportunities
///
/// ## Cumulation of Remedies: Detailed Rules
///
/// Article 1217, al. 2 provides:
/// > Les sanctions qui ne sont pas incompatibles peuvent être cumulées ; des dommages et
/// > intérêts peuvent toujours s'y ajouter.
///
/// (Remedies that are not incompatible may be cumulated; damages may always be added.)
///
/// ### Compatible Cumulations:
///
/// **1. Termination + Damages** (Most common)
/// - Terminate contract AND receive damages for loss caused by breach
/// - Example: Buyer terminates purchase contract + receives damages for lost profits
///
/// **2. Specific Performance + Damages** (For delay)
/// - Obtain performance AND damages for harm caused by delay
/// - Example: Seller delivers goods late; buyer receives goods + damages for delay losses
///
/// **3. Price Reduction + Damages** (For additional harm)
/// - Reduce price for defective performance + damages for consequential harm
/// - Example: Defective machine; reduce price + damages for production losses
///
/// **4. Exception d'inexécution + Damages**
/// - Suspend performance + damages for harm caused by other party's breach
/// - Example: Buyer suspends payment + damages for cost of finding substitute goods
///
/// ### Incompatible Cumulations:
///
/// **1. Specific Performance + Termination** ❌
/// - **Logical incompatibility**: Cannot simultaneously keep contract (specific performance)
///   and dissolve it (termination)
/// - Must choose one or the other
///
/// **2. Price Reduction + Termination** ❌
/// - **Logical incompatibility**: Price reduction presumes contract continues; termination
///   dissolves contract
/// - If defect serious enough for termination, choose termination + damages
/// - If defect not serious, choose price reduction (± damages)
///
/// **3. Price Reduction + Specific Performance** ❌ (Generally)
/// - **Logical tension**: Price reduction accepts defective performance; specific performance
///   demands conforming performance
/// - Exception: If specific performance repairs defect, then price reduction for interim loss
///
/// ### Damages: Universal Additive Remedy
///
/// Article 1217 expressly provides damages can **always** be added to other remedies.
/// This reflects damages' compensatory function: even when non-monetary remedy granted,
/// creditor entitled to compensation for residual harm.
///
/// **Examples:**
/// - **Termination + Damages**: Contract dissolved + compensation for loss of bargain
/// - **Specific Performance + Damages**: Late performance + compensation for delay harm
/// - **Price Reduction + Damages**: Reduced price + compensation for consequential losses
///   (e.g., production downtime)
///
/// ## Modern Applications and Contemporary Examples
///
/// ### E-Commerce Breach Remedies
///
/// **Online marketplace disputes:**
/// - **Exception d'inexécution**: Buyer suspends payment via credit card chargeback when seller fails to ship
/// - **Price reduction**: Buyer receives partial refund for defective product via platform dispute resolution
/// - **Termination**: Buyer cancels subscription service for breach (unilateral termination)
///
/// **Platform economy challenges:**
/// - **Uber/Airbnb cancellations**: Unilateral termination provisions scrutinized under consumer protection law
/// - **App store refunds**: Price reduction/termination rights for defective software
///
/// ### Smart Contracts and Blockchain
///
/// **Automated remedies:**
/// - **Smart contract escrow**: Exception d'inexécution implemented via escrow release conditions
/// - **Proportional payment**: Price reduction implemented via algorithmic valuation
/// - **Termination triggers**: Smart contract automatically terminates upon specified breach
///
/// **Legal questions:**
/// - Does smart contract "code is law" approach satisfy Article 1217's "sufficiently serious
///   breach" requirement for unilateral termination?
/// - Can algorithmic price reduction satisfy "proportionality" requirement?
///
/// ### COVID-19 Pandemic Impact
///
/// **Force majeure vs. remedies:**
/// - **Force majeure** (Article 1218): Exempts from liability; no damages
/// - **Breach remedies** (Article 1217): Applicable when no force majeure
///
/// **Common scenarios:**
/// - **Event cancellations**: Organizers claim force majeure; attendees seek termination + refunds
/// - **Supply chain disruptions**: Buyers invoke exception d'inexécution (suspend payment) when
///   suppliers delay delivery
/// - **Restaurant closures**: Landlords seek rent despite closure; tenants claim impossibility
///   (force majeure) or seek price reduction
///
/// **Leading case**: Cass. civ. 1re, 25 nov. 2020
/// - Government-ordered closure constitutes force majeure suspending lease obligations
/// - But rent obligations resume once closure lifted (no termination)
///
/// ### Construction Contract Remedies
///
/// **Common applications:**
/// - **Latent defects**: Price reduction proportionate to repair cost
/// - **Delay in completion**: Exception d'inexécution (owner withholds progress payments)
///   + damages for delay
/// - **Serious defects**: Termination + damages + remplacement (hire new contractor)
///
/// **Example**: €1 million construction contract
/// - Contractor completes but with €100,000 defects
/// - Owner's options:
///   1. **Price reduction**: Reduce price by €100,000
///   2. **Specific performance**: Demand contractor repair at contractor's expense
///   3. **Remplacement**: Hire third party to repair at contractor's expense (€120,000)
///   4. **Damages alone**: Accept defective work + €100,000 damages
///
/// ### Consumer Protection Enhancements (B2C)
///
/// **Code de la consommation overlays:**
/// - **Conformity guarantee** (garantie de conformité): Consumer may demand repair,
///   replacement, price reduction, or termination for non-conforming goods (Article L217-9)
/// - **14-day withdrawal**: Consumer may terminate without cause within 14 days (distance contracts)
/// - **Unfair terms**: Termination clauses requiring excessive notice or penalty may be void
///
/// **B2C vs. B2B differences:**
/// - **B2C**: Lower "sufficiently serious breach" threshold for consumer termination
/// - **B2B**: Higher threshold; courts presume commercial parties can bear minor breaches
///
/// ### International Sales (CISG Application)
///
/// **CISG Article 45** (similar to French Article 1217):
/// Buyer may:
/// - Require performance (specific performance)
/// - Fix additional period for performance
/// - Declare contract avoided (termination)
/// - Claim damages
///
/// **Key difference from French law:**
/// - **CISG**: "Fundamental breach" required for termination (Article 25)
/// - **French**: "Sufficiently serious breach" (inexécution suffisamment grave)
/// - CISG standard generally higher threshold
///
/// French courts applying CISG to international sales must apply CISG's fundamental
/// breach standard, not Article 1217's sufficiently serious breach standard.
///
/// ## Case Law Examples (Leading Decisions)
///
/// ### Exception of Non-Performance
///
/// **1. Recognition of Exception d'inexécution (Cass. civ., 11 mai 1808)**
/// - **Facts**: Party suspended performance when other party failed to perform
/// - **Holding**: Court recognized right to suspend based on interdependence of synallagmatic obligations
/// - **Significance**: First recognition of exception d'inexécution principle; later codified in Article 1219
///
/// **2. Proportionality Requirement (Cass. com., 21 janv. 1997, n°95-11.593)**
/// - **Facts**: Party suspended all performance for minor breach
/// - **Holding**: Exception d'inexécution must be proportionate to other party's breach
/// - **Significance**: Established proportionality requirement now codified in Article 1219, al. 2
///
/// ### Specific Performance vs. Damages
///
/// **3. Primacy of Specific Performance (Cass. civ. 1re, 16 janv. 2007, n°04-20.218)**
/// - **Facts**: Creditor sought damages instead of specific performance
/// - **Holding**: Creditor entitled to choose, but civil law tradition favors specific performance
/// - **Significance**: Reaffirms civil law preference for performance over damages
///
/// **4. Manifest Disproportion Exception (Cass. civ. 3e, 11 mai 2005, n°03-21.136)**
/// - **Facts**: Repair cost €80,000 for defect reducing value by €15,000
/// - **Holding**: Specific performance denied; manifestly disproportionate cost
/// - **Significance**: Applied manifest disproportion exception now codified in Article 1221
///
/// ### Price Reduction
///
/// **5. Price Reduction for General Contracts (Cass. civ. 1re, 28 oct. 2003, n°01-03.662)**
/// - **Facts**: Buyer sought price reduction for defective service (not sale contract)
/// - **Holding**: Price reduction available for general contracts, not just sales
/// - **Significance**: Extended price reduction beyond sales contracts; codified in Article 1223
///
/// ### Unilateral Termination
///
/// **6. Recognition of Unilateral Termination (Cass. civ. 1re, 13 oct. 1998, n°96-21.485)**
/// - **Facts**: Creditor terminated contract unilaterally after formal notice for serious breach
/// - **Holding**: Unilateral termination valid if breach sufficiently serious after formal notice
/// - **Significance**: Landmark decision recognizing unilateral termination; codified in Article 1226
///
/// **7. "Sufficiently Serious Breach" Standard (Cass. civ. 1re, 28 oct. 2003, n°01-13.018)**
/// - **Facts**: Seller delivered 60% of contracted goods
/// - **Holding**: Breach sufficiently serious to justify unilateral termination
/// - **Significance**: Established "sufficiently serious" standard for unilateral termination
///
/// **8. Wrongful Termination Liability (Cass. civ. 1re, 20 févr. 2001, n°98-22.913)**
/// - **Facts**: Creditor terminated for breach court found not sufficiently serious
/// - **Holding**: Creditor liable for wrongful termination; must compensate debtor
/// - **Significance**: Unilateral termination risk: if breach insufficient, terminating party liable
///
/// ### Cumulation of Remedies
///
/// **9. Termination + Damages (Cass. civ. 1re, 9 nov. 1999, n°97-19.793)**
/// - **Facts**: Buyer terminated contract and sought damages for breach
/// - **Holding**: Termination and damages compatible; may be cumulated
/// - **Significance**: Confirmed cumulation principle now codified in Article 1217, al. 2
///
/// **10. Incompatibility of Specific Performance + Termination (Cass. civ. 1re, 7 oct. 1998)**
/// - **Facts**: Creditor sought both specific performance and termination
/// - **Holding**: Incompatible remedies; must choose one
/// - **Significance**: Logical incompatibility prevents cumulation
///
/// ### Damages as Universal Additive
///
/// **11. Damages Always Available (Cass. civ. 1re, 15 déc. 2011, n°10-25.775)**
/// - **Facts**: Court granted specific performance; creditor also sought damages for delay
/// - **Holding**: Damages may always be added to other remedies
/// - **Significance**: Damages' universal availability codified in Article 1217, al. 2
///
/// ## International and Comparative Law Analysis
///
/// ### Germany (BGB §§ 275-326)
///
/// **Remedies hierarchy:**
/// - **Primary**: Specific performance (Nacherfüllung) - BGB § 281
/// - **Secondary**: Withdrawal (Rücktritt) and damages (Schadensersatz) - BGB § 323
///
/// **Key differences from French law:**
/// 1. **Mandatory sequence**: German law requires creditor first demand performance (Nachfrist),
///    then may withdraw or seek damages. French law allows immediate choice among remedies.
/// 2. **Impossibility**: BGB § 275 excuses performance if impossible or disproportionate.
///    French Article 1221 similar but less detailed.
/// 3. **Price reduction**: BGB § 441 (Minderung) only for sale/service contracts. French
///    Article 1223 applies to all contracts.
/// 4. **No exception d'inexécution**: German law lacks express codification; recognized
///    jurisprudentially based on good faith (Treu und Glauben, § 242)
///
/// ### Japan (Minpō §§ 415-548)
///
/// **2017 reform parallels:**
/// Japan's 2017 Civil Code reform parallels France's 2016 reform, including comprehensive
/// remedies provisions:
///
/// - **Specific performance** (履行の強制): Article 414 (similar to French Article 1221)
/// - **Termination** (契約の解除): Article 541-543 (similar to French Articles 1224-1230)
/// - **Damages** (損害賠償): Article 415 (similar to French Article 1231)
///
/// **Key differences:**
/// 1. **No price reduction**: Japanese law lacks general price reduction remedy
///    (available only for sale contracts with defects)
/// 2. **No exception d'inexécution**: Not codified; recognized as "simultaneous performance defense"
///    (同時履行の抗弁, Article 533)
/// 3. **Termination requirement**: Requires formal notice (催告) with reasonable period;
///    similar to French mise en demeure
///
/// **Similarities:**
/// - Both prefer specific performance over damages (civil law tradition)
/// - Both require "sufficiently serious breach" for termination (Japan: Article 541;
///   France: Article 1224)
///
/// ### United States (UCC, Restatement 2d)
///
/// **Common law approach - Damages primary:**
///
/// **Remedies hierarchy:**
/// 1. **Primary**: Damages (expectation, reliance, restitution)
/// 2. **Secondary**: Specific performance (exceptional remedy)
///
/// **Key differences from French law:**
/// 1. **Damages preference**: US prefers damages over specific performance (opposite of French)
///    - Rationale: Damages provide adequate remedy; courts avoid supervising performance
/// 2. **Specific performance**: Only when damages inadequate (unique goods, land)
///    - Restatement (Second) § 359: Specific performance for unique goods
///    - French law: Specific performance is default remedy
/// 3. **Perfect tender rule** (UCC § 2-601): Buyer may reject goods for any nonconformity
///    - Much lower threshold than French "sufficiently serious breach"
/// 4. **Cover and resale** (UCC §§ 2-712, 2-706): Buyer covers; seller resells
///    - Similar to French price reduction but through market substitutes
///
/// **Exception of non-performance**: US has "anticipatory repudiation" (UCC § 2-609)
/// allowing suspension when reasonable insecurity about performance. Similar to French
/// exception d'inexécution but forward-looking.
///
/// ### United Kingdom (Common Law)
///
/// **Remedies:**
/// 1. **Damages**: Primary remedy (expectation, reliance, restitution)
/// 2. **Specific performance**: Equitable remedy, discretionary
/// 3. **Termination** ("repudiation"): For breach of condition or fundamental breach
///
/// **Key differences from French law:**
/// 1. **Conditions vs. warranties**: UK distinguishes "conditions" (material terms;
///    breach allows termination) from "warranties" (minor terms; breach allows only damages).
///    French law uses single "sufficiently serious breach" standard.
/// 2. **Specific performance**: Equity courts grant specific performance only when damages
///    inadequate and performance feasible. French courts grant specific performance as default.
/// 3. **Termination**: UK requires "repudiation" (renunciation of contract) or breach of
///    condition. French requires "sufficiently serious breach" (similar threshold).
/// 4. **No price reduction**: UK lacks general price reduction remedy (only for sale of
///    goods with defects)
///
/// ### CISG (Vienna Convention on International Sale of Goods)
///
/// **Article 45 - Buyer's Remedies (similar to French Article 1217):**
/// - Require performance (Article 46)
/// - Fix additional period (Nachfrist) (Article 47)
/// - Declare contract avoided (Article 49) - termination
/// - Reduce price (Article 50)
/// - Claim damages (Article 74)
///
/// **Key features:**
/// 1. **Fundamental breach**: Article 25 requires "fundamental breach" for termination
///    (similar to French "sufficiently serious breach" but generally higher threshold)
/// 2. **Price reduction**: Article 50 provides proportional price reduction for non-conforming goods
///    - Influenced French Article 1223
/// 3. **Nachfrist**: Germanic concept allowing buyer to fix additional period for performance
///    - French law has similar mise en demeure (formal notice)
/// 4. **No exception of non-performance**: CISG Article 71 allows suspension for anticipated
///    breach (anticipatory insecurity)
///
/// **Harmonization**: CISG influenced 2016 French reform, particularly Article 1223 price
/// reduction modeled on CISG Article 50.
///
/// ### UNIDROIT Principles of International Commercial Contracts
///
/// **Article 7.3.1 - Right to Performance:**
/// Creditor entitled to performance unless:
/// - Performance impossible
/// - Performance unreasonably burdensome
/// - Creditor may reasonably obtain performance elsewhere
///
/// **Article 7.3.2 - Right to Withhold Performance (Exception):**
/// Party may withhold performance if other party has not performed and non-performance
/// is fundamental. Similar to French exception d'inexécution but requires fundamental breach.
///
/// **Article 7.3.3 - Termination:**
/// May terminate for fundamental non-performance. Similar to French "sufficiently serious breach."
///
/// **Article 7.4.1 - Right to Damages:**
/// Non-performance gives right to damages. Cumulative with other remedies.
///
/// **Convergence with French law:**
/// UNIDROIT Principles closely resemble French Article 1217:
/// - Comprehensive remedies menu
/// - Specific performance as primary remedy (civil law approach)
/// - Termination for serious breach
/// - Cumulation of compatible remedies
///
/// ### China (Contract Law 1999, Civil Code 2020)
///
/// **Remedies (Civil Code Chapter 3, Section 7):**
/// - **Continued performance** (继续履行): Article 580 (specific performance)
/// - **Remedial measures** (采取补救措施): Article 582 (repair, replacement)
/// - **Contract termination** (解除合同): Article 563
/// - **Damages** (损害赔偿): Article 584
///
/// **Key features:**
/// 1. **Specific performance preference**: Article 580 prioritizes continued performance
///    (civil law tradition, similar to French)
/// 2. **Termination**: Requires "material breach" or failure to perform after notice
///    (similar to French "sufficiently serious breach")
/// 3. **Price reduction**: Not explicitly codified as general remedy (unlike French Article 1223)
/// 4. **Simultaneous performance defense** (同时履行抗辩权): Article 525 (similar to French
///    exception d'inexécution)
///
/// **Evolution**: China's 2020 Civil Code shows convergence toward Western civil law
/// principles, particularly French model of comprehensive remedies.
///
/// ### European Contract Law Harmonization
///
/// **PECL (Principles of European Contract Law) Article 8:101:**
/// > Remedies available are:
/// > (a) enforce specific performance;
/// > (b) withhold performance;
/// > (c) terminate the contract;
/// > (d) price reduction;
/// > (e) damages.
///
/// **DCFR (Draft Common Frame of Reference) III.-3:101:**
/// Similar remedies menu.
///
/// **Influence on French reform:**
/// Article 1217 directly inspired by PECL/DCFR comprehensive remedies structure. The 2016
/// reform explicitly pursued EU harmonization, making French law compatible with these
/// European instruments.
///
/// ## Example
///
/// ```rust
/// use legalis_fr::contract::article1217;
///
/// let statute = article1217();
/// assert_eq!(statute.id, "code-civil-1217");
/// ```
#[must_use]
pub fn article1217() -> Statute {
    Statute::new(
        "code-civil-1217",
        "Code civil Article 1217 - Sanctions de l'inexécution / Breach Remedies",
        Effect::new(
            EffectType::Grant,
            "Droit de choisir parmi les sanctions / Right to choose among remedies",
        )
        .with_parameter("remedy_1", "Exception d'inexécution / Exception of non-performance (Art. 1219)")
        .with_parameter("remedy_2", "Exécution forcée en nature / Specific performance (Art. 1221)")
        .with_parameter("remedy_3", "Réduction du prix / Price reduction (Art. 1223)")
        .with_parameter("remedy_4", "Résolution du contrat / Termination (Art. 1224)")
        .with_parameter("remedy_5", "Dommages-intérêts / Damages (Art. 1231)")
        .with_parameter("cumulation", "Les sanctions compatibles peuvent être cumulées / Compatible remedies may be cumulated"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    // Precondition: Non-performance or imperfect performance
    .with_precondition(Condition::Or(
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "non_performance".to_string(),
                value: "true".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "imperfect_performance".to_string(),
                value: "true".to_string(),
            }),
        )),
        Box::new(Condition::AttributeEquals {
            key: "delayed_performance".to_string(),
            value: "true".to_string(),
        }),
    ))
    // The creditor is the one entitled to remedies
    .with_precondition(Condition::AttributeEquals {
        key: "is_creditor".to_string(),
        value: "true".to_string(),
    })
    .with_discretion(
        "L'article 1217 offre au créancier un éventail complet de sanctions en cas d'inexécution. \
        C'est une innovation majeure de la réforme de 2016 : les sanctions sont désormais \
        énoncées de manière claire et structurée dans un seul article. \
        \n\nLe créancier peut choisir la sanction appropriée, et même les cumuler si elles \
        sont compatibles. Les dommages-intérêts peuvent toujours s'ajouter aux autres sanctions. \
        \n\nHiérarchie des sanctions : \
        • Exception d'inexécution (1219) : sanction provisoire \
        • Exécution forcée (1221) : sanction en nature (préférée en droit civil) \
        • Réduction du prix (1223) : sanction proportionnelle \
        • Résolution (1224) : sanction radicale (grave inexécution) \
        • Dommages-intérêts (1231) : sanction pécuniaire (toujours possible) \
        \n\nArticle 1217 provides the creditor with a complete menu of remedies for breach. \
        This is a major innovation of the 2016 reform: remedies are now clearly and systematically \
        set out in a single article. \
        \n\nThe creditor may choose the appropriate remedy, and even cumulate them if compatible. \
        Damages may always be added to other remedies. \
        \n\n【比較法的考察】\n\
        フランス法は、大陸法の伝統に従い、強制履行(exécution forcée)を第一次的救済手段とする。\
        これは日本民法414条と同様である。対照的に、英米法は損害賠償を原則とし、\
        特定履行(specific performance)は例外的救済とする。\
        フランス法の特徴は、救済手段の選択肢を債権者に広く認め、かつ併用を許容する点にある。",
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article1217_creation() {
        let statute = article1217();
        assert_eq!(statute.id, "code-civil-1217");
        assert_eq!(statute.jurisdiction, Some("FR".to_string()));
        assert_eq!(statute.version, 1);
        assert_eq!(statute.effect.effect_type, EffectType::Grant);
    }

    #[test]
    fn test_article1217_five_remedies() {
        let statute = article1217();

        let params = &statute.effect.parameters;
        assert!(params.contains_key("remedy_1")); // Exception
        assert!(params.contains_key("remedy_2")); // Specific performance
        assert!(params.contains_key("remedy_3")); // Price reduction
        assert!(params.contains_key("remedy_4")); // Termination
        assert!(params.contains_key("remedy_5")); // Damages
        assert!(params.contains_key("cumulation")); // Cumulation rule
    }

    #[test]
    fn test_article1217_preconditions() {
        let statute = article1217();
        // Should have 2 preconditions
        assert_eq!(statute.preconditions.len(), 2);

        // First: non-performance OR imperfect OR delayed
        assert!(matches!(statute.preconditions[0], Condition::Or(_, _)));

        // Second: is_creditor
        assert!(matches!(
            statute.preconditions[1],
            Condition::AttributeEquals { .. }
        ));
    }

    #[test]
    fn test_article1217_validation() {
        let statute = article1217();
        assert!(statute.is_valid());
        assert_eq!(statute.validate().len(), 0);
    }

    #[test]
    fn test_article1217_has_discretion() {
        let statute = article1217();
        assert!(statute.discretion_logic.is_some());

        let discretion = statute.discretion_logic.unwrap();
        assert!(discretion.contains("sanctions"));
        assert!(discretion.contains("remedies"));
    }
}
