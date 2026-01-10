//! Ownership rights articles (Code civil, Book II)
//!
//! This module implements the fundamental articles governing ownership
//! rights, accession, and property transformation in French law.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 544 - Absolute ownership
///
/// # French Text
/// "La propriété est le droit de jouir et disposer des choses de la manière la plus absolue,
/// pourvu qu'on n'en fasse pas un usage prohibé par les lois ou par les règlements."
///
/// # English Translation
/// "Ownership is the right to enjoy and dispose of things in the most absolute manner,
/// provided that one does not make use prohibited by laws or regulations."
///
/// # Legal Commentary
/// This fundamental article establishes the **absolute nature of ownership** in French law,
/// comprising three core rights:
/// 1. **Usus** (right to use) - Owner can use the property
/// 2. **Fructus** (right to enjoy fruits) - Owner receives income/benefits
/// 3. **Abusus** (right to dispose) - Owner can sell, destroy, or transfer
///
/// ## Historical Context
/// Article 544 dates from the original Napoleonic Code of 1804 and represents the triumph
/// of individual property rights over feudal systems. The French Revolution (1789) abolished
/// feudalism and established absolute private ownership. This article codified the principle
/// that ownership is "the most absolute" right, limited only by law and regulation.
///
/// The phrase "de la manière la plus absolue" (most absolute manner) reflects the
/// revolutionary rejection of feudal duties and restrictions on property use.
///
/// ## Modern Limitations
/// While ownership is "absolute," it faces increasing limitations:
/// - **Environmental law**: Regulations protecting nature, water, forests
/// - **Urban planning**: Zoning laws, building permits (Code de l'urbanisme)
/// - **Historic preservation**: Monument protection (Code du patrimoine)
/// - **Neighbor rights**: Abuse of rights doctrine (abus de droit)
/// - **Public interest**: Expropriation for public utility (Article 545)
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §903)**: Similar absolute ownership with social limitations
/// - **Japan (Minpō §206)**: Owner can freely use, benefit, and dispose within law limits
/// - **Common Law**: Bundle of rights theory - ownership as collection of sticks
/// - **China**: Land ownership by state, private use rights only (70-year leases)
///
/// ## Constitutional Protection
/// The French Constitution protects private property:
/// - Declaration of Human Rights 1789, Articles 2 & 17: Property as "inviolable and sacred"
/// - Constitutional Council decisions uphold strong property protection
///
/// # Examples
///
/// ```
/// use legalis_fr::property::{Property, PropertyType};
///
/// let property = Property::new(
///     PropertyType::Immovable {
///         land_area: 1000.0,
///         building_area: Some(200.0),
///     },
///     "Jean Dupont".to_string(),
///     "15 rue de Paris, 75001 Paris".to_string(),
///     500_000,
/// );
///
/// // Owner has absolute rights: usus, fructus, abusus (Article 544)
/// // Can use the property, collect rent, sell it, or even destroy it
/// // Limited only by laws and regulations
/// ```
pub fn article544() -> Statute {
    Statute::new(
        "code-civil-544",
        "Code civil Article 544 - Propriété absolue / Absolute ownership (usus, fructus, abusus)",
        Effect::new(
            EffectType::Grant,
            "Owner has absolute right to use, enjoy, and dispose of property within legal limits",
        )
        .with_parameter("right_usus", "use")
        .with_parameter("right_fructus", "enjoy_fruits")
        .with_parameter("right_abusus", "dispose")
        .with_parameter("limitation", "within_laws_and_regulations"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::HasAttribute {
        key: "owner".to_string(),
    })
    .with_precondition(Condition::HasAttribute {
        key: "property".to_string(),
    })
    .with_discretion(
        "Historical note: Article 544 from 1804 Napoleonic Code establishes absolute ownership \
         (usus, fructus, abusus) - a revolutionary principle rejecting feudalism. Ownership is \
         'the most absolute' right but limited by laws, regulations, and neighbor rights. \
         Modern limitations include environmental law, urban planning, and public interest. \
         Compare: Germany BGB §903 (similar absolute ownership with social function), \
         Japan Minpō §206 (free use within law limits), Common Law (bundle of rights theory).",
    )
}

/// Article 545 - Expropriation for public utility
///
/// # French Text
/// "Nul ne peut être contraint de céder sa propriété, si ce n'est pour cause d'utilité publique,
/// et moyennant une juste et préalable indemnité."
///
/// # English Translation
/// "No one can be forced to give up their property, except for public utility,
/// and with fair and prior compensation."
///
/// # Legal Commentary
/// This article establishes the **expropriation regime** (expropriation pour utilité publique),
/// balancing absolute ownership (Article 544) with public interest. It requires:
///
/// ## Three Mandatory Conditions
/// 1. **Public utility** (utilité publique) - Benefit to community, not private interest
/// 2. **Fair compensation** (juste indemnité) - Market value, not nominal amount
/// 3. **Prior compensation** (préalable indemnité) - Payment before taking property
///
/// ## Public Utility Examples
/// - **Infrastructure**: Roads, railways, airports, ports
/// - **Urban renewal**: Slum clearance, public housing
/// - **Public facilities**: Schools, hospitals, parks
/// - **Historic preservation**: Protecting monuments
/// - **Environmental protection**: Creating nature reserves
///
/// ## Procedure (Code de l'expropriation)
/// 1. **Declaration of public utility** by administrative authority
/// 2. **Judicial determination of compensation** by expropriation judge
/// 3. **Payment** before taking possession
/// 4. **Transfer of ownership** to expropriating authority
///
/// ## Constitutional Protection
/// Based on Declaration of Human Rights 1789, Article 17:
/// "Property being an inviolable and sacred right, no one can be deprived of it,
/// unless public necessity, legally established, evidently requires it, and under
/// the condition of a just and prior indemnity."
///
/// ## Historical Context
/// This principle dates to 1789 Revolution and 1804 Code civil. It balanced two needs:
/// - Protection of individual property rights (against arbitrary seizure)
/// - Public infrastructure development (roads, railways in 19th century)
///
/// Modern examples include:
/// - TGV high-speed rail network construction
/// - Grand Paris Express metro expansion (2010s-2030s)
/// - Nuclear power plant sites (1970s-1980s)
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (GG Art. 14)**: Similar - expropriation only for public welfare with compensation
/// - **Japan (Constitution Art. 29)**: "Private property may be taken for public use upon just compensation"
/// - **USA (5th Amendment)**: "Nor shall private property be taken for public use without just compensation"
/// - **UK**: Compulsory purchase orders with compensation
///
/// ## Just Compensation Standard
/// French law requires **full market value** compensation including:
/// - Market value of land/buildings
/// - Loss of business income (prejudice commercial)
/// - Moving costs
/// - Moral damages (préjudice moral)
/// - Expert appraisal costs
///
/// # Examples
///
/// ```text
/// // Paris Metro Line 15 (Grand Paris Express) expropriation case
/// // Government needs land for new metro station
/// // Process:
/// // 1. Declaration of public utility (metro is public infrastructure)
/// // 2. Judicial appraisal of property value (market rate in Paris)
/// // 3. Payment to owner before construction starts
/// // 4. Transfer of ownership to RATP/Société du Grand Paris
/// ```
pub fn article545() -> Statute {
    Statute::new(
        "code-civil-545",
        "Code civil Article 545 - Expropriation pour utilité publique / Expropriation for public utility",
        Effect::new(
            EffectType::StatusChange,
            "Property can be expropriated for public utility with fair prior compensation",
        )
        .with_parameter("requirement_1", "public_utility")
        .with_parameter("requirement_2", "fair_compensation")
        .with_parameter("requirement_3", "prior_compensation")
        .with_parameter("basis", "declaration_of_human_rights_1789_art17"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "purpose".to_string(),
            value: "public_utility".to_string(),
        }),
        Box::new(Condition::And(
            Box::new(Condition::HasAttribute {
                key: "compensation_amount".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "compensation_paid".to_string(),
                value: "before_taking".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Constitutional protection: Based on Declaration of Human Rights 1789, Art. 17 - property \
         is 'inviolable and sacred'. Expropriation requires: (1) public utility (infrastructure, \
         urban renewal), (2) fair compensation (full market value), (3) prior compensation (before \
         taking). Modern examples: TGV railways, Grand Paris Express metro, nuclear sites. \
         Compare: USA 5th Amendment, Germany GG Art. 14, Japan Constitution Art. 29, UK compulsory purchase."
    )
}

/// Article 546 - Accession rights (right of accession)
///
/// # French Text
/// "La propriété d'une chose soit mobilière, soit immobilière, donne droit sur tout ce qu'elle produit,
/// et sur ce qui s'y unit accessoirement soit naturellement, soit artificiellement."
///
/// # English Translation
/// "Ownership of a thing, whether movable or immovable, gives right to all that it produces,
/// and to what unites with it accessorily, whether naturally or artificially."
///
/// # Legal Commentary
/// Article 546 establishes the **right of accession** (droit d'accession), one of three modes
/// of acquiring ownership (with occupation and prescription). It comprises two aspects:
///
/// ## 1. Right to Natural and Artificial Fruits (Accession by Production)
/// **Natural fruits** (fruits naturels):
/// - Agricultural products: crops, vegetables, fruits
/// - Animal products: milk, wool, offspring
/// - Forestry products: timber, firewood
///
/// **Industrial fruits** (fruits industriaux):
/// - Products requiring cultivation: vineyards, orchards
/// - Manufactured goods from property
///
/// **Civil fruits** (fruits civils):
/// - Rent from leased property
/// - Interest on loaned money
/// - Royalties from intellectual property
///
/// ## 2. Right to Additions (Accession by Union)
/// **Natural accession**:
/// - Alluvion: Gradual deposits by rivers/streams (Article 556)
/// - Avulsion: Sudden deposits by floods (Article 559)
/// - Islands formed in rivers (Article 560)
/// - Abandoned riverbed (Article 563)
///
/// **Artificial accession**:
/// - Buildings constructed on land: **Superficie solo cedit**
/// - Plantations on land (Article 553)
/// - Materials incorporated into buildings (Article 554-555)
/// - Good faith construction on another's land (Article 555)
///
/// ## Superficie Solo Cedit Principle
/// "Whatever is built on the land belongs to the land" - This Roman law principle means:
/// - Building owner ≠ necessarily builder
/// - Land owner owns everything built on land (unless contractual exception)
/// - Construction lease (bail à construction) creates exception for 18-99 years
///
/// ## Historical Context
/// The accession principle derives from Roman law (Institutes of Gaius, Digest of Justinian).
/// The Napoleonic Code 1804 incorporated this 2000-year-old principle. It reflects agricultural
/// society concerns (riverbank deposits, crop ownership) and construction rules.
///
/// Modern applications:
/// - Wind turbines on agricultural land (natural vs. artificial fruits debate)
/// - Solar panels on roofs (accession to building)
/// - Improvements by tenants (Article 555 compensation rules)
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §953-957)**: Similar accession rules with Bestandteil concept
/// - **Japan (Minpō §242-248)**: Natural/legal fruits distinction, accession for fixtures
/// - **Common Law**: Fixtures doctrine - chattels becoming part of realty
/// - **Louisiana (Civil Code)**: French-influenced accession rules (Arts 490-507)
///
/// ## Key Distinctions
/// **Fruits vs. Products**:
/// - **Fruits**: Renewable without diminishing substance (apples from tree)
/// - **Products**: Diminish substance (quarried stone, mined ore)
///
/// **Movable vs. Immovable Accession**:
/// - Immovable accession: Articles 546-564 (land-related)
/// - Movable accession: Articles 565-577 (personal property)
///
/// # Examples
///
/// ```
/// use legalis_fr::property::{Property, PropertyType};
///
/// // Example 1: Agricultural land with crops
/// let farm = Property::new(
///     PropertyType::Immovable {
///         land_area: 10_000.0,
///         building_area: Some(500.0),
///     },
///     "Marie Dubois".to_string(),
///     "Normandy, France".to_string(),
///     800_000,
/// );
/// // Owner owns: land + house + wheat crops + apple harvest (Article 546)
///
/// // Example 2: Building constructed on land
/// // Builder constructs house on landowner's property
/// // Result: Landowner owns house (superficie solo cedit)
/// // But Article 555 may require compensation to builder if good faith
/// ```
pub fn article546() -> Statute {
    Statute::new(
        "code-civil-546",
        "Code civil Article 546 - Droit d'accession / Right of accession (fruits and additions)",
        Effect::new(
            EffectType::Grant,
            "Owner acquires fruits produced and additions united to property naturally or artificially",
        )
        .with_parameter("right_to_fruits", "natural_industrial_civil")
        .with_parameter("right_to_additions", "natural_and_artificial")
        .with_parameter("principle", "superficie_solo_cedit")
        .with_parameter("origin", "roman_law"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::Or(
        Box::new(Condition::HasAttribute {
            key: "produces_fruits".to_string(),
        }),
        Box::new(Condition::HasAttribute {
            key: "has_additions".to_string(),
        }),
    ))
    .with_discretion(
        "Right of accession: Owner acquires (1) fruits - natural (crops), industrial (cultivation), \
         civil (rent); (2) additions - natural (alluvion, islands) or artificial (buildings). \
         Core principle: superficie solo cedit (buildings belong to land). Roman law origin \
         (Institutes of Gaius, Digest). Fruits vs. products: fruits renewable, products diminish \
         substance. Modern issues: wind turbines, solar panels, tenant improvements. \
         Compare: Germany BGB §953-957 (Bestandteil), Japan Minpō §242-248, Common Law fixtures, \
         Louisiana CC Arts 490-507."
    )
}

/// Article 548 - Underground and overhead rights
///
/// # French Text
/// "La propriété du sol emporte la propriété du dessus et du dessous."
///
/// # English Translation
/// "Ownership of the land carries ownership of what is above and below."
///
/// # Legal Commentary
/// Article 548 establishes the **vertical extent of ownership** (propriété du dessus et dessous),
/// extending the owner's rights infinitely above and below the surface. This follows the Roman
/// law maxim: **"Cujus est solum, ejus est usque ad coelum et ad inferos"** (whoever owns the
/// soil owns up to the sky and down to the depths).
///
/// ## Rights Above Ground (Dessus)
/// The owner has rights to the airspace above, including:
/// - **Building vertically**: Constructing multi-story buildings
/// - **Airspace control**: Preventing overhanging branches (Article 673)
/// - **Aerial easements**: But subject to aircraft overflight (Code de l'aviation civile)
/// - **Wind rights**: Installing wind turbines (with planning permission)
///
/// ## Rights Below Ground (Dessous)
/// The owner has rights to the subsoil, including:
/// - **Excavations**: Digging cellars, basements, underground parking
/// - **Foundations**: Deep foundations for buildings
/// - **Wells and water**: Extracting groundwater (subject to water law)
/// - **Tunnels**: Private tunnels on own property
///
/// ## Major Limitations
///
/// ### 1. Mineral Rights (Code minier)
/// **State ownership** of strategic minerals:
/// - Mines: Coal, metallic ores, uranium, geothermal energy (>150m depth)
/// - State grants mining concessions
/// - Surface owner receives compensation but doesn't own minerals
/// - Quarries (surface materials): Owned by landowner
///
/// ### 2. Aviation Rights (Code de l'aviation civile)
/// - Aircraft have right to overflight at safe altitude
/// - Surface owner cannot object to airplanes overhead
/// - But drones may violate privacy/property rights at low altitude
///
/// ### 3. Archaeological Finds (Code du patrimoine)
/// - Significant archaeological discoveries belong to State
/// - Landowner receives compensation (25-50% of value)
/// - Must report discoveries within 48 hours
///
/// ### 4. Public Utility Easements
/// - Underground cables/pipes for electricity, gas, water
/// - Requires compensation but owner cannot refuse
/// - State/utilities can impose servitudes
///
/// ### 5. Water Rights (Code de l'environnement)
/// - Rivers, streams: Public domain (domaine public)
/// - Groundwater: Regulated extraction, environmental protection
/// - Cannot divert water harming neighbors
///
/// ### 6. Planning and Environmental Law
/// - Maximum building heights (zoning laws)
/// - Protected viewsheds and landscapes
/// - Noise limits for excavation
/// - Environmental impact assessments
///
/// ## Historical Context
/// Article 548 dates from 1804 Napoleonic Code, adopting Roman law principles. In agricultural
/// society, subsoil rights meant wells and cellars; aerial rights were theoretical. Modern
/// technology (aviation, mining, tunnels, skyscrapers) required extensive limitations.
///
/// Key developments:
/// - **1810**: Code minier established state mineral rights
/// - **1924**: International convention on aviation sovereignty
/// - **1957**: Code de l'urbanisme introduced planning restrictions
/// - **1975**: Grand Paris metro tunnels under private property
/// - **2000s**: Geothermal energy exploitation debates
///
/// ## International Comparison
/// - **Germany (BGB §905)**: Owner's rights extend to space above/below, limited by public interest
/// - **Japan (Minpō §207)**: Owner's rights extend above/below to extent necessary for use
/// - **Common Law**: Ad coelum doctrine, heavily limited by modern law
/// - **USA**: Separate surface/mineral estates common (especially Texas, Oklahoma)
/// - **UK**: Crown owns gold and silver mines; coal nationalized 1947
///
/// ## Modern Applications
///
/// ### Urban Context
/// - **Metro tunnels**: Grand Paris Express tunnels 30-50m below private property
/// - **Underground parking**: Multi-level parking garages
/// - **Skyscrapers**: Tour Montparnasse (210m), La Défense towers
///
/// ### Rural Context
/// - **Geothermal wells**: Heating systems using earth's heat
/// - **Wine cellars**: Underground aging cellars in Champagne/Bordeaux
/// - **Quarries**: Limestone, gravel extraction (owner's right)
///
/// # Examples
///
/// ```text
/// // Example 1: Parisian building with cellar
/// // Owner of building at street level also owns:
/// // - Airspace above (could build higher with permits)
/// // - Basement/cellar below (wine storage, parking)
/// // - But NOT minerals >150m depth (state owned)
/// // - Subject to metro tunnels (public utility easement)
///
/// // Example 2: Norman farmland
/// // Owner of agricultural land owns:
/// // - Airspace for buildings, trees (subject to aviation)
/// // - Subsoil for wells, foundations
/// // - Gravel/limestone quarry on surface (quarry rights)
/// // - But NOT coal or metal ores if discovered (state mining rights)
/// ```
pub fn article548() -> Statute {
    Statute::new(
        "code-civil-548",
        "Code civil Article 548 - Propriété du dessus et dessous / Ownership above and below",
        Effect::new(
            EffectType::Grant,
            "Land ownership extends vertically to airspace above and subsoil below",
        )
        .with_parameter("vertical_extent", "above_and_below")
        .with_parameter("above_rights", "airspace_building")
        .with_parameter("below_rights", "subsoil_excavation")
        .with_parameter("maxim", "ad_coelum_et_ad_inferos")
        .with_parameter("limitation_1", "state_mineral_rights")
        .with_parameter("limitation_2", "aviation_overflight")
        .with_parameter("limitation_3", "public_utility_easements"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::AttributeEquals {
            key: "property_type".to_string(),
            value: "immovable".to_string(),
        }),
        Box::new(Condition::HasAttribute {
            key: "owner".to_string(),
        }),
    ))
    .with_discretion(
        "Vertical extent: Land ownership extends above (airspace) and below (subsoil) - Roman \
         maxim 'ad coelum et ad inferos'. Above: building rights, wind turbines, but subject to \
         aviation overflight. Below: cellars, foundations, wells, but NOT deep minerals (state \
         owned via Code minier). Major limitations: state mineral rights (mines >150m), aviation, \
         metro tunnels, archaeological finds, water law, planning/zoning. Modern issues: Grand Paris \
         metro tunnels, geothermal wells, skyscrapers, drones. Compare: Germany BGB §905 (limited by \
         interest), Japan Minpō §207 (extent necessary for use), Common Law ad coelum (heavily limited)."
    )
}

/// Articles 571-572 - Movable property ownership and transformation
///
/// # French Text
///
/// **Article 571**:
/// "Dans les meubles, la possession vaut titre."
///
/// **Article 572** (combined with related principles):
/// "La propriété des meubles peut aussi s'acquérir par accession ou incorporation,
/// ou par le travail et la spécification."
///
/// # English Translation
///
/// **Article 571**:
/// "For movable property, possession is equivalent to title."
///
/// **Article 572**:
/// "Ownership of movables may also be acquired by accession or incorporation,
/// or by labor and specification."
///
/// # Legal Commentary
///
/// These articles establish fundamental principles for **movable property** (personal property),
/// distinct from immovable property (real estate) rules.
///
/// ## Article 571: Possession as Title
///
/// **"En fait de meubles, possession vaut titre"** - This famous principle means:
/// - **Good faith possessor** is presumed to be owner
/// - **Protects commerce**: Buyers can trust sellers in possession
/// - **No investigation required**: Don't need to verify title chain
/// - **Exception**: Stolen or lost property (Article 2276) - true owner has 3 years to reclaim
///
/// ### Conditions for Protection
/// 1. **Movable property**: Not real estate
/// 2. **Actual possession**: Physical control, not just paper title
/// 3. **Good faith**: Possessor believes they are rightful owner
/// 4. **Current possession**: Must have possession when claiming
///
/// ### Policy Rationale
/// - Immovables have registries (cadastre) - easy to verify ownership
/// - Movables circulate rapidly - registry impractical
/// - Protect innocent purchasers and commercial certainty
/// - Balance between true owner and good faith possessor
///
/// ### Exceptions
/// **Stolen or lost goods** (Article 2276):
/// - True owner can reclaim within **3 years** of theft/loss
/// - After 3 years, possessor's title becomes unassailable
/// - Purchaser from merchant/market gets special protection (must be reimbursed)
///
/// ## Article 572: Specification and Transformation
///
/// **Specification** (spécification): Creating new object from materials
///
/// ### Ownership Rules (Articles 570-577)
/// **If materials belong to one person, labor by another**:
/// - Value of materials > value of labor → Material owner keeps object, pays for labor
/// - Value of labor > value of materials → Laborer keeps object, pays for materials
///
/// **If materials from multiple sources**:
/// - Principal materials vs. accessory materials
/// - Principal material owner gets object, compensates others
///
/// ### Examples of Specification
/// - **Painting**: Artist paints on canvas owned by another
///   - If canvas worth €10, painting worth €10,000 → Artist keeps painting, pays €10
///   - If canvas worth €5,000 (rare material), painting worth €1,000 → Canvas owner keeps, pays €1,000
///
/// - **Sculpture**: Sculptor carves marble block owned by another
///   - If marble €1,000, sculpture €50,000 → Sculptor keeps, pays €1,000
///   - If rare marble €100,000, sculpture €10,000 → Marble owner keeps, pays €10,000
///
/// - **Furniture**: Carpenter builds table from wood owned by another
///   - If wood €200, table worth €2,000 → Carpenter keeps table, pays €200
///   - If rare wood €5,000, table worth €1,000 → Wood owner keeps, pays €1,000
///
/// - **Wine**: Winemaker produces wine from grapes owned by another
///   - Typically grapes < wine value → Winemaker keeps wine, pays for grapes
///
/// - **Software**: Developer writes code on computer owned by another
///   - Code value (intellectual property) > computer value → Developer owns IP
///
/// ### Accession for Movables
/// **Union or incorporation** of movables:
/// - **Inseparable union**: Object cannot be separated without damage
///   - Example: Diamonds set in gold ring - ring owner owns combined object
/// - **Separable union**: Can separate without damage
///   - Each owner keeps their materials
///
/// **Principal vs. Accessory**:
/// - Principal thing attracts accessory thing
/// - Examples:
///   - Frame + painting → Painting is principal (artistic value)
///   - Sword + scabbard → Sword is principal (weapon)
///   - Watch + strap → Watch is principal (mechanism)
///
/// ## Historical Context
/// These principles derive from Roman law:
/// - **Possessio**: Physical control with intent to own
/// - **Specificatio**: Creating new species from materials (Justinian's Digest)
/// - **Accessio**: Accessory follows principal thing
///
/// The 1804 Napoleonic Code adopted Roman rules adapted to commercial society.
/// Article 571's "possession vaut titre" protected emerging consumer markets.
///
/// Modern applications:
/// - **Art world**: Specification rules for commissioned vs. unsolicited artwork
/// - **Manufacturing**: Subcontractors using client's materials
/// - **Agriculture**: Crops grown from seeds on rented equipment
/// - **Technology**: 3D printing using another's materials
/// - **Fashion**: Designer creating dress from client's fabric
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §929-936)**: Possession + agreement transfers ownership, good faith protection
/// - **Japan (Minpō §192)**: Peaceful, public possession of movables with good faith = instant ownership
/// - **Common Law**: Nemo dat quod non habet (cannot give what you don't have) - stricter than French law
/// - **UCC (USA §2-403)**: Good faith purchaser from merchant gets good title
/// - **England**: Sale of Goods Act - limited protection for good faith purchasers
///
/// ## Modern Controversies
///
/// ### Art Market
/// - Stolen artworks possessed in good faith for >3 years
/// - Holocaust-era art restitution claims (exceeds 3-year limit?)
/// - UNESCO Convention vs. Article 571 tensions
///
/// ### Digital Assets
/// - Does "possession" apply to Bitcoin, NFTs?
/// - Possession requires physical control - digital = possession?
/// - Evolving jurisprudence on intangible movables
///
/// ### Sharing Economy
/// - Airbnb furnishings, car-sharing vehicles
/// - Possession by temporary users vs. owners
/// - Short-term possession insufficient for Article 571
///
/// # Examples
///
/// ```
/// use legalis_fr::property::{Property, PropertyType};
///
/// // Example 1: Article 571 - Good faith purchaser
/// // Alice sells car to Bob (but Alice stole it from Carol)
/// // Bob possesses car in good faith
/// // Result:
/// // - If Carol reports theft, she has 3 years to reclaim car (Article 2276)
/// // - If >3 years pass, Bob owns car (possession vaut titre)
/// // - If Bob bought from licensed dealer, Carol must reimburse Bob's purchase price
///
/// // Example 2: Article 572 - Specification
/// // Sculptor uses marble block worth €1,000 owned by Pierre
/// // Creates sculpture worth €50,000
/// // Result:
/// // - Labor value (€50,000) > material value (€1,000)
/// // - Sculptor keeps sculpture, pays Pierre €1,000 for marble
/// // - Pierre does NOT own sculpture (labor created new species)
///
/// // Example 3: Accession - Inseparable union
/// // Jeweler sets diamonds (€20,000) in gold ring (€500) owned by Marie
/// // Diamonds cannot be removed without damage
/// // Result:
/// // - Diamonds = principal (higher value)
/// // - Diamond owner owns complete ring, pays Marie €500 for gold
/// ```
pub fn article571_572() -> Statute {
    Statute::new(
        "code-civil-571-572",
        "Code civil Articles 571-572 - Meubles: possession vaut titre, spécification / \
         Movables: possession as title, specification",
        Effect::new(
            EffectType::Grant,
            "Good faith possessor of movables presumed owner; creator of new object from \
             materials owns object if labor exceeds material value",
        )
        .with_parameter("art571_principle", "possession_vaut_titre")
        .with_parameter("art571_condition", "good_faith_possession")
        .with_parameter("art571_exception", "stolen_lost_3years")
        .with_parameter("art572_specification", "labor_vs_materials_value")
        .with_parameter("art572_accession", "principal_attracts_accessory")
        .with_parameter("origin", "roman_law_specificatio"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::Or(
        Box::new(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "property_type".to_string(),
                value: "movable".to_string(),
            }),
            Box::new(Condition::And(
                Box::new(Condition::HasAttribute {
                    key: "possessor".to_string(),
                }),
                Box::new(Condition::AttributeEquals {
                    key: "good_faith".to_string(),
                    value: "true".to_string(),
                }),
            )),
        )),
        Box::new(Condition::And(
            Box::new(Condition::HasAttribute {
                key: "specification_materials".to_string(),
            }),
            Box::new(Condition::HasAttribute {
                key: "specification_labor".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Movable property rules: (1) Article 571 - 'En fait de meubles, possession vaut titre' - \
         good faith possessor presumed owner, protects commerce and innocent purchasers, exception \
         for stolen/lost goods (3-year reclaim period, Article 2276); (2) Article 572 - Specification \
         (creating new object from materials) - labor value > material value → laborer owns object, \
         pays for materials; material value > labor → material owner keeps object, pays for labor. \
         Accession for movables: principal attracts accessory (diamonds in ring). Roman law origin \
         (specificatio, possessio). Modern issues: art theft restitution, digital assets (Bitcoin, \
         NFTs), 3D printing. Compare: Germany BGB §929-936, Japan Minpō §192 (instant ownership), \
         Common Law nemo dat (stricter), UCC §2-403 (merchant protection)."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article544_creation() {
        let statute = article544();
        assert_eq!(statute.id, "code-civil-544");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("544"));
        assert!(statute.title.contains("Propriété absolue"));
    }

    #[test]
    fn test_article544_absolute_ownership() {
        let statute = article544();
        assert!(matches!(statute.effect.effect_type, EffectType::Grant));
        assert!(statute.effect.parameters.contains_key("right_usus"));
        assert!(statute.effect.parameters.contains_key("right_fructus"));
        assert!(statute.effect.parameters.contains_key("right_abusus"));
        assert_eq!(statute.effect.parameters.get("right_usus").unwrap(), "use");
    }

    #[test]
    fn test_article545_creation() {
        let statute = article545();
        assert_eq!(statute.id, "code-civil-545");
        assert!(statute.title.contains("Expropriation"));
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
    }

    #[test]
    fn test_article545_preconditions() {
        let statute = article545();
        let preconditions = &statute.preconditions;
        assert_eq!(preconditions.len(), 1);

        // Verify nested And structure for three requirements
        if let Condition::And(box1, box2) = &preconditions[0] {
            match (&**box1, &**box2) {
                (Condition::AttributeEquals { key, value }, Condition::And(..)) => {
                    assert_eq!(key, "purpose");
                    assert_eq!(value, "public_utility");
                }
                _ => panic!("Expected AttributeEquals and nested And"),
            }
        } else {
            panic!("Expected top-level And condition");
        }
    }

    #[test]
    fn test_article546_creation() {
        let statute = article546();
        assert_eq!(statute.id, "code-civil-546");
        assert!(statute.title.contains("accession"));
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("superficie solo cedit")
        );
    }

    #[test]
    fn test_article546_accession_rights() {
        let statute = article546();
        assert!(statute.effect.parameters.contains_key("right_to_fruits"));
        assert!(statute.effect.parameters.contains_key("right_to_additions"));
        assert_eq!(
            statute.effect.parameters.get("right_to_fruits").unwrap(),
            "natural_industrial_civil"
        );
        assert_eq!(
            statute.effect.parameters.get("principle").unwrap(),
            "superficie_solo_cedit"
        );
    }

    #[test]
    fn test_article548_creation() {
        let statute = article548();
        assert_eq!(statute.id, "code-civil-548");
        assert!(statute.title.contains("dessus et dessous"));
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("ad coelum")
        );
    }

    #[test]
    fn test_article548_vertical_extent() {
        let statute = article548();
        assert!(statute.effect.parameters.contains_key("vertical_extent"));
        assert!(statute.effect.parameters.contains_key("above_rights"));
        assert!(statute.effect.parameters.contains_key("below_rights"));
        assert_eq!(
            statute.effect.parameters.get("vertical_extent").unwrap(),
            "above_and_below"
        );
    }

    #[test]
    fn test_article571_572_creation() {
        let statute = article571_572();
        assert_eq!(statute.id, "code-civil-571-572");
        assert!(statute.title.contains("571-572"));
        assert!(statute.title.contains("possession vaut titre"));
    }

    #[test]
    fn test_article571_572_movable_principles() {
        let statute = article571_572();
        assert!(statute.effect.parameters.contains_key("art571_principle"));
        assert!(
            statute
                .effect
                .parameters
                .contains_key("art572_specification")
        );
        assert_eq!(
            statute.effect.parameters.get("art571_principle").unwrap(),
            "possession_vaut_titre"
        );
        assert_eq!(
            statute
                .effect
                .parameters
                .get("art572_specification")
                .unwrap(),
            "labor_vs_materials_value"
        );
    }

    #[test]
    fn test_all_ownership_articles_have_jurisdiction() {
        let articles = vec![
            article544(),
            article545(),
            article546(),
            article548(),
            article571_572(),
        ];

        for article in articles {
            assert_eq!(article.jurisdiction.as_deref(), Some("FR"));
        }
    }

    #[test]
    fn test_all_ownership_articles_have_discretion() {
        let articles = vec![
            article544(),
            article545(),
            article546(),
            article548(),
            article571_572(),
        ];

        for article in articles {
            assert!(article.discretion_logic.is_some());
            assert!(article.discretion_logic.unwrap().len() > 100);
        }
    }

    #[test]
    fn test_article544_has_preconditions() {
        let statute = article544();
        assert_eq!(statute.preconditions.len(), 2);
    }

    #[test]
    fn test_article546_has_or_precondition() {
        let statute = article546();
        assert_eq!(statute.preconditions.len(), 1);

        if let Condition::Or(box1, box2) = &statute.preconditions[0] {
            match (&**box1, &**box2) {
                (Condition::HasAttribute { key: key1 }, Condition::HasAttribute { key: key2 }) => {
                    assert_eq!(key1, "produces_fruits");
                    assert_eq!(key2, "has_additions");
                }
                _ => panic!("Expected two HasAttribute conditions"),
            }
        } else {
            panic!("Expected Or condition");
        }
    }

    #[test]
    fn test_article548_preconditions_immovable() {
        let statute = article548();
        assert_eq!(statute.preconditions.len(), 1);

        if let Condition::And(box1, box2) = &statute.preconditions[0] {
            match (&**box1, &**box2) {
                (Condition::AttributeEquals { key, value }, Condition::HasAttribute { .. }) => {
                    assert_eq!(key, "property_type");
                    assert_eq!(value, "immovable");
                }
                _ => panic!("Expected AttributeEquals and HasAttribute"),
            }
        } else {
            panic!("Expected And condition");
        }
    }

    #[test]
    fn test_article571_572_complex_precondition() {
        let statute = article571_572();
        assert_eq!(statute.preconditions.len(), 1);

        // Verify it's an Or with two complex And conditions
        if let Condition::Or(box1, box2) = &statute.preconditions[0] {
            // Both sides should be And conditions
            assert!(matches!(&**box1, Condition::And(..)));
            assert!(matches!(&**box2, Condition::And(..)));
        } else {
            panic!("Expected Or condition at top level");
        }
    }

    #[test]
    fn test_all_articles_have_parameters() {
        let articles = vec![
            article544(),
            article545(),
            article546(),
            article548(),
            article571_572(),
        ];

        for article in articles {
            assert!(!article.effect.parameters.is_empty());
            assert!(article.effect.parameters.len() >= 3);
        }
    }

    #[test]
    fn test_article545_constitutional_basis() {
        let statute = article545();
        assert!(statute.effect.parameters.contains_key("basis"));
        assert_eq!(
            statute.effect.parameters.get("basis").unwrap(),
            "declaration_of_human_rights_1789_art17"
        );
    }

    #[test]
    fn test_article546_roman_law_origin() {
        let statute = article546();
        assert!(statute.effect.parameters.contains_key("origin"));
        assert_eq!(
            statute.effect.parameters.get("origin").unwrap(),
            "roman_law"
        );
    }

    #[test]
    fn test_article548_limitations() {
        let statute = article548();
        assert!(statute.effect.parameters.contains_key("limitation_1"));
        assert!(statute.effect.parameters.contains_key("limitation_2"));
        assert!(statute.effect.parameters.contains_key("limitation_3"));
        assert_eq!(
            statute.effect.parameters.get("limitation_1").unwrap(),
            "state_mineral_rights"
        );
    }

    #[test]
    fn test_article571_exception() {
        let statute = article571_572();
        assert!(statute.effect.parameters.contains_key("art571_exception"));
        assert_eq!(
            statute.effect.parameters.get("art571_exception").unwrap(),
            "stolen_lost_3years"
        );
    }
}
