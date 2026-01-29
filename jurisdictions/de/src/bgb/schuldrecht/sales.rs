//! Sales Contract (Kaufvertrag) - §§433-479 BGB
//!
//! Type-safe implementation of German sales law under the BGB.
//!
//! # Legal Context
//!
//! A sales contract (Kaufvertrag) is a contract where the seller obligates
//! to transfer ownership of a thing to the buyer and the buyer obligates to
//! pay the purchase price and accept the thing (§433 BGB).
//!
//! ## Core Provisions
//!
//! ### §433 BGB - Obligations
//! - **Seller (Verkäufer)**: Transfer ownership, deliver thing free of defects
//! - **Buyer (Käufer)**: Pay purchase price, accept delivery
//!
//! ### §437-442 BGB - Warranty for Defects (Gewährleistung)
//! - Material defects (Sachmangel): Thing lacks agreed quality
//! - Legal defects (Rechtsmangel): Third-party rights burden the thing
//! - Buyer remedies: Supplementary performance, price reduction, damages, withdrawal
//!
//! ### §445a-445b BGB - Right of Recourse (Rückgriffsrecht)
//! - Seller's recourse against supplier in supply chain
//! - Special rules for commercial sellers
//!
//! ### §474-479 BGB - Consumer Sales (Verbrauchsgüterkauf)
//! - Stronger protections for consumers
//! - Reversal of burden of proof for defects within 6 months (§477 BGB)
//! - Limitations on excluding warranty rights

#[cfg(test)]
use chrono::Duration;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::bgb::schuldrecht::error::{Result, SchuldrechtError};
use crate::bgb::schuldrecht::types::{Contract, ContractTerms, Party, PartyType};
use crate::gmbhg::Capital;

/// Sales contract type (Kaufvertrag)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SalesContract {
    /// Base contract information
    pub base_contract: Contract,
    /// Seller (Verkäufer)
    pub seller: Party,
    /// Buyer (Käufer)
    pub buyer: Party,
    /// Purchase price (Kaufpreis)
    pub purchase_price: Capital,
    /// Thing being sold (Kaufsache)
    pub subject_matter: String,
    /// Delivery date/deadline (Lieferdatum)
    pub delivery_date: Option<DateTime<Utc>>,
    /// Payment date/deadline (Zahlungsdatum)
    pub payment_date: Option<DateTime<Utc>>,
    /// Whether this is a consumer sale (§474 BGB)
    pub is_consumer_sale: bool,
    /// Warranty information
    pub warranty: WarrantyInfo,
    /// Seller's obligations
    pub seller_obligations: SellerObligations,
    /// Buyer's obligations
    pub buyer_obligations: BuyerObligations,
}

/// Seller's main obligations per §433 Abs. 1 BGB
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SellerObligations {
    /// Transfer ownership (Eigentum verschaffen)
    pub transfer_ownership: bool,
    /// Deliver the thing (Sache übergeben)
    pub deliver_thing: bool,
    /// Deliver free of material defects (Sachmangel)
    pub free_of_material_defects: bool,
    /// Deliver free of legal defects (Rechtsmangel)
    pub free_of_legal_defects: bool,
    /// Hand over documents and accessories (§434 BGB)
    pub hand_over_documents: bool,
}

/// Buyer's main obligations per §433 Abs. 2 BGB
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuyerObligations {
    /// Pay the purchase price (Kaufpreis zahlen)
    pub pay_purchase_price: bool,
    /// Accept delivery (Abnahme)
    pub accept_delivery: bool,
    /// Payment made on time
    pub payment_timely: bool,
    /// Acceptance timely
    pub acceptance_timely: bool,
}

/// Warranty information (Gewährleistung)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WarrantyInfo {
    /// Warranty period (Verjährungsfrist §438 BGB)
    /// Standard: 2 years for movables, 5 years for immovables
    pub warranty_period_months: u32,
    /// Whether defect exists
    pub has_defect: bool,
    /// Type of defect
    pub defect_type: Option<DefectType>,
    /// When defect was discovered
    pub defect_discovered_at: Option<DateTime<Utc>>,
    /// Buyer's chosen remedy
    pub chosen_remedy: Option<WarrantyRemedy>,
    /// Whether supplementary performance attempted
    pub supplementary_performance_attempted: bool,
    /// Whether supplementary performance failed
    pub supplementary_performance_failed: bool,
}

/// Types of defects (Mängel)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DefectType {
    /// Material defect (Sachmangel §434 BGB)
    /// Thing lacks agreed quality or fitness for ordinary/contractual use
    Material,
    /// Legal defect (Rechtsmangel §435 BGB)
    /// Third-party rights burden the thing
    Legal,
}

/// Warranty remedies per §437 BGB
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarrantyRemedy {
    /// Supplementary performance (Nacherfüllung §439 BGB)
    /// - Repair (Nachbesserung) or
    /// - Replacement delivery (Ersatzlieferung)
    SupplementaryPerformance,
    /// Price reduction (Minderung §441 BGB)
    PriceReduction,
    /// Withdrawal from contract (Rücktritt §§323, 326 Abs. 5 BGB)
    Withdrawal,
    /// Damages (Schadensersatz §§280, 281, 283 BGB)
    Damages,
}

/// Consumer sales protections (§§474-479 BGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsumerSalesProtection {
    /// Whether buyer is a consumer (§13 BGB)
    pub buyer_is_consumer: bool,
    /// Whether seller is an entrepreneur (§14 BGB)
    pub seller_is_entrepreneur: bool,
    /// Reversal of burden of proof applies (§477 BGB)
    /// Within 6 months: defect presumed to exist at delivery
    pub burden_of_proof_reversed: bool,
    /// When contract was concluded
    pub concluded_at: DateTime<Utc>,
    /// Whether warranty exclusions attempted (limited by §475 BGB)
    pub attempted_warranty_exclusions: Vec<String>,
}

/// Right of recourse for commercial sellers (§§445a-445b BGB)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecourseClaim {
    /// Commercial seller who resold defective goods
    pub seller_in_recourse: Party,
    /// Supplier who sold to commercial seller
    pub supplier: Party,
    /// Consumer who bought from commercial seller
    pub end_consumer: Party,
    /// Defect in goods
    pub defect: DefectType,
    /// Remedy provided to consumer
    pub remedy_provided: WarrantyRemedy,
    /// Amount of recourse claim
    pub recourse_amount: Capital,
    /// Whether statutory recourse period (§445b: 2 months) observed
    pub within_recourse_period: bool,
}

/// Builder for sales contracts
#[derive(Debug, Clone, Default)]
pub struct SalesContractBuilder {
    seller: Option<Party>,
    buyer: Option<Party>,
    purchase_price: Option<Capital>,
    subject_matter: Option<String>,
    delivery_date: Option<DateTime<Utc>>,
    payment_date: Option<DateTime<Utc>>,
    is_consumer_sale: bool,
    warranty_period_months: Option<u32>,
}

impl SalesContractBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the seller
    pub fn seller(mut self, seller: Party) -> Self {
        self.seller = Some(seller);
        self
    }

    /// Set the buyer
    pub fn buyer(mut self, buyer: Party) -> Self {
        self.buyer = Some(buyer);
        self
    }

    /// Set the purchase price
    pub fn purchase_price(mut self, price: Capital) -> Self {
        self.purchase_price = Some(price);
        self
    }

    /// Set the subject matter
    pub fn subject_matter(mut self, subject: String) -> Self {
        self.subject_matter = Some(subject);
        self
    }

    /// Set the delivery date
    pub fn delivery_date(mut self, date: DateTime<Utc>) -> Self {
        self.delivery_date = Some(date);
        self
    }

    /// Set the payment date
    pub fn payment_date(mut self, date: DateTime<Utc>) -> Self {
        self.payment_date = Some(date);
        self
    }

    /// Mark as consumer sale (§474 BGB)
    pub fn consumer_sale(mut self, is_consumer: bool) -> Self {
        self.is_consumer_sale = is_consumer;
        self
    }

    /// Set warranty period in months (default: 24 for movables per §438 BGB)
    pub fn warranty_period_months(mut self, months: u32) -> Self {
        self.warranty_period_months = Some(months);
        self
    }

    /// Build the sales contract
    pub fn build(self) -> Result<SalesContract> {
        let seller = self
            .seller
            .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                missing_terms: vec!["Seller".to_string()],
            })?;

        let buyer = self
            .buyer
            .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                missing_terms: vec!["Buyer".to_string()],
            })?;

        let purchase_price =
            self.purchase_price
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Purchase price".to_string()],
                })?;

        let subject_matter =
            self.subject_matter
                .ok_or_else(|| SchuldrechtError::OfferLacksEssentialTerms {
                    missing_terms: vec!["Subject matter".to_string()],
                })?;

        // Default warranty period: 24 months for movables (§438 Abs. 1 Nr. 3 BGB)
        let warranty_period_months = self.warranty_period_months.unwrap_or(24);

        let contract = Contract {
            contract_id: format!("SALES-{}", Utc::now().timestamp()),
            parties: vec![seller.clone(), buyer.clone()],
            terms: ContractTerms {
                subject_matter: subject_matter.clone(),
                consideration: Some(purchase_price),
                essential_terms: vec![
                    format!("Seller: {}", seller.name),
                    format!("Buyer: {}", buyer.name),
                    format!("Subject: {}", subject_matter),
                    format!("Price: € {:.2}", purchase_price.to_euros()),
                ],
                additional_terms: vec![],
                includes_gtc: false,
            },
            concluded_at: Utc::now(),
            status: crate::bgb::schuldrecht::types::ContractStatus::Concluded,
            contract_type: crate::bgb::schuldrecht::types::ContractType::Sale,
            obligations: vec![],
            in_writing: false,
        };

        Ok(SalesContract {
            base_contract: contract,
            seller,
            buyer,
            purchase_price,
            subject_matter,
            delivery_date: self.delivery_date,
            payment_date: self.payment_date,
            is_consumer_sale: self.is_consumer_sale,
            warranty: WarrantyInfo {
                warranty_period_months,
                has_defect: false,
                defect_type: None,
                defect_discovered_at: None,
                chosen_remedy: None,
                supplementary_performance_attempted: false,
                supplementary_performance_failed: false,
            },
            seller_obligations: SellerObligations {
                transfer_ownership: false,
                deliver_thing: false,
                free_of_material_defects: true,
                free_of_legal_defects: true,
                hand_over_documents: false,
            },
            buyer_obligations: BuyerObligations {
                pay_purchase_price: false,
                accept_delivery: false,
                payment_timely: false,
                acceptance_timely: false,
            },
        })
    }
}

/// Validate a sales contract per §433 BGB
pub fn validate_sales_contract(contract: &SalesContract) -> Result<()> {
    // Validate seller has legal capacity
    crate::bgb::schuldrecht::validator::validate_party_capacity(&contract.seller)?;

    // Validate buyer has legal capacity
    crate::bgb::schuldrecht::validator::validate_party_capacity(&contract.buyer)?;

    // Validate purchase price is positive
    if contract.purchase_price.amount_cents == 0 {
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: vec!["Valid purchase price".to_string()],
        });
    }

    // Validate subject matter is not empty
    if contract.subject_matter.is_empty() {
        return Err(SchuldrechtError::OfferLacksEssentialTerms {
            missing_terms: vec!["Subject matter".to_string()],
        });
    }

    // Validate consumer sale rules (§474 BGB)
    if contract.is_consumer_sale {
        validate_consumer_sale_rules(contract)?;
    }

    Ok(())
}

/// Validate consumer sale rules per §§474-479 BGB
pub fn validate_consumer_sale_rules(contract: &SalesContract) -> Result<()> {
    // Consumer must be natural person
    if contract.buyer.party_type != PartyType::NaturalPerson {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Consumer must be natural person (§13 BGB)".to_string(),
        });
    }

    // Warranty period cannot be less than 12 months for consumer sales (§475 BGB)
    if contract.warranty.warranty_period_months < 12 {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Warranty period cannot be shortened below 12 months for consumer sales (§475 Abs. 2 BGB)".to_string(),
        });
    }

    Ok(())
}

/// Validate warranty claim per §§437-442 BGB
pub fn validate_warranty_claim(warranty: &WarrantyInfo, sale_date: DateTime<Utc>) -> Result<()> {
    // Must have a defect
    if !warranty.has_defect {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "No defect present - warranty claim invalid".to_string(),
        });
    }

    // Defect must be discovered within warranty period
    if let Some(discovered_at) = warranty.defect_discovered_at {
        let elapsed_months = (discovered_at - sale_date).num_days() / 30;
        if elapsed_months > i64::from(warranty.warranty_period_months) {
            return Err(SchuldrechtError::InvalidContractTerms {
                reason: format!(
                    "Defect discovered after warranty period expired (§438 BGB): {} months",
                    elapsed_months
                ),
            });
        }
    }

    // Validate remedy choice (§437 BGB)
    if let Some(remedy) = warranty.chosen_remedy {
        validate_warranty_remedy(remedy, warranty)?;
    }

    Ok(())
}

/// Validate chosen warranty remedy per §437 BGB
pub fn validate_warranty_remedy(remedy: WarrantyRemedy, warranty: &WarrantyInfo) -> Result<()> {
    match remedy {
        WarrantyRemedy::SupplementaryPerformance => {
            // Always available as first remedy (§439 BGB)
            Ok(())
        }
        WarrantyRemedy::PriceReduction | WarrantyRemedy::Withdrawal => {
            // Price reduction and withdrawal require either:
            // 1. Supplementary performance attempted and failed (§§440, 323 Abs. 1 BGB), or
            // 2. Supplementary performance unreasonable (§§440, 323 Abs. 2 BGB)
            if !warranty.supplementary_performance_attempted
                && !warranty.supplementary_performance_failed
            {
                return Err(SchuldrechtError::InvalidContractTerms {
                    reason: "Price reduction/withdrawal requires supplementary performance attempt first (§§437, 440 BGB)".to_string(),
                });
            }
            Ok(())
        }
        WarrantyRemedy::Damages => {
            // Damages available if fault proven (§§280, 281 BGB)
            Ok(())
        }
    }
}

/// Validate recourse claim per §§445a-445b BGB
pub fn validate_recourse_claim(claim: &RecourseClaim) -> Result<()> {
    // Seller in recourse must be commercial (§445a BGB)
    if claim.seller_in_recourse.party_type != PartyType::LegalEntity {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Recourse seller must be commercial entity (§445a BGB)".to_string(),
        });
    }

    // End consumer must be consumer (natural person)
    if claim.end_consumer.party_type != PartyType::NaturalPerson {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "End buyer must be consumer for recourse claim (§445a BGB)".to_string(),
        });
    }

    // Must be within recourse period (§445b: 2 months)
    if !claim.within_recourse_period {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Recourse claim must be asserted within 2 months (§445b BGB)".to_string(),
        });
    }

    // Recourse amount must be positive
    if claim.recourse_amount.amount_cents == 0 {
        return Err(SchuldrechtError::InvalidContractTerms {
            reason: "Recourse amount must be positive".to_string(),
        });
    }

    Ok(())
}

/// Check if burden of proof is reversed per §477 BGB (consumer sales)
pub fn is_burden_of_proof_reversed(
    concluded_at: DateTime<Utc>,
    defect_discovered_at: DateTime<Utc>,
) -> bool {
    // Within 6 months of delivery: defect presumed to exist at delivery
    let months_elapsed = (defect_discovered_at - concluded_at).num_days() / 30;
    months_elapsed <= 6
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bgb::schuldrecht::types::LegalCapacity;

    fn create_test_seller() -> Party {
        Party {
            name: "Max Mustermann GmbH".to_string(),
            address: "Berlin".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::LegalEntity,
        }
    }

    fn create_test_buyer() -> Party {
        Party {
            name: "Erika Schmidt".to_string(),
            address: "Munich".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::NaturalPerson,
        }
    }

    #[test]
    fn test_sales_contract_builder_valid() {
        let seller = create_test_seller();
        let buyer = create_test_buyer();

        let contract = SalesContractBuilder::new()
            .seller(seller.clone())
            .buyer(buyer.clone())
            .purchase_price(Capital::from_euros(15_000))
            .subject_matter("VW Golf 2020".to_string())
            .consumer_sale(true)
            .build();

        assert!(contract.is_ok());
        let contract = contract.unwrap();
        assert_eq!(contract.purchase_price, Capital::from_euros(15_000));
        assert_eq!(contract.subject_matter, "VW Golf 2020");
        assert!(contract.is_consumer_sale);
    }

    #[test]
    fn test_sales_contract_builder_missing_seller() {
        let buyer = create_test_buyer();

        let result = SalesContractBuilder::new()
            .buyer(buyer)
            .purchase_price(Capital::from_euros(15_000))
            .subject_matter("VW Golf 2020".to_string())
            .build();

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            SchuldrechtError::OfferLacksEssentialTerms { .. }
        ));
    }

    #[test]
    fn test_sales_contract_builder_missing_buyer() {
        let seller = create_test_seller();

        let result = SalesContractBuilder::new()
            .seller(seller)
            .purchase_price(Capital::from_euros(15_000))
            .subject_matter("VW Golf 2020".to_string())
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_sales_contract_builder_missing_price() {
        let seller = create_test_seller();
        let buyer = create_test_buyer();

        let result = SalesContractBuilder::new()
            .seller(seller)
            .buyer(buyer)
            .subject_matter("VW Golf 2020".to_string())
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_sales_contract_builder_missing_subject_matter() {
        let seller = create_test_seller();
        let buyer = create_test_buyer();

        let result = SalesContractBuilder::new()
            .seller(seller)
            .buyer(buyer)
            .purchase_price(Capital::from_euros(15_000))
            .build();

        assert!(result.is_err());
    }

    #[test]
    fn test_validate_sales_contract_valid() {
        let seller = create_test_seller();
        let buyer = create_test_buyer();

        let contract = SalesContractBuilder::new()
            .seller(seller)
            .buyer(buyer)
            .purchase_price(Capital::from_euros(15_000))
            .subject_matter("VW Golf 2020".to_string())
            .build()
            .unwrap();

        assert!(validate_sales_contract(&contract).is_ok());
    }

    #[test]
    fn test_validate_consumer_sale_natural_person() {
        let seller = create_test_seller();
        let buyer = create_test_buyer();

        let contract = SalesContractBuilder::new()
            .seller(seller)
            .buyer(buyer)
            .purchase_price(Capital::from_euros(15_000))
            .subject_matter("VW Golf 2020".to_string())
            .consumer_sale(true)
            .build()
            .unwrap();

        assert!(validate_consumer_sale_rules(&contract).is_ok());
    }

    #[test]
    fn test_validate_consumer_sale_legal_entity_buyer_fails() {
        let seller = create_test_seller();
        let buyer = Party {
            name: "ABC Corp".to_string(),
            address: "Frankfurt".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::LegalEntity,
        };

        let contract = SalesContractBuilder::new()
            .seller(seller)
            .buyer(buyer)
            .purchase_price(Capital::from_euros(15_000))
            .subject_matter("VW Golf 2020".to_string())
            .consumer_sale(true)
            .build()
            .unwrap();

        assert!(validate_consumer_sale_rules(&contract).is_err());
    }

    #[test]
    fn test_validate_consumer_sale_warranty_period_minimum() {
        let seller = create_test_seller();
        let buyer = create_test_buyer();

        let contract = SalesContractBuilder::new()
            .seller(seller)
            .buyer(buyer)
            .purchase_price(Capital::from_euros(15_000))
            .subject_matter("VW Golf 2020".to_string())
            .consumer_sale(true)
            .warranty_period_months(6)
            .build()
            .unwrap();

        // Consumer sale cannot have warranty period less than 12 months
        assert!(validate_consumer_sale_rules(&contract).is_err());
    }

    #[test]
    fn test_warranty_claim_valid() {
        let warranty = WarrantyInfo {
            warranty_period_months: 24,
            has_defect: true,
            defect_type: Some(DefectType::Material),
            defect_discovered_at: Some(Utc::now()),
            chosen_remedy: Some(WarrantyRemedy::SupplementaryPerformance),
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
        };

        let sale_date = Utc::now() - Duration::days(30);
        assert!(validate_warranty_claim(&warranty, sale_date).is_ok());
    }

    #[test]
    fn test_warranty_claim_no_defect_fails() {
        let warranty = WarrantyInfo {
            warranty_period_months: 24,
            has_defect: false,
            defect_type: None,
            defect_discovered_at: Some(Utc::now()),
            chosen_remedy: Some(WarrantyRemedy::SupplementaryPerformance),
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
        };

        let sale_date = Utc::now() - Duration::days(30);
        assert!(validate_warranty_claim(&warranty, sale_date).is_err());
    }

    #[test]
    fn test_warranty_claim_expired_period_fails() {
        let warranty = WarrantyInfo {
            warranty_period_months: 24,
            has_defect: true,
            defect_type: Some(DefectType::Material),
            defect_discovered_at: Some(Utc::now()),
            chosen_remedy: Some(WarrantyRemedy::SupplementaryPerformance),
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
        };

        // Sale was 30 months ago (beyond 24 month warranty)
        let sale_date = Utc::now() - Duration::days(900);
        assert!(validate_warranty_claim(&warranty, sale_date).is_err());
    }

    #[test]
    fn test_warranty_remedy_supplementary_performance_always_valid() {
        let warranty = WarrantyInfo {
            warranty_period_months: 24,
            has_defect: true,
            defect_type: Some(DefectType::Material),
            defect_discovered_at: Some(Utc::now()),
            chosen_remedy: None,
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
        };

        assert!(
            validate_warranty_remedy(WarrantyRemedy::SupplementaryPerformance, &warranty).is_ok()
        );
    }

    #[test]
    fn test_warranty_remedy_price_reduction_requires_supplementary_attempt() {
        let warranty = WarrantyInfo {
            warranty_period_months: 24,
            has_defect: true,
            defect_type: Some(DefectType::Material),
            defect_discovered_at: Some(Utc::now()),
            chosen_remedy: None,
            supplementary_performance_attempted: false,
            supplementary_performance_failed: false,
        };

        // Price reduction without supplementary performance attempt should fail
        assert!(validate_warranty_remedy(WarrantyRemedy::PriceReduction, &warranty).is_err());
    }

    #[test]
    fn test_warranty_remedy_price_reduction_after_failed_supplementary() {
        let warranty = WarrantyInfo {
            warranty_period_months: 24,
            has_defect: true,
            defect_type: Some(DefectType::Material),
            defect_discovered_at: Some(Utc::now()),
            chosen_remedy: None,
            supplementary_performance_attempted: true,
            supplementary_performance_failed: true,
        };

        // Price reduction after failed supplementary performance should succeed
        assert!(validate_warranty_remedy(WarrantyRemedy::PriceReduction, &warranty).is_ok());
    }

    #[test]
    fn test_recourse_claim_valid() {
        let seller_in_recourse = create_test_seller();
        let supplier = Party {
            name: "Supplier GmbH".to_string(),
            address: "Hamburg".to_string(),
            legal_capacity: LegalCapacity::Full,
            legal_representative: None,
            party_type: PartyType::LegalEntity,
        };
        let consumer = create_test_buyer();

        let claim = RecourseClaim {
            seller_in_recourse,
            supplier,
            end_consumer: consumer,
            defect: DefectType::Material,
            remedy_provided: WarrantyRemedy::SupplementaryPerformance,
            recourse_amount: Capital::from_euros(1_000),
            within_recourse_period: true,
        };

        assert!(validate_recourse_claim(&claim).is_ok());
    }

    #[test]
    fn test_recourse_claim_seller_not_commercial_fails() {
        let seller_in_recourse = create_test_buyer(); // Natural person, not commercial
        let supplier = create_test_seller();
        let consumer = create_test_buyer();

        let claim = RecourseClaim {
            seller_in_recourse,
            supplier,
            end_consumer: consumer,
            defect: DefectType::Material,
            remedy_provided: WarrantyRemedy::SupplementaryPerformance,
            recourse_amount: Capital::from_euros(1_000),
            within_recourse_period: true,
        };

        assert!(validate_recourse_claim(&claim).is_err());
    }

    #[test]
    fn test_recourse_claim_expired_period_fails() {
        let seller_in_recourse = create_test_seller();
        let supplier = create_test_seller();
        let consumer = create_test_buyer();

        let claim = RecourseClaim {
            seller_in_recourse,
            supplier,
            end_consumer: consumer,
            defect: DefectType::Material,
            remedy_provided: WarrantyRemedy::SupplementaryPerformance,
            recourse_amount: Capital::from_euros(1_000),
            within_recourse_period: false,
        };

        assert!(validate_recourse_claim(&claim).is_err());
    }

    #[test]
    fn test_burden_of_proof_reversed_within_6_months() {
        let concluded_at = Utc::now() - Duration::days(90); // 3 months ago
        let defect_discovered_at = Utc::now();

        assert!(is_burden_of_proof_reversed(
            concluded_at,
            defect_discovered_at
        ));
    }

    #[test]
    fn test_burden_of_proof_not_reversed_after_6_months() {
        let concluded_at = Utc::now() - Duration::days(210); // 7 months ago
        let defect_discovered_at = Utc::now();

        assert!(!is_burden_of_proof_reversed(
            concluded_at,
            defect_discovered_at
        ));
    }

    #[test]
    fn test_default_warranty_period_24_months() {
        let seller = create_test_seller();
        let buyer = create_test_buyer();

        let contract = SalesContractBuilder::new()
            .seller(seller)
            .buyer(buyer)
            .purchase_price(Capital::from_euros(15_000))
            .subject_matter("VW Golf 2020".to_string())
            .build()
            .unwrap();

        // Default warranty period should be 24 months per §438 Abs. 1 Nr. 3 BGB
        assert_eq!(contract.warranty.warranty_period_months, 24);
    }
}
