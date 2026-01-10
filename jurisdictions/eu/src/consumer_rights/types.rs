//! Core types for Consumer Rights Directive implementation

use chrono::{DateTime, Utc};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Contract type under Consumer Rights Directive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ContractType {
    /// Distance contract (Article 2(7))
    ///
    /// Contract concluded without the simultaneous physical presence of trader and consumer,
    /// with exclusive use of means of distance communication (internet, phone, mail, etc.)
    Distance,

    /// Off-premises contract (Article 2(8))
    ///
    /// Contract concluded in the simultaneous physical presence of trader and consumer,
    /// in a place which is not the trader's business premises
    OffPremises,
}

/// Distance contract builder (Article 2(7))
#[derive(Debug, Clone)]
pub struct DistanceContract {
    pub trader: Option<String>,
    pub consumer: Option<String>,
    pub contract_date: Option<DateTime<Utc>>,
    pub goods_description: Option<String>,
    pub price_eur: Option<f64>,
    pub delivery_date: Option<DateTime<Utc>>,
    pub information_provided: Vec<InformationRequirement>,
    pub withdrawal_form_provided: bool,
}

impl DistanceContract {
    pub fn new() -> Self {
        Self {
            trader: None,
            consumer: None,
            contract_date: None,
            goods_description: None,
            price_eur: None,
            delivery_date: None,
            information_provided: Vec::new(),
            withdrawal_form_provided: false,
        }
    }

    pub fn with_trader(mut self, trader: impl Into<String>) -> Self {
        self.trader = Some(trader.into());
        self
    }

    pub fn with_consumer(mut self, consumer: impl Into<String>) -> Self {
        self.consumer = Some(consumer.into());
        self
    }

    pub fn with_contract_date(mut self, date: DateTime<Utc>) -> Self {
        self.contract_date = Some(date);
        self
    }

    pub fn with_goods_description(mut self, description: impl Into<String>) -> Self {
        self.goods_description = Some(description.into());
        self
    }

    pub fn with_price_eur(mut self, price: f64) -> Self {
        self.price_eur = Some(price);
        self
    }

    pub fn with_delivery_date(mut self, date: DateTime<Utc>) -> Self {
        self.delivery_date = Some(date);
        self
    }

    pub fn with_information(mut self, info: InformationRequirement) -> Self {
        self.information_provided.push(info);
        self
    }

    pub fn with_withdrawal_form(mut self, provided: bool) -> Self {
        self.withdrawal_form_provided = provided;
        self
    }
}

impl Default for DistanceContract {
    fn default() -> Self {
        Self::new()
    }
}

/// Off-premises contract builder (Article 2(8))
#[derive(Debug, Clone)]
pub struct OffPremisesContract {
    pub trader: Option<String>,
    pub consumer: Option<String>,
    pub contract_date: Option<DateTime<Utc>>,
    pub location: Option<String>, // Where contract was concluded (not trader's premises)
    pub goods_description: Option<String>,
    pub price_eur: Option<f64>,
    pub information_provided: Vec<InformationRequirement>,
    pub withdrawal_form_provided: bool,
}

impl OffPremisesContract {
    pub fn new() -> Self {
        Self {
            trader: None,
            consumer: None,
            contract_date: None,
            location: None,
            goods_description: None,
            price_eur: None,
            information_provided: Vec::new(),
            withdrawal_form_provided: false,
        }
    }

    pub fn with_trader(mut self, trader: impl Into<String>) -> Self {
        self.trader = Some(trader.into());
        self
    }

    pub fn with_consumer(mut self, consumer: impl Into<String>) -> Self {
        self.consumer = Some(consumer.into());
        self
    }

    pub fn with_contract_date(mut self, date: DateTime<Utc>) -> Self {
        self.contract_date = Some(date);
        self
    }

    pub fn with_location(mut self, location: impl Into<String>) -> Self {
        self.location = Some(location.into());
        self
    }

    pub fn with_goods_description(mut self, description: impl Into<String>) -> Self {
        self.goods_description = Some(description.into());
        self
    }

    pub fn with_price_eur(mut self, price: f64) -> Self {
        self.price_eur = Some(price);
        self
    }

    pub fn with_information(mut self, info: InformationRequirement) -> Self {
        self.information_provided.push(info);
        self
    }

    pub fn with_withdrawal_form(mut self, provided: bool) -> Self {
        self.withdrawal_form_provided = provided;
        self
    }
}

impl Default for OffPremisesContract {
    fn default() -> Self {
        Self::new()
    }
}

/// Information requirements under Article 6
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum InformationRequirement {
    /// Main characteristics of goods/services
    MainCharacteristics,

    /// Identity and address of trader
    TraderIdentity,

    /// Total price including taxes
    TotalPrice,

    /// Arrangements for payment, delivery, performance
    ArrangementsForPaymentDelivery,

    /// Existence of right of withdrawal
    RightOfWithdrawal,

    /// Model withdrawal form
    ModelWithdrawalForm,

    /// Cost of returning goods if applicable
    CostOfReturning,

    /// Reminder of guarantee of conformity (2 years)
    GuaranteeOfConformity,

    /// After-sales customer assistance and commercial guarantees
    AfterSalesService,

    /// Complaint handling policy
    ComplaintHandling,
}

/// Exceptions to right of withdrawal (Article 17)
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum WithdrawalException {
    /// Article 17(a) - Service contracts fully performed, with consent, before end of withdrawal period
    ServiceFullyPerformed,

    /// Article 17(b) - Goods/services with price dependent on financial market fluctuations
    PriceDependentOnMarket,

    /// Article 17(c) - Goods made to consumer's specifications or clearly personalized
    CustomMadeGoods,

    /// Article 17(d) - Goods liable to deteriorate or expire rapidly (perishable)
    PerishableGoods,

    /// Article 17(e) - Sealed goods unsuitable for return (hygiene/health, unsealed by consumer)
    SealedGoodsUnsealed,

    /// Article 17(f) - Goods inseparably mixed with other items after delivery
    GoodsMixed,

    /// Article 17(g) - Alcoholic beverages (price agreed at conclusion, delivery only after 30 days)
    AlcoholicBeverages,

    /// Article 17(h) - Urgent repairs/maintenance requested by consumer
    UrgentRepairs,

    /// Article 17(i) - Sealed audio/video/software unsealed by consumer
    SealedMediaUnsealed,

    /// Article 17(j) - Newspapers, periodicals, magazines (except subscription contracts)
    Newspapers,

    /// Article 17(k) - Public auctions
    PublicAuction,

    /// Article 17(l) - Accommodation, transport, car rental, catering, leisure (specific date/period)
    AccommodationTransportSpecificDate,

    /// Article 17(m) - Digital content not on tangible medium, with consumer's consent
    DigitalContentWithConsent,
}

/// Exception to withdrawal right
#[derive(Debug, Clone)]
pub struct ExceptionToWithdrawal {
    pub exception_type: WithdrawalException,
    pub justification: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_contract_builder() {
        let contract = DistanceContract::new()
            .with_trader("Online Shop")
            .with_consumer("John Doe")
            .with_price_eur(99.99)
            .with_goods_description("Laptop");

        assert_eq!(contract.trader, Some("Online Shop".to_string()));
        assert_eq!(contract.price_eur, Some(99.99));
    }

    #[test]
    fn test_off_premises_contract_builder() {
        let contract = OffPremisesContract::new()
            .with_trader("Door-to-door Sales")
            .with_location("Consumer's home")
            .with_price_eur(199.99);

        assert_eq!(contract.location, Some("Consumer's home".to_string()));
    }

    #[test]
    fn test_information_requirements() {
        let contract = DistanceContract::new()
            .with_information(InformationRequirement::TotalPrice)
            .with_information(InformationRequirement::RightOfWithdrawal);

        assert_eq!(contract.information_provided.len(), 2);
    }
}
