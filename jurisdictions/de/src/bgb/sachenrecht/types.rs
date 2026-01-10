//! BGB Property Law Types (Sachenrecht)
//!
//! Comprehensive type system for German property law under BGB Book 3.

use crate::gmbhg::Capital;
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

// ============================================================================
// Core Property Concepts
// ============================================================================

/// Property party (natural person or legal entity)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PropertyParty {
    pub name: String,
    pub address: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub is_natural_person: bool,
}

/// Property type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PropertyType {
    Movable,   // Bewegliche Sache (§90 BGB)
    Immovable, // Unbewegliche Sache (Grundstück)
}

/// Thing classification (§90 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Thing {
    pub description: String,
    pub property_type: PropertyType,
    pub value: Capital,
    pub is_consumable: bool, // Verbrauchbare Sache (§92 BGB)
    pub is_fungible: bool,   // Vertretbare Sache (§91 BGB)
    pub location: Option<String>,
}

// ============================================================================
// Ownership (Eigentum) - §§903-924 BGB
// ============================================================================

/// Ownership structure (§903 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ownership {
    pub owner: PropertyParty,
    pub thing: Thing,
    pub acquired_at: DateTime<Utc>,
    pub acquisition_method: AcquisitionMethod,
    pub encumbrances: Vec<Encumbrance>,
    pub restrictions: Vec<OwnershipRestriction>,
}

/// Method of acquiring ownership
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquisitionMethod {
    Transfer,          // Übertragung (§§929-936 BGB)
    Inheritance,       // Erbfolge
    Accession,         // Verbindung, Vermischung, Verarbeitung (§§946-950 BGB)
    Acquisition,       // Aneignung (§958 BGB)
    FindersRights,     // Fund (§§965-984 BGB)
    Adversepossession, // Ersitzung (§§937-945 BGB)
}

/// Encumbrance on property (Belastung)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Encumbrance {
    Mortgage {
        amount: Capital,
        creditor: String,
        registered_at: DateTime<Utc>,
    },
    LandCharge {
        amount: Capital,
        creditor: String,
        purpose: String,
    },
    Easement {
        easement_type: EasementType,
        beneficiary: String,
    },
    Lien {
        amount: Capital,
        creditor: String,
    },
}

/// Ownership restrictions (§903 BGB limitations)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnershipRestriction {
    LegalRestriction {
        statute: String,
        description: String,
    },
    ContractualRestriction {
        agreement: String,
        description: String,
    },
    PublicLaw {
        regulation: String,
        description: String,
    },
}

// ============================================================================
// Transfer of Movables (§§929-936 BGB)
// ============================================================================

/// Transfer of movable property (§929 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovableTransfer {
    pub transferor: PropertyParty,
    pub transferee: PropertyParty,
    pub thing: Thing,
    pub transfer_type: MovableTransferType,
    pub agreement: TransferAgreement,
    pub delivery: Option<Delivery>,
    pub consideration: Option<Capital>,
    pub transferred_at: DateTime<Utc>,
    pub good_faith: bool,
}

/// Type of movable transfer
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MovableTransferType {
    ActualDelivery,         // §929 S. 1 BGB - mit Übergabe
    BriefHandDelivery,      // §929 S. 2 BGB - Brief hand
    ConstructivePossession, // §930 BGB - Besitzkonstitut
    AssignmentOfClaim,      // §931 BGB - Abtretung des Herausgabeanspruchs
}

/// Transfer agreement (Einigung)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferAgreement {
    pub agreement_reached: bool,
    pub agreed_at: DateTime<Utc>,
    pub transfer_intent: bool,   // Willen, Eigentum zu übertragen
    pub acceptance_intent: bool, // Willen, Eigentum zu erwerben
}

/// Delivery of possession (Übergabe)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Delivery {
    pub delivered: bool,
    pub delivered_at: Option<DateTime<Utc>>,
    pub delivery_method: DeliveryMethod,
}

/// Method of delivery
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeliveryMethod {
    PhysicalHandover,     // Tatsächliche Übergabe
    SymbolicDelivery,     // Symbolische Übergabe
    PossessionTransfer,   // Besitzübertragung
    ConstructiveDelivery, // Übergabesurrogat
}

// ============================================================================
// Transfer of Immovables (§§873-902 BGB)
// ============================================================================

/// Transfer of immovable property (§873 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImmovableTransfer {
    pub transferor: PropertyParty,
    pub transferee: PropertyParty,
    pub land_parcel: LandParcel,
    pub agreement: TransferAgreement,
    pub registration: LandRegistryEntry,
    pub consideration: Option<Capital>,
    pub transferred_at: DateTime<Utc>,
}

/// Land parcel (Grundstück)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandParcel {
    pub parcel_number: String,          // Flurstücksnummer
    pub land_registry_district: String, // Grundbuchbezirk
    pub size_square_meters: u64,
    pub location: String,
    pub description: String,
    pub value: Capital,
}

/// Land registry entry (Grundbucheintragung)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LandRegistryEntry {
    pub registered: bool,
    pub registration_date: Option<NaiveDate>,
    pub registry_office: String,
    pub section: LandRegistrySection,
    pub entry_number: Option<String>,
}

/// Land registry sections (Abteilungen)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LandRegistrySection {
    SectionI,   // Abteilung I - Eigentümer (Owner)
    SectionII,  // Abteilung II - Lasten und Beschränkungen (Encumbrances)
    SectionIII, // Abteilung III - Hypotheken, Grundschulden (Mortgages, land charges)
}

// ============================================================================
// Possession (Besitz) - §§854-872, 1006-1011 BGB
// ============================================================================

/// Possession structure (§854 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Possession {
    pub possessor: PropertyParty,
    pub thing: Thing,
    pub possession_type: PossessionType,
    pub acquired_at: DateTime<Utc>,
    pub factual_control: bool, // Tatsächliche Gewalt
    pub possession_will: bool, // Besitzwille
}

/// Type of possession
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PossessionType {
    DirectPossession,   // Unmittelbarer Besitz (§854 BGB)
    IndirectPossession, // Mittelbarer Besitz (§868 BGB)
    JointPossession,    // Mitbesitz (§866 BGB)
    OwnerPossession,    // Eigenbesitz (§872 BGB)
    HolderPossession,   // Fremdbesitz (possession for another)
}

/// Possession protection claim (§§861-862 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PossessionProtectionClaim {
    pub claimant: PropertyParty,
    pub respondent: PropertyParty,
    pub thing: Thing,
    pub claim_type: PossessionClaimType,
    pub interference_date: DateTime<Utc>,
    pub unlawful_interference: bool,
    pub within_one_year: bool, // §864 BGB - one year limitation
}

/// Type of possession protection claim
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PossessionClaimType {
    Restitution,      // §861 BGB - Rückgabe bei Entziehung
    CessationOfForce, // §862 BGB - Beseitigung/Unterlassung bei Störung
}

// ============================================================================
// Easements (Dienstbarkeiten) - §§1018-1093 BGB
// ============================================================================

/// Easement structure (Grunddienstbarkeit §1018 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Easement {
    pub easement_type: EasementType,
    pub servient_land: LandParcel,          // Dienendes Grundstück
    pub dominant_land: Option<LandParcel>,  // Herrschendes Grundstück (for predial easements)
    pub beneficiary: Option<PropertyParty>, // For personal easements
    pub established_at: DateTime<Utc>,
    pub establishment_method: EasementEstablishment,
    pub registered: bool,
}

/// Type of easement
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementType {
    RightOfWay { path_description: String },        // Wegerecht
    RightOfPassage { access_description: String },  // Durchgangsrecht
    RightToDrawWater { source_location: String },   // Wasserschöpfungsrecht
    RightToLight { protected_windows: String },     // Lichtrecht
    RightToSupport { support_description: String }, // Stützungsrecht
    OtherEasement { description: String },
}

/// Method of easement establishment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementEstablishment {
    Agreement,    // Vertrag
    Inheritance,  // Erbfolge
    Prescription, // Ersitzung (§1028 BGB)
    CourtOrder,   // Gerichtliche Entscheidung
}

// ============================================================================
// Mortgages and Land Charges (§§1113-1203 BGB)
// ============================================================================

/// Mortgage (Hypothek §1113 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mortgage {
    pub land_parcel: LandParcel,
    pub creditor: PropertyParty,
    pub debtor: PropertyParty,
    pub secured_claim: SecuredClaim,
    pub mortgage_amount: Capital,
    pub priority_rank: u32, // Rang
    pub registered_at: DateTime<Utc>,
    pub registry_entry: LandRegistryEntry,
}

/// Land charge (Grundschuld §1191 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LandCharge {
    pub land_parcel: LandParcel,
    pub creditor: PropertyParty,
    pub debtor: PropertyParty,
    pub charge_amount: Capital,
    pub purpose: String,
    pub priority_rank: u32,
    pub registered_at: DateTime<Utc>,
    pub registry_entry: LandRegistryEntry,
    pub is_owner_land_charge: bool, // Eigentümergrundschuld
}

/// Secured claim for mortgage
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SecuredClaim {
    pub claim_description: String,
    pub claim_amount: Capital,
    pub interest_rate: Option<f64>, // Annual percentage
    pub maturity_date: Option<NaiveDate>,
    pub claim_exists: bool,
}

// ============================================================================
// Pledges (Pfandrecht) - §§1204-1259 BGB
// ============================================================================

/// Pledge on movables (Pfandrecht an beweglichen Sachen §1204 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovablePledge {
    pub pledgor: PropertyParty,
    pub pledgee: PropertyParty,
    pub pledged_thing: Thing,
    pub secured_claim: SecuredClaim,
    pub possession_transferred: bool,
    pub established_at: DateTime<Utc>,
}

/// Pledge on rights (Pfandrecht an Rechten §1273 BGB)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RightsPledge {
    pub pledgor: PropertyParty,
    pub pledgee: PropertyParty,
    pub pledged_right: PledgedRight,
    pub secured_claim: SecuredClaim,
    pub established_at: DateTime<Utc>,
}

/// Type of right that can be pledged
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PledgedRight {
    Claim {
        claim_description: String,
        amount: Capital,
    },
    Share {
        company_name: String,
        share_count: u64,
    },
    IntellectualProperty {
        description: String,
    },
    Other {
        description: String,
    },
}

// ============================================================================
// Good Faith Acquisition (§§932-936 BGB)
// ============================================================================

/// Good faith acquisition (Gutgläubiger Erwerb)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoodFaithAcquisition {
    pub transfer: MovableTransfer,
    pub transferor_not_owner: bool,
    pub good_faith: bool,                          // Guter Glaube (§932 BGB)
    pub no_gross_negligence: bool,                 // Keine grobe Fahrlässigkeit
    pub acquired_through_voluntary_transfer: bool, // §935 BGB - no theft
    pub acquisition_valid: bool,
}

// ============================================================================
// Builder Patterns
// ============================================================================

/// Builder for movable transfers
#[derive(Debug, Default)]
pub struct MovableTransferBuilder {
    transferor: Option<PropertyParty>,
    transferee: Option<PropertyParty>,
    thing: Option<Thing>,
    transfer_type: Option<MovableTransferType>,
    agreement: Option<TransferAgreement>,
    delivery: Option<Delivery>,
    consideration: Option<Capital>,
    good_faith: Option<bool>,
}

impl MovableTransferBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn transferor(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.transferor = Some(PropertyParty {
            name: name.into(),
            address: Some(address.into()),
            date_of_birth: None,
            is_natural_person: true,
        });
        self
    }

    pub fn transferee(mut self, name: impl Into<String>, address: impl Into<String>) -> Self {
        self.transferee = Some(PropertyParty {
            name: name.into(),
            address: Some(address.into()),
            date_of_birth: None,
            is_natural_person: true,
        });
        self
    }

    pub fn thing(mut self, description: impl Into<String>, value: Capital) -> Self {
        self.thing = Some(Thing {
            description: description.into(),
            property_type: PropertyType::Movable,
            value,
            is_consumable: false,
            is_fungible: false,
            location: None,
        });
        self
    }

    pub fn transfer_type(mut self, transfer_type: MovableTransferType) -> Self {
        self.transfer_type = Some(transfer_type);
        self
    }

    pub fn agreement(mut self, agreed_at: DateTime<Utc>) -> Self {
        self.agreement = Some(TransferAgreement {
            agreement_reached: true,
            agreed_at,
            transfer_intent: true,
            acceptance_intent: true,
        });
        self
    }

    pub fn delivery(mut self, delivered_at: DateTime<Utc>, method: DeliveryMethod) -> Self {
        self.delivery = Some(Delivery {
            delivered: true,
            delivered_at: Some(delivered_at),
            delivery_method: method,
        });
        self
    }

    pub fn consideration(mut self, amount: Capital) -> Self {
        self.consideration = Some(amount);
        self
    }

    pub fn good_faith(mut self, is_good_faith: bool) -> Self {
        self.good_faith = Some(is_good_faith);
        self
    }

    pub fn build(self) -> Result<MovableTransfer, String> {
        let agreement = self.agreement.ok_or("Agreement required")?;
        let transferred_at = agreement.agreed_at;

        Ok(MovableTransfer {
            transferor: self.transferor.ok_or("Transferor required")?,
            transferee: self.transferee.ok_or("Transferee required")?,
            thing: self.thing.ok_or("Thing required")?,
            transfer_type: self.transfer_type.ok_or("Transfer type required")?,
            agreement,
            delivery: self.delivery,
            consideration: self.consideration,
            transferred_at,
            good_faith: self.good_faith.unwrap_or(true),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_movable_transfer_builder() {
        let transfer = MovableTransferBuilder::new()
            .transferor("Max Mustermann", "Berlin")
            .transferee("Erika Schmidt", "Munich")
            .thing("Used car - VW Golf", Capital::from_euros(15_000))
            .transfer_type(MovableTransferType::ActualDelivery)
            .agreement(Utc::now())
            .delivery(Utc::now(), DeliveryMethod::PhysicalHandover)
            .consideration(Capital::from_euros(15_000))
            .good_faith(true)
            .build();

        assert!(transfer.is_ok());
        let transfer = transfer.unwrap();
        assert_eq!(transfer.transferor.name, "Max Mustermann");
        assert_eq!(transfer.transferee.name, "Erika Schmidt");
        assert!(transfer.good_faith);
    }

    #[test]
    fn test_property_types() {
        let movable = Thing {
            description: "Bicycle".to_string(),
            property_type: PropertyType::Movable,
            value: Capital::from_euros(500),
            is_consumable: false,
            is_fungible: false,
            location: None,
        };
        assert_eq!(movable.property_type, PropertyType::Movable);
    }

    #[test]
    fn test_possession_types() {
        let possession = Possession {
            possessor: PropertyParty {
                name: "Owner".to_string(),
                address: None,
                date_of_birth: None,
                is_natural_person: true,
            },
            thing: Thing {
                description: "Book".to_string(),
                property_type: PropertyType::Movable,
                value: Capital::from_euros(20),
                is_consumable: false,
                is_fungible: true,
                location: None,
            },
            possession_type: PossessionType::DirectPossession,
            acquired_at: Utc::now(),
            factual_control: true,
            possession_will: true,
        };
        assert_eq!(possession.possession_type, PossessionType::DirectPossession);
    }
}
