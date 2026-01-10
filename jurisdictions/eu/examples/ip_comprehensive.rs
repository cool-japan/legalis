//! Comprehensive EU IP Example
//!
//! Demonstrates all EU IP protection mechanisms for a tech startup.

use legalis_eu::intellectual_property::{
    AcquisitionMethod, CommunityDesign, CopyrightWork, DesignAppearance, DesignType, EuTrademark,
    MarkType, TradeSecret, TradeSecretCharacteristics, WorkType,
};

fn main() {
    println!("=== Comprehensive EU IP Strategy for Tech Startup ===\n");
    println!("Scenario: 'DataViz Pro' - A data visualization software company\n");

    // 1. Trademark Protection
    println!("1. TRADEMARK PROTECTION (EU Regulation 2017/1001)");
    println!("==================================================");
    let brand = EuTrademark::new()
        .with_mark_text("DATAVIZ PRO")
        .with_mark_type(MarkType::WordMark)
        .with_applicant("DataViz Technologies GmbH")
        .add_nice_class(9)
        .unwrap() // Computer software
        .add_nice_class(42)
        .unwrap() // Software as a service
        .add_goods_services("Computer software for data visualization")
        .add_goods_services("Software as a service (SaaS) for business intelligence");

    match brand.validate() {
        Ok(validation) => {
            println!("✅ Trademark 'DATAVIZ PRO' is registrable");
            println!("   Nice Classes: 9 (software), 42 (SaaS)");
            println!(
                "   Distinctiveness: {}",
                validation.distinctiveness_established
            );
            println!("   Protection: Pan-EU unitary right via EUIPO");
            println!("   Duration: 10 years, renewable indefinitely");
        }
        Err(e) => println!("❌ Trademark issue: {}", e),
    }

    // 2. Copyright Protection for Software
    println!("\n\n2. COPYRIGHT PROTECTION (Software Directive 2009/24/EC)");
    println!("========================================================");
    let software_copyright = CopyrightWork::new()
        .with_title("DataViz Pro v2.0")
        .with_author("DataViz Technologies GmbH")
        .with_work_type(WorkType::Software)
        .with_creation_date(chrono::Utc::now() - chrono::Duration::days(90))
        .with_originality(true)
        .with_fixation(true)
        .with_country_of_origin("Germany");

    match software_copyright.validate() {
        Ok(validation) => {
            println!("✅ Software protected as literary work");
            println!("   Originality: Author's own intellectual creation");
            println!("   Protection: Automatic upon creation (no registration)");
            println!("   Duration: Life + 70 years (for company: 70 years from publication)");
            println!("   Currently protected: {}", validation.is_protected);
            println!("\n   Exclusive rights:");
            println!("     - Reproduction");
            println!("     - Translation, adaptation, arrangement");
            println!("     - Distribution to public");
        }
        Err(e) => println!("❌ Copyright issue: {}", e),
    }

    // 3. Design Protection for UI
    println!("\n\n3. DESIGN PROTECTION (Community Design Regulation EC No 6/2002)");
    println!("================================================================");
    let ui_design = CommunityDesign::new()
        .with_design_type(DesignType::Registered)
        .with_appearance(DesignAppearance {
            features: vec![
                "Curved interface elements".to_string(),
                "Gradient color scheme from blue to purple".to_string(),
                "Floating data cards with soft shadows".to_string(),
                "Minimalist icon set".to_string(),
            ],
            product_indication: "Graphical user interface for computer software".to_string(),
        })
        .with_creator("Lead Designer")
        .with_owner("DataViz Technologies GmbH")
        .with_novelty(true)
        .with_individual_character(true);

    match ui_design.validate() {
        Ok(validation) => {
            println!("✅ UI design protectable as Registered Community Design");
            println!("   Novelty: No identical design publicly available");
            println!("   Individual character: Different overall impression from prior designs");
            println!(
                "   Protection: {} years maximum (5 renewal periods)",
                validation.max_protection_years
            );
            println!("   Scope: Pan-EU unitary right");
        }
        Err(e) => println!("❌ Design issue: {}", e),
    }

    // 4. Trade Secret Protection for Algorithm
    println!("\n\n4. TRADE SECRET PROTECTION (Directive EU 2016/943)");
    println!("===================================================");
    let algorithm = TradeSecret::new()
        .with_description(
            "Proprietary machine learning algorithm for anomaly detection in time-series data",
        )
        .with_holder("DataViz Technologies GmbH")
        .with_characteristics(TradeSecretCharacteristics {
            is_secret: true,
            has_commercial_value: true,
            reasonable_steps_taken: true,
        })
        .add_protective_measure("Source code stored on air-gapped development servers")
        .add_protective_measure("Comprehensive NDAs with all employees and contractors")
        .add_protective_measure("Code obfuscation in compiled product")
        .add_protective_measure("Need-to-know access policy with audit logs")
        .add_protective_measure("Physical access control to R&D facility")
        .add_protective_measure("Exit interviews reminding of confidentiality obligations");

    match algorithm.validate() {
        Ok(validation) => {
            println!("✅ Algorithm protected as trade secret");
            println!("   Three-part test passed:");
            println!("     ✓ Information is secret");
            println!("     ✓ Has commercial value (competitive advantage)");
            println!("     ✓ Reasonable protective measures in place");
            println!(
                "   Protective measures adequate: {}",
                validation.protective_measures_adequate
            );
            println!("   Duration: Indefinite (as long as secret maintained)");
        }
        Err(e) => println!("❌ Trade secret issue: {}", e),
    }

    // 5. Copyright for Documentation
    println!("\n\n5. COPYRIGHT FOR DOCUMENTATION");
    println!("================================");
    let documentation = CopyrightWork::new()
        .with_title("DataViz Pro User Manual")
        .with_author("Technical Writing Team")
        .with_work_type(WorkType::Literary)
        .with_originality(true);

    match documentation.validate() {
        Ok(validation) => {
            println!("✅ Documentation protected by copyright");
            println!("   Type: Literary work");
            println!(
                "   Applicable exceptions: {:?}",
                validation.applicable_exceptions.len()
            );
        }
        Err(e) => println!("❌ Copyright issue: {}", e),
    }

    // 6. Misappropriation Risk Analysis
    println!("\n\n6. MISAPPROPRIATION RISK ANALYSIS");
    println!("===================================");

    println!("\nScenario A: Former employee joins competitor");
    let analysis_breach = algorithm.analyze_misappropriation(AcquisitionMethod::Breach);
    if analysis_breach.is_unlawful {
        println!("  ❌ Risk: HIGH");
        println!("  If employee discloses algorithm:");
        for remedy in &analysis_breach.remedies_available {
            println!("    - Available remedy: {}", remedy);
        }
    }

    println!("\nScenario B: Competitor reverse-engineers product");
    let analysis_re = algorithm.analyze_misappropriation(AcquisitionMethod::ReverseEngineering);
    if !analysis_re.is_unlawful {
        println!("  ✅ Risk: LOW");
        println!("  Reverse engineering is LAWFUL under EU law");
        println!("  Mitigation: Code obfuscation makes RE economically unfeasible");
    }

    println!("\nScenario C: Competitor independently develops similar algorithm");
    let analysis_indep =
        algorithm.analyze_misappropriation(AcquisitionMethod::IndependentDiscovery);
    if !analysis_indep.is_unlawful {
        println!("  ✅ Risk: ACCEPTED");
        println!("  Independent discovery is LAWFUL");
        println!("  Trade secrets do not prevent independent development");
    }

    // Summary
    println!("\n\n=== IP STRATEGY SUMMARY ===");
    println!("\n┌─────────────────────────────────────────────────────────────┐");
    println!("│ LAYERED IP PROTECTION FOR DATAVIZ PRO                       │");
    println!("├─────────────────────────────────────────────────────────────┤");
    println!("│ 1. TRADEMARK: 'DATAVIZ PRO'                                 │");
    println!("│    Protects: Brand identity                                 │");
    println!("│    Duration: Indefinite (with renewals)                     │");
    println!("│    Scope: Pan-EU                                            │");
    println!("│                                                             │");
    println!("│ 2. COPYRIGHT: Software code & documentation                 │");
    println!("│    Protects: Expression of ideas                            │");
    println!("│    Duration: Life + 70 years                                │");
    println!("│    Scope: Automatic, no registration                        │");
    println!("│                                                             │");
    println!("│ 3. DESIGN: User interface appearance                        │");
    println!("│    Protects: Visual design                                  │");
    println!("│    Duration: Up to 25 years                                 │");
    println!("│    Scope: Registered Community Design                       │");
    println!("│                                                             │");
    println!("│ 4. TRADE SECRET: ML algorithm                               │");
    println!("│    Protects: Confidential know-how                          │");
    println!("│    Duration: Indefinite (while secret)                      │");
    println!("│    Scope: Protected by security measures                    │");
    println!("└─────────────────────────────────────────────────────────────┘");

    println!("\n🔑 KEY BENEFITS OF LAYERED APPROACH:");
    println!("   • Trademark: Prevents confusion, builds brand value");
    println!("   • Copyright: Automatic protection, prevents copying");
    println!("   • Design: Protects look & feel from knockoffs");
    println!("   • Trade Secret: Protects competitive advantage");

    println!("\n⚠️  CRITICAL COMPLIANCE POINTS:");
    println!("   1. Trademark: File EU application via EUIPO (Alicante)");
    println!("   2. Copyright: No formalities, but document creation dates");
    println!("   3. Design: File RCD application within 12 months of first disclosure");
    println!("   4. Trade Secrets: Maintain strict confidentiality measures");

    println!("\n📚 LEGAL REFERENCES:");
    println!("   • EU Trademark Regulation (EU) 2017/1001");
    println!("   • Software Directive 2009/24/EC");
    println!("   • InfoSoc Directive 2001/29/EC");
    println!("   • Community Design Regulation (EC) No 6/2002");
    println!("   • Trade Secrets Directive (EU) 2016/943");

    println!("\n✅ DataViz Pro has comprehensive IP protection across all layers!");
}
