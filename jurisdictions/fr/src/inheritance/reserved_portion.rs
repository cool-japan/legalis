//! Reserved portion articles (Code civil, Book III)
//!
//! This module implements the réserve héréditaire (reserved portion) system,
//! a fundamental principle of French inheritance law that guarantees
//! compulsory shares for descendants.

use legalis_core::{Condition, Effect, EffectType, Statute};

/// Article 912 - Reserved portion definition
///
/// # French Text
/// "La réserve héréditaire est la part des biens et droits successoraux dont la loi assure la dévolution libre de charges \
/// à certains héritiers dits réservataires, s'ils sont appelés à la succession et s'ils l'acceptent."
///
/// # English Translation
/// "The reserved portion is the share of property and succession rights which the law ensures free transfer without charges \
/// to certain heirs called forced heirs, if they are called to the succession and if they accept it."
///
/// # Legal Commentary
/// The **réserve héréditaire** is one of the most distinctive features of French succession law.
/// It fundamentally limits testamentary freedom to protect family interests.
///
/// ## Core Principles
/// 1. **Compulsory share**: Certain heirs (descendants) cannot be disinherited
/// 2. **Free of charges**: Reserved portion passes without conditions or burdens
/// 3. **Acceptance required**: Heir must accept the succession to receive reserved portion
/// 4. **Public policy**: Cannot be waived by agreement (ordre public)
///
/// ## Who Are Forced Heirs (Héritiers Réservataires)?
/// Under current French law (as of 2006 reform):
/// - **Descendants**: Children, grandchildren, great-grandchildren (by representation)
/// - **NOT spouse**: Since 2001, surviving spouse is no longer a forced heir
/// - **NOT ascendants**: Parents no longer have reserved portion rights (2006 reform)
///
/// ## Historical Evolution
/// This represents a dramatic shift in French law:
///
/// ### Before 2001
/// - Spouse had reserved portion rights (1/4 in usufruct)
/// - Non-marital children had reduced rights
/// - Ascendants had reserved portions in some cases
///
/// ### 2001 Reform
/// - Removed spouse's reserved portion
/// - Granted equal rights to non-marital children
/// - Simplified the system to focus on descendants
///
/// ### 2006 Reform (Loi n° 2006-728)
/// - Further reduced reserved portions
/// - Removed ascendants' reserved portions entirely
/// - Increased available portion (quotité disponible)
/// - Enhanced testamentary freedom
///
/// ## Calculation (Article 913)
/// Reserved portion varies by number of children:
/// - **1 child**: 1/2 reserved (1/2 available)
/// - **2 children**: 2/3 reserved (1/3 available)
/// - **3+ children**: 3/4 reserved (1/4 available)
///
/// ## Comparison with Other Jurisdictions
///
/// ### Common Law (UK, US)
/// - **No reserved portion** - Complete testamentary freedom
/// - Surviving spouse may have statutory rights
/// - Children can be completely disinherited
///
/// ### Germany (BGB §2303-2338)
/// - **Pflichtteil** (compulsory portion) = 1/2 of statutory share
/// - Payable in money, not in kind
/// - Can be claimed by descendants AND parents
///
/// ### Japan (Minpo §1042-1049)
/// - **Iryubun** (reserved portion) system similar to France
/// - 1/2 of statutory share for direct descendants
/// - 1/3 if only ascendants inherit
/// - Can be claimed within 1 year of knowing violation
///
/// ### Switzerland (CC Art 470-480)
/// - Reserved portions for descendants, parents, and spouse
/// - More restrictive than France
/// - 3/4 for descendants, 1/2 for parents, 1/2 for spouse
///
/// ### Spain (Código Civil Art 806-822)
/// - **Legítima** system
/// - 2/3 reserved for children (1/3 strict + 1/3 improvable)
/// - More complex than French system
///
/// ## Policy Justifications
/// The reserved portion reflects French social values:
/// 1. **Family solidarity**: Obligation to provide for descendants
/// 2. **Wealth transmission**: Preventing concentration of wealth
/// 3. **Protection of vulnerable heirs**: Children cannot be disinherited arbitrarily
/// 4. **Social stability**: Ensuring minimum inheritance for family continuity
///
/// ## Modern Criticisms
/// The system faces ongoing debate:
/// - **Excessive restriction** on testamentary freedom
/// - **Outdated** in modern family structures (blended families, etc.)
/// - **Economic inefficiency**: May fragment business assets
/// - **Unequal treatment**: Adult children receive same protection as minors
///
/// ## Reform Proposals
/// Ongoing discussions include:
/// - Further reduction of reserved portions
/// - Distinction between minor and adult children
/// - Optional reserved portion system
/// - Alignment with common law jurisdictions
///
/// # Examples
///
/// ```
/// use legalis_fr::inheritance::{ReservedPortion, Succession, Heir, Person, Relationship};
/// use chrono::NaiveDate;
///
/// // Calculate reserved portion for family with 2 children
/// let reserved = ReservedPortion::calculate(2);
/// assert!((reserved.reserved_portion - 2.0 / 3.0).abs() < 1e-10); // 2/3 reserved
/// assert!((reserved.available_portion - 1.0 / 3.0).abs() < 1e-10); // 1/3 available
///
/// // Each child receives 1/3 of estate (half of 2/3 reserved)
/// assert!((reserved.share_per_child() - 1.0 / 3.0).abs() < 1e-10);
/// ```
pub fn article912() -> Statute {
    Statute::new(
        "code-civil-912",
        "Code civil Article 912 - Definition of reserved portion (réserve héréditaire)",
        Effect::new(
            EffectType::Grant,
            "Forced heirs receive reserved portion free of charges if they accept succession",
        )
        .with_parameter("reserved_for", "descendants_only")
        .with_parameter("condition", "acceptance_required")
        .with_parameter("public_policy", "cannot_be_waived")
        .with_parameter("reform_2006", "ascendants_no_longer_protected"),
    )
    .with_jurisdiction("FR")
    .with_version(3) // 2006 reform
    .with_precondition(Condition::Or(
        // Descendants exist
        Box::new(Condition::HasAttribute {
            key: "descendants".to_string(),
        }),
        // Or checking eligibility
        Box::new(Condition::AttributeEquals {
            key: "checking_reserved_portion".to_string(),
            value: "true".to_string(),
        }),
    ))
    .with_discretion(
        "Reserved portion (réserve héréditaire) is a fundamental French succession principle \
         guaranteeing compulsory shares for descendants. 2006 Reform: Removed ascendants' \
         reserved portions, enhanced testamentary freedom.\\n\\n\
         Only descendants (children, grandchildren by representation) are forced heirs. \
         Spouse and ascendants no longer have reserved portions.\\n\\n\
         Comparison: Unlike common law (US/UK) with complete testamentary freedom, France \
         protects descendants with compulsory shares. Germany has similar Pflichtteil system \
         (1/2 of statutory share payable in money). Japan has iryubun (1/2 of statutory share).\
         \\n\\nPolicy justification: Family solidarity, wealth transmission, protection of \
         vulnerable heirs, social stability. Modern criticisms: Excessive restriction on freedom, \
         outdated for blended families, economic inefficiency.",
    )
}

/// Article 913 - Reserved portion calculation
///
/// # French Text
/// "Les libéralités, soit par actes entre vifs, soit par testament, ne pourront excéder la moitié des biens du disposant, \
/// s'il ne laisse à son décès qu'un enfant ; le tiers, s'il laisse deux enfants ; le quart, s'il en laisse trois ou un plus grand nombre."
///
/// # English Translation
/// "Gifts, whether inter vivos or by will, cannot exceed half the property of the donor if leaving only one child at death; \
/// one-third if leaving two children; one-quarter if leaving three or more children."
///
/// # Legal Commentary
/// This article provides the **mathematical formula** for calculating reserved and available portions.
/// It is the operational core of the French forced heirship system.
///
/// ## The Formula
///
/// | Number of Children | Reserved Portion | Available Portion (Quotité Disponible) |
/// |-------------------|------------------|---------------------------------------|
/// | **1 child**       | **1/2** (50%)    | **1/2** (50%)                         |
/// | **2 children**    | **2/3** (66.67%) | **1/3** (33.33%)                      |
/// | **3+ children**   | **3/4** (75%)    | **1/4** (25%)                         |
/// | **0 children**    | **0** (0%)       | **1** (100%)                          |
///
/// ## Share Per Child
/// Each child receives an equal share of the reserved portion:
/// - 1 child: 1/2 each
/// - 2 children: 1/3 each (2/3 ÷ 2)
/// - 3 children: 1/4 each (3/4 ÷ 3)
/// - 4 children: 3/16 each (3/4 ÷ 4)
///
/// ## Available Portion (Quotité Disponible)
/// The testator may freely dispose of the available portion:
/// - By will to any beneficiary
/// - By inter vivos gifts (donations)
/// - No restrictions on recipients
///
/// ## Representation (Représentation)
/// If a child predeceases the testator:
/// - Grandchildren take by representation
/// - They share their parent's reserved portion equally
/// - Example: 1 living child + 2 grandchildren (representing deceased child)
///   - Reserved = 2/3 (for 2 "branches")
///   - Living child: 1/3
///   - Each grandchild: 1/6 (1/3 ÷ 2)
///
/// ## Historical Evolution
///
/// ### Pre-2006 (Old Formula)
/// Reserved portions were HIGHER:
/// - 1 child: 1/2 reserved
/// - 2 children: 2/3 reserved
/// - 3+ children: 3/4 reserved
/// - **PLUS spouse** had 1/4 usufruct rights
/// - **PLUS ascendants** had rights if no descendants
///
/// ### 2006 Reform
/// - Removed spouse's reserved portion entirely
/// - Removed ascendants' reserved portions
/// - Kept same formula for descendants
/// - Net effect: Increased available portion by removing competing claims
///
/// ## Violation and Reduction (Réduction)
/// If gifts/wills exceed available portion:
/// 1. **Calculate mass** (masse de calcul): All property + gifts made
/// 2. **Apply formula** to determine reserved vs available
/// 3. **Reduce excess gifts** (réduction) proportionally
/// 4. **Inter vivos gifts reduced first**, then legacies
///
/// ## Comparison with Other Jurisdictions
///
/// ### Germany (BGB §2303)
/// **Pflichtteil** = 1/2 of statutory intestate share
/// - 1 child alone: Statutory = 1/1, Pflichtteil = 1/2
/// - 2 children: Statutory = 1/2 each, Pflichtteil = 1/4 each
/// - More generous testamentary freedom than France
/// - Paid in MONEY, not in kind
///
/// ### Japan (Minpo §1042)
/// **Iryubun** = 1/2 of statutory share for descendants
/// - Similar to German system
/// - 1 child: Statutory = 1/1, Iryubun = 1/2
/// - 2 children: Statutory = 1/2 each, Iryubun = 1/4 each
/// - More restrictive than Germany, less than France
///
/// ### Spain (Código Civil Art 808)
/// **Legítima** = 2/3 of estate for children
/// - 1/3 "legítima estricta" (strict, divided equally)
/// - 1/3 "mejora" (improvement, testator chooses which children)
/// - More restrictive than France
///
/// ### Switzerland (CC Art 471)
/// - Descendants: 3/4 of statutory share
/// - Parents: 1/2 of statutory share
/// - Spouse: 1/2 of statutory share
/// - Multiple competing forced heirs (more complex than France)
///
/// ### Louisiana (USA)
/// **Forced heirship** (unique in US):
/// - Only for children under 24 or permanently disabled
/// - Very limited compared to France
/// - Reflects French civil law heritage
///
/// ## Policy Analysis
///
/// ### Arguments For Reserved Portion
/// 1. **Family solidarity**: Moral obligation to children
/// 2. **Prevent disinheritance**: Protection from arbitrary decisions
/// 3. **Wealth equality**: Prevents concentration in one heir
/// 4. **Social stability**: Ensures minimum inheritance
///
/// ### Arguments Against
/// 1. **Restricts freedom**: Limits individual autonomy
/// 2. **One-size-fits-all**: Adult children same as minors
/// 3. **Blended families**: Difficult with step-children
/// 4. **Business continuity**: May force fragmentation of assets
/// 5. **Economic efficiency**: Prevents optimal asset allocation
///
/// ## Practical Application
///
/// ### Example 1: Estate of €1,000,000 with 1 Child
/// - Reserved portion: €500,000 (child must receive)
/// - Available portion: €500,000 (testator can give to anyone)
/// - If will gives €600,000 to charity:
///   - Excess: €100,000 (€600k - €500k available)
///   - Reduction: Charity receives only €500,000
///   - Child receives: €500,000 (reserved)
///
/// ### Example 2: Estate of €900,000 with 3 Children
/// - Reserved portion: €675,000 (3/4)
/// - Each child: €225,000 (€675k ÷ 3)
/// - Available portion: €225,000 (1/4)
/// - Testator can give €225,000 to anyone
///
/// # Examples
///
/// ```
/// use legalis_fr::inheritance::ReservedPortion;
///
/// // 1 child: 1/2 reserved, 1/2 available
/// let one_child = ReservedPortion::calculate(1);
/// assert_eq!(one_child.reserved_portion, 0.5);
/// assert_eq!(one_child.available_portion, 0.5);
/// assert_eq!(one_child.share_per_child(), 0.5);
///
/// // 2 children: 2/3 reserved, 1/3 available
/// let two_children = ReservedPortion::calculate(2);
/// assert!((two_children.reserved_portion - 2.0 / 3.0).abs() < 1e-10);
/// assert!((two_children.available_portion - 1.0 / 3.0).abs() < 1e-10);
/// assert!((two_children.share_per_child() - 1.0 / 3.0).abs() < 1e-10);
///
/// // 3 children: 3/4 reserved, 1/4 available
/// let three_children = ReservedPortion::calculate(3);
/// assert_eq!(three_children.reserved_portion, 0.75);
/// assert_eq!(three_children.available_portion, 0.25);
/// assert_eq!(three_children.share_per_child(), 0.25);
///
/// // 0 children: 0 reserved, 100% available (complete freedom)
/// let no_children = ReservedPortion::calculate(0);
/// assert_eq!(no_children.reserved_portion, 0.0);
/// assert_eq!(no_children.available_portion, 1.0);
/// ```
pub fn article913() -> Statute {
    Statute::new(
        "code-civil-913",
        "Code civil Article 913 - Reserved portion calculation formula",
        Effect::new(
            EffectType::Grant,
            "Calculate reserved and available portions based on number of children",
        )
        .with_parameter("one_child", "reserved_1_2_available_1_2")
        .with_parameter("two_children", "reserved_2_3_available_1_3")
        .with_parameter("three_plus_children", "reserved_3_4_available_1_4")
        .with_parameter("no_children", "reserved_0_available_1"),
    )
    .with_jurisdiction("FR")
    .with_version(1)
    .with_precondition(Condition::HasAttribute {
        key: "number_of_children".to_string(),
    })
    .with_discretion(
        "Mathematical formula for French reserved portion (réserve héréditaire):\\n\
         • 1 child: 1/2 reserved, 1/2 available\\n\
         • 2 children: 2/3 reserved, 1/3 available\\n\
         • 3+ children: 3/4 reserved, 1/4 available\\n\
         • 0 children: 100% available (complete testamentary freedom)\\n\\n\
         Historical: Formula unchanged since 2006 reform, but effective available portion \
         increased due to removal of spouse and ascendant reserved portions.\\n\\n\
         Comparison with other systems:\\n\
         • Germany: Pflichtteil = 1/2 statutory share (more freedom than France)\\n\
         • Japan: Iryubun = 1/2 statutory share (similar to Germany)\\n\
         • Spain: Legítima = 2/3 estate (more restrictive than France)\\n\
         • Common Law (US/UK): No reserved portion (complete freedom)\\n\
         • Louisiana (USA): Only for children <24 or disabled (very limited)\\n\\n\
         Practical application: If gifts/legacies exceed available portion, they are reduced \
         (réduction) proportionally. Inter vivos gifts reduced before testamentary legacies.",
    )
}

/// Article 1493 - Interaction with matrimonial property regimes
///
/// # French Text
/// "Le conjoint survivant qui opte pour l'usufruit de la totalité des biens existants prend les biens dans l'état où ils se trouvent \
/// au jour du décès, mais il est dispensé de faire emploi des sommes comprises dans la succession."
///
/// # English Translation
/// "The surviving spouse who opts for usufruct of all existing property takes the property in the state it is in at the day of death, \
/// but is exempt from investing sums included in the succession."
///
/// # Legal Commentary
/// This article governs the **relationship between reserved portions and matrimonial property regimes**.
/// It is critical for understanding how succession law interacts with family property law.
///
/// ## Matrimonial Property Regimes in France
///
/// French law recognizes several matrimonial property regimes that affect succession:
///
/// ### 1. Community Property (Communauté de biens)
/// Default regime if no marriage contract:
/// - **Communauté réduite aux acquêts** (most common): Assets acquired during marriage are joint
/// - **Communauté universelle**: All assets joint (less common)
/// - At death: Surviving spouse receives 1/2 of community property by operation of law
/// - Reserved portions apply only to deceased's 1/2 share + separate property
///
/// ### 2. Separation of Property (Séparation de biens)
/// - Each spouse owns property separately
/// - No automatic share for surviving spouse
/// - Reserved portions apply to deceased's entire estate
///
/// ### 3. Participation in Acquisitions (Participation aux acquêts)
/// - Hybrid system: Separation during marriage, liquidation at death
/// - Surviving spouse receives share of net gains
/// - Then reserved portions apply
///
/// ## Surviving Spouse's Rights
///
/// The surviving spouse is **not a forced heir** since 2001, but has significant rights:
///
/// ### Option 1: Usufruct
/// Article 757 allows surviving spouse to choose:
/// - **1/4 in full ownership** (propriété), OR
/// - **Usufruct of entire estate**
///
/// Usufruct (usufruit) means:
/// - Right to use property and collect income
/// - No right to sell or consume capital
/// - Ends at death of spouse
/// - Bare ownership (nue-propriété) goes to children
///
/// ### Option 2: Full Ownership
/// - 1/4 of estate in full ownership
/// - More limited but provides more control
///
/// ## Interaction with Reserved Portion
///
/// **Critical principle**: Spouse's usufruct does NOT reduce children's reserved portions.
///
/// ### Calculation Method
/// 1. **Community property**: Spouse receives 1/2 by operation of law
/// 2. **Spouse's option**: Choose usufruct of all OR 1/4 ownership
/// 3. **Reserved portion**: Apply to deceased's share (NOT reduced by spouse's usufruct)
/// 4. **Children receive**: Bare ownership of reserved portion
///
/// ### Example: Estate €1,000,000, Community Property, 2 Children
///
/// **Step 1**: Divide community property
/// - Surviving spouse: €500,000 (1/2 community)
/// - Succession estate: €500,000 (deceased's 1/2)
///
/// **Step 2**: Spouse chooses usufruct of all
/// - Spouse: Usufruct of €500,000 succession estate
/// - Children: Bare ownership
///
/// **Step 3**: Apply reserved portion to €500,000
/// - Children's reserved: 2/3 × €500,000 = €333,333
/// - Available portion: 1/3 × €500,000 = €166,667
///
/// **Result**:
/// - Spouse: €500,000 community + usufruct of €500,000 succession
/// - Each child: €166,667 in bare ownership (reserved portion)
/// - Available: €166,667 (testator can dispose freely)
///
/// ## Historical Evolution
///
/// ### Before 2001
/// - Spouse had reserved portion (1/4 in usufruct if children)
/// - Complex interaction with children's reserved portions
/// - Frequent litigation over calculations
///
/// ### 2001 Reform
/// - **Removed spouse's reserved portion**
/// - Spouse becomes "ordinary heir" with option rights
/// - Simplified calculation significantly
/// - Enhanced children's protection
///
/// ### Policy Rationale
/// Why remove spouse's reserved portion?
/// 1. **Blended families**: Biological children vs step-children conflict
/// 2. **Modern marriage**: Spouses often have separate assets
/// 3. **Flexibility**: Option system more adaptable
/// 4. **Protection remains**: Spouse still has strong usufruct rights
///
/// ## Conversion of Usufruct to Full Ownership
///
/// Usufruct can be converted to full ownership by:
/// 1. **Agreement** between spouse and bare owners (children)
/// 2. **Court order** if agreement impossible
/// 3. **Valuation**: Based on spouse's age (actuarial tables)
///
/// ### Example Valuation (2024 French Tables)
/// - Age 60: Usufruct = 40% of full value
/// - Age 70: Usufruct = 30% of full value
/// - Age 80: Usufruct = 20% of full value
///
/// ## Comparison with Other Jurisdictions
///
/// ### Germany (BGB §1371, §2303)
/// - Spouse has reserved portion (Pflichtteil)
/// - 1/4 of estate in community property regime
/// - More protection for spouse than France
///
/// ### Japan (Minpo §900)
/// - Spouse has reserved portion (iryubun)
/// - Spouse + children: Spouse receives 1/2, children share 1/2
/// - Much stronger spouse protection
///
/// ### Spain (Código Civil)
/// - Spouse has reserved portion (cuota legal usufructuaria)
/// - Usufruct of 1/3 or 1/2 depending on other heirs
/// - Similar to France pre-2001
///
/// ### Common Law (UK, US)
/// - No reserved portions at all
/// - Spouse may have "elective share" (statutory minimum)
/// - Typically 1/3 to 1/2 in US states with elective share
/// - UK: Spouse can claim "reasonable financial provision"
///
/// ## Practical Considerations
///
/// ### For Estate Planning
/// 1. **Choice of regime**: Community property vs separation affects calculations
/// 2. **Spouse's option**: Usufruct vs ownership affects liquidity
/// 3. **Children's interests**: May prefer to buy out usufruct
/// 4. **Tax implications**: Usufruct and bare ownership taxed separately
///
/// ### For Blended Families
/// - Step-children are NOT forced heirs
/// - Biological children from different relationships all have equal reserved portions
/// - Spouse's usufruct may cause tension with children from prior marriage
/// - Estate planning crucial to balance interests
///
/// # Examples
///
/// ```
/// use legalis_fr::inheritance::{Succession, Heir, Person, Relationship, ReservedPortion};
/// use chrono::NaiveDate;
///
/// // Estate: €1,000,000 community property, 2 children
/// // Step 1: Spouse receives €500,000 community property by law
/// let succession_estate = 500_000u64;
///
/// // Step 2: Calculate reserved portion for children
/// let reserved = ReservedPortion::calculate(2); // 2/3 reserved
/// let children_reserved = (succession_estate as f64 * reserved.reserved_portion) as u64;
/// let available = succession_estate - children_reserved;
///
/// assert_eq!(children_reserved, 333_333); // 2/3 of €500k (rounded)
/// assert_eq!(available, 166_667); // 1/3 of €500k
///
/// // If spouse chooses usufruct:
/// // - Spouse: €500k community + usufruct of €500k succession
/// // - Children: Bare ownership of €333,333 (reserved) + €166,667 (if testator chooses)
/// ```
pub fn article1493() -> Statute {
    Statute::new(
        "code-civil-1493",
        "Code civil Article 1493 - Interaction between reserved portion and matrimonial property regimes",
        Effect::new(
            EffectType::Grant,
            "Reserved portion applies after dividing community property; spouse's usufruct does not reduce children's reserved portions",
        )
        .with_parameter("community_property", "divide_first")
        .with_parameter("spouse_option", "usufruct_all_or_quarter_ownership")
        .with_parameter("reserved_portion_applies_to", "deceased_share_only")
        .with_parameter("reform_2001", "spouse_no_longer_forced_heir"),
    )
    .with_jurisdiction("FR")
    .with_version(2) // 2001 reform
    .with_precondition(Condition::Or(
        Box::new(Condition::And(
            Box::new(Condition::HasAttribute {
                key: "matrimonial_regime".to_string(),
            }),
            Box::new(Condition::HasAttribute {
                key: "surviving_spouse".to_string(),
            }),
        )),
        Box::new(Condition::AttributeEquals {
            key: "calculating_succession".to_string(),
            value: "true".to_string(),
        }),
    ))
    .with_discretion(
        "Interaction between reserved portions and matrimonial property regimes:\\n\\n\
         1. Community property: Spouse receives 1/2 by law, reserved portion applies to deceased's 1/2\\n\
         2. Spouse's option (Article 757): Usufruct of all OR 1/4 full ownership\\n\
         3. Reserved portion NOT reduced by spouse's usufruct (children receive bare ownership)\\n\
         4. 2001 Reform: Removed spouse as forced heir, simplified calculations\\n\\n\
         Example: €1M community estate, 2 children:\\n\
         • Spouse: €500k community + usufruct of €500k succession\\n\
         • Children: Bare ownership of €333k (2/3 reserved) + €167k available\\n\\n\
         Comparison:\\n\
         • Germany: Spouse HAS reserved portion (1/4) - more protection\\n\
         • Japan: Spouse HAS reserved portion (1/2 with children) - much more protection\\n\
         • Spain: Spouse has usufruct reserved portion (similar to France pre-2001)\\n\
         • Common Law: No reserved portions, but 'elective share' (US) or 'reasonable provision' (UK)\\n\\n\
         Practical: Usufruct can be converted to full ownership by agreement/court order based on \
         actuarial tables (age 60=40%, age 70=30%, age 80=20%)."
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_article912_creation() {
        let statute = article912();
        assert_eq!(statute.id, "code-civil-912");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert_eq!(statute.version, 3); // 2006 reform
        assert!(statute.title.contains("912"));
        assert!(statute.title.contains("reserved portion"));
    }

    #[test]
    fn test_article912_preconditions() {
        let statute = article912();
        let preconditions = &statute.preconditions;
        assert!(!preconditions.is_empty());
    }

    #[test]
    fn test_article912_reform_version() {
        let statute = article912();
        assert_eq!(statute.version, 3); // 2006 reform version
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("2006 Reform"));
        assert!(discretion.contains("ascendants"));
    }

    #[test]
    fn test_article912_discretion_content() {
        let statute = article912();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("réserve héréditaire"));
        assert!(discretion.contains("descendants"));
        assert!(discretion.contains("Germany"));
        assert!(discretion.contains("Japan"));
    }

    #[test]
    fn test_article913_creation() {
        let statute = article913();
        assert_eq!(statute.id, "code-civil-913");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert!(statute.title.contains("913"));
        assert!(statute.title.contains("calculation"));
    }

    #[test]
    fn test_article913_parameters() {
        let statute = article913();
        let effect = &statute.effect;
        assert!(&effect.description.contains("reserved and available"));
    }

    #[test]
    fn test_article913_formula_documentation() {
        let statute = article913();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("1/2 reserved"));
        assert!(discretion.contains("2/3 reserved"));
        assert!(discretion.contains("3/4 reserved"));
        assert!(discretion.contains("100% available"));
    }

    #[test]
    fn test_article913_jurisdiction_comparisons() {
        let statute = article913();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("Germany"));
        assert!(discretion.contains("Japan"));
        assert!(discretion.contains("Spain"));
        assert!(discretion.contains("Louisiana"));
    }

    #[test]
    fn test_article1493_creation() {
        let statute = article1493();
        assert_eq!(statute.id, "code-civil-1493");
        assert_eq!(statute.jurisdiction.as_deref(), Some("FR"));
        assert_eq!(statute.version, 2); // 2001 reform
        assert!(statute.title.contains("1493"));
    }

    #[test]
    fn test_article1493_matrimonial_regime_focus() {
        let statute = article1493();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("matrimonial property"));
        assert!(discretion.contains("Community property"));
        assert!(discretion.contains("usufruct"));
    }

    #[test]
    fn test_article1493_spouse_option() {
        let statute = article1493();
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("Spouse's option"));
        assert!(discretion.contains("1/4 full ownership"));
    }

    #[test]
    fn test_article1493_reform_note() {
        let statute = article1493();
        assert_eq!(statute.version, 2); // 2001 reform
        let discretion = statute.discretion_logic.as_ref().unwrap();
        assert!(discretion.contains("2001 Reform"));
        assert!(discretion.contains("forced heir"));
    }

    #[test]
    fn test_all_reserved_portion_articles_have_jurisdiction() {
        let articles = vec![article912(), article913(), article1493()];

        for article in articles {
            assert_eq!(article.jurisdiction.as_deref(), Some("FR"));
        }
    }

    #[test]
    fn test_all_reserved_portion_articles_have_discretion() {
        let articles = vec![article912(), article913(), article1493()];

        for article in articles {
            assert!(article.discretion_logic.is_some());
            assert!(!article.discretion_logic.unwrap().is_empty());
        }
    }

    #[test]
    fn test_reserved_portion_articles_comprehensive() {
        let articles = vec![article912(), article913(), article1493()];

        for article in articles {
            let discretion = article.discretion_logic.unwrap();
            // Each article should mention international comparisons
            assert!(
                discretion.contains("Germany")
                    || discretion.contains("Japan")
                    || discretion.contains("Spain")
                    || discretion.contains("Common Law")
            );
        }
    }
}
