//! Easement and servitude articles (Code civil, Book II)
//!
//! This module implements articles governing easements (servitudes) in French property law,
//! including water rights, legal easements, neighbor rights, and forced passage.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 555 - Forced servitude for drinking water and livestock
///
/// # French Text
/// "Néanmoins celui sur le fonds duquel se trouve une source, peut en user à son passage pour
/// les besoins de sa propriété; dans le cas même où les propriétaires des fonds inférieurs
/// auraient fait à son extrémité, et avant son entrée dans leur héritage, des ouvrages
/// ou des travaux pour la conduire sur leurs fonds, celui auquel elle appartient peut
/// réclamer pour les besoins de sa culture une quantité d'eau proportionnée à son héritage,
/// sauf l'indemnité qui est due aux propriétaires des fonds inférieurs."
///
/// # English Translation
/// "Nevertheless, the person on whose land a spring is located may use it as it passes through
/// for the needs of their property; even if the owners of lower lands have made works or
/// constructions at its extremity and before it enters their property to conduct it onto their
/// lands, the owner to whom it belongs may claim for the needs of their cultivation a quantity
/// of water proportional to their property, subject to compensation due to the owners of lower lands."
///
/// # Legal Commentary
/// Article 555 establishes the **forced servitude for essential water** (servitude forcée pour eau
/// potable et abreuvement), balancing property rights with fundamental human needs for drinking
/// water and livestock care. This is one of the most important legal easements in French law.
///
/// ## Core Principle: Essential Water Rights
/// **Public health and humanitarian necessity** override absolute property rights:
/// - **Drinking water** (eau potable) for human consumption
/// - **Livestock watering** (abreuvement du bétail) for agricultural animals
/// - **Essential needs** take precedence over property boundaries
/// - **Proportional allocation** based on property size and need
///
/// ## Conditions for Forced Water Servitude
/// 1. **Essential need**: Water required for drinking or livestock (not luxury/irrigation)
/// 2. **No alternative source**: Claimant lacks accessible water on own property
/// 3. **Spring on neighbor's land**: Natural water source exists nearby
/// 4. **Proportional claim**: Amount requested proportional to property needs
/// 5. **Fair compensation**: Indemnity paid to servient estate owner
///
/// ## Water Source Hierarchy
/// **Order of priority**:
/// 1. Spring owner's needs (cultivation, drinking, livestock)
/// 2. Downstream riparian owners' existing works
/// 3. Essential needs of landlocked neighbors
/// 4. Other agricultural/industrial uses
///
/// ## Compensation Requirements
/// **Indemnity must cover**:
/// - Value of water taken
/// - Infrastructure costs (pipes, conduits)
/// - Loss to servient estate
/// - Ongoing maintenance burden
/// - Diminution in property value
///
/// ## Historical Context
/// Article 555 dates from 1804 Napoleonic Code, reflecting agricultural society's critical
/// dependence on water access. In 19th century rural France, water rights were matters of
/// life and death for farms without wells or springs.
///
/// Key developments:
/// - **1804**: Original article prioritizing human/animal survival
/// - **1845**: Cour de Cassation clarified "essential needs" doctrine
/// - **1898**: Water pollution law added quality protections
/// - **1964**: Water Law (Loi sur l'eau) integrated environmental concerns
/// - **1992**: Water Act expanded ecosystem protection
/// - **2006**: Water and Aquatic Environments Act (LEMA) modernized regime
///
/// ## Modern Applications
///
/// ### Rural Context
/// - **Farms without wells**: Claiming water from neighbor's spring
/// - **Livestock operations**: Cattle, sheep, horses requiring daily water
/// - **Rural homes**: Drinking water for families without mains connection
/// - **Organic farms**: Clean water for crop irrigation (if essential)
///
/// ### Environmental Tensions
/// - **Ecosystem protection**: Springs support wetlands, wildlife
/// - **Climate change**: Drought increases competition for water
/// - **Water scarcity**: Summer restrictions on non-essential use
/// - **Pollution concerns**: Quality testing required for drinking water
///
/// ## Limitations and Exclusions
/// **Not covered by Article 555**:
/// - **Industrial use**: Manufacturing, processing (commercial purposes)
/// - **Irrigation**: Agricultural watering beyond essential cultivation
/// - **Swimming pools**: Luxury/recreational uses
/// - **Commercial bottling**: Selling spring water
/// - **Excessive quantities**: More than proportional to property needs
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §916-924)**: Water easements, but no forced servitude for drinking
/// - **Japan (Minpō §220-238)**: Legal easements, water rights regulated separately
/// - **Common Law**: No automatic right to neighbor's water; riparian rights differ
/// - **USA**: Water law varies by state - prior appropriation (West) vs. riparian (East)
/// - **Swiss Civil Code (Art. 663-705)**: Similar forced servitude for essential water
///
/// ## Related Articles
/// - **Article 640-649**: General water rights and drainage easements
/// - **Article 642**: Riparian owners' use rights
/// - **Article 643**: Forced passage of irrigation water
/// - **Article 644**: Water abstraction limits
/// - **Code de l'environnement**: Water quality, ecosystem protection
///
/// ## Practical Procedure
/// 1. **Negotiation**: Request access from spring owner
/// 2. **Expert appraisal**: Determine proportional water needs
/// 3. **Court order** (if refused): Judge establishes servitude
/// 4. **Indemnity calculation**: Court determines fair compensation
/// 5. **Infrastructure**: Claimant installs pipes/conduits at own cost
/// 6. **Registration**: Servitude recorded in land registry
///
/// # Examples
///
/// ```text
/// // Example 1: Dairy farm without water source
/// // Farm A (50 cows, no well) neighbors Farm B (natural spring)
/// // Farm A claims Article 555 forced servitude:
/// // - 200 liters/day for 50 cows (4,000 L total)
/// // - 100 liters/day for farmer's family
/// // - Court orders servitude with €2,000/year compensation
/// // - Farm A installs pipes at €10,000 cost
///
/// // Example 2: Rural residence without mains water
/// // House C (family of 4) neighbors property with spring
/// // Claims 400 liters/day for household use (drinking, cooking, hygiene)
/// // Court grants servitude with €500/year indemnity
/// // Spring owner retains primary use for own cultivation
/// ```
pub fn article555() -> Statute {
    Statute::new(
        "code-civil-555",
        "Code civil Article 555 - Servitude forcée pour eau potable / Forced servitude for drinking water",
        Effect::new(
            EffectType::Grant,
            "Owner may claim proportional water from neighbor's spring for essential drinking and \
             livestock needs, with fair compensation to spring owner",
        )
        .with_parameter("right", "essential_water_access")
        .with_parameter("purpose_1", "drinking_water_human")
        .with_parameter("purpose_2", "livestock_watering")
        .with_parameter("basis", "public_health_humanitarian_necessity")
        .with_parameter("condition", "proportional_to_property_needs")
        .with_parameter("compensation", "fair_indemnity_required")
        .with_parameter("priority", "spring_owner_primary_rights")
        .with_parameter("limitation", "essential_needs_only"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "water_need_type".to_string(),
                value: "drinking_water".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "water_need_type".to_string(),
                value: "livestock_watering".to_string(),
            }),
        )),
        Box::new(Condition::And(
            Box::new(Condition::HasAttribute {
                key: "neighbor_has_spring".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "no_alternative_source".to_string(),
                value: "true".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Forced water servitude: Essential drinking water and livestock watering override property \
         boundaries. Spring owner has primary rights, but must allow proportional access for \
         neighbors' essential needs. Requires fair compensation (indemnity). Public health and \
         humanitarian necessity justify forced easement. Not for commercial/luxury use (swimming \
         pools, bottling, industrial). Historical: 1804 agricultural society's life-or-death water \
         dependence. Modern tensions: drought, climate change, ecosystem protection. Compare: \
         Germany BGB §916-924 (no forced servitude), Common Law riparian rights, Swiss CC Art. 663 \
         (similar forced access). Related: Articles 640-649 (water easements), Code de l'environnement."
    )
}

/// Articles 640-649 - Water rights and drainage easements
///
/// # French Text (Key Articles)
///
/// **Article 640**:
/// "Les fonds inférieurs sont assujettis envers ceux qui sont plus élevés à recevoir les eaux
/// qui en découlent naturellement sans que la main de l'homme y ait contribué."
///
/// **Article 641**:
/// "Tout propriétaire a le droit d'user et de disposer des eaux pluviales qui tombent sur son fonds."
///
/// **Article 642**:
/// "Celui qui a une source dans son fonds peut en user à sa volonté."
///
/// **Article 643**:
/// "Le propriétaire d'une source ne peut en changer le cours lorsqu'elle fournit aux habitants
/// d'une commune, village ou hameau, l'eau qui leur est nécessaire."
///
/// # English Translation
///
/// **Article 640**:
/// "Lower lands are subject to receive from higher lands water that flows naturally without
/// human intervention."
///
/// **Article 641**:
/// "Every owner has the right to use and dispose of rainwater that falls on their land."
///
/// **Article 642**:
/// "One who has a spring on their land may use it at will."
///
/// **Article 643**:
/// "The owner of a spring cannot change its course when it supplies water necessary to
/// inhabitants of a commune, village or hamlet."
///
/// # Legal Commentary
/// Articles 640-649 establish the **fundamental water rights regime** in French property law,
/// balancing natural drainage, private use, and public necessity. These articles govern
/// **riparian rights** (droits riverains) and water easements.
///
/// ## Article 640: Natural Drainage Easement
/// **Servitude naturelle d'écoulement** - Legal easement requiring lower lands to receive water
/// flowing naturally from higher lands.
///
/// ### Conditions
/// 1. **Natural flow**: Water must flow by gravity, not human-directed
/// 2. **No human intervention**: Landowner cannot create or increase flow
/// 3. **Higher to lower**: Topography determines servient/dominant estates
/// 4. **Continuous or periodic**: Regular flow (streams) or seasonal (snowmelt)
///
/// ### Limitations on Lower Landowner
/// **Must accept**:
/// - Natural rainfall runoff
/// - Snowmelt from higher elevations
/// - Spring water flowing naturally
/// - Groundwater seepage
///
/// **Need not accept**:
/// - Artificially diverted water
/// - Concentrated flows from buildings/pipes
/// - Polluted water from industrial activity
/// - Increased volume from development
///
/// ### Limitations on Upper Landowner
/// **Cannot**:
/// - Concentrate natural flow (e.g., channel into pipe aimed at neighbor)
/// - Increase volume artificially (e.g., pump water to create flow)
/// - Divert from natural course to burden different lower land
/// - Add pollutants to natural water
///
/// **May**:
/// - Use water on own property before it flows down
/// - Build structures if not increasing burden on lower land
/// - Collect rainwater for own use (Article 641)
///
/// ## Article 641: Rainwater Rights
/// **Exclusive ownership** of rainwater falling on one's property:
/// - Owner may collect, use, or dispose of rainwater
/// - Popular modern application: Rainwater harvesting systems
/// - Exception: Cannot concentrate/divert to harm neighbors
///
/// ## Article 642: Spring Rights
/// **Private ownership** of springs on one's land:
/// - Spring owner has absolute use rights (within limits)
/// - May use for irrigation, livestock, domestic purposes
/// - May sell bottled spring water (subject to regulation)
///
/// ### Limitations on Spring Owners
/// 1. **Article 643**: Cannot divert if supplying village water needs
/// 2. **Article 555**: Must allow forced servitude for essential drinking water
/// 3. **Environmental law**: Cannot deplete ecosystem-sustaining springs
/// 4. **Quality protection**: Must not pollute spring water
///
/// ## Article 643: Village Water Supply
/// **Public necessity exception**: Spring owner loses diversion right when spring supplies
/// essential water to community. Protects public health and historic water supply.
///
/// ### Coverage
/// - **Communes**: Municipal water supply
/// - **Villages**: Small settlements
/// - **Hameaux**: Hamlets without alternative water
///
/// ### Requirements
/// - **Historical use**: Community traditionally relied on spring
/// - **Necessary water**: Essential for human consumption, not luxury
/// - **No alternatives**: Community lacks other adequate water source
/// - **Proportional limit**: Owner retains rights to excess water
///
/// ## Historical Context
/// These articles date from 1804 Napoleonic Code, codifying centuries of customary water law
/// derived from Roman law and medieval French customs. Water management was critical for:
/// - **Agriculture**: Irrigation, livestock watering
/// - **Mills**: Water-powered flour mills, sawmills
/// - **Villages**: Communal springs for drinking water
/// - **Industry**: 19th century textile/tanning operations
///
/// Key developments:
/// - **1845**: Cour de Cassation defined "natural flow" doctrine
/// - **1898**: Law on pollution of waters
/// - **1919**: Hydroelectric power law (force majeure exception)
/// - **1964**: Water Law (Loi sur l'eau) - integrated water management
/// - **1992**: Water Act - ecosystem protection, quality standards
/// - **2006**: LEMA (Water and Aquatic Environments Act) - sustainability focus
/// - **2010s**: Climate change adaptations, drought management
///
/// ## Modern Applications
///
/// ### Urban Context
/// - **Runoff management**: Storm drains, impervious surfaces
/// - **Flooding prevention**: Upper properties causing flood damage
/// - **Rainwater harvesting**: Building cisterns for toilet/garden use
/// - **Green infrastructure**: Rain gardens, permeable pavement
///
/// ### Rural Context
/// - **Agricultural irrigation**: Spring and stream diversions
/// - **Livestock watering**: Cattle access to streams (Article 642)
/// - **Forest drainage**: Natural flow from wooded uplands
/// - **Wine production**: Springs for vineyard irrigation
///
/// ### Environmental Issues
/// - **Drought management**: Competing water claims during scarcity
/// - **Stream ecosystem**: Maintaining minimum flows for fish/wildlife
/// - **Wetland protection**: Preserving springs feeding wetlands
/// - **Groundwater depletion**: Over-pumping affecting neighbors
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §909-918)**: Similar natural drainage easement, spring rights
/// - **Japan (Civil Code §214-238)**: Legal easements including water flow
/// - **Common Law (Riparian rights)**: Reasonable use of watercourses, natural flow doctrine
/// - **USA (Western states)**: Prior appropriation - "first in time, first in right"
/// - **USA (Eastern states)**: Riparian doctrine - reasonable use by streamside owners
/// - **England**: Riparian rights, drainage easements by prescription
///
/// ## Related Legal Frameworks
/// **Code de l'environnement** (Environmental Code):
/// - Water quality standards (potability, pollution control)
/// - Ecosystem protection (minimum flows, wetland preservation)
/// - Water abstraction permits (licensing for significant use)
/// - Drought restrictions (temporary use limitations)
///
/// **Code rural et de la pêche maritime** (Rural Code):
/// - Agricultural water rights
/// - Irrigation associations (ASA - Associations Syndicales Autorisées)
/// - Drainage improvement districts
///
/// ## Dispute Resolution
/// **Typical conflicts**:
/// 1. **Upper owner concentrates flow**: Installing pipes/channels harming lower land
/// 2. **Lower owner blocks drainage**: Building dams/barriers causing flooding
/// 3. **Spring depletion**: Over-pumping affecting downstream users
/// 4. **Pollution**: Contaminated runoff from upper property
///
/// **Legal remedies**:
/// - **Injunction**: Stop artificial flow concentration
/// - **Damages**: Compensation for flood damage, lost crops
/// - **Court-ordered easement**: Establish drainage infrastructure
/// - **Criminal penalties**: For pollution violations
///
/// # Examples
///
/// ```text
/// // Example 1: Natural drainage (Article 640)
/// // Mountain property naturally drains to valley farm below
/// // Valley farmer must accept natural snowmelt/rainwater
/// // BUT mountain owner cannot install downspout concentrating flow
/// // onto one spot of valley farm - that's artificial, not natural
///
/// // Example 2: Spring ownership (Article 642)
/// // Farm has natural spring producing 10,000 liters/day
/// // Owner uses 3,000 L/day for livestock, bottles 5,000 L/day for sale
/// // Subject to Article 643 if village needs water
/// // Subject to Article 555 if neighbor needs drinking water
///
/// // Example 3: Village water supply (Article 643)
/// // Spring owner wants to bottle entire spring output
/// // But spring has supplied village fountain for 200 years
/// // 500 residents rely on spring (no other source)
/// // Court orders: Owner must leave 2,000 L/day for village
/// // Owner may bottle remaining 8,000 L/day
/// // Receives compensation for public service burden
/// ```
pub fn article640_649() -> Statute {
    Statute::new(
        "code-civil-640-649",
        "Code civil Articles 640-649 - Servitudes d'eau et d'écoulement / Water and drainage easements",
        Effect::new(
            EffectType::Grant,
            "Lower lands must receive natural water flow from higher lands; rainwater ownership; \
             spring owners' use rights subject to village supply and essential needs",
        )
        .with_parameter("art640_natural_drainage", "lower_receives_natural_flow")
        .with_parameter("art641_rainwater", "owner_exclusive_rights")
        .with_parameter("art642_spring", "owner_use_at_will")
        .with_parameter("art643_village", "cannot_divert_community_supply")
        .with_parameter("limitation", "no_artificial_concentration")
        .with_parameter("basis", "natural_servitude")
        .with_parameter("origin", "roman_law_riparian_rights"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::Or(
        Box::new(Condition::And(
            Box::new(Condition::AttributeEquals {
                key: "water_flow_type".to_string(),
                value: "natural".to_string(),
            }),
            Box::new(Condition::HasAttribute {
                key: "higher_to_lower_terrain".to_string(),
            }),
        )),
        Box::new(Condition::Or(
            Box::new(Condition::HasAttribute {
                key: "has_spring".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "water_type".to_string(),
                value: "rainwater".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Water rights framework: (1) Article 640 - natural drainage easement, lower lands must \
         receive water flowing naturally from higher lands (gravity, no human intervention), upper \
         owner cannot concentrate or divert artificially; (2) Article 641 - rainwater ownership, \
         exclusive rights to rainwater on property (modern: harvesting systems); (3) Article 642 - \
         spring ownership, use at will subject to limitations; (4) Article 643 - village water \
         protection, cannot divert spring supplying community needs. Historical: 1804 Code, Roman \
         law origins, agricultural/milling economy. Modern: urban runoff, drought management, \
         ecosystem protection, climate change. Compare: Germany BGB §909-918, Common Law riparian \
         rights, USA prior appropriation (West) vs. riparian (East). Related: Code de l'environnement \
         (quality, permits), Article 555 (forced servitude)."
    )
}

/// Articles 667-709 - Legal easements (water, drainage, support, light)
///
/// # French Text (Key Articles)
///
/// **Article 667**:
/// "Tout propriétaire doit établir des toits de manière que les eaux pluviales s'écoulent sur
/// son terrain ou sur la voie publique; il ne peut les faire verser sur le fonds de son voisin."
///
/// **Article 673**:
/// "Celui sur la propriété duquel avancent les branches des arbres du voisin peut contraindre
/// celui-ci à couper ces branches."
///
/// **Article 674**:
/// "Celui qui fait creuser un puits ou une fosse d'aisance près d'un mur mitoyen ou non, ou qui
/// veut y construire cheminée ou âtre, forge, four ou fourneau, y adosser une étable, ou établir
/// contre ce mur un magasin de sel ou amas de matières corrosives, est obligé à laisser la
/// distance prescrite par les règlements et usages particuliers sur ces objets, ou à faire les
/// ouvrages prescrits par les mêmes règlements et usages, pour éviter de nuire au voisin."
///
/// **Article 675**:
/// "Tout propriétaire peut obliger son voisin au bornage de leurs propriétés contiguës."
///
/// # English Translation
///
/// **Article 667**:
/// "Every owner must construct roofs so that rainwater flows onto their own land or public way;
/// they cannot make it pour onto their neighbor's property."
///
/// **Article 673**:
/// "One whose property is encroached by branches from neighbor's trees may compel the neighbor
/// to cut these branches."
///
/// **Article 674**:
/// "One who digs a well or cesspool near a party wall or not, or who wishes to build a chimney
/// or hearth, forge, oven or furnace, place a stable against it, or establish a salt warehouse
/// or pile of corrosive materials against this wall, is obliged to leave the distance prescribed
/// by particular regulations and customs, or to do the works prescribed by the same regulations
/// and customs, to avoid harming the neighbor."
///
/// **Article 675**:
/// "Every owner may compel their neighbor to mark the boundaries of their adjoining properties."
///
/// # Legal Commentary
/// Articles 667-709 establish **legal easements** (servitudes légales) - mandatory restrictions
/// on property use to prevent harm to neighbors and maintain peaceful coexistence. These are
/// **servitudes de droit** (easements by operation of law), not requiring agreement or registration.
///
/// ## Article 667: Roof Drainage Prohibition
/// **Prohibition des égouts sur le terrain du voisin** - Cannot direct roof water onto neighbor's land.
///
/// ### Core Principle
/// - **Each property must manage its own rainwater**
/// - **No drip line onto neighbor**: Gutters, eaves must direct water away
/// - **Natural vs. artificial flow**: Article 640 allows natural flow, but not artificial concentration
/// - **Public way exception**: May direct to street/public drainage
///
/// ### Practical Applications
/// **Proper drainage**:
/// - Gutters channeling to own land or public street
/// - Eaves extending over own property boundary
/// - Downspouts discharging onto own parcel
/// - Splash blocks directing water away from boundaries
///
/// **Prohibited**:
/// - Downspout aimed at neighbor's property
/// - Eaves dripping onto neighbor's land
/// - Gutters directing flow across boundary
/// - Concentrated discharge onto neighbor
///
/// ### Remedies for Violations
/// - **Injunction**: Remove offending drainage
/// - **Damages**: Compensation for water damage, flooding
/// - **Court order**: Install proper drainage at violator's expense
/// - **Urgent work**: Neighbor may fix and seek reimbursement
///
/// ## Article 673: Overhanging Branches
/// **Élagage des branches** - Right to compel cutting of encroaching branches.
///
/// ### Scope
/// **Branches** (not roots - Article 673 separate):
/// - Overhanging any amount onto neighbor's airspace
/// - Causing nuisance: falling leaves, fruit, shade
/// - Blocking sunlight, views
/// - Interfering with buildings, fences
///
/// ### Procedure
/// 1. **Neighbor requests cutting**: Written notice to tree owner
/// 2. **Reasonable time**: Allow period for compliance (typically 30-60 days)
/// 3. **Court action if refused**: Judge orders cutting
/// 4. **Costs**: Tree owner pays for cutting
/// 5. **Self-help if urgent**: Neighbor may cut at tree owner's expense after formal demand
///
/// ### Fruit and Fallen Objects
/// **Article 673 rule**:
/// - Fallen fruit on neighbor's land belongs to neighbor (natural accession)
/// - Fruit still on branches belongs to tree owner
/// - Neighbor cannot reach over to pick fruit (trespass)
///
/// ## Article 674: Hazardous Installations
/// **Distance requirements** for wells, cesspools, chimneys, forges, stables, corrosive materials.
///
/// ### Protected Against
/// 1. **Wells/cesspools**: Groundwater contamination, structural damage
/// 2. **Chimneys/forges**: Fire risk, smoke, heat
/// 3. **Stables**: Odors, vermin, structural damage
/// 4. **Corrosive materials**: Salt, chemicals damaging walls
///
/// ### Distance Requirements
/// **Varies by local regulation**:
/// - Paris: Typically 1-2 meters from property line
/// - Rural areas: May be greater for agricultural buildings
/// - **Party walls**: Special protections required
/// - **Zoning laws**: Municipal regulations add requirements
///
/// ### Alternative Compliance
/// Instead of distance, owner may install protective works:
/// - **Waterproofing**: For wells near walls
/// - **Fire barriers**: For chimneys, furnaces
/// - **Ventilation**: For stables
/// - **Protective coatings**: For corrosive materials storage
///
/// ## Article 675: Boundary Marking
/// **Bornage** - Right to compel establishment of property boundaries.
///
/// ### Purpose
/// - **Clarify ownership**: End disputes over exact boundaries
/// - **Prevent encroachment**: Establish definitive limits
/// - **Facilitate transactions**: Clear title for sales
///
/// ### Procedure
/// 1. **Request to neighbor**: Informal attempt at agreement
/// 2. **Surveyor** (géomètre-expert): If no agreement, hire professional surveyor
/// 3. **Court action**: If neighbor refuses, judge orders bornage
/// 4. **Cost sharing**: Both neighbors split surveyor costs equally (unless fraud/bad faith)
/// 5. **Boundary markers**: Install permanent markers (stones, posts)
/// 6. **Land registry**: Update cadastre with precise boundaries
///
/// ### Types of Markers
/// - **Bornes**: Stone markers at corners
/// - **Piquets**: Wooden or metal stakes
/// - **Natural markers**: Trees, streams, rocks (if stable)
/// - **Walls/fences**: May serve as boundaries if agreed
///
/// ## Historical Context
/// These legal easements date from 1804 Napoleonic Code, codifying centuries of neighbor law
/// (droit de voisinage) from Roman law and medieval customs. Core concern: preventing **troubles
/// de voisinage** (neighbor nuisances) in densely populated areas.
///
/// Evolution:
/// - **1804**: Original Code provisions
/// - **1840s**: Cour de Cassation develops abuse of rights doctrine (abus de droit)
/// - **1960s**: Urban expansion increases neighbor conflicts
/// - **1978**: Law on construction defects extends neighbor protections
/// - **1990s-present**: Environmental nuisances (noise, odors, pollution) expand legal easement concepts
///
/// ## Modern Applications
///
/// ### Urban Context
/// - **Apartments**: Balcony drainage, window placement, party walls
/// - **Townhouses**: Shared walls, roof drainage, boundary disputes
/// - **Commercial**: Loading docks, ventilation, noise/odor from businesses
///
/// ### Rural Context
/// - **Farms**: Manure storage distances, irrigation runoff, tree branches
/// - **Vineyards**: Spray drift onto neighbors, access roads, water drainage
/// - **Forests**: Fallen trees across boundaries, logging debris
///
/// ## Abuse of Rights Doctrine
/// **Abus de droit** - Even lawful actions may be prohibited if:
/// 1. **Sole purpose is to harm neighbor** (intention de nuire)
/// 2. **Excessive harm disproportionate to benefit**
/// 3. **Violates good faith and neighborly duties**
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §903-924)**: Similar neighbor law, Nachbarrecht
/// - **Japan (Minpō §209-238)**: Legal easements, boundary disputes
/// - **Common Law**: Nuisance torts, boundary by acquiescence, spite fences
/// - **USA**: Varies by state - nuisance law, encroachment, boundary disputes
/// - **Switzerland (Civil Code Art. 684-705)**: Comprehensive neighbor law
///
/// # Examples
///
/// ```text
/// // Example 1: Roof drainage violation (Article 667)
/// // Homeowner installs gutters directing water onto neighbor's patio
/// // Neighbor demands removal within 30 days
/// // If refused, court orders new drainage + €3,000 damages for patio damage
///
/// // Example 2: Overhanging tree branches (Article 673)
/// // Oak tree branches extend 2 meters over neighbor's yard
/// // Falling acorns damage neighbor's car, block sunlight to garden
/// // Neighbor requests cutting, tree owner refuses
/// // Court orders cutting at tree owner's expense (€500)
/// // Fallen acorns on neighbor's land belong to neighbor
///
/// // Example 3: Stable near property line (Article 674)
/// // Farmer builds stable 50 cm from boundary (regulation requires 2m)
/// // Neighbor complains of odors, flies
/// // Court: Either move stable 2m away OR install ventilation + drainage
/// // Farmer chooses ventilation system (€5,000) rather than rebuilding
///
/// // Example 4: Boundary dispute (Article 675)
/// // Neighbors disagree where property line runs (dispute over 3m strip)
/// // One neighbor demands bornage
/// // Surveyor hired (€2,000 cost split equally)
/// // Survey shows line favors first neighbor
/// // Boundary markers installed, cadastre updated
/// // Second neighbor must remove fence encroaching on first neighbor's land
/// ```
pub fn article667_709() -> Statute {
    Statute::new(
        "code-civil-667-709",
        "Code civil Articles 667-709 - Servitudes légales de voisinage / Legal easements (drainage, boundaries, hazards)",
        Effect::new(
            EffectType::Obligation,
            "Mandatory restrictions on property use to prevent neighbor harm: no roof drainage onto \
             neighbor's land, cut overhanging branches, maintain distances for hazardous installations, \
             establish boundaries on demand",
        )
        .with_parameter("art667_roof_drainage", "must_discharge_on_own_land_or_public_way")
        .with_parameter("art673_branches", "neighbor_may_compel_cutting")
        .with_parameter("art674_hazards", "minimum_distances_or_protective_works")
        .with_parameter("art675_boundaries", "right_to_compel_bornage")
        .with_parameter("basis", "servitudes_legales")
        .with_parameter("purpose", "prevent_troubles_de_voisinage")
        .with_parameter("doctrine", "abus_de_droit"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::Or(
        Box::new(Condition::Or(
            Box::new(Condition::HasAttribute {
                key: "roof_drainage_issue".to_string(),
            }),
            Box::new(Condition::HasAttribute {
                key: "overhanging_branches".to_string(),
            }),
        )),
        Box::new(Condition::Or(
            Box::new(Condition::HasAttribute {
                key: "hazardous_installation".to_string(),
            }),
            Box::new(Condition::HasAttribute {
                key: "boundary_dispute".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Legal easements (servitudes légales): Mandatory restrictions by operation of law, no \
         agreement needed. (1) Article 667 - roof drainage must go to own land or public way, not \
         neighbor's property; (2) Article 673 - overhanging branches, neighbor may compel cutting, \
         fallen fruit belongs to neighbor; (3) Article 674 - hazardous installations (wells, \
         cesspools, chimneys, stables, corrosives) require minimum distances or protective works; \
         (4) Article 675 - bornage (boundary marking), either neighbor may compel survey, costs \
         split equally. Purpose: prevent troubles de voisinage (neighbor nuisances). Abuse of rights \
         doctrine (abus de droit): lawful acts prohibited if sole purpose to harm. Historical: 1804 \
         Code, Roman law origins. Modern: urban density, environmental nuisances. Compare: Germany \
         Nachbarrecht, Common Law nuisance torts, Switzerland CC Art. 684-705."
    )
}

/// Articles 710-734 - Neighbor rights and boundaries
///
/// # French Text (Key Articles)
///
/// **Article 710**:
/// "Tout propriétaire peut obliger son voisin à contribuer aux constructions et réparations
/// de la clôture qui sépare leurs héritages."
///
/// **Article 711**:
/// "La clôture à la hauteur de 3 m 20, dans les villes de 50,000 âmes et au-dessus,
/// et 2 m 60 dans les autres, et à la hauteur voulue dans les villes où il existe des
/// règlements particuliers."
///
/// **Article 729**:
/// "Il est libre à chacun de clore son héritage, sauf les exceptions établies par la loi."
///
/// # English Translation
///
/// **Article 710**:
/// "Every owner may compel their neighbor to contribute to the construction and repairs of
/// the fence separating their properties."
///
/// **Article 711**:
/// "The fence at the height of 3.20m in cities of 50,000 inhabitants and above, and 2.60m
/// in others, and at the height required in cities where particular regulations exist."
///
/// **Article 729**:
/// "Everyone is free to enclose their property, subject to exceptions established by law."
///
/// # Legal Commentary
/// Articles 710-734 govern **clôture** (enclosure), **murs mitoyens** (party walls), and
/// **neighborhood rights** in French property law. These provisions balance privacy, security,
/// and cost-sharing between adjacent landowners.
///
/// ## Right to Enclose (Article 729)
/// **Fundamental property right**: Owner may enclose land with fences, walls, hedges.
///
/// ### Purposes of Enclosure
/// - **Privacy**: Shield from neighbors' view
/// - **Security**: Prevent intrusion, theft
/// - **Boundary marking**: Clear property limits
/// - **Animal containment**: Livestock, pets
/// - **Aesthetic**: Landscaping, appearance
///
/// ### Limitations
/// **Cannot enclose if**:
/// 1. **Landlocked property**: Must leave right of way (Article 682-685)
/// 2. **Urban planning**: Zoning prohibits or limits fences
/// 3. **Historic buildings**: Preservation laws restrict walls
/// 4. **Excessive height**: Beyond Article 711 limits (absent local rules)
/// 5. **Aesthetic rules**: Neighborhood covenants, historic districts
///
/// ## Forced Cost-Sharing (Article 710)
/// **Contribution forcée** - Either neighbor may compel cost-sharing for boundary fence.
///
/// ### When Cost-Sharing Required
/// **Clôture forcée** (forced fence) applies in:
/// - **Urban areas**: Cities, towns, developed areas
/// - **Gardens and yards**: Residential curtilage
/// - **Built-up zones**: Where enclosure is customary
///
/// **Not required in**:
/// - **Rural agricultural land**: Fields, pastures (Article 647)
/// - **Forests**: Wooded lands
/// - **Wasteland**: Undeveloped land
///
/// ### Cost-Sharing Rules
/// **Equal sharing** (50/50) of:
/// - **Construction**: Initial fence building
/// - **Repairs**: Maintenance, rebuilding after damage
/// - **Replacement**: When fence wears out
///
/// **Limitations**:
/// - Only for **reasonable** fence (not luxury materials)
/// - Standard height per Article 711 (higher requires consent)
/// - Type: Solid wall, wood fence, hedge (per local custom)
///
/// ## Party Walls (Murs Mitoyens)
/// **Shared ownership** of walls between properties.
///
/// ### Creation of Party Walls
/// 1. **By agreement**: Neighbors build together, share costs
/// 2. **By presumption**: Wall on boundary presumed shared in urban areas
/// 3. **By purchase**: One neighbor buys half-ownership from other (Article 661)
///
/// ### Rights and Duties of Party Wall Co-Owners
/// **May**:
/// - Use wall for support (attach beams, joists)
/// - Build higher with consent or buy exclusive upper rights
/// - Repair at own expense if urgent
///
/// **Must**:
/// - Share repair costs equally
/// - Not weaken wall (no excessive openings)
/// - Not damage neighbor's use
/// - Maintain weather-resistance
///
/// ## Height Limitations (Article 711)
/// **Standard fence heights**:
/// - **Large cities** (>50,000 inhabitants): 3.20 meters (10.5 feet)
/// - **Other areas**: 2.60 meters (8.5 feet)
/// - **Local regulations**: May specify different heights
///
/// ### Exceeding Standard Height
/// **Surélévation** (raising height):
/// - Requires neighbor's consent if party wall
/// - Must not harm neighbor (light, air, views)
/// - Sole owner's expense for extra height
/// - Subject to zoning laws, permits
///
/// ## Historical Context
/// These provisions date from 1804 Napoleonic Code, codifying medieval customs on enclosure
/// and party walls. In densely populated European cities, party walls were economically
/// necessary and socially expected.
///
/// Key developments:
/// - **1804**: Original Code provisions on enclosure rights
/// - **1810**: Height limits codified (updated from royal ordinances)
/// - **1943**: Vichy regime modified some urban enclosure rules
/// - **1967**: Land Use Orientation Law (LOF) added planning restrictions
/// - **1973**: Party wall jurisprudence expanded for modern construction
/// - **2000s**: Aesthetic regulations, historic preservation constraints
///
/// ## Modern Applications
///
/// ### Urban Context
/// - **Townhouses**: Shared walls between units
/// - **Row houses**: Party walls for entire building length
/// - **Gardens**: Boundary fences between yards
/// - **Noise barriers**: Soundproofing walls between properties
///
/// ### Suburban Context
/// - **Privacy fences**: Wood fences between houses
/// - **Hedges**: Living fences (Article 671 distances apply)
/// - **Stone walls**: Decorative/functional enclosures
///
/// ### Rural Context
/// - **Agricultural fences**: Optional for fields (unless livestock)
/// - **Vineyard walls**: Traditional stone walls in wine regions
/// - **Livestock fences**: Required for animal containment
///
/// ## Boundary Trees and Hedges
/// **Article 671**: Distances from property line:
/// - **Trees >2m height**: Plant 2 meters from boundary
/// - **Trees/hedges <2m**: Plant 0.5 meters from boundary
/// - **Local custom**: May override statutory distances
///
/// ## Party Wall Purchase (Article 661)
/// **Acquisition de mitoyenneté**: Buying half-ownership from sole owner.
///
/// ### Procedure
/// 1. **Request**: Neighbor offers to buy half of wall
/// 2. **Valuation**: Appraise half of wall + land value
/// 3. **Payment**: Buy half-ownership at appraised value
/// 4. **Registration**: Record co-ownership in land registry
///
/// ### Benefits
/// - Right to use wall for support
/// - Right to raise height (with conditions)
/// - Share future repair costs (but also future maintenance)
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §921-923)**: Party walls (Grenzwand), forced enclosure in towns
/// - **Japan (Minpō §229-231)**: Boundary walls, cost-sharing rules
/// - **Common Law**: Party wall acts (England 1996 Act), spite fences doctrine (USA)
/// - **USA**: Varies by state - boundary fence statutes, adverse possession of walls
/// - **Switzerland (CC Art. 670-671)**: Enclosure rights, boundary walls
///
/// ## Dispute Resolution
/// **Common conflicts**:
/// 1. **Refusing cost-sharing**: Neighbor won't pay fence share
/// 2. **Excessive height**: One neighbor wants higher fence
/// 3. **Wall maintenance**: Neglect causing damage
/// 4. **Encroachment**: Fence/wall on wrong side of boundary
///
/// **Remedies**:
/// - **Court order**: Compel contribution at law
/// - **Advance payment**: One neighbor builds, sues for half
/// - **Emergency repairs**: Fix dangerous wall, seek reimbursement
/// - **Bornage**: Establish exact boundary (Article 675)
///
/// # Examples
///
/// ```text
/// // Example 1: Forced fence cost-sharing (Article 710)
/// // Urban neighbors share 20-meter boundary
/// // One wants fence for privacy, other refuses to pay
/// // Court orders: Build 2.60m fence, costs split 50/50 (€6,000 each)
/// // Other neighbor must pay half or fence becomes first neighbor's property
///
/// // Example 2: Party wall (murs mitoyens)
/// // Row houses share wall between properties
/// // Wall needs repainting (€3,000) and crack repair (€2,000)
/// // Both owners split costs: €2,500 each
/// // One owner wants to attach beam to wall for renovation
/// // Allowed, but cannot weaken wall
///
/// // Example 3: Height increase (Article 711)
/// // City property has 2.60m party wall
/// // One owner wants 3.20m fence (allowed in large cities)
/// // Other owner consents
/// // First owner pays full cost of extra 0.60m height (€1,500)
/// // Both still share base 2.60m maintenance
///
/// // Example 4: Party wall purchase (Article 661)
/// // Neighbor has wall entirely on own property, on boundary
/// // Other neighbor wants to attach garage to wall
/// // Offers to buy half-ownership of wall
/// // Appraisal: Wall worth €8,000, land value €1,000
/// // Purchase price: €4,500 (half of wall + half of land)
/// // Now co-owners, share future repairs
/// ```
pub fn article710_734() -> Statute {
    Statute::new(
        "code-civil-710-734",
        "Code civil Articles 710-734 - Clôtures et murs mitoyens / Enclosures and party walls",
        Effect::new(
            EffectType::Grant,
            "Right to enclose property and compel neighbor's cost-sharing for boundary fence; \
             party walls shared ownership with equal repair costs; height limits and local regulations",
        )
        .with_parameter("art729_right", "enclose_property")
        .with_parameter("art710_forced_sharing", "compel_fence_cost_contribution")
        .with_parameter("art711_height_large_cities", "3.20_meters")
        .with_parameter("art711_height_other", "2.60_meters")
        .with_parameter("party_wall", "murs_mitoyens_shared_ownership")
        .with_parameter("cost_sharing", "equal_50_50_split")
        .with_parameter("art661_purchase", "acquire_half_ownership"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::Or(
        Box::new(Condition::And(
            Box::new(Condition::HasAttribute {
                key: "boundary_fence_needed".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "property_type".to_string(),
                value: "urban_or_garden".to_string(),
            }),
        )),
        Box::new(Condition::Or(
            Box::new(Condition::HasAttribute {
                key: "party_wall_exists".to_string(),
            }),
            Box::new(Condition::HasAttribute {
                key: "wants_to_enclose".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Enclosure and party walls: (1) Article 729 - right to enclose property (fences, walls, \
         hedges), limited by planning law, historic preservation, must leave right of way; (2) \
         Article 710 - forced cost-sharing (contribution forcée), either neighbor may compel \
         50/50 split for boundary fence in urban/garden areas, not rural fields; (3) Article 711 - \
         height limits: 3.20m (large cities >50k pop), 2.60m (others), local rules may vary; (4) \
         Party walls (murs mitoyens) - shared ownership, equal repair costs, may use for support, \
         buy half-ownership per Article 661. Purpose: balance privacy, security, cost-sharing. \
         Historical: 1804 Code, medieval customs, urban necessity. Modern: townhouses, privacy \
         fences, aesthetic regulations. Compare: Germany BGB §921-923, Common Law party wall acts \
         (England 1996), Japan Minpō §229-231, USA boundary fence statutes."
    )
}

/// Article 713 - Landlocked property right of way (servitude de passage forcé)
///
/// # French Text
/// **Article 682** (primary article, mistakenly called 713 in requirements):
/// "Le propriétaire dont les fonds sont enclavés et qui n'a sur la voie publique aucune issue,
/// ou qu'une issue insuffisante, soit pour l'exploitation agricole, industrielle ou commerciale
/// de sa propriété, soit pour la réalisation d'opérations de construction ou de lotissement,
/// est fondé à réclamer sur les fonds de ses voisins un passage suffisant pour assurer la
/// desserte complète de ses fonds, à charge d'une indemnité proportionnée au dommage qu'il peut occasionner."
///
/// **Article 682 also covers**:
/// "Le passage doit régulièrement être pris du côté où le trajet est le plus court du fonds
/// enclavé à la voie publique."
///
/// # English Translation
/// **Article 682**:
/// "The owner whose land is landlocked and who has no access to public road, or only insufficient
/// access, whether for agricultural, industrial or commercial exploitation of their property, or
/// for carrying out construction or subdivision operations, is entitled to claim a sufficient
/// passage over their neighbors' land to ensure complete service of their property, subject to
/// compensation proportional to the damage caused."
///
/// "The passage must normally be taken from the side where the route is shortest from the
/// landlocked property to the public road."
///
/// # Legal Commentary
/// Article 682 establishes the **servitude de passage forcé** (forced right of way) - one of the
/// most fundamental easements in French property law. This legal easement ensures that no property
/// can be rendered useless by lack of access, balancing absolute ownership with practical necessity.
///
/// ## Core Principle: No Landlocked Property
/// **Prohibition of enclavement** (ban on landlocking):
/// - Every property must have access to public road
/// - Access is **essential utility** for property use
/// - Necessity overrides neighbor's absolute ownership
/// - But requires **fair compensation** to servient estate
///
/// ## Conditions for Forced Right of Way
/// 1. **Enclavement** (landlocked state): Property has no or insufficient access to public road
/// 2. **Necessity**: Access needed for property use (residence, agriculture, business)
/// 3. **Proportionality**: Passage must be minimally burdensome route
/// 4. **Compensation**: Fair indemnity to servient estate owner
/// 5. **Not self-created**: Owner didn't voluntarily create landlocked situation
///
/// ## What Qualifies as "Insufficient Access"
/// **No access**:
/// - Completely surrounded by neighbors' properties
/// - All sides lack road frontage
/// - No legal right to cross any neighbor's land
///
/// **Insufficient access**:
/// - Access too narrow for vehicles (e.g., 1-meter footpath for farm needing tractor access)
/// - Access too steep, dangerous (e.g., cliff path)
/// - Access seasonally impassable (e.g., flooded in winter)
/// - Access inadequate for property's use (e.g., single-family path for commercial development)
///
/// ## Shortest Route Requirement
/// **Trajet le plus court** - Passage must take shortest reasonable route from landlocked
/// property to public road, minimizing burden on servient estates.
///
/// ### Factors Considered
/// - **Distance**: Shorter is better
/// - **Damage**: Less harmful route may be chosen even if slightly longer
/// - **Existing routes**: Follow existing paths, driveways when possible
/// - **Terrain**: Avoid steep slopes, wetlands, valuable crops
/// - **Cost**: Less expensive route preferred (installation, maintenance)
///
/// ### Width and Specifications
/// **Passage width** depends on property use:
/// - **Residential**: 3-4 meters (vehicle access)
/// - **Agricultural**: 4-6 meters (tractor, equipment access)
/// - **Commercial/industrial**: 6-10 meters (delivery trucks)
/// - **Construction projects**: Temporary wider access during building
///
/// ## Compensation (Indemnité)
/// **Fair indemnity** must cover servient estate's losses:
///
/// ### Components of Compensation
/// 1. **Land value**: Value of strip used for passage
/// 2. **Severance damages**: Loss to remaining property (if divided)
/// 3. **Loss of use**: Agricultural production lost, building sites eliminated
/// 4. **Diminution in value**: Decrease in servient property's market value
/// 5. **Ongoing burden**: Noise, dust, traffic from passage
///
/// ### Calculation Methods
/// - **One-time payment**: Lump sum based on permanent easement value
/// - **Annual rent**: Ongoing payments (rare, only if agreed)
/// - **Expert appraisal**: Court appoints expert (géomètre-expert, real estate appraiser)
/// - **Market value**: Percentage of servient property value (typically 10-30%)
///
/// ## Exceptions and Limitations
///
/// ### Self-Created Landlocking
/// **No right to passage** if owner voluntarily created landlocked situation:
/// - Selling property with road frontage, keeping landlocked parcel
/// - Subdividing land and retaining interior parcel without reserving access
/// - Solution: Claim passage over property owner previously owned (former co-owner)
///
/// ### Multiple Neighbors
/// If multiple routes possible:
/// - Choose shortest and least burdensome
/// - May cross multiple properties if necessary
/// - Each servient owner receives proportional compensation
///
/// ### Temporary vs. Permanent
/// - **Permanent passage**: For ongoing property use
/// - **Temporary passage**: For construction projects (narrower right, lower compensation)
///
/// ## Historical Context
/// The forced right of way principle dates from Roman law (via necessitatis) and medieval
/// French customs. The 1804 Napoleonic Code codified this ancient right, recognizing that
/// property without access is economically worthless.
///
/// Evolution:
/// - **1804**: Original Articles 682-685 established forced passage
/// - **1845**: Cour de Cassation refined compensation standards
/// - **1936**: Jurisprudence expanded to commercial/industrial access
/// - **1970s**: Construction and subdivision uses added
/// - **2000s**: Temporary construction access jurisprudence developed
/// - **2014**: Modernization clarified insufficient access concept
///
/// ## Modern Applications
///
/// ### Residential Development
/// - **Subdivisions**: Interior lots need access to public street
/// - **Infill development**: Building on landlocked urban parcels
/// - **Family property**: Dividing inherited land among heirs
///
/// ### Agricultural Context
/// - **Farm consolidation**: Merging parcels creating landlocked fields
/// - **Equipment access**: Modern tractors need wider passages
/// - **Timber harvesting**: Temporary access for logging operations
///
/// ### Commercial/Industrial
/// - **Business parks**: Access for commercial developments
/// - **Industrial sites**: Heavy truck access requirements
/// - **Construction projects**: Temporary access for building materials
///
/// ## Procedure for Establishing Forced Passage
/// 1. **Negotiation**: Request access from neighbor(s), offer compensation
/// 2. **Surveyor**: Hire géomètre-expert to identify optimal route
/// 3. **Court action** (if refused): File lawsuit in Tribunal Judiciaire
/// 4. **Expert appointment**: Court appoints expert to determine route and compensation
/// 5. **Judgment**: Court orders passage, sets compensation amount
/// 6. **Payment**: Dominant estate owner pays indemnity
/// 7. **Installation**: Build road/path at dominant owner's expense
/// 8. **Registration**: Record easement in land registry (hypothèque)
///
/// ## Maintenance Obligations
/// **Dominant estate** (landlocked property owner):
/// - Pays for initial construction of passage
/// - Maintains passage in good condition
/// - Repairs damage from use
/// - Upgrades if needs increase
///
/// **Servient estate**:
/// - Receives compensation
/// - Cannot block or interfere with passage
/// - May use passage if not interfering with dominant estate
/// - May request increased compensation if use intensifies beyond original scope
///
/// ## Comparison with Other Jurisdictions
/// - **Germany (BGB §917-919)**: Notwegrecht (emergency way), similar forced passage
/// - **Japan (Minpō §210-213)**: Landlocked property right of way, shorter route rule
/// - **Common Law**: Easement by necessity, implied from prior common ownership
/// - **USA**: Varies by state - easement by necessity, prescriptive easements
/// - **Switzerland (CC Art. 694)**: Passage rights for landlocked property
///
/// ## Related Legal Concepts
/// - **Article 699**: Right to plant ladders against neighbor's wall (minor access)
/// - **Article 682-685**: Detailed passage rules, compensation, route selection
/// - **Code de l'urbanisme**: Planning law may require access for building permits
/// - **Code rural**: Agricultural access rights, farm equipment passages
///
/// # Examples
///
/// ```text
/// // Example 1: Residential landlocked property
/// // Property A: 2000 m² surrounded by Properties B, C, D
/// // All sides landlocked, no road frontage
/// // Owner wants to build house (requires vehicle access)
/// // Shortest route: 50 meters across Property B to public road
/// // Court orders: 4m wide passage, €15,000 compensation to Property B
/// // Owner A pays: €15,000 indemnity + €8,000 paving/grading
///
/// // Example 2: Agricultural landlocked field
/// // Farm parcel: 5 hectares, needs tractor access for cultivation
/// // Existing footpath 1.5m wide (insufficient for equipment)
/// // Shortest route: Widen path to 5m across neighbor's field
/// // Court orders: 5m passage, €8,000 compensation for lost crop area
/// // Annual crops lost: 0.25 hectare (5m x 50m = 2,500 m²)
///
/// // Example 3: Self-created landlocking (NO right to passage)
/// // Owner owned parcels A (road frontage) and B (landlocked)
/// // Sold parcel A to buyer, kept parcel B without reserving access
/// // Claims Article 682 passage across parcel A
/// // Court denies: Self-created landlocking, owner's fault
/// // Solution: Negotiate purchase or rental of easement from buyer of A
///
/// // Example 4: Temporary construction access
/// // Building lot: Road frontage too narrow for construction vehicles
/// // Needs temporary access across neighbor's driveway for 12 months
/// // Court orders: Temporary passage, €3,000 compensation (lower for temporary)
/// // After construction: Remove temporary road, restore neighbor's property
/// ```
pub fn article713() -> Statute {
    Statute::new(
        "code-civil-682-685",
        "Code civil Articles 682-685 - Servitude de passage forcé / Forced right of way for landlocked property",
        Effect::new(
            EffectType::Grant,
            "Landlocked property owner entitled to passage over neighbors' land to reach public \
             road via shortest reasonable route, with fair compensation for damage caused",
        )
        .with_parameter("right", "forced_passage")
        .with_parameter("condition", "landlocked_or_insufficient_access")
        .with_parameter("route", "shortest_least_burdensome")
        .with_parameter("compensation", "proportional_indemnity_required")
        .with_parameter("width", "sufficient_for_property_use")
        .with_parameter("exception", "not_self_created_landlocking")
        .with_parameter("basis", "enclavement_prohibition")
        .with_parameter("origin", "roman_law_via_necessitatis"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::And(
        Box::new(Condition::Or(
            Box::new(Condition::AttributeEquals {
                key: "access_to_public_road".to_string(),
                value: "none".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "access_to_public_road".to_string(),
                value: "insufficient".to_string(),
            }),
        )),
        Box::new(Condition::And(
            Box::new(Condition::HasAttribute {
                key: "property_use_need".to_string(),
            }),
            Box::new(Condition::AttributeEquals {
                key: "landlocking_self_created".to_string(),
                value: "false".to_string(),
            }),
        )),
    ))
    .with_discretion(
        "Forced right of way (servitude de passage forcé): Landlocked property (enclavé) owner \
         entitled to passage across neighbors' land to reach public road. Conditions: (1) no or \
         insufficient access, (2) needed for property use (residential, agricultural, commercial), \
         (3) not self-created landlocking, (4) shortest reasonable route (trajet le plus court), \
         (5) fair compensation proportional to damage. Width: 3-4m residential, 4-6m agricultural, \
         6-10m commercial. Compensation: land value + severance + loss of use + diminution. \
         Dominant owner pays for construction and maintenance. Exception: no right if owner \
         voluntarily created landlocked situation (sold road frontage). Historical: Roman via \
         necessitatis, 1804 Code. Modern: subdivisions, farm access, construction projects. \
         Compare: Germany Notwegrecht, Japan Minpō §210-213, Common Law easement by necessity, \
         Switzerland CC Art. 694. Related: Articles 682-685 (passage details), Code urbanisme."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article555_creation() {
        let statute = article555();
        assert_eq!(statute.id, "code-civil-555");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("555"));
        assert!(statute.title.contains("eau potable"));
    }

    #[test]
    fn test_article555_essential_water() {
        let statute = article555();
        assert!(matches!(statute.effect.effect_type, EffectType::Grant));
        assert_eq!(
            statute.effect.parameters.get("purpose_1").unwrap(),
            "drinking_water_human"
        );
        assert_eq!(
            statute.effect.parameters.get("purpose_2").unwrap(),
            "livestock_watering"
        );
        assert_eq!(
            statute.effect.parameters.get("basis").unwrap(),
            "public_health_humanitarian_necessity"
        );
    }

    #[test]
    fn test_article555_preconditions() {
        let statute = article555();
        assert_eq!(statute.preconditions.len(), 1);

        // Verify nested And/Or structure
        if let Condition::And(box1, box2) = &statute.preconditions[0] {
            // First part: water need type (drinking or livestock)
            assert!(matches!(&**box1, Condition::Or(..)));
            // Second part: neighbor has spring AND no alternative
            assert!(matches!(&**box2, Condition::And(..)));
        } else {
            panic!("Expected top-level And condition");
        }
    }

    #[test]
    fn test_article640_649_creation() {
        let statute = article640_649();
        assert_eq!(statute.id, "code-civil-640-649");
        assert!(statute.title.contains("640-649"));
        assert!(statute.title.contains("eau"));
    }

    #[test]
    fn test_article640_649_water_rights() {
        let statute = article640_649();
        assert!(
            statute
                .effect
                .parameters
                .contains_key("art640_natural_drainage")
        );
        assert!(statute.effect.parameters.contains_key("art641_rainwater"));
        assert!(statute.effect.parameters.contains_key("art642_spring"));
        assert!(statute.effect.parameters.contains_key("art643_village"));
        assert_eq!(
            statute
                .effect
                .parameters
                .get("art640_natural_drainage")
                .unwrap(),
            "lower_receives_natural_flow"
        );
    }

    #[test]
    fn test_article667_709_creation() {
        let statute = article667_709();
        assert_eq!(statute.id, "code-civil-667-709");
        assert!(statute.title.contains("667-709"));
        assert!(
            statute
                .discretion_logic
                .as_ref()
                .unwrap()
                .contains("servitudes légales")
        );
    }

    #[test]
    fn test_article667_709_legal_easements() {
        let statute = article667_709();
        assert!(matches!(statute.effect.effect_type, EffectType::Obligation));
        assert!(
            statute
                .effect
                .parameters
                .contains_key("art667_roof_drainage")
        );
        assert!(statute.effect.parameters.contains_key("art673_branches"));
        assert!(statute.effect.parameters.contains_key("art674_hazards"));
        assert!(statute.effect.parameters.contains_key("art675_boundaries"));
    }

    #[test]
    fn test_article710_734_creation() {
        let statute = article710_734();
        assert_eq!(statute.id, "code-civil-710-734");
        assert!(statute.title.contains("710-734"));
        assert!(statute.title.contains("Clôtures"));
    }

    #[test]
    fn test_article710_734_enclosure_rights() {
        let statute = article710_734();
        assert!(statute.effect.parameters.contains_key("art729_right"));
        assert!(
            statute
                .effect
                .parameters
                .contains_key("art710_forced_sharing")
        );
        assert_eq!(
            statute
                .effect
                .parameters
                .get("art711_height_large_cities")
                .unwrap(),
            "3.20_meters"
        );
        assert_eq!(
            statute
                .effect
                .parameters
                .get("art711_height_other")
                .unwrap(),
            "2.60_meters"
        );
    }

    #[test]
    fn test_article713_creation() {
        let statute = article713();
        assert_eq!(statute.id, "code-civil-682-685");
        assert!(statute.title.contains("682-685"));
        assert!(statute.title.contains("passage forcé"));
    }

    #[test]
    fn test_article713_landlocked_passage() {
        let statute = article713();
        assert!(matches!(statute.effect.effect_type, EffectType::Grant));
        assert_eq!(
            statute.effect.parameters.get("right").unwrap(),
            "forced_passage"
        );
        assert_eq!(
            statute.effect.parameters.get("route").unwrap(),
            "shortest_least_burdensome"
        );
        assert_eq!(
            statute.effect.parameters.get("exception").unwrap(),
            "not_self_created_landlocking"
        );
    }

    #[test]
    fn test_article713_preconditions_landlocked() {
        let statute = article713();
        assert_eq!(statute.preconditions.len(), 1);

        // Verify nested And/Or structure for landlocked conditions
        if let Condition::And(box1, box2) = &statute.preconditions[0] {
            // First part: no access OR insufficient access
            assert!(matches!(&**box1, Condition::Or(..)));
            // Second part: has use need AND not self-created
            assert!(matches!(&**box2, Condition::And(..)));
        } else {
            panic!("Expected top-level And condition");
        }
    }

    #[test]
    fn test_all_easement_articles_have_jurisdiction() {
        let articles = vec![
            article555(),
            article640_649(),
            article667_709(),
            article710_734(),
            article713(),
        ];

        for article in articles {
            assert_eq!(article.jurisdiction.as_deref(), Some("FR"));
        }
    }

    #[test]
    fn test_all_easement_articles_have_discretion() {
        let articles = vec![
            article555(),
            article640_649(),
            article667_709(),
            article710_734(),
            article713(),
        ];

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
    fn test_all_articles_have_parameters() {
        let articles = vec![
            article555(),
            article640_649(),
            article667_709(),
            article710_734(),
            article713(),
        ];

        for article in articles {
            assert!(!article.effect.parameters.is_empty());
            assert!(article.effect.parameters.len() >= 4);
        }
    }

    #[test]
    fn test_article555_compensation_required() {
        let statute = article555();
        assert_eq!(
            statute.effect.parameters.get("compensation").unwrap(),
            "fair_indemnity_required"
        );
        assert_eq!(
            statute.effect.parameters.get("limitation").unwrap(),
            "essential_needs_only"
        );
    }

    #[test]
    fn test_article640_649_origin() {
        let statute = article640_649();
        assert_eq!(
            statute.effect.parameters.get("origin").unwrap(),
            "roman_law_riparian_rights"
        );
    }

    #[test]
    fn test_article667_709_complex_precondition() {
        let statute = article667_709();
        assert_eq!(statute.preconditions.len(), 1);

        // Verify nested Or structure covering multiple easement types
        if let Condition::Or(box1, box2) = &statute.preconditions[0] {
            assert!(matches!(&**box1, Condition::Or(..))); // roof or branches
            assert!(matches!(&**box2, Condition::Or(..))); // hazards or boundaries
        } else {
            panic!("Expected top-level Or condition");
        }
    }

    #[test]
    fn test_article710_734_cost_sharing() {
        let statute = article710_734();
        assert_eq!(
            statute.effect.parameters.get("cost_sharing").unwrap(),
            "equal_50_50_split"
        );
        assert!(statute.effect.parameters.contains_key("party_wall"));
    }

    #[test]
    fn test_article713_roman_origin() {
        let statute = article713();
        assert_eq!(
            statute.effect.parameters.get("origin").unwrap(),
            "roman_law_via_necessitatis"
        );
    }

    #[test]
    fn test_all_articles_version_1() {
        let articles = vec![
            article555(),
            article640_649(),
            article667_709(),
            article710_734(),
            article713(),
        ];

        for article in articles {
            assert_eq!(article.version, 1);
        }
    }
}
