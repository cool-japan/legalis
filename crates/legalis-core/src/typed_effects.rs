//! Strongly-typed effects with generic parameter constraints.
//!
//! This module provides a type-safe alternative to the stringly-typed Effect
//! parameters, allowing effects to have compile-time validated parameters.
//!
//! # Examples
//!
//! ```
//! use legalis_core::typed_effects::{TypedEffect, GrantEffect, MonetaryEffect};
//!
//! // Create a grant effect with type-safe parameters
//! let grant = GrantEffect::new("driver_license")
//!     .with_duration_years(5)
//!     .with_renewable(true);
//!
//! let effect: TypedEffect = grant.into();
//! assert_eq!(effect.description(), "Grant: driver_license");
//! ```

use crate::{Effect, EffectType};
use std::collections::HashMap;
use std::marker::PhantomData;

/// Trait for typed effect parameters.
///
/// Implement this trait to create custom effect parameter types.
pub trait EffectParameter: Clone {
    /// Get the parameter as a string value.
    fn as_string(&self) -> String;

    /// Try to parse from a string value.
    fn from_string(s: &str) -> Result<Self, String>
    where
        Self: Sized;
}

/// Strongly-typed effect wrapper.
///
/// Wraps the base Effect type with compile-time parameter validation.
#[derive(Debug, Clone, PartialEq)]
pub struct TypedEffect {
    effect: Effect,
}

impl TypedEffect {
    /// Create from a base Effect.
    pub fn from_effect(effect: Effect) -> Self {
        Self { effect }
    }

    /// Get the underlying Effect.
    pub fn into_effect(self) -> Effect {
        self.effect
    }

    /// Get a reference to the underlying Effect.
    pub fn as_effect(&self) -> &Effect {
        &self.effect
    }

    /// Get the effect type.
    pub fn effect_type(&self) -> &EffectType {
        &self.effect.effect_type
    }

    /// Get the description.
    pub fn description(&self) -> &str {
        &self.effect.description
    }

    /// Get a typed parameter value.
    pub fn get_parameter<T: EffectParameter>(&self, key: &str) -> Result<T, String> {
        self.effect
            .get_parameter(key)
            .ok_or_else(|| format!("Parameter '{}' not found", key))
            .and_then(|s| T::from_string(s))
    }
}

/// Grant effect with typed parameters.
///
/// # Examples
///
/// ```
/// use legalis_core::typed_effects::GrantEffect;
///
/// let grant = GrantEffect::new("passport")
///     .with_duration_years(10)
///     .with_renewable(true)
///     .with_region("US");
///
/// assert_eq!(grant.resource(), "passport");
/// assert_eq!(grant.duration_years(), Some(10));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GrantEffect {
    resource: String,
    duration_years: Option<u32>,
    renewable: bool,
    region: Option<String>,
    conditions: Vec<String>,
}

impl GrantEffect {
    /// Create a new grant effect.
    pub fn new(resource: impl Into<String>) -> Self {
        Self {
            resource: resource.into(),
            duration_years: None,
            renewable: false,
            region: None,
            conditions: Vec::new(),
        }
    }

    /// Set the duration in years.
    pub fn with_duration_years(mut self, years: u32) -> Self {
        self.duration_years = Some(years);
        self
    }

    /// Set whether the grant is renewable.
    pub fn with_renewable(mut self, renewable: bool) -> Self {
        self.renewable = renewable;
        self
    }

    /// Set the applicable region.
    pub fn with_region(mut self, region: impl Into<String>) -> Self {
        self.region = Some(region.into());
        self
    }

    /// Add a condition.
    pub fn with_condition(mut self, condition: impl Into<String>) -> Self {
        self.conditions.push(condition.into());
        self
    }

    /// Get the resource being granted.
    pub fn resource(&self) -> &str {
        &self.resource
    }

    /// Get the duration in years.
    pub fn duration_years(&self) -> Option<u32> {
        self.duration_years
    }

    /// Check if renewable.
    pub fn is_renewable(&self) -> bool {
        self.renewable
    }

    /// Get the region.
    pub fn region(&self) -> Option<&str> {
        self.region.as_deref()
    }

    /// Get conditions.
    pub fn conditions(&self) -> &[String] {
        &self.conditions
    }
}

impl From<GrantEffect> for TypedEffect {
    fn from(grant: GrantEffect) -> Self {
        let mut effect = Effect::grant(format!("Grant: {}", grant.resource));

        if let Some(years) = grant.duration_years {
            effect = effect.with_parameter("duration_years", years.to_string());
        }

        effect = effect.with_parameter("renewable", grant.renewable.to_string());

        if let Some(ref region) = grant.region {
            effect = effect.with_parameter("region", region);
        }

        for (i, condition) in grant.conditions.iter().enumerate() {
            effect = effect.with_parameter(format!("condition_{}", i), condition);
        }

        TypedEffect::from_effect(effect)
    }
}

/// Monetary effect with typed parameters (taxes, fines, subsidies).
///
/// # Examples
///
/// ```
/// use legalis_core::typed_effects::{MonetaryEffect, MonetaryType};
///
/// let tax = MonetaryEffect::new(MonetaryType::Tax, 5000)
///     .with_currency("USD")
///     .with_frequency("annual")
///     .with_recipient("IRS");
///
/// assert_eq!(tax.amount(), 5000);
/// assert_eq!(tax.monetary_type(), &MonetaryType::Tax);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MonetaryEffect {
    monetary_type: MonetaryType,
    amount: i64,
    currency: String,
    frequency: Option<String>,
    recipient: Option<String>,
    payment_method: Option<String>,
}

/// Type of monetary transfer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonetaryType {
    /// Tax payment
    Tax,
    /// Fine or penalty
    Fine,
    /// Subsidy or grant
    Subsidy,
    /// Fee for service
    Fee,
    /// Compensation or damages
    Compensation,
}

impl MonetaryEffect {
    /// Create a new monetary effect.
    pub fn new(monetary_type: MonetaryType, amount: i64) -> Self {
        Self {
            monetary_type,
            amount,
            currency: "USD".to_string(),
            frequency: None,
            recipient: None,
            payment_method: None,
        }
    }

    /// Set the currency.
    pub fn with_currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = currency.into();
        self
    }

    /// Set the frequency (e.g., "annual", "monthly", "one-time").
    pub fn with_frequency(mut self, frequency: impl Into<String>) -> Self {
        self.frequency = Some(frequency.into());
        self
    }

    /// Set the recipient.
    pub fn with_recipient(mut self, recipient: impl Into<String>) -> Self {
        self.recipient = Some(recipient.into());
        self
    }

    /// Set the payment method.
    pub fn with_payment_method(mut self, method: impl Into<String>) -> Self {
        self.payment_method = Some(method.into());
        self
    }

    /// Get the monetary type.
    pub fn monetary_type(&self) -> &MonetaryType {
        &self.monetary_type
    }

    /// Get the amount.
    pub fn amount(&self) -> i64 {
        self.amount
    }

    /// Get the currency.
    pub fn currency(&self) -> &str {
        &self.currency
    }

    /// Get the frequency.
    pub fn frequency(&self) -> Option<&str> {
        self.frequency.as_deref()
    }

    /// Get the recipient.
    pub fn recipient(&self) -> Option<&str> {
        self.recipient.as_deref()
    }

    /// Get the payment method.
    pub fn payment_method(&self) -> Option<&str> {
        self.payment_method.as_deref()
    }
}

impl From<MonetaryEffect> for TypedEffect {
    fn from(monetary: MonetaryEffect) -> Self {
        let type_str = match monetary.monetary_type {
            MonetaryType::Tax => "Tax",
            MonetaryType::Fine => "Fine",
            MonetaryType::Subsidy => "Subsidy",
            MonetaryType::Fee => "Fee",
            MonetaryType::Compensation => "Compensation",
        };

        let mut effect = Effect::new(
            EffectType::MonetaryTransfer,
            format!("{}: {} {}", type_str, monetary.amount, monetary.currency),
        );

        effect = effect.with_parameter("type", type_str);
        effect = effect.with_parameter("amount", monetary.amount.to_string());
        effect = effect.with_parameter("currency", monetary.currency.clone());

        if let Some(ref frequency) = monetary.frequency {
            effect = effect.with_parameter("frequency", frequency);
        }

        if let Some(ref recipient) = monetary.recipient {
            effect = effect.with_parameter("recipient", recipient);
        }

        if let Some(ref method) = monetary.payment_method {
            effect = effect.with_parameter("payment_method", method);
        }

        TypedEffect::from_effect(effect)
    }
}

/// Status change effect with typed parameters.
///
/// # Examples
///
/// ```
/// use legalis_core::typed_effects::StatusChangeEffect;
///
/// let status = StatusChangeEffect::new("citizenship", "resident", "citizen")
///     .with_effective_after_days(365)
///     .with_reversible(false);
///
/// assert_eq!(status.category(), "citizenship");
/// assert_eq!(status.from_status(), "resident");
/// assert_eq!(status.to_status(), "citizen");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct StatusChangeEffect {
    category: String,
    from_status: String,
    to_status: String,
    effective_after_days: Option<u32>,
    reversible: bool,
    requires_approval: bool,
}

impl StatusChangeEffect {
    /// Create a new status change effect.
    pub fn new(
        category: impl Into<String>,
        from_status: impl Into<String>,
        to_status: impl Into<String>,
    ) -> Self {
        Self {
            category: category.into(),
            from_status: from_status.into(),
            to_status: to_status.into(),
            effective_after_days: None,
            reversible: true,
            requires_approval: false,
        }
    }

    /// Set effective delay in days.
    pub fn with_effective_after_days(mut self, days: u32) -> Self {
        self.effective_after_days = Some(days);
        self
    }

    /// Set whether the change is reversible.
    pub fn with_reversible(mut self, reversible: bool) -> Self {
        self.reversible = reversible;
        self
    }

    /// Set whether approval is required.
    pub fn with_requires_approval(mut self, requires: bool) -> Self {
        self.requires_approval = requires;
        self
    }

    /// Get the status category.
    pub fn category(&self) -> &str {
        &self.category
    }

    /// Get the source status.
    pub fn from_status(&self) -> &str {
        &self.from_status
    }

    /// Get the target status.
    pub fn to_status(&self) -> &str {
        &self.to_status
    }

    /// Get the effective delay.
    pub fn effective_after_days(&self) -> Option<u32> {
        self.effective_after_days
    }

    /// Check if reversible.
    pub fn is_reversible(&self) -> bool {
        self.reversible
    }

    /// Check if approval is required.
    pub fn requires_approval(&self) -> bool {
        self.requires_approval
    }
}

impl From<StatusChangeEffect> for TypedEffect {
    fn from(status: StatusChangeEffect) -> Self {
        let mut effect = Effect::new(
            EffectType::StatusChange,
            format!(
                "Status change: {} from '{}' to '{}'",
                status.category, status.from_status, status.to_status
            ),
        );

        effect = effect.with_parameter("category", status.category.clone());
        effect = effect.with_parameter("from_status", status.from_status.clone());
        effect = effect.with_parameter("to_status", status.to_status.clone());
        effect = effect.with_parameter("reversible", status.reversible.to_string());
        effect = effect.with_parameter("requires_approval", status.requires_approval.to_string());

        if let Some(days) = status.effective_after_days {
            effect = effect.with_parameter("effective_after_days", days.to_string());
        }

        TypedEffect::from_effect(effect)
    }
}

/// Generic typed effect with parameter constraints.
///
/// Uses phantom types to ensure type safety at compile time.
///
/// # Examples
///
/// ```
/// use legalis_core::typed_effects::{GenericTypedEffect, EffectParameter};
///
/// #[derive(Clone, Debug)]
/// struct MyParam(String);
///
/// impl EffectParameter for MyParam {
///     fn as_string(&self) -> String {
///         self.0.clone()
///     }
///
///     fn from_string(s: &str) -> Result<Self, String> {
///         Ok(MyParam(s.to_string()))
///     }
/// }
///
/// let effect: GenericTypedEffect<MyParam> = GenericTypedEffect::new("test");
/// ```
#[derive(Debug, Clone)]
pub struct GenericTypedEffect<P: EffectParameter> {
    description: String,
    parameters: HashMap<String, P>,
    _phantom: PhantomData<P>,
}

impl<P: EffectParameter> GenericTypedEffect<P> {
    /// Create a new generic typed effect.
    pub fn new(description: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            parameters: HashMap::new(),
            _phantom: PhantomData,
        }
    }

    /// Add a parameter.
    pub fn with_parameter(mut self, key: impl Into<String>, value: P) -> Self {
        self.parameters.insert(key.into(), value);
        self
    }

    /// Get a parameter.
    pub fn get_parameter(&self, key: &str) -> Option<&P> {
        self.parameters.get(key)
    }

    /// Get the description.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Get all parameters.
    pub fn parameters(&self) -> &HashMap<String, P> {
        &self.parameters
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grant_effect() {
        let grant = GrantEffect::new("license")
            .with_duration_years(5)
            .with_renewable(true)
            .with_region("US-CA");

        assert_eq!(grant.resource(), "license");
        assert_eq!(grant.duration_years(), Some(5));
        assert!(grant.is_renewable());
        assert_eq!(grant.region(), Some("US-CA"));
    }

    #[test]
    fn test_grant_effect_to_typed() {
        let grant = GrantEffect::new("permit").with_duration_years(3);

        let typed: TypedEffect = grant.into();
        assert_eq!(typed.description(), "Grant: permit");
    }

    #[test]
    fn test_monetary_effect() {
        let tax = MonetaryEffect::new(MonetaryType::Tax, 10000)
            .with_currency("EUR")
            .with_frequency("annual")
            .with_recipient("Tax Authority");

        assert_eq!(tax.amount(), 10000);
        assert_eq!(tax.currency(), "EUR");
        assert_eq!(tax.frequency(), Some("annual"));
        assert_eq!(tax.recipient(), Some("Tax Authority"));
    }

    #[test]
    fn test_status_change_effect() {
        let status = StatusChangeEffect::new("membership", "guest", "member")
            .with_effective_after_days(30)
            .with_reversible(false);

        assert_eq!(status.category(), "membership");
        assert_eq!(status.from_status(), "guest");
        assert_eq!(status.to_status(), "member");
        assert_eq!(status.effective_after_days(), Some(30));
        assert!(!status.is_reversible());
    }

    #[test]
    fn test_generic_typed_effect() {
        #[derive(Clone, Debug, PartialEq)]
        struct TestParam(u32);

        impl EffectParameter for TestParam {
            fn as_string(&self) -> String {
                self.0.to_string()
            }

            fn from_string(s: &str) -> Result<Self, String> {
                s.parse().map(TestParam).map_err(|e| format!("{}", e))
            }
        }

        let effect = GenericTypedEffect::new("test").with_parameter("value", TestParam(42));

        assert_eq!(effect.get_parameter("value"), Some(&TestParam(42)));
    }
}
