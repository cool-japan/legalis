//! Integration tests for competition law.

use legalis_my::competition_law::*;

#[test]
fn test_dominant_position() {
    let position = MarketPosition::new("Tech Giant Sdn Bhd", 60.0);
    assert!(position.is_dominant());

    let position2 = MarketPosition::new("Small Player", 15.0);
    assert!(!position2.is_dominant());
}

#[test]
fn test_merger_notification() {
    let merger = MergerNotification::new("Acquirer Bhd", "Target Sdn Bhd", 35.0);
    assert!(merger.requires_notification());

    let merger2 = MergerNotification::new("Small Co", "Tiny Co", 5.0);
    assert!(!merger2.requires_notification());
}

#[test]
fn test_price_fixing_assessment() {
    let assessment = assess_agreement(
        AgreementType::Horizontal,
        AntiCompetitivePractice::PriceFixing,
    )
    .expect("Assessment succeeds");

    assert!(!assessment.compliant);
    assert!(!assessment.concerns.is_empty());
}
