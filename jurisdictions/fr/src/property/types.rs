//! Types for French property law
//!
//! This module defines the core types for representing properties, easements,
//! and encumbrances under French law.

use serde::{Deserialize, Serialize};

/// Represents a property (bien) under French law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Property {
    pub property_type: PropertyType,
    pub owner: String,
    pub location: String,
    pub value: u64,
    pub easements: Vec<Easement>,
    pub encumbrances: Vec<Encumbrance>,
}

impl Property {
    /// Creates a new property
    pub fn new(property_type: PropertyType, owner: String, location: String, value: u64) -> Self {
        Self {
            property_type,
            owner,
            location,
            value,
            easements: Vec::new(),
            encumbrances: Vec::new(),
        }
    }

    /// Adds an easement to the property
    pub fn with_easement(mut self, easement: Easement) -> Self {
        self.easements.push(easement);
        self
    }

    /// Adds multiple easements
    pub fn with_easements(mut self, easements: Vec<Easement>) -> Self {
        self.easements = easements;
        self
    }

    /// Adds an encumbrance to the property
    pub fn with_encumbrance(mut self, encumbrance: Encumbrance) -> Self {
        self.encumbrances.push(encumbrance);
        self
    }

    /// Adds multiple encumbrances
    pub fn with_encumbrances(mut self, encumbrances: Vec<Encumbrance>) -> Self {
        self.encumbrances = encumbrances;
        self
    }

    /// Checks if property is immovable (real estate)
    pub fn is_immovable(&self) -> bool {
        matches!(self.property_type, PropertyType::Immovable { .. })
    }

    /// Checks if property is movable
    pub fn is_movable(&self) -> bool {
        matches!(self.property_type, PropertyType::Movable { .. })
    }
}

/// Types of property under French law
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropertyType {
    /// Immovable property (immeuble) - real estate
    Immovable {
        land_area: f64,             // in square meters
        building_area: Option<f64>, // in square meters
    },
    /// Movable property (meuble)
    Movable { description: String },
}

/// Represents an easement (servitude) on property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Easement {
    pub easement_type: EasementType,
    pub servient_estate: String, //承役地 (property bearing the burden)
    pub dominant_estate: Option<String>, // 要役地 (property receiving the benefit)
    pub description: String,
}

impl Easement {
    /// Creates a new easement
    pub fn new(easement_type: EasementType, servient_estate: String) -> Self {
        Self {
            easement_type,
            servient_estate,
            dominant_estate: None,
            description: String::new(),
        }
    }

    /// Sets the dominant estate (property receiving benefit)
    pub fn with_dominant_estate(mut self, estate: String) -> Self {
        self.dominant_estate = Some(estate);
        self
    }

    /// Sets the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

/// Types of easements under French law
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasementType {
    /// Right of way (droit de passage) - Article 682
    RightOfWay,
    /// Water rights (servitude d'eau) - Article 640-649
    WaterRights,
    /// Support and lateral support (servitude de support)
    Support,
    /// Light and air (servitude de vue et de jour)
    Light,
    /// Drainage (servitude d'écoulement)
    Drainage,
    /// Landlocked property access (servitude de passage forcé) - Article 682
    LandlockedAccess,
    /// Custom easement
    Custom(String),
}

/// Represents an encumbrance (charge) on property
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Encumbrance {
    pub encumbrance_type: EncumbranceType,
    pub amount: Option<u64>,
    pub beneficiary: String,
    pub description: String,
}

impl Encumbrance {
    /// Creates a new encumbrance
    pub fn new(encumbrance_type: EncumbranceType, beneficiary: String) -> Self {
        Self {
            encumbrance_type,
            amount: None,
            beneficiary,
            description: String::new(),
        }
    }

    /// Sets the amount
    pub fn with_amount(mut self, amount: u64) -> Self {
        self.amount = Some(amount);
        self
    }

    /// Sets the description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = description;
        self
    }
}

/// Types of encumbrances under French law
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncumbranceType {
    /// Mortgage (hypothèque)
    Mortgage,
    /// Lien (privilège)
    Lien,
    /// Usufruct rights (usufruit)
    UsufructRights,
    /// Right of use and habitation (droit d'usage et d'habitation)
    UseAndHabitation,
}

/// Represents an asset type (for Article 490 classification)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AssetType {
    /// Immovable by nature (immeuble par nature)
    ImmovableByNature,
    /// Immovable by destination (immeuble par destination)
    ImmovableByDestination,
    /// Movable by nature (meuble par nature)
    MovableByNature,
    /// Movable by anticipation (meuble par anticipation)
    MovableByAnticipation,
}

/// Represents an asset classification result
#[derive(Debug, Clone, PartialEq)]
pub struct Asset {
    pub description: String,
    pub asset_type: AssetType,
    pub value: Option<u64>,
}

impl Asset {
    /// Creates a new asset
    pub fn new(description: String, asset_type: AssetType) -> Self {
        Self {
            description,
            asset_type,
            value: None,
        }
    }

    /// Sets the value
    pub fn with_value(mut self, value: u64) -> Self {
        self.value = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_creation() {
        let property = Property::new(
            PropertyType::Immovable {
                land_area: 1000.0,
                building_area: Some(200.0),
            },
            "Jean Dupont".to_string(),
            "Paris, France".to_string(),
            500_000,
        );

        assert_eq!(property.owner, "Jean Dupont");
        assert_eq!(property.value, 500_000);
        assert!(property.is_immovable());
        assert!(!property.is_movable());
    }

    #[test]
    fn test_property_with_easement() {
        let easement = Easement::new(EasementType::RightOfWay, "Servient property".to_string())
            .with_dominant_estate("Dominant property".to_string())
            .with_description("Access to public road".to_string());

        let property = Property::new(
            PropertyType::Immovable {
                land_area: 1000.0,
                building_area: None,
            },
            "Owner".to_string(),
            "Location".to_string(),
            300_000,
        )
        .with_easement(easement);

        assert_eq!(property.easements.len(), 1);
        assert_eq!(
            property.easements[0].easement_type,
            EasementType::RightOfWay
        );
    }

    #[test]
    fn test_property_with_encumbrance() {
        let mortgage = Encumbrance::new(EncumbranceType::Mortgage, "Bank".to_string())
            .with_amount(200_000)
            .with_description("First mortgage".to_string());

        let property = Property::new(
            PropertyType::Immovable {
                land_area: 500.0,
                building_area: Some(150.0),
            },
            "Owner".to_string(),
            "Location".to_string(),
            400_000,
        )
        .with_encumbrance(mortgage);

        assert_eq!(property.encumbrances.len(), 1);
        assert_eq!(
            property.encumbrances[0].encumbrance_type,
            EncumbranceType::Mortgage
        );
        assert_eq!(property.encumbrances[0].amount, Some(200_000));
    }

    #[test]
    fn test_movable_property() {
        let property = Property::new(
            PropertyType::Movable {
                description: "Car".to_string(),
            },
            "Owner".to_string(),
            "Garage".to_string(),
            20_000,
        );

        assert!(property.is_movable());
        assert!(!property.is_immovable());
    }

    #[test]
    fn test_easement_types() {
        let right_of_way = EasementType::RightOfWay;
        let water = EasementType::WaterRights;
        let custom = EasementType::Custom("Special access".to_string());

        assert_eq!(right_of_way, EasementType::RightOfWay);
        assert_eq!(water, EasementType::WaterRights);
        assert!(matches!(custom, EasementType::Custom(_)));
    }

    #[test]
    fn test_encumbrance_types() {
        assert_eq!(EncumbranceType::Mortgage, EncumbranceType::Mortgage);
        assert_eq!(EncumbranceType::Lien, EncumbranceType::Lien);
        assert_eq!(
            EncumbranceType::UsufructRights,
            EncumbranceType::UsufructRights
        );
    }

    #[test]
    fn test_asset_classification() {
        let land = Asset::new("Land".to_string(), AssetType::ImmovableByNature).with_value(500_000);
        let furniture = Asset::new("Furniture".to_string(), AssetType::MovableByNature);

        assert_eq!(land.asset_type, AssetType::ImmovableByNature);
        assert_eq!(land.value, Some(500_000));
        assert_eq!(furniture.asset_type, AssetType::MovableByNature);
        assert_eq!(furniture.value, None);
    }

    #[test]
    fn test_easement_builder() {
        let easement = Easement::new(EasementType::LandlockedAccess, "Property A".to_string())
            .with_dominant_estate("Property B".to_string())
            .with_description("Forced passage for landlocked property".to_string());

        assert_eq!(easement.easement_type, EasementType::LandlockedAccess);
        assert_eq!(easement.servient_estate, "Property A");
        assert_eq!(easement.dominant_estate, Some("Property B".to_string()));
        assert!(easement.description.contains("Forced passage"));
    }

    #[test]
    fn test_property_multiple_easements() {
        let easement1 = Easement::new(EasementType::RightOfWay, "Property".to_string());
        let easement2 = Easement::new(EasementType::WaterRights, "Property".to_string());

        let property = Property::new(
            PropertyType::Immovable {
                land_area: 2000.0,
                building_area: None,
            },
            "Owner".to_string(),
            "Rural area".to_string(),
            300_000,
        )
        .with_easements(vec![easement1, easement2]);

        assert_eq!(property.easements.len(), 2);
    }

    #[test]
    fn test_property_multiple_encumbrances() {
        let mortgage =
            Encumbrance::new(EncumbranceType::Mortgage, "Bank 1".to_string()).with_amount(300_000);
        let usufruct = Encumbrance::new(EncumbranceType::UsufructRights, "Spouse".to_string());

        let property = Property::new(
            PropertyType::Immovable {
                land_area: 1000.0,
                building_area: Some(250.0),
            },
            "Owner".to_string(),
            "City center".to_string(),
            600_000,
        )
        .with_encumbrances(vec![mortgage, usufruct]);

        assert_eq!(property.encumbrances.len(), 2);
    }
}
