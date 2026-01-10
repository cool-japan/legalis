//! Consumer Protection - Type Definitions
//!
//! This module provides type-safe representations of Singapore's consumer protection framework,
//! covering the Sale of Goods Act (Cap. 393) and Consumer Protection (Fair Trading) Act (Cap. 52A).
//!
//! ## Key Statutes
//!
//! ### Sale of Goods Act (Cap. 393)
//! - **s. 13**: Implied term - goods correspond to description
//! - **s. 14(2)**: Implied term - merchantable quality (if seller in business)
//! - **s. 14(3)**: Implied term - fitness for particular purpose
//! - **s. 15**: Implied term - sale by sample
//!
//! ### Consumer Protection (Fair Trading) Act (Cap. 52A)
//! - **s. 4**: False or misleading representation
//! - **s. 5**: Unconscionable conduct
//! - **s. 6**: Bait advertising
//! - **s. 7**: Harassment or coercion
//!
//! ## Small Claims Tribunals
//! - Max claim: SGD 20,000 (SGD 30,000 with consent)
//! - Covers goods/services disputes, motor vehicle accidents
//!
//! ## Lemon Law
//! - Applies to defective goods within 6 months of delivery
//! - Repair, replacement, price reduction, or refund

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Consumer contract with risk assessment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ConsumerContract {
    /// Contract ID
    pub contract_id: String,

    /// Seller/supplier name
    pub seller_name: String,

    /// Seller UEN (if business)
    pub seller_uen: Option<String>,

    /// Consumer name
    pub consumer_name: String,

    /// Contract date
    pub contract_date: DateTime<Utc>,

    /// Type of transaction
    pub transaction_type: TransactionType,

    /// Contract amount in SGD cents
    pub amount_cents: u64,

    /// Description of goods/services
    pub description: String,

    /// Warranty terms (if any)
    pub warranty_terms: Option<WarrantyTerms>,

    /// Contract terms
    pub terms: Vec<ContractTerm>,

    /// Detected unfair practices
    pub unfair_practices: Vec<UnfairPractice>,

    /// Risk score (0-100)
    pub risk_score: u32,
}

impl ConsumerContract {
    /// Creates a new consumer contract
    pub fn new(
        contract_id: impl Into<String>,
        seller_name: impl Into<String>,
        consumer_name: impl Into<String>,
        transaction_type: TransactionType,
        amount_cents: u64,
        description: impl Into<String>,
    ) -> Self {
        Self {
            contract_id: contract_id.into(),
            seller_name: seller_name.into(),
            seller_uen: None,
            consumer_name: consumer_name.into(),
            contract_date: Utc::now(),
            transaction_type,
            amount_cents,
            description: description.into(),
            warranty_terms: None,
            terms: Vec::new(),
            unfair_practices: Vec::new(),
            risk_score: 0,
        }
    }

    /// Adds a contract term
    pub fn add_term(&mut self, term: ContractTerm) {
        self.terms.push(term);
    }

    /// Adds an unfair practice detection
    pub fn add_unfair_practice(&mut self, practice: UnfairPractice) {
        self.unfair_practices.push(practice);
    }

    /// Calculates risk score based on unfair practices
    pub fn calculate_risk_score(&mut self) {
        let mut score = 0u32;

        for practice in &self.unfair_practices {
            score += practice.severity_score();
        }

        // Cap at 100
        self.risk_score = score.min(100);
    }

    /// Checks if eligible for Small Claims Tribunal
    pub fn is_sct_eligible(&self) -> bool {
        let amount_sgd = self.amount_cents / 100;
        amount_sgd <= 20_000 // Can be 30,000 with consent
    }

    /// Checks if Lemon Law applies (within 6 months)
    pub fn is_lemon_law_applicable(&self) -> bool {
        if !matches!(self.transaction_type, TransactionType::SaleOfGoods) {
            return false;
        }

        let elapsed = Utc::now().signed_duration_since(self.contract_date);
        elapsed.num_days() <= 180 // 6 months
    }
}

/// Type of consumer transaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionType {
    /// Sale of physical goods
    SaleOfGoods,
    /// Provision of services
    Services,
    /// Digital goods/content
    DigitalGoods,
    /// Subscription service
    Subscription,
    /// Hire purchase
    HirePurchase,
}

/// Warranty terms
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarrantyTerms {
    /// Warranty duration in days
    pub duration_days: u32,

    /// Warranty type
    pub warranty_type: WarrantyType,

    /// Warranty coverage description
    pub coverage: String,

    /// Exclusions
    pub exclusions: Vec<String>,

    /// Whether transferable
    pub is_transferable: bool,
}

impl WarrantyTerms {
    /// Creates new warranty terms
    pub fn new(
        duration_days: u32,
        warranty_type: WarrantyType,
        coverage: impl Into<String>,
    ) -> Self {
        Self {
            duration_days,
            warranty_type,
            coverage: coverage.into(),
            exclusions: Vec::new(),
            is_transferable: false,
        }
    }

    /// Adds an exclusion
    pub fn add_exclusion(&mut self, exclusion: impl Into<String>) {
        self.exclusions.push(exclusion.into());
    }
}

/// Warranty type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WarrantyType {
    /// Manufacturer warranty
    Manufacturer,
    /// Seller warranty
    Seller,
    /// Extended warranty (paid)
    Extended,
}

/// Contract term
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContractTerm {
    /// Term ID
    pub term_id: String,

    /// Term description
    pub description: String,

    /// Term category
    pub category: TermCategory,

    /// Whether term is potentially unfair
    pub is_potentially_unfair: bool,

    /// Risk indicators
    pub risk_indicators: Vec<String>,
}

impl ContractTerm {
    /// Creates a new contract term
    pub fn new(
        term_id: impl Into<String>,
        description: impl Into<String>,
        category: TermCategory,
    ) -> Self {
        Self {
            term_id: term_id.into(),
            description: description.into(),
            category,
            is_potentially_unfair: false,
            risk_indicators: Vec::new(),
        }
    }

    /// Marks term as potentially unfair
    pub fn mark_unfair(&mut self, reason: impl Into<String>) {
        self.is_potentially_unfair = true;
        self.risk_indicators.push(reason.into());
    }
}

/// Contract term category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TermCategory {
    /// Payment terms
    Payment,
    /// Delivery terms
    Delivery,
    /// Return/refund policy
    ReturnRefund,
    /// Limitation of liability
    LiabilityLimitation,
    /// Dispute resolution
    DisputeResolution,
    /// Termination clause
    Termination,
    /// Warranty disclaimer
    WarrantyDisclaimer,
}

/// Unfair practice under CPFTA
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnfairPractice {
    /// Practice ID
    pub practice_id: String,

    /// Type of unfair practice
    pub practice_type: UnfairPracticeType,

    /// Description of the practice
    pub description: String,

    /// Statute reference
    pub statute_reference: String,

    /// Severity (1-10)
    pub severity: u32,

    /// Evidence/indicators
    pub evidence: Vec<String>,
}

impl UnfairPractice {
    /// Creates a new unfair practice record
    pub fn new(
        practice_id: impl Into<String>,
        practice_type: UnfairPracticeType,
        description: impl Into<String>,
    ) -> Self {
        let statute_ref = practice_type.statute_reference();
        let severity = practice_type.default_severity();

        Self {
            practice_id: practice_id.into(),
            practice_type,
            description: description.into(),
            statute_reference: statute_ref.to_string(),
            severity,
            evidence: Vec::new(),
        }
    }

    /// Adds evidence
    pub fn add_evidence(&mut self, evidence: impl Into<String>) {
        self.evidence.push(evidence.into());
    }

    /// Calculates severity score for risk assessment
    pub fn severity_score(&self) -> u32 {
        self.severity * 5 // Scale to 50 max per practice
    }
}

/// Type of unfair practice (CPFTA s. 4-7)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnfairPracticeType {
    /// False or misleading representation (s. 4)
    FalseRepresentation,

    /// Misleading conduct (s. 4)
    MisleadingConduct,

    /// Unconscionable conduct (s. 5)
    UnconscionableConduct,

    /// Bait advertising (s. 6)
    BaitAdvertising,

    /// Harassment or coercion (s. 7)
    Harassment,

    /// Pyramid selling scheme (s. 7A)
    PyramidScheme,
}

impl UnfairPracticeType {
    /// Returns the statute reference
    pub fn statute_reference(&self) -> &'static str {
        match self {
            UnfairPracticeType::FalseRepresentation => "CPFTA s. 4",
            UnfairPracticeType::MisleadingConduct => "CPFTA s. 4",
            UnfairPracticeType::UnconscionableConduct => "CPFTA s. 5",
            UnfairPracticeType::BaitAdvertising => "CPFTA s. 6",
            UnfairPracticeType::Harassment => "CPFTA s. 7",
            UnfairPracticeType::PyramidScheme => "CPFTA s. 7A",
        }
    }

    /// Returns default severity (1-10)
    pub fn default_severity(&self) -> u32 {
        match self {
            UnfairPracticeType::PyramidScheme => 10,
            UnfairPracticeType::Harassment => 8,
            UnfairPracticeType::UnconscionableConduct => 7,
            UnfairPracticeType::FalseRepresentation => 6,
            UnfairPracticeType::BaitAdvertising => 5,
            UnfairPracticeType::MisleadingConduct => 5,
        }
    }
}

/// Sale of goods contract (SOGA)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SaleOfGoods {
    /// Contract ID
    pub contract_id: String,

    /// Seller is in business (triggers implied terms)
    pub seller_in_business: bool,

    /// Goods description
    pub goods_description: String,

    /// Particular purpose communicated to seller
    pub particular_purpose: Option<String>,

    /// Sale by sample
    pub sale_by_sample: bool,

    /// Implied terms that apply
    pub implied_terms: Vec<ImpliedTerm>,

    /// Whether goods are defective
    pub is_defective: bool,

    /// Defect description (if any)
    pub defect_description: Option<String>,

    /// Purchase date
    pub purchase_date: DateTime<Utc>,
}

impl SaleOfGoods {
    /// Creates a new sale of goods contract
    pub fn new(
        contract_id: impl Into<String>,
        seller_in_business: bool,
        goods_description: impl Into<String>,
    ) -> Self {
        let mut sale = Self {
            contract_id: contract_id.into(),
            seller_in_business,
            goods_description: goods_description.into(),
            particular_purpose: None,
            sale_by_sample: false,
            implied_terms: Vec::new(),
            is_defective: false,
            defect_description: None,
            purchase_date: Utc::now(),
        };

        // Automatically add applicable implied terms
        sale.determine_implied_terms();
        sale
    }

    /// Determines which implied terms apply
    pub fn determine_implied_terms(&mut self) {
        // s. 13 always applies
        self.implied_terms
            .push(ImpliedTerm::CorrespondsToDescription);

        if self.seller_in_business {
            // s. 14(2) applies when seller in business
            self.implied_terms.push(ImpliedTerm::MerchantableQuality);
        }

        if self.particular_purpose.is_some() {
            // s. 14(3) applies when purpose made known
            self.implied_terms.push(ImpliedTerm::FitnessForPurpose);
        }

        if self.sale_by_sample {
            // s. 15 applies
            self.implied_terms.push(ImpliedTerm::SaleBySample);
        }
    }

    /// Reports a defect
    pub fn report_defect(&mut self, description: impl Into<String>) {
        self.is_defective = true;
        self.defect_description = Some(description.into());
    }

    /// Checks if Lemon Law applies
    pub fn is_lemon_law_applicable(&self) -> bool {
        let elapsed = Utc::now().signed_duration_since(self.purchase_date);
        elapsed.num_days() <= 180 // 6 months
    }
}

/// Implied term under Sale of Goods Act
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImpliedTerm {
    /// s. 13: Goods correspond to description
    CorrespondsToDescription,

    /// s. 14(2): Merchantable quality (seller in business)
    MerchantableQuality,

    /// s. 14(3): Fitness for particular purpose
    FitnessForPurpose,

    /// s. 15: Sale by sample
    SaleBySample,
}

impl ImpliedTerm {
    /// Returns the statute reference
    pub fn statute_reference(&self) -> &'static str {
        match self {
            ImpliedTerm::CorrespondsToDescription => "SOGA s. 13",
            ImpliedTerm::MerchantableQuality => "SOGA s. 14(2)",
            ImpliedTerm::FitnessForPurpose => "SOGA s. 14(3)",
            ImpliedTerm::SaleBySample => "SOGA s. 15",
        }
    }

    /// Returns description
    pub fn description(&self) -> &'static str {
        match self {
            ImpliedTerm::CorrespondsToDescription => "Goods must correspond with the description",
            ImpliedTerm::MerchantableQuality => {
                "Goods must be of merchantable quality (fit for ordinary purposes)"
            }
            ImpliedTerm::FitnessForPurpose => {
                "Goods must be reasonably fit for the particular purpose"
            }
            ImpliedTerm::SaleBySample => "Goods must correspond with the sample",
        }
    }
}

/// Remedy for breach of consumer contract
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsumerRemedy {
    /// Repair of goods
    Repair,
    /// Replacement of goods
    Replacement,
    /// Price reduction
    PriceReduction,
    /// Full refund
    Refund,
    /// Rescission of contract
    Rescission,
    /// Damages
    Damages,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consumer_contract_creation() {
        let contract = ConsumerContract::new(
            "c001",
            "Tech Store Pte Ltd",
            "John Tan",
            TransactionType::SaleOfGoods,
            150_000, // SGD 1,500
            "Laptop computer",
        );

        assert_eq!(contract.contract_id, "c001");
        assert_eq!(contract.amount_cents, 150_000);
        assert!(contract.is_sct_eligible());
    }

    #[test]
    fn test_risk_score_calculation() {
        let mut contract = ConsumerContract::new(
            "c002",
            "Dodgy Dealer",
            "Jane Lim",
            TransactionType::Services,
            500_000,
            "Home renovation",
        );

        let practice1 = UnfairPractice::new(
            "p1",
            UnfairPracticeType::FalseRepresentation,
            "Claimed materials were premium grade",
        );

        let practice2 = UnfairPractice::new(
            "p2",
            UnfairPracticeType::Harassment,
            "Threatened legal action for bad review",
        );

        contract.add_unfair_practice(practice1);
        contract.add_unfair_practice(practice2);
        contract.calculate_risk_score();

        // 6*5 + 8*5 = 30 + 40 = 70
        assert_eq!(contract.risk_score, 70);
    }

    #[test]
    fn test_lemon_law_applicability() {
        let contract = ConsumerContract::new(
            "c003",
            "Phone Shop",
            "Ali Rahman",
            TransactionType::SaleOfGoods,
            80_000,
            "Smartphone",
        );

        assert!(contract.is_lemon_law_applicable());
    }

    #[test]
    fn test_sale_of_goods_implied_terms() {
        let sale = SaleOfGoods::new("s001", true, "Washing machine");

        assert!(
            sale.implied_terms
                .contains(&ImpliedTerm::CorrespondsToDescription)
        );
        assert!(
            sale.implied_terms
                .contains(&ImpliedTerm::MerchantableQuality)
        );
    }

    #[test]
    fn test_sale_of_goods_with_purpose() {
        let mut sale = SaleOfGoods::new("s002", true, "Paint");
        sale.particular_purpose = Some("Outdoor metal surfaces".to_string());
        sale.determine_implied_terms();

        assert!(sale.implied_terms.contains(&ImpliedTerm::FitnessForPurpose));
    }

    #[test]
    fn test_unfair_practice_severity() {
        let practice = UnfairPractice::new(
            "p1",
            UnfairPracticeType::PyramidScheme,
            "Multi-level marketing scheme",
        );

        assert_eq!(practice.severity, 10);
        assert_eq!(practice.severity_score(), 50);
        assert_eq!(practice.statute_reference, "CPFTA s. 7A");
    }

    #[test]
    fn test_warranty_terms() {
        let mut warranty = WarrantyTerms::new(
            365,
            WarrantyType::Manufacturer,
            "Defects in materials and workmanship",
        );

        warranty.add_exclusion("Damage from misuse");
        warranty.add_exclusion("Normal wear and tear");

        assert_eq!(warranty.duration_days, 365);
        assert_eq!(warranty.exclusions.len(), 2);
    }

    #[test]
    fn test_sct_eligibility() {
        let contract1 = ConsumerContract::new(
            "c1",
            "Seller A",
            "Buyer A",
            TransactionType::SaleOfGoods,
            1_500_000, // SGD 15,000
            "Item",
        );
        assert!(contract1.is_sct_eligible());

        let contract2 = ConsumerContract::new(
            "c2",
            "Seller B",
            "Buyer B",
            TransactionType::SaleOfGoods,
            2_500_000, // SGD 25,000
            "Item",
        );
        assert!(!contract2.is_sct_eligible());
    }

    #[test]
    fn test_contract_term_marking() {
        let mut term = ContractTerm::new(
            "t1",
            "Seller not liable for any consequential damages",
            TermCategory::LiabilityLimitation,
        );

        term.mark_unfair("Excessively broad liability exclusion");

        assert!(term.is_potentially_unfair);
        assert_eq!(term.risk_indicators.len(), 1);
    }
}
