//! Negligence and tort claim analysis (Singapore common law).
//!
//! Demonstrates the `legalis_sg::tort` module: assessing a negligence claim
//! through the *Spandeck* two-stage duty test, breach, causation and damage;
//! apportioning for contributory negligence; and assessing a defamation claim.
//!
//! Run with:
//!
//! ```bash
//! cargo run -p legalis-sg --example negligence_claim_analysis
//! ```

use legalis_sg::tort::*;

fn main() {
    println!("=== Singapore Tort Law: Claim Analysis ===\n");

    // 1. Negligence (Spandeck two-stage duty test) ---------------------------
    let claim = NegligenceClaim::new(
        "neg-2026-01",
        "Injured Pedestrian",
        "Distracted Motorist",
        DutyOfCareAnalysis::established(HarmCategory::PersonalInjury),
        BreachAnalysis::new(StandardOfCare::ReasonablePerson, true).with_risk_factors(75, 95, 5),
        CausationAnalysis::established(),
        4_500_000, // SGD 45,000
    );

    println!("[Negligence] Duty owed:   {}", claim.duty.duty_owed());
    println!("[Negligence] Breach:      {}", claim.breach.is_breach());
    println!(
        "[Negligence] Causation:   {}",
        claim.causation.causation_established()
    );
    match assess_negligence(&claim) {
        Err(TortError::NegligenceEstablished { detail }) => {
            println!("[Negligence] ESTABLISHED: {detail}");
        }
        Err(e) => println!("[Negligence] Not established: {e}"),
        Ok(()) => println!("[Negligence] (no finding)"),
    }

    // 2. Contributory negligence ---------------------------------------------
    let claimant_fault = 30u8; // crossing against the signal
    let reduced =
        apportion_for_contributory_negligence(4_500_000, claimant_fault).expect("apportion");
    println!(
        "\n[Apportionment] Claimant {claimant_fault}% at fault -> recoverable SGD {:.2} \
         (Contributory Negligence and Personal Injuries Act 1953, s. 3)",
        reduced as f64 / 100.0
    );

    // 3. A full report taking defences into account --------------------------
    let report = TortAssessmentReport::for_negligence(
        &claim,
        &[TortDefence::ContributoryNegligence {
            claimant_fault_percent: claimant_fault,
        }],
    );
    println!(
        "[Report] Liability established: {} (contributory negligence apportions, it does not defeat)",
        report.liability_established
    );

    // 4. Defamation ----------------------------------------------------------
    let mut defamation = DefamationClaim::new(
        "def-2026-01",
        "Local Business Owner",
        "Anonymous Reviewer",
        "the business cheats its customers and sells fake goods",
        DefamationForm::Libel,
    );
    println!(
        "\n[Defamation] Libel actionable per se: {}",
        defamation.actionable_per_se()
    );
    println!(
        "[Defamation] Succeeds (no defence): {}",
        defamation_succeeds(&defamation)
    );

    // The reviewer pleads justification (truth) -- a complete defence.
    defamation.add_defence(DefamationDefence::Justification);
    println!(
        "[Defamation] After plea of justification (Defamation Act 1957 s. 8) succeeds: {}",
        defamation_succeeds(&defamation)
    );

    println!("\nAnalysis complete.");
}
