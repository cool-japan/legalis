//! End of Service Gratuity (EOSG) - Article 51

use crate::common::Aed;
use serde::{Deserialize, Serialize};

/// End of Service Gratuity calculation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndOfServiceGratuity {
    pub years_of_service: u32,
    pub basic_salary: Aed,
    pub gratuity_amount: Aed,
    pub daily_wage: Aed,
}

impl EndOfServiceGratuity {
    pub fn calculate(years_of_service: u32, basic_salary: Aed) -> Self {
        let daily_wage = Aed::from_fils(basic_salary.fils() / 30);

        let years_first_five = years_of_service.min(5);
        let years_after_five = years_of_service.saturating_sub(5);

        let gratuity_first_five = daily_wage.fils() * 21 * years_first_five as i64;
        let gratuity_after_five = daily_wage.fils() * 30 * years_after_five as i64;

        let total_gratuity = gratuity_first_five + gratuity_after_five;
        let max_gratuity = basic_salary.fils() * 24;

        let gratuity_amount = Aed::from_fils(total_gratuity.min(max_gratuity));

        Self {
            years_of_service,
            basic_salary,
            gratuity_amount,
            daily_wage,
        }
    }
}
