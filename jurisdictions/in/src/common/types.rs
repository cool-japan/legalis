//! Common types for Indian legal system

use serde::{Deserialize, Serialize};

/// Indian state or union territory
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum State {
    // States (28)
    AndhraPradesh,
    ArunachalPradesh,
    Assam,
    Bihar,
    Chhattisgarh,
    Goa,
    Gujarat,
    Haryana,
    HimachalPradesh,
    Jharkhand,
    Karnataka,
    Kerala,
    MadhyaPradesh,
    Maharashtra,
    Manipur,
    Meghalaya,
    Mizoram,
    Nagaland,
    Odisha,
    Punjab,
    Rajasthan,
    Sikkim,
    TamilNadu,
    Telangana,
    Tripura,
    UttarPradesh,
    Uttarakhand,
    WestBengal,
    // Union Territories (8)
    AndamanAndNicobarIslands,
    Chandigarh,
    DadraAndNagarHaveliAndDamanAndDiu,
    Delhi,
    JammuAndKashmir,
    Ladakh,
    Lakshadweep,
    Puducherry,
}

impl State {
    /// Get the full name of the state
    pub fn full_name(&self) -> &'static str {
        match self {
            State::AndhraPradesh => "Andhra Pradesh",
            State::ArunachalPradesh => "Arunachal Pradesh",
            State::Assam => "Assam",
            State::Bihar => "Bihar",
            State::Chhattisgarh => "Chhattisgarh",
            State::Goa => "Goa",
            State::Gujarat => "Gujarat",
            State::Haryana => "Haryana",
            State::HimachalPradesh => "Himachal Pradesh",
            State::Jharkhand => "Jharkhand",
            State::Karnataka => "Karnataka",
            State::Kerala => "Kerala",
            State::MadhyaPradesh => "Madhya Pradesh",
            State::Maharashtra => "Maharashtra",
            State::Manipur => "Manipur",
            State::Meghalaya => "Meghalaya",
            State::Mizoram => "Mizoram",
            State::Nagaland => "Nagaland",
            State::Odisha => "Odisha",
            State::Punjab => "Punjab",
            State::Rajasthan => "Rajasthan",
            State::Sikkim => "Sikkim",
            State::TamilNadu => "Tamil Nadu",
            State::Telangana => "Telangana",
            State::Tripura => "Tripura",
            State::UttarPradesh => "Uttar Pradesh",
            State::Uttarakhand => "Uttarakhand",
            State::WestBengal => "West Bengal",
            State::AndamanAndNicobarIslands => "Andaman and Nicobar Islands",
            State::Chandigarh => "Chandigarh",
            State::DadraAndNagarHaveliAndDamanAndDiu => "Dadra and Nagar Haveli and Daman and Diu",
            State::Delhi => "Delhi (National Capital Territory)",
            State::JammuAndKashmir => "Jammu and Kashmir",
            State::Ladakh => "Ladakh",
            State::Lakshadweep => "Lakshadweep",
            State::Puducherry => "Puducherry",
        }
    }

    /// Get the state code
    pub fn code(&self) -> &'static str {
        match self {
            State::AndhraPradesh => "AP",
            State::ArunachalPradesh => "AR",
            State::Assam => "AS",
            State::Bihar => "BR",
            State::Chhattisgarh => "CG",
            State::Goa => "GA",
            State::Gujarat => "GJ",
            State::Haryana => "HR",
            State::HimachalPradesh => "HP",
            State::Jharkhand => "JH",
            State::Karnataka => "KA",
            State::Kerala => "KL",
            State::MadhyaPradesh => "MP",
            State::Maharashtra => "MH",
            State::Manipur => "MN",
            State::Meghalaya => "ML",
            State::Mizoram => "MZ",
            State::Nagaland => "NL",
            State::Odisha => "OD",
            State::Punjab => "PB",
            State::Rajasthan => "RJ",
            State::Sikkim => "SK",
            State::TamilNadu => "TN",
            State::Telangana => "TS",
            State::Tripura => "TR",
            State::UttarPradesh => "UP",
            State::Uttarakhand => "UK",
            State::WestBengal => "WB",
            State::AndamanAndNicobarIslands => "AN",
            State::Chandigarh => "CH",
            State::DadraAndNagarHaveliAndDamanAndDiu => "DD",
            State::Delhi => "DL",
            State::JammuAndKashmir => "JK",
            State::Ladakh => "LA",
            State::Lakshadweep => "LD",
            State::Puducherry => "PY",
        }
    }

    /// Check if this is a union territory
    pub fn is_union_territory(&self) -> bool {
        matches!(
            self,
            State::AndamanAndNicobarIslands
                | State::Chandigarh
                | State::DadraAndNagarHaveliAndDamanAndDiu
                | State::Delhi
                | State::JammuAndKashmir
                | State::Ladakh
                | State::Lakshadweep
                | State::Puducherry
        )
    }

    /// Get all states
    pub fn all_states() -> Vec<State> {
        vec![
            State::AndhraPradesh,
            State::ArunachalPradesh,
            State::Assam,
            State::Bihar,
            State::Chhattisgarh,
            State::Goa,
            State::Gujarat,
            State::Haryana,
            State::HimachalPradesh,
            State::Jharkhand,
            State::Karnataka,
            State::Kerala,
            State::MadhyaPradesh,
            State::Maharashtra,
            State::Manipur,
            State::Meghalaya,
            State::Mizoram,
            State::Nagaland,
            State::Odisha,
            State::Punjab,
            State::Rajasthan,
            State::Sikkim,
            State::TamilNadu,
            State::Telangana,
            State::Tripura,
            State::UttarPradesh,
            State::Uttarakhand,
            State::WestBengal,
        ]
    }

    /// Get all union territories
    pub fn all_union_territories() -> Vec<State> {
        vec![
            State::AndamanAndNicobarIslands,
            State::Chandigarh,
            State::DadraAndNagarHaveliAndDamanAndDiu,
            State::Delhi,
            State::JammuAndKashmir,
            State::Ladakh,
            State::Lakshadweep,
            State::Puducherry,
        ]
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.full_name())
    }
}

/// Indian address
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Address {
    /// Building/house number and street
    pub street: String,
    /// Area or locality
    pub locality: Option<String>,
    /// City or town
    pub city: String,
    /// State or union territory
    pub state: State,
    /// PIN code (6 digits)
    pub pin_code: String,
}

impl Address {
    /// Create a new address
    pub fn new(
        street: impl Into<String>,
        city: impl Into<String>,
        state: State,
        pin_code: impl Into<String>,
    ) -> Self {
        Self {
            street: street.into(),
            locality: None,
            city: city.into(),
            state,
            pin_code: pin_code.into(),
        }
    }

    /// Set the locality
    pub fn with_locality(mut self, locality: impl Into<String>) -> Self {
        self.locality = Some(locality.into());
        self
    }

    /// Validate PIN code format
    pub fn is_valid_pin(&self) -> bool {
        self.pin_code.len() == 6 && self.pin_code.chars().all(|c| c.is_ascii_digit())
    }

    /// Format address for display
    pub fn format(&self) -> String {
        let mut parts = vec![self.street.clone()];

        if let Some(locality) = &self.locality {
            parts.push(locality.clone());
        }

        parts.push(format!("{}, {}", self.city, self.state.full_name()));
        parts.push(format!("PIN: {}", self.pin_code));

        parts.join("\n")
    }

    /// Format address as single line
    pub fn format_single_line(&self) -> String {
        let mut parts = vec![self.street.clone()];

        if let Some(locality) = &self.locality {
            parts.push(locality.clone());
        }

        parts.push(self.city.clone());
        parts.push(self.state.full_name().to_string());
        parts.push(self.pin_code.clone());

        parts.join(", ")
    }
}

impl std::fmt::Display for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_details() {
        assert_eq!(State::Maharashtra.full_name(), "Maharashtra");
        assert_eq!(State::Maharashtra.code(), "MH");
        assert!(!State::Maharashtra.is_union_territory());

        assert_eq!(
            State::Delhi.full_name(),
            "Delhi (National Capital Territory)"
        );
        assert_eq!(State::Delhi.code(), "DL");
        assert!(State::Delhi.is_union_territory());
    }

    #[test]
    fn test_all_states_union_territories() {
        let states = State::all_states();
        assert_eq!(states.len(), 28);

        let uts = State::all_union_territories();
        assert_eq!(uts.len(), 8);
    }

    #[test]
    fn test_address_creation() {
        let addr = Address::new("123 MG Road", "Mumbai", State::Maharashtra, "400001")
            .with_locality("Andheri East");

        assert_eq!(addr.street, "123 MG Road");
        assert_eq!(addr.city, "Mumbai");
        assert_eq!(addr.state, State::Maharashtra);
        assert_eq!(addr.pin_code, "400001");
        assert_eq!(addr.locality, Some("Andheri East".to_string()));
    }

    #[test]
    fn test_pin_code_validation() {
        let valid = Address::new("123 Street", "Mumbai", State::Maharashtra, "400001");
        assert!(valid.is_valid_pin());

        let invalid = Address::new("123 Street", "Mumbai", State::Maharashtra, "40001");
        assert!(!invalid.is_valid_pin());

        let invalid2 = Address::new("123 Street", "Mumbai", State::Maharashtra, "40000A");
        assert!(!invalid2.is_valid_pin());
    }

    #[test]
    fn test_address_formatting() {
        let addr = Address::new("123 MG Road", "Mumbai", State::Maharashtra, "400001")
            .with_locality("Andheri East");

        let formatted = addr.format();
        assert!(formatted.contains("123 MG Road"));
        assert!(formatted.contains("Andheri East"));
        assert!(formatted.contains("Mumbai"));
        assert!(formatted.contains("400001"));

        let single_line = addr.format_single_line();
        assert!(single_line.contains("123 MG Road"));
        assert!(single_line.contains(","));
    }
}
