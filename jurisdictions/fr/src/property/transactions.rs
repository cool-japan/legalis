//! Real estate transactions articles (Code civil, Book II)
//!
//! This module implements articles governing real estate transactions in French property law,
//! including property classification, long-term leases (bail emphytéotique), and sales formalities.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 490 - Classification of property (immovable vs. movable)
///
/// # French Text
/// "Les biens sont immeubles, ou par leur nature, ou par leur destination, ou par l'objet
/// auquel ils s'appliquent. Les fonds de terre et les bâtiments sont immeubles par leur nature.
/// Les meubles sont par leur nature ceux qui peuvent se transporter d'un lieu à un autre,
/// soit qu'ils se meuvent par eux-mêmes, comme les animaux, soit qu'ils ne puissent changer
/// de place que par l'effet d'une force étrangère, comme les choses inanimées."
///
/// # English Translation
/// "Property is immovable, either by its nature, by its destination, or by the object to which
/// it applies. Land and buildings are immovable by their nature. Movable property by its nature
/// is that which can be transported from one place to another, whether moving by themselves,
/// like animals, or whether they can only change place by the effect of an external force,
/// like inanimate things."
///
/// # Legal Commentary
/// Article 490 establishes the **fundamental classification** of property in French law,
/// distinguishing between **immovables** (real property) and **movables** (personal property).
/// This distinction has profound legal consequences for taxation, transfer formalities,
/// inheritance, and creditor rights.
///
/// ## Four Categories of Property
///
/// ### 1. Immovables by Nature (Immeubles par Nature)
/// **Inherently immovable** property that cannot be moved without destruction:
/// - **Land** (fonds de terre): Soil, earth, terrain
/// - **Buildings** (bâtiments): Houses, apartments, commercial structures
/// - **Standing crops** (récoltes sur pied): Unharvested wheat, grapes on vine
/// - **Trees** (arbres): Rooted trees, forests
/// - **Fixtures**: Permanently attached to land/buildings
///
/// ### 2. Immovables by Destination (Immeubles par Destination)
/// **Movable objects** treated as immovable due to their functional relationship to immovable property:
///
/// **Article 524 examples**:
/// - **Agricultural equipment**: Tractors, plows essential to farm operation
/// - **Mill machinery**: Grinding wheels permanently installed in flour mill
/// - **Hotel furnishings**: Furniture in hotel rooms integral to business
/// - **Vineyard equipment**: Wine presses, fermentation tanks for winery
/// - **Livestock**: Farm animals essential to agricultural exploitation
///
/// **Requirements**:
/// 1. **Same ownership**: Owner of land must own the movables
/// 2. **Functional connection**: Movables serve the immovable's purpose
/// 3. **Permanence**: Long-term attachment to property use
/// 4. **Intent**: Owner intended movables for property's service
///
/// ### 3. Immovables by Object (Immeubles par l'Objet)
/// **Rights** relating to immovable property:
/// - **Usufruct**: Life estate on land/buildings
/// - **Easements**: Rights of way, drainage servitudes
/// - **Mortgages**: Security interests in real property
/// - **Real estate shares**: Shares in property companies (SCI)
/// - **Construction leases**: Bail à construction (18-99 years)
/// - **Emphyteutic leases**: Bail emphytéotique (18-99 years)
///
/// ### 4. Movables by Anticipation (Meubles par Anticipation)
/// **Future separation** from immovable transforms classification:
/// - **Timber sold for cutting**: Trees become movable upon sale contract
/// - **Harvested crops**: Wheat in field (immovable) → wheat in barn (movable)
/// - **Demolished materials**: Building materials become movable when demolition contracted
/// - **Quarried stone**: Rock in quarry (immovable) → extracted stone (movable)
///
/// ## Legal Consequences of Classification
///
/// ### Transfer Formalities
/// **Immovables**:
/// - **Notarial deed** required (acte notarié, Articles 1873-1878)
/// - **Public registry** (publicité foncière) - must record in land registry
/// - **Transfer tax** (droits d'enregistrement) - typically 5-6% of value
/// - **Notary fees** (frais de notaire) - 2-3% of value
///
/// **Movables**:
/// - **Simple contract** sufficient (private agreement)
/// - **Delivery** transfers ownership (tradition)
/// - **No registration** required (except vehicles, aircraft)
/// - **Lower transfer costs** (minimal taxes)
///
/// ### Inheritance
/// **Immovables**:
/// - Subject to **forced heirship** rules (réserve héréditaire)
/// - Cannot freely dispose by will (children have protected shares)
/// - **Partage**: Complex division among heirs
/// - **Special valuation** rules for family home
///
/// **Movables**:
/// - More **flexibility** in testamentary disposition
/// - **Possession vaut titre** (Article 571) - possession as title
/// - Easier division among heirs
///
/// ### Creditor Rights
/// **Immovables**:
/// - **Mortgage** (hypothèque) - consensual security interest
/// - **Judicial mortgage** (hypothèque judiciaire) - court-ordered
/// - **Legal mortgage** (hypothèque légale) - by operation of law
/// - **Privilege**: Special creditor priorities
///
/// **Movables**:
/// - **Pledge** (gage) - physical possession by creditor
/// - **Lien** (gage sans dépossession) - registered security interest
/// - **Seizure** easier than for immovables
///
/// ### Taxation
/// **Immovables**:
/// - **Property tax** (taxe foncière) - annual local tax
/// - **Housing tax** (taxe d'habitation) - abolished for primary residences 2023
/// - **Wealth tax** (IFI - Impôt sur la Fortune Immobilière) - for high-value portfolios
/// - **Capital gains tax**: Special rates for real estate (19% + 17.2% social charges)
///
/// **Movables**:
/// - **No annual tax** (except vehicles)
/// - **Capital gains**: Different rates, exemptions
/// - **Lower transfer taxes**
///
/// ## Historical Context
/// Article 490 dates from 1804 Napoleonic Code, adopting Roman law's distinction between
/// *res mobiles* and *res immobiles*. This classification reflected agricultural society's
/// priorities: land was wealth, stability, family patrimony.
///
/// Key developments:
/// - **1804**: Original Code provisions on property classification
/// - **1855**: Jurisprudence developed immeubles par destination doctrine
/// - **1935**: Registration reform (publicité foncière modernized)
/// - **1955**: Decree on land registry (conservation des hypothèques)
/// - **2000**: Electronic land registry implementation
/// - **2020**: Digital notarial deeds authorized (Covid acceleration)
///
/// ## Modern Applications
///
/// ### Urban Context
/// **Immovables by nature**:
/// - **Condominiums**: Apartments in multi-unit buildings (copropriété)
/// - **Office buildings**: Commercial real estate
/// - **Parking spaces**: Underground parking (part of building)
///
/// **Immovables by destination**:
/// - **Hotel equipment**: Beds, TVs, kitchen equipment in hotel
/// - **Restaurant fixtures**: Commercial kitchen, dining furniture
/// - **Medical equipment**: Permanently installed in private clinic
///
/// ### Rural Context
/// **Immovables by nature**:
/// - **Agricultural land**: Fields, pastures, vineyards
/// - **Farm buildings**: Barns, silos, stables
///
/// **Immovables by destination**:
/// - **Farm machinery**: Tractors, harvesters essential to operation
/// - **Livestock**: Cattle, horses for agricultural exploitation
/// - **Irrigation systems**: Permanently installed equipment
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §90-103)**: Similar movable/immovable distinction (*Sache*/*Grundstück*)
/// - **Japan (Minpō §86-87)**: Real property vs. personal property, fixtures doctrine
/// - **Common Law**: Real property vs. personal property, fixtures test (degree of attachment)
/// - **USA**: Realty vs. personalty, UCC Article 9 for secured transactions
/// - **Spain (Código Civil Art. 333-345)**: Bienes inmuebles/muebles, similar classification
///
/// ## Practical Importance for Transactions
/// Proper classification is **critical** for:
/// - **Determining required formalities**: Notary, registration
/// - **Calculating transfer taxes**: Immovables taxed higher
/// - **Establishing priority**: Creditor security interests
/// - **Estate planning**: Inheritance, gift tax consequences
/// - **Insurance**: Different coverage for real vs. personal property
///
/// ## Immeubles par Destination Controversies
///
/// ### Modern Disputes
/// - **Solar panels**: Immovable by destination if serving building's energy needs?
/// - **Cryptocurrency mining rigs**: If essential to property's commercial purpose?
/// - **Airbnb furnishings**: Hotel analogy for short-term rental apartments?
/// - **Data centers**: Servers as immeubles par destination?
///
/// ### Traditional Rules
/// **Courts consider**:
/// 1. **Functional necessity**: Is movable essential to property's purpose?
/// 2. **Same ownership**: Does landowner own the movables?
/// 3. **Permanence**: Long-term attachment or temporary?
/// 4. **Intent**: Owner's subjective intention matters
///
/// # Examples
///
/// ```text
/// // Example 1: Immeubles par nature
/// // Land parcel: 5,000 m² with house → Immovable by nature
/// // Transfer requires: Notarial deed + land registry + 5.8% transfer tax
/// // Example: €300,000 house
/// // - Transfer tax: €17,400
/// // - Notary fees: €6,000-9,000
/// // - Total costs: ~€25,000 (8% of price)
///
/// // Example 2: Immeubles par destination
/// // Vineyard with wine press, fermentation tanks, tractors
/// // Owner sells vineyard → Wine equipment included as immeubles par destination
/// // Even though movable by nature, legally treated as part of land
/// // Buyer cannot claim equipment excluded from sale
///
/// // Example 3: Meubles par anticipation
/// // Forest owner sells timber "on the stump" (sur pied)
/// // Contract signed → Trees become meubles par anticipation
/// // Transfer as movable (simple contract, no notary)
/// // After cutting → Fully movable (timber transported)
///
/// // Example 4: Immeubles par l'objet
/// // Usufruct on apartment for life
/// // Right is immovable (relates to real property)
/// // Transfer requires notarial deed, registration
/// // Taxed as immovable transfer
/// ```
pub fn article490() -> Statute {
    Statute::new(
        "code-civil-490",
        "Code civil Article 490 - Classification des biens / Property classification (immovable vs. movable)",
        Effect::new(
            EffectType::Grant,
            "Property classified as immovable (by nature, destination, or object) or movable, \
             determining transfer formalities, taxation, and creditor rights",
        )
        .with_parameter("immovable_by_nature", "land_buildings_attached")
        .with_parameter("immovable_by_destination", "movables_serving_immovable")
        .with_parameter("immovable_by_object", "rights_relating_to_immovables")
        .with_parameter("movable_by_nature", "transportable_property")
        .with_parameter("movable_by_anticipation", "future_separation_from_land")
        .with_parameter("consequence_immovable", "notarial_deed_registration_required")
        .with_parameter("consequence_movable", "simple_contract_delivery")
        .with_parameter("origin", "roman_law_res_mobiles_immobiles"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::HasAttribute {
        key: "property".to_string(),
    })
    .with_discretion(
        "Property classification: Four categories determine legal treatment: (1) Immovables by \
         nature (land, buildings, attached fixtures) - cannot move without destruction; (2) \
         Immovables by destination (movables serving immovable, e.g., farm equipment, hotel \
         furnishings) - requires same ownership, functional connection, permanence; (3) Immovables \
         by object (rights relating to real property: usufruct, easements, mortgages); (4) Movables \
         by nature (transportable property) and anticipation (future separation from land, e.g., \
         timber sold for cutting). Legal consequences: Immovables require notarial deed, land \
         registry, high transfer tax (5-6%); movables transfer by simple contract, delivery. \
         Inheritance: immovables subject to forced heirship; creditor rights: mortgage vs. pledge. \
         Historical: 1804 Code, Roman law res mobiles/immobiles. Modern disputes: solar panels, \
         data centers, Airbnb furnishings. Compare: Germany BGB §90-103 (Sache/Grundstück), Japan \
         Minpō §86-87, Common Law fixtures test, USA realty/personalty. Critical for: transfer \
         formalities, taxes, creditor priority, estate planning."
    )
}

/// Articles 1741-1749 - Long-term leases (bail emphytéotique)
///
/// # French Text (Key Articles)
///
/// **Article 1741** (now Code rural L451-1):
/// "Le bail emphytéotique ne peut être fait pour un terme moindre de dix-huit ans, et n'est
/// susceptible de renouvellement qu'après l'expiration du délai de dix-huit ans. Il doit être
/// consenti par acte notarié et publié au bureau des hypothèques."
///
/// **Core Provisions**:
/// "L'emphytéote peut disposer de son droit, le céder, l'hypothéquer, le sous-louer, sans
/// autorisation du propriétaire. Il fait les améliorations et les plantations. Il jouit des
/// fruits naturels et industriels."
///
/// # English Translation
///
/// **Article 1741**:
/// "The emphyteutic lease cannot be made for a term less than eighteen years, and is only
/// renewable after expiration of the eighteen-year period. It must be executed by notarial
/// deed and published in the mortgage registry."
///
/// **Core Provisions**:
/// "The emphyteuta may dispose of their right, transfer it, mortgage it, sublease it, without
/// the owner's authorization. They make improvements and plantings. They enjoy natural and
/// industrial fruits."
///
/// # Legal Commentary
/// Articles 1741-1749 govern the **bail emphytéotique** (emphyteutic lease), a unique long-term
/// lease (18-99 years) granting tenant **quasi-ownership rights** over immovable property. This
/// creates a **real right** (droit réel) on property, not merely personal contract rights.
///
/// ## Defining Characteristics
///
/// ### Duration: 18-99 Years
/// **Minimum**: 18 years (statutory requirement)
/// **Maximum**: 99 years (traditional limit, though Code rural allows longer for specific purposes)
/// **Typical**: 30-50 years for private emphyteusis, 60-99 years for public projects
///
/// **No automatic renewal**: Expires at term end unless explicitly renewed
///
/// ### Real Right (Droit Réel)
/// Emphyteutic lease creates **real property right**, not just personal contract:
/// - **Transferable**: Emphyteuta may sell, gift, bequeath the lease
/// - **Mortgageable**: Can secure loans with lease as collateral
/// - **Registered**: Recorded in land registry (publicité foncière)
/// - **Opposable**: Enforceable against third parties, not just lessor
/// - **Survives sale**: If owner sells land, emphyteusis continues
///
/// ### Transferability Without Consent
/// **Emphyteuta's freedom**:
/// - **Sell lease**: Transfer to third party without lessor approval
/// - **Mortgage**: Use lease as security for financing (hypothèque)
/// - **Sublease**: Grant sub-emphyteusis or ordinary leases
/// - **Bequeath**: Pass to heirs by inheritance
///
/// Contrasts with ordinary leases requiring landlord consent for assignment.
///
/// ### Improvement Rights and Obligations
/// **Emphyteuta must**:
/// - **Improve property**: Make lasting improvements (buildings, drainage, plantings)
/// - **Maintain property**: Keep in good repair, pay for major repairs
/// - **Pay modest rent**: Below market rate (canon emphytéotique)
/// - **Not diminish value**: Cannot waste or destroy property
///
/// **Benefits at term end**:
/// - **No compensation**: Improvements become owner's property without payment (Article 1742)
/// - **Accession**: Buildings, plantations accede to land (superficie solo cedit)
/// - **Exception**: Contract may provide compensation for improvements
///
/// ## Formal Requirements
///
/// ### Notarial Deed Mandatory
/// **Acte notarié** required (Articles 1741, 1743):
/// - **Notary drafts**: Licensed notaire prepares deed
/// - **Parties execute**: Lessor and lessee sign before notary
/// - **Formal record**: Notary maintains official copy
/// - **Authentication**: Notarial signature gives legal certainty
///
/// **Penalty for non-compliance**: Invalid emphyteusis, may be ordinary lease instead
///
/// ### Land Registry Publication
/// **Publicité foncière** essential:
/// - **Registration**: File deed with land registry (bureau des hypothèques, now Service de publicité foncière)
/// - **Public notice**: Third parties can discover emphyteusis
/// - **Priority**: Registration date determines creditor priorities
/// - **Opposability**: Only enforceable against third parties after registration
///
/// ## Economic Purpose
/// Emphyteutic lease encourages **long-term investment** in property improvements:
///
/// ### Tenant's Perspective
/// **Advantages**:
/// - **Quasi-ownership**: Control property for decades
/// - **Build equity**: Lease itself gains value with improvements
/// - **Financing**: Can mortgage lease for construction loans
/// - **Commercial use**: Develop property for business purposes
///
/// **Disadvantages**:
/// - **No compensation**: Improvements revert to owner without payment (unless contracted)
/// - **Long commitment**: 18+ year obligation
/// - **Improvement duty**: Must actually improve property
///
/// ### Owner's Perspective
/// **Advantages**:
/// - **Property improvement**: Receives enhanced property at term end
/// - **Ongoing income**: Modest rent for long term
/// - **Tax benefits**: Lower property tax due to divided ownership
/// - **Retain ownership**: Never loses title, just use rights
///
/// **Disadvantages**:
/// - **Loss of control**: Cannot manage property for decades
/// - **Tenant's freedom**: Cannot prevent transfer, mortgage, sublease
/// - **Below-market rent**: Canon emphytéotique typically low
///
/// ## Historical Context
/// Emphyteusis derives from **Roman law** (*emphyteusis*) and Byzantine law, where tenants
/// could improve imperial lands for long terms. The institution entered French law via Roman
/// legal tradition and canon law (Church property leases).
///
/// Key developments:
/// - **Roman Empire**: *Emphyteusis* for imperial domain cultivation
/// - **Medieval**: Church used emphyteusis for monastery lands
/// - **1804**: Napoleonic Code Articles 1741-1749 codified emphyteusis
/// - **1902**: Reform clarified transferability rights
/// - **1967**: Code rural provisions added for agricultural emphyteusis
/// - **2008**: Simplified registration procedures
///
/// ## Modern Applications
///
/// ### Urban Context
/// **Commercial development**:
/// - **Shopping centers**: Developer builds on land owned by institutional investor
/// - **Office buildings**: Tenant constructs building, operates for 50 years
/// - **Social housing**: Non-profit builds affordable housing on municipal land
///
/// **Public-private partnerships**:
/// - **Infrastructure**: Private company develops public land (transit stations)
/// - **Cultural facilities**: Private operator builds museum on public property
///
/// ### Rural Context
/// **Agricultural improvement**:
/// - **Land reclamation**: Tenant drains swamp, converts to farmland
/// - **Vineyards**: Emphyteuta plants vines, builds winery (30-50 year term)
/// - **Forestry**: Long-term forest management, reforestation
///
/// ### Public Domain Emphyteusis
/// **State/municipal property**:
/// - **Historic buildings**: Private restoration of public monuments
/// - **Parks**: Private development of public recreational facilities
/// - **Defense sites**: Conversion of military bases to civilian use
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (Erbbaurecht BGB §1012-1017)**: Heritable building right (similar long-term development right)
/// - **England (Leasehold)**: Long leases (99-999 years) common, but different legal structure
/// - **Japan**: Shakuchi-ken (superficies) - similar long-term building right
/// - **Common Law**: Ground lease (tenant owns improvements, leases land)
/// - **USA**: Ground lease (99-year terms common for commercial development)
///
/// ## Related Lease Types
///
/// ### Bail à Construction (Construction Lease)
/// **Similar** to emphyteusis:
/// - **Duration**: 18-99 years
/// - **Building obligation**: Must construct within 5 years
/// - **Differences**: More prescriptive construction requirements
///
/// ### Bail à Réhabilitation (Rehabilitation Lease)
/// **Urban renewal** focus:
/// - **Duration**: 12-99 years
/// - **Purpose**: Restore dilapidated buildings
/// - **Incentives**: Reduced rent, tax benefits
///
/// # Examples
///
/// ```text
/// // Example 1: Commercial shopping center
/// // Landowner owns €5M land parcel, City A prime location
/// // Developer wants to build €20M shopping center
/// // Structure:
/// // - 50-year emphyteutic lease (bail emphytéotique)
/// // - Canon emphytéotique: €100,000/year (2% of land value)
/// // - Developer builds shopping center at own cost (€20M)
/// // - Developer may mortgage lease for construction financing
/// // - Developer may sell lease (with shopping center) without owner consent
/// // - After 50 years: Owner receives land + shopping center (worth €25M+)
/// // - Developer: No compensation for improvements (building)
///
/// // Example 2: Agricultural vineyard
/// // Landowner: 10 hectares agricultural land, Burgundy
/// // Winemaker: Wants to plant premium vineyard
/// // Structure:
/// // - 40-year emphyteutic lease
/// // - Canon: €5,000/year (low rent)
/// // - Winemaker plants vines, builds winery, cellars (€500,000 investment)
/// // - After 40 years: Owner receives mature vineyard (worth €2M+)
/// // - Winemaker: Built valuable business but improvements revert to owner
///
/// // Example 3: Public-private partnership
/// // Municipality owns downtown plaza land
/// // Private company wants to build cultural center + underground parking
/// // Structure:
/// // - 60-year emphyteutic lease on public domain
/// // - Nominal rent: €10,000/year
/// // - Company builds €15M cultural facility
/// // - Company operates facility, generates revenue (ticket sales, events)
/// // - After 60 years: Facility becomes public property
/// // - Municipality: Receives major asset without capital expenditure
/// ```
pub fn article1741_1749() -> Statute {
    Statute::new(
        "code-civil-1741-1749",
        "Code civil Articles 1741-1749 - Bail emphytéotique / Emphyteutic lease (long-term 18-99 years)",
        Effect::new(
            EffectType::Grant,
            "Long-term lease (18-99 years) creating real right allowing tenant to transfer, \
             mortgage, improve property; improvements revert to owner at term end without compensation",
        )
        .with_parameter("duration_minimum", "18_years")
        .with_parameter("duration_maximum", "99_years_typical")
        .with_parameter("right_type", "droit_reel_real_right")
        .with_parameter("transferability", "freely_transferable_without_consent")
        .with_parameter("mortgageable", "true")
        .with_parameter("improvement_obligation", "must_improve_property")
        .with_parameter("improvements_revert", "owner_receives_without_compensation")
        .with_parameter("formality", "notarial_deed_required")
        .with_parameter("registration", "land_registry_publication_mandatory")
        .with_parameter("origin", "roman_byzantine_emphyteusis"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "lease_type".to_string(),
            value: "emphyteutic".to_string(),
        }),
        Box::new(Condition::And(
            Box::new(Condition::HasAttribute {
                key: "duration_years".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "notarial_deed".to_string(),
                value: "true".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Emphyteutic lease (bail emphytéotique): Long-term lease (18-99 years) creating real right \
         (droit réel), not mere personal contract. Emphyteuta may transfer, mortgage, sublease \
         without owner consent. Must improve property (buildings, plantings). Modest rent (canon \
         emphytéotique). Improvements revert to owner at term end without compensation (superficie \
         solo cedit) unless contracted. Formal requirements: notarial deed (acte notarié) + land \
         registry publication (publicité foncière). Real right: transferable, mortgageable, \
         registered, opposable to third parties, survives owner sale. Economic purpose: encourage \
         long-term investment - tenant gets quasi-ownership, owner receives improved property. \
         Historical: Roman/Byzantine emphyteusis, medieval Church lands, 1804 Code Articles 1741-1749. \
         Modern uses: commercial development (shopping centers), agricultural (vineyards), public-private \
         partnerships (infrastructure). Compare: Germany Erbbaurecht (BGB §1012-1017), England leasehold \
         (99-999 years), USA ground lease, Japan shakuchi-ken. Related: bail à construction (building \
         obligation), bail à réhabilitation (urban renewal). Key: real right vs. contract, transferability, \
         improvement duty, accession at term."
    )
}

/// Articles 1873-1878 - Real estate sales and formalities
///
/// # French Text (Key Articles)
///
/// **Article 1873** (now Civil Code Article 1582):
/// "La vente est parfaite entre les parties, et la propriété est acquise de droit à l'acheteur
/// à l'égard du vendeur, dès qu'on est convenu de la chose et du prix, quoique la chose n'ait
/// pas encore été livrée ni le prix payé."
///
/// **Code civil Article 1589**:
/// "La vente d'un immeuble doit être constatée par acte notarié et publiée au bureau de la
/// publicité foncière, à peine d'inopposabilité aux tiers."
///
/// **Code civil Article 1625**:
/// "Le vendeur a deux obligations principales: celle de délivrer, et celle de garantir la chose
/// qu'il vend."
///
/// # English Translation
///
/// **Article 1873 (1582)**:
/// "The sale is complete between the parties, and ownership is acquired by right by the buyer
/// with respect to the seller, as soon as the parties agree on the thing and the price, even
/// though the thing has not yet been delivered nor the price paid."
///
/// **Article 1589**:
/// "The sale of an immovable must be evidenced by notarial deed and published in the land
/// publicity office, on pain of being unenforceable against third parties."
///
/// **Article 1625**:
/// "The seller has two main obligations: that of delivery, and that of guaranteeing the thing
/// they sell."
///
/// # Legal Commentary
/// Articles 1873-1878 (now integrated throughout Code civil Articles 1582-1650) govern **real
/// estate sales** in French law, establishing the formalities, obligations, and warranties for
/// transferring immovable property. French law follows the **consensual principle** (ownership
/// transfers by agreement) but requires **notarial formalities** for opposability and registration.
///
/// ## Consensual Principle vs. Formal Requirements
///
/// ### Article 1582: Consensual Sale
/// **Ownership transfers by agreement alone**:
/// - **Meeting of minds**: Agreement on property (chose) and price (prix)
/// - **Inter partes**: Ownership passes between parties immediately
/// - **Before delivery**: Even without physical possession transfer
/// - **Before payment**: Even without price paid
///
/// **Example**: Oral agreement to sell house for €300,000 → Ownership transfers immediately
/// between parties (but see formal requirements below for third-party effect)
///
/// ### Article 1589: Notarial Deed Requirement
/// **Acte notarié mandatory for opposability**:
/// - **Formal document**: Licensed notaire drafts and authenticates deed
/// - **Public registry**: File with Service de publicité foncière
/// - **Third-party effect**: Without notarial deed + registration, sale unenforceable against third parties
/// - **Not validity requirement**: Sale valid between parties even with informal agreement
///
/// **Key distinction**:
/// - **Validity** (validité): Simple agreement sufficient (Article 1582)
/// - **Opposability** (opposabilité): Notarial deed + registration required (Article 1589)
///
/// ## Notarial Deed Requirements
///
/// ### Content of Acte de Vente (Sale Deed)
/// **Mandatory elements**:
/// 1. **Identification**: Parties' full names, addresses, birthdates
/// 2. **Property description**: Address, cadastral number, boundaries, area
/// 3. **Price**: Purchase price clearly stated (€ amount)
/// 4. **Warranties**: Seller's guarantees (title, hidden defects)
/// 5. **Conditions**: Suspensive conditions (financing, planning permission)
/// 6. **Origin of ownership**: Seller's title chain (origine de propriété)
/// 7. **Easements**: Servitudes affecting property (rights of way, etc.)
/// 8. **Mortgages**: Existing mortgages, liens to be cleared
/// 9. **Urban planning**: Zoning information, building permits
/// 10. **Pre-emption rights**: Public right of first refusal disclosures
///
/// ### Notary's Role
/// **Notaire's duties**:
/// - **Impartiality**: Serve both parties, not advocate for either
/// - **Title search**: Verify seller owns property, check 30-year title chain
/// - **Lien search**: Identify mortgages, judgments, tax liens
/// - **Drafting**: Prepare deed compliant with law
/// - **Advice**: Explain legal consequences to parties
/// - **Authentication**: Official signature makes deed authentic act
/// - **Registration**: File deed with land registry
/// - **Fund custody**: Hold purchase price in escrow, disburse at closing
///
/// ## Land Registry (Publicité Foncière)
///
/// ### Registration System
/// **Service de publicité foncière** (formerly Conservation des hypothèques):
/// - **Public records**: Anyone may search property titles
/// - **Priority**: Registration order determines creditor priorities ("first in time, first in right")
/// - **Opposability**: Rights enforceable against third parties only after registration
/// - **Constructive notice**: Registered rights presumed known to all
///
/// ### Information in Registry
/// **Fichier immobilier** contains:
/// - **Current owner**: Name, acquisition date, price
/// - **Previous owners**: 30-year title chain minimum
/// - **Mortgages**: All hypothèques, liens on property
/// - **Easements**: Servitudes benefiting or burdening property
/// - **Seizures**: Judicial seizures, bankruptcy proceedings
/// - **Pre-emption**: Public authorities' rights of first refusal
///
/// ## Seller's Obligations
///
/// ### 1. Obligation to Deliver (Délivrer)
/// **Article 1625**: Seller must deliver property in agreed condition:
/// - **Physical possession**: Transfer keys, vacant possession
/// - **Legal possession**: Clear title, no undisclosed encumbrances
/// - **Conformity**: Property matches description in deed
/// - **Accessories**: Include fixtures, fittings agreed in sale
///
/// ### 2. Obligation to Guarantee (Garantir)
///
/// #### Warranty of Title (Garantie d'éviction)
/// **Seller guarantees buyer's peaceful possession**:
/// - **Full ownership**: Seller actually owns property
/// - **No encumbrances**: Except those disclosed in deed
/// - **No eviction**: Third party won't claim superior title
/// - **Easements**: Disclose all servitudes affecting property
///
/// **Breach consequences**:
/// - **Total eviction**: Buyer loses property to rightful owner → Full refund + damages
/// - **Partial eviction**: Third party proves easement → Price reduction + damages
/// - **Non-waivable**: Cannot exclude this warranty (ordre public)
///
/// #### Warranty Against Hidden Defects (Garantie des vices cachés)
/// **Seller liable for concealed defects** (Articles 1641-1649):
///
/// **Conditions**:
/// 1. **Hidden**: Not apparent on reasonable inspection
/// 2. **Serious**: Render property unfit for intended use or substantially diminish value
/// 3. **Pre-existing**: Existed at time of sale
/// 4. **Unknown to buyer**: Buyer didn't know or couldn't reasonably discover
///
/// **Examples of hidden defects**:
/// - **Structural**: Foundation cracks, termite damage, dry rot
/// - **Water**: Roof leaks, basement flooding, plumbing defects
/// - **Legal**: Illegal construction, zoning violations
/// - **Environmental**: Asbestos, lead paint, contaminated soil
///
/// **Buyer's remedies** (must act within 2 years):
/// - **Rédhibition**: Rescind sale, return property, recover price + costs
/// - **Price reduction**: Keep property, get partial refund
/// - **Damages**: If seller knew of defect (bad faith)
///
/// **Professional seller**: Stricter liability, cannot exclude warranty
///
/// ## Pre-Sale Process
///
/// ### 1. Compromis de Vente (Preliminary Agreement)
/// **Private agreement** before notarial deed:
/// - **Binding contract**: Parties commit to sale at stated price
/// - **Suspensive conditions**: Financing approval, building inspection
/// - **Deposit**: Buyer pays 5-10% deposit (séquestre)
/// - **Cooling-off period**: Buyer has 10-day withdrawal right
/// - **Penalty**: €15,000+ if buyer breaches without valid condition
///
/// ### 2. Due Diligence Period
/// **Buyer's investigations** (typically 2-3 months):
/// - **Building inspection**: Structural survey, defect identification
/// - **Title search**: Notary verifies ownership, liens
/// - **Urban planning**: Check zoning, building permits
/// - **Mortgage approval**: Bank financing confirmation
/// - **Easements**: Identify servitudes affecting property
///
/// ### 3. Acte Définitif de Vente (Final Deed)
/// **Notarial deed** at closing:
/// - **Parties sign**: Before notaire, in person or by power of attorney
/// - **Price paid**: Buyer pays balance (typically by bank wire)
/// - **Ownership transfers**: Effective immediately upon signature
/// - **Keys delivered**: Buyer receives possession
/// - **Registration**: Notary files deed with land registry within 1 month
///
/// ## Transfer Costs
///
/// ### Frais de Notaire (Notary Fees)
/// **Total costs approximately 7-8%** of purchase price:
///
/// **Breakdown for €300,000 property**:
/// 1. **Transfer tax** (droits de mutation): €17,400 (5.8% to State/local)
/// 2. **Notary's fee** (émoluments): €3,000-4,000 (regulated scale)
/// 3. **Registration fees**: €500 (land registry)
/// 4. **Disbursements**: €500 (searches, copies)
/// 5. **Total**: ~€22,000 (7.3%)
///
/// **Buyer pays** transfer costs (customary rule)
///
/// ## Historical Context
/// French sale law dates from 1804 Napoleonic Code, adopting Roman law's consensual principle
/// (*consensus* transfers ownership) but adding notarial formalities from medieval French customs
/// (notaries protected feudal land transfers).
///
/// Key developments:
/// - **1804**: Code civil Articles 1582-1650 on sales, including real estate
/// - **1855**: Creation of land registry system (conservation des hypothèques)
/// - **1955**: Decree reorganizing land registry
/// - **1978**: Law on defects and liabilities in construction
/// - **1998**: Consumer protection, cooling-off period for buyers
/// - **2016**: Electronic registration system implemented
/// - **2020**: Covid pandemic → remote notarial closings authorized
///
/// ## Modern Practice
///
/// ### Urban Residential Sales
/// **Typical timeline**: 3-4 months
/// 1. **Viewing + offer**: 1-2 weeks
/// 2. **Compromis de vente**: Signed, 10-day cooling-off starts
/// 3. **Due diligence**: 2-3 months (financing, inspections)
/// 4. **Acte définitif**: Final closing at notary's office
///
/// ### Commercial Real Estate
/// **More complex**:
/// - **Asset vs. share sale**: Buy building directly or buy company owning building
/// - **Share sale advantages**: Avoid transfer tax (company remains owner)
/// - **Due diligence**: Extensive (environmental, tenants, leases)
/// - **Timeline**: 6-12 months
///
/// ### Agricultural Land
/// **Special rules**:
/// - **SAFER**: Public agency has pre-emption right to buy farmland
/// - **Purpose**: Maintain agricultural use, prevent speculation
/// - **Notification**: Must notify SAFER of sale, 2-month right to match offer
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §433-453)**: Notarial deed required for validity (not just opposability)
/// - **England**: No notary, solicitors handle conveyancing, Land Registry
/// - **USA**: Recording acts (race, notice, race-notice systems), title insurance common
/// - **Spain**: Notarial deed (escritura pública) + Land Registry (Registro de la Propiedad)
/// - **Japan (Minpō §555-585)**: Registration required for opposability, judicial scriveners
///
/// ## Seller's Disclosure Duties
///
/// ### Mandatory Diagnostics (Diagnostic Techniques)
/// **Seller must provide** before compromis:
/// 1. **Energy performance** (DPE): Building's energy efficiency rating
/// 2. **Lead**: For buildings built before 1949
/// 3. **Asbestos**: For buildings built before 1997
/// 4. **Termites**: In designated zones
/// 5. **Natural risks**: Flooding, earthquake zones
/// 6. **Electrical safety**: For installations >15 years old
/// 7. **Gas safety**: For installations >15 years old
/// 8. **Sanitation**: For rural properties without mains sewer
///
/// **Penalty**: Buyer may claim damages or price reduction for missing/false diagnostics
///
/// # Examples
///
/// ```text
/// // Example 1: Typical residential sale
/// // Property: Paris apartment, €400,000
/// // Timeline:
/// // - Week 1: Offer accepted, draft compromis
/// // - Week 2: Compromis signed, buyer pays €40,000 deposit (10%)
/// // - Days 1-10: Buyer's cooling-off period (may withdraw, full refund)
/// // - Months 1-3: Suspensive conditions (mortgage approval, inspections)
/// // - Month 3: Acte définitif at notary
/// //   * Buyer pays balance: €360,000
/// //   * Buyer pays transfer costs: €28,000 (7%)
/// //   * Total cash needed: €388,000 (plus original €40,000 deposit = €428,000)
/// //   * Ownership transfers immediately
/// //   * Notary files with land registry within 1 month
///
/// // Example 2: Hidden defect claim
/// // Buyer purchases house for €300,000
/// // 6 months later: Discovers foundation cracks (structural engineer: €80,000 repair)
/// // Hidden defect: Not visible during inspection, pre-existing
/// // Buyer's options (within 2 years):
/// //   (a) Rédhibition: Return house, recover €300,000 + €28,000 costs = €328,000
/// //   (b) Price reduction: Keep house, get ~€80,000 refund
/// //   (c) If seller knew: Damages for fraud
///
/// // Example 3: Asset vs. share sale
/// // Commercial building: €5,000,000
/// // Option A - Asset sale (traditional):
/// //   - Transfer tax: €290,000 (5.8%)
/// //   - Notary fees: €40,000
/// //   - Total costs: €330,000 (6.6%)
/// // Option B - Share sale (buy company owning building):
/// //   - Transfer tax: €250,000 (5% on shares)
/// //   - No notary, no land registry
/// //   - Lower costs but inherit company liabilities
/// //   - Due diligence more extensive
/// ```
pub fn article1873_1878() -> Statute {
    Statute::new(
        "code-civil-1873-1878",
        "Code civil Articles 1873-1878 (1582-1650) - Vente immobilière / Real estate sales and formalities",
        Effect::new(
            EffectType::Obligation,
            "Real estate sale requires notarial deed and land registry publication for third-party \
             effect; seller must deliver property and guarantee title and against hidden defects",
        )
        .with_parameter("art1582_consensual", "ownership_transfers_by_agreement")
        .with_parameter("art1589_form", "notarial_deed_required_opposability")
        .with_parameter("registration", "land_registry_publication_mandatory")
        .with_parameter("seller_obligation_1", "delivery_of_property")
        .with_parameter("seller_obligation_2", "warranty_of_title")
        .with_parameter("seller_obligation_3", "warranty_against_hidden_defects")
        .with_parameter("notary_role", "impartial_title_search_authentication")
        .with_parameter("transfer_costs", "7_8_percent_buyer_pays")
        .with_parameter("pre_sale", "compromis_10_day_cooling_off")
        .with_parameter("diagnostics", "mandatory_technical_reports"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "property_type".to_string(),
            value: "immovable".to_string(),
        }),
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "transaction_type".to_string(),
                value: "sale".to_string(),
            }),
            Box::new(Condition::HasAttribute {
                key: "seller".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Real estate sales: Consensual principle (Article 1582) - ownership transfers by agreement \
         on property and price, even before delivery/payment, BUT notarial deed + land registry \
         required for opposability to third parties (Article 1589). Notarial deed (acte notarié): \
         licensed notaire drafts, authenticates, verifies title (30-year chain), searches liens, \
         files with Service de publicité foncière. Seller obligations: (1) deliver property \
         (physical + legal possession), (2) warranty of title (garantie d'éviction - no eviction, \
         disclosed encumbrances), (3) warranty against hidden defects (garantie des vices cachés - \
         structural, pre-existing, buyer remedies within 2 years: rescission or price reduction). \
         Transfer costs: ~7-8% of price (5.8% transfer tax + 2% notary fee), buyer pays. Pre-sale: \
         compromis de vente (preliminary agreement), 10-day cooling-off, suspensive conditions \
         (financing, inspections), 5-10% deposit. Mandatory diagnostics: energy, lead, asbestos, \
         termites, natural risks. Historical: 1804 Code consensual principle + notarial formalities. \
         Modern: electronic registration, remote closings. Compare: Germany BGB §433-453 (notary \
         for validity), England (solicitors, Land Registry), USA (recording acts, title insurance), \
         Spain (escritura pública). Key: validity vs. opposability distinction, notary's impartiality, \
         comprehensive warranties."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article490_creation() {
        let statute = article490();
        assert_eq!(statute.id, "code-civil-490");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("490"));
        assert!(statute.title.contains("Classification"));
    }

    #[test]
    fn test_article490_property_classification() {
        let statute = article490();
        assert!(matches!(statute.effect.effect_type, EffectType::Grant));
        assert!(
            statute
                .effect
                .parameters
                .contains_key("immovable_by_nature")
        );
        assert!(
            statute
                .effect
                .parameters
                .contains_key("immovable_by_destination")
        );
        assert!(
            statute
                .effect
                .parameters
                .contains_key("immovable_by_object")
        );
        assert!(statute.effect.parameters.contains_key("movable_by_nature"));
        assert!(
            statute
                .effect
                .parameters
                .contains_key("movable_by_anticipation")
        );
        assert_eq!(
            statute
                .effect
                .parameters
                .get("consequence_immovable")
                .unwrap(),
            "notarial_deed_registration_required"
        );
    }

    #[test]
    fn test_article490_has_precondition() {
        let statute = article490();
        assert_eq!(statute.preconditions.len(), 1);
        assert!(matches!(
            &statute.preconditions[0],
            Condition::HasAttribute { key } if key == "property"
        ));
    }

    #[test]
    fn test_article1741_1749_creation() {
        let statute = article1741_1749();
        assert_eq!(statute.id, "code-civil-1741-1749");
        assert!(statute.title.contains("1741-1749"));
        assert!(statute.title.contains("emphytéotique"));
    }

    #[test]
    fn test_article1741_1749_emphyteutic_lease() {
        let statute = article1741_1749();
        assert!(matches!(statute.effect.effect_type, EffectType::Grant));
        assert_eq!(
            statute.effect.parameters.get("duration_minimum").unwrap(),
            "18_years"
        );
        assert_eq!(
            statute.effect.parameters.get("right_type").unwrap(),
            "droit_reel_real_right"
        );
        assert_eq!(
            statute.effect.parameters.get("transferability").unwrap(),
            "freely_transferable_without_consent"
        );
        assert_eq!(
            statute.effect.parameters.get("mortgageable").unwrap(),
            "true"
        );
    }

    #[test]
    fn test_article1741_1749_preconditions() {
        let statute = article1741_1749();
        assert_eq!(statute.preconditions.len(), 1);

        // Verify nested And structure
        if let Condition::And(box1, box2) = &statute.preconditions[0] {
            match (&**box1, &**box2) {
                (Condition::AttributeEquals { key, value }, Condition::And(..)) => {
                    assert_eq!(key, "lease_type");
                    assert_eq!(value, "emphyteutic");
                }
                _ => panic!("Expected AttributeEquals and nested And"),
            }
        } else {
            panic!("Expected top-level And condition");
        }
    }

    #[test]
    fn test_article1873_1878_creation() {
        let statute = article1873_1878();
        assert_eq!(statute.id, "code-civil-1873-1878");
        assert!(statute.title.contains("1873-1878"));
        assert!(statute.title.contains("Vente immobilière"));
    }

    #[test]
    fn test_article1873_1878_sale_obligations() {
        let statute = article1873_1878();
        assert!(matches!(statute.effect.effect_type, EffectType::Obligation));
        assert!(statute.effect.parameters.contains_key("art1582_consensual"));
        assert!(statute.effect.parameters.contains_key("art1589_form"));
        assert!(
            statute
                .effect
                .parameters
                .contains_key("seller_obligation_1")
        );
        assert!(
            statute
                .effect
                .parameters
                .contains_key("seller_obligation_2")
        );
        assert!(
            statute
                .effect
                .parameters
                .contains_key("seller_obligation_3")
        );
        assert_eq!(
            statute.effect.parameters.get("transfer_costs").unwrap(),
            "7_8_percent_buyer_pays"
        );
    }

    #[test]
    fn test_article1873_1878_preconditions() {
        let statute = article1873_1878();
        assert_eq!(statute.preconditions.len(), 1);

        // Verify nested And/Or structure
        if let Condition::And(box1, box2) = &statute.preconditions[0] {
            assert!(matches!(
                &**box1,
                Condition::AttributeEquals { key, value }
                if key == "property_type" && value == "immovable"
            ));
            assert!(matches!(&**box2, Condition::Or(..)));
        } else {
            panic!("Expected top-level And condition");
        }
    }

    #[test]
    fn test_all_transaction_articles_have_jurisdiction() {
        let articles = vec![article490(), article1741_1749(), article1873_1878()];

        for article in articles {
            assert_eq!(article.jurisdiction.as_deref(), Some("FR"));
        }
    }

    #[test]
    fn test_all_transaction_articles_have_discretion() {
        let articles = vec![article490(), article1741_1749(), article1873_1878()];

        for article in articles {
            assert!(article.discretion_logic.is_some());
            let discretion = article.discretion_logic.unwrap();
            assert!(
                discretion.len() > 150,
                "Discretion too short for {}",
                article.id
            );
        }
    }

    #[test]
    fn test_all_transaction_articles_have_parameters() {
        let articles = vec![article490(), article1741_1749(), article1873_1878()];

        for article in articles {
            assert!(!article.effect.parameters.is_empty());
            assert!(article.effect.parameters.len() >= 5);
        }
    }

    #[test]
    fn test_all_articles_version_1() {
        let articles = vec![article490(), article1741_1749(), article1873_1878()];

        for article in articles {
            assert_eq!(article.version, 1);
        }
    }

    #[test]
    fn test_article490_roman_origin() {
        let statute = article490();
        assert_eq!(
            statute.effect.parameters.get("origin").unwrap(),
            "roman_law_res_mobiles_immobiles"
        );
    }

    #[test]
    fn test_article1741_1749_improvements_revert() {
        let statute = article1741_1749();
        assert_eq!(
            statute
                .effect
                .parameters
                .get("improvements_revert")
                .unwrap(),
            "owner_receives_without_compensation"
        );
        assert_eq!(
            statute.effect.parameters.get("formality").unwrap(),
            "notarial_deed_required"
        );
    }

    #[test]
    fn test_article1873_1878_warranties() {
        let statute = article1873_1878();
        assert_eq!(
            statute
                .effect
                .parameters
                .get("seller_obligation_2")
                .unwrap(),
            "warranty_of_title"
        );
        assert_eq!(
            statute
                .effect
                .parameters
                .get("seller_obligation_3")
                .unwrap(),
            "warranty_against_hidden_defects"
        );
    }

    #[test]
    fn test_article490_discretion_comprehensive() {
        let statute = article490();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("Four categories"));
        assert!(discretion.contains("Immovables by nature"));
        assert!(discretion.contains("destination"));
        assert!(discretion.contains("notarial deed"));
    }

    #[test]
    fn test_article1741_1749_discretion_comprehensive() {
        let statute = article1741_1749();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("18-99 years"));
        assert!(discretion.contains("real right"));
        assert!(discretion.contains("transferable"));
        assert!(discretion.contains("Roman"));
    }

    #[test]
    fn test_article1873_1878_discretion_comprehensive() {
        let statute = article1873_1878();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("Consensual"));
        assert!(discretion.contains("opposability"));
        assert!(discretion.contains("notarial deed"));
        assert!(discretion.contains("warranty"));
    }
}
