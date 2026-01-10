//! Contract Template Types (契約テンプレート型定義)
//!
//! This module defines types for contract template generation and management.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contract template type (契約テンプレート種別)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TemplateType {
    /// Employment contract (雇用契約書)
    Employment,
    /// Sales contract (売買契約書)
    Sales,
    /// Service agreement (業務委託契約書)
    ServiceAgreement,
    /// Non-disclosure agreement (秘密保持契約書)
    NDA,
    /// Lease agreement (賃貸借契約書)
    Lease,
    /// Partnership agreement (パートナーシップ契約書)
    Partnership,
    /// License agreement (ライセンス契約書)
    License,
    /// Custom template (カスタムテンプレート)
    Custom,
}

impl TemplateType {
    /// Returns the Japanese name
    pub fn japanese_name(&self) -> &'static str {
        match self {
            TemplateType::Employment => "雇用契約書",
            TemplateType::Sales => "売買契約書",
            TemplateType::ServiceAgreement => "業務委託契約書",
            TemplateType::NDA => "秘密保持契約書",
            TemplateType::Lease => "賃貸借契約書",
            TemplateType::Partnership => "パートナーシップ契約書",
            TemplateType::License => "ライセンス契約書",
            TemplateType::Custom => "カスタムテンプレート",
        }
    }

    /// Returns the English name
    pub fn english_name(&self) -> &'static str {
        match self {
            TemplateType::Employment => "Employment Contract",
            TemplateType::Sales => "Sales Contract",
            TemplateType::ServiceAgreement => "Service Agreement",
            TemplateType::NDA => "Non-Disclosure Agreement",
            TemplateType::Lease => "Lease Agreement",
            TemplateType::Partnership => "Partnership Agreement",
            TemplateType::License => "License Agreement",
            TemplateType::Custom => "Custom Template",
        }
    }
}

/// Employment contract subtype (雇用契約サブタイプ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentSubtype {
    /// Full-time employee (正社員)
    FullTime,
    /// Part-time employee (パートタイム)
    PartTime,
    /// Fixed-term contract (有期雇用)
    FixedTerm,
    /// Contract employee (契約社員)
    Contract,
}

/// NDA subtype (NDAサブタイプ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NDASubtype {
    /// Mutual NDA (相互NDA)
    Mutual,
    /// One-way NDA (片務NDA)
    OneWay,
}

/// Lease subtype (賃貸借サブタイプ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaseSubtype {
    /// Residential lease (居住用)
    Residential,
    /// Commercial lease (事業用)
    Commercial,
}

/// Template variable value (テンプレート変数値)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum VariableValue {
    /// String value
    String(String),
    /// Number value
    Number(f64),
    /// Integer value
    Integer(i64),
    /// Boolean value
    Boolean(bool),
    /// Date value (YYYY-MM-DD)
    Date(String),
    /// List of strings
    List(Vec<String>),
}

impl VariableValue {
    /// Convert to string representation
    pub fn as_string(&self) -> String {
        match self {
            VariableValue::String(s) => s.clone(),
            VariableValue::Number(n) => format!("{}", n),
            VariableValue::Integer(i) => format!("{}", i),
            VariableValue::Boolean(b) => format!("{}", b),
            VariableValue::Date(d) => d.clone(),
            VariableValue::List(items) => items.join(", "),
        }
    }

    /// Check if value is empty/false/zero
    pub fn is_truthy(&self) -> bool {
        match self {
            VariableValue::String(s) => !s.is_empty(),
            VariableValue::Number(n) => *n != 0.0,
            VariableValue::Integer(i) => *i != 0,
            VariableValue::Boolean(b) => *b,
            VariableValue::Date(d) => !d.is_empty(),
            VariableValue::List(items) => !items.is_empty(),
        }
    }
}

/// Template context for variable substitution (テンプレートコンテキスト)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TemplateContext {
    /// Variable map
    pub variables: HashMap<String, VariableValue>,
}

impl TemplateContext {
    /// Create a new empty context
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
        }
    }

    /// Set a string variable
    pub fn set_string(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.variables
            .insert(key.into(), VariableValue::String(value.into()));
        self
    }

    /// Set a number variable
    pub fn set_number(&mut self, key: impl Into<String>, value: f64) -> &mut Self {
        self.variables
            .insert(key.into(), VariableValue::Number(value));
        self
    }

    /// Set an integer variable
    pub fn set_integer(&mut self, key: impl Into<String>, value: i64) -> &mut Self {
        self.variables
            .insert(key.into(), VariableValue::Integer(value));
        self
    }

    /// Set a boolean variable
    pub fn set_boolean(&mut self, key: impl Into<String>, value: bool) -> &mut Self {
        self.variables
            .insert(key.into(), VariableValue::Boolean(value));
        self
    }

    /// Set a date variable
    pub fn set_date(&mut self, key: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.variables
            .insert(key.into(), VariableValue::Date(value.into()));
        self
    }

    /// Set a list variable
    pub fn set_list(&mut self, key: impl Into<String>, value: Vec<String>) -> &mut Self {
        self.variables
            .insert(key.into(), VariableValue::List(value));
        self
    }

    /// Get a variable value
    pub fn get(&self, key: &str) -> Option<&VariableValue> {
        self.variables.get(key)
    }

    /// Check if a variable exists and is truthy
    pub fn is_truthy(&self, key: &str) -> bool {
        self.get(key).map(|v| v.is_truthy()).unwrap_or(false)
    }
}

/// Contract clause (契約条項)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clause {
    /// Clause identifier
    pub id: String,
    /// Clause title (Japanese)
    pub title_ja: String,
    /// Clause title (English)
    pub title_en: Option<String>,
    /// Clause content (Japanese)
    pub content_ja: String,
    /// Clause content (English)
    pub content_en: Option<String>,
    /// Risk level (low, medium, high)
    pub risk_level: RiskLevel,
    /// Category
    pub category: ClauseCategory,
    /// Whether clause is optional
    pub is_optional: bool,
}

impl Clause {
    /// Create a new clause
    pub fn new(
        id: impl Into<String>,
        title_ja: impl Into<String>,
        content_ja: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            title_ja: title_ja.into(),
            title_en: None,
            content_ja: content_ja.into(),
            content_en: None,
            risk_level: RiskLevel::Low,
            category: ClauseCategory::General,
            is_optional: false,
        }
    }

    /// Set English title
    pub fn with_english_title(mut self, title: impl Into<String>) -> Self {
        self.title_en = Some(title.into());
        self
    }

    /// Set English content
    pub fn with_english_content(mut self, content: impl Into<String>) -> Self {
        self.content_en = Some(content.into());
        self
    }

    /// Set risk level
    pub fn with_risk_level(mut self, level: RiskLevel) -> Self {
        self.risk_level = level;
        self
    }

    /// Set category
    pub fn with_category(mut self, category: ClauseCategory) -> Self {
        self.category = category;
        self
    }

    /// Mark as optional
    pub fn optional(mut self) -> Self {
        self.is_optional = true;
        self
    }
}

/// Clause risk level (条項リスクレベル)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskLevel {
    /// Low risk (低リスク)
    Low,
    /// Medium risk (中リスク)
    Medium,
    /// High risk (高リスク)
    High,
}

/// Clause category (条項カテゴリ)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClauseCategory {
    /// General provisions (一般条項)
    General,
    /// Payment terms (支払条件)
    Payment,
    /// Termination (解除・解約)
    Termination,
    /// Liability & indemnification (責任・補償)
    Liability,
    /// Confidentiality (機密保持)
    Confidentiality,
    /// Intellectual property (知的財産権)
    IntellectualProperty,
    /// Dispute resolution (紛争解決)
    DisputeResolution,
    /// Scope of work (業務範囲)
    ScopeOfWork,
    /// Working conditions (労働条件)
    WorkingConditions,
}

/// Contract template (契約テンプレート)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractTemplate {
    /// Template identifier
    pub id: String,
    /// Template name (Japanese)
    pub name_ja: String,
    /// Template name (English)
    pub name_en: Option<String>,
    /// Template type
    pub template_type: TemplateType,
    /// Template content with placeholders
    pub content: String,
    /// Required variables
    pub required_variables: Vec<String>,
    /// Optional variables
    pub optional_variables: Vec<String>,
    /// Clauses
    pub clauses: Vec<Clause>,
    /// Description (Japanese)
    pub description_ja: Option<String>,
    /// Description (English)
    pub description_en: Option<String>,
}

impl ContractTemplate {
    /// Create a new template
    pub fn new(
        id: impl Into<String>,
        name_ja: impl Into<String>,
        template_type: TemplateType,
        content: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name_ja: name_ja.into(),
            name_en: None,
            template_type,
            content: content.into(),
            required_variables: Vec::new(),
            optional_variables: Vec::new(),
            clauses: Vec::new(),
            description_ja: None,
            description_en: None,
        }
    }

    /// Add a required variable
    pub fn require_variable(&mut self, var: impl Into<String>) -> &mut Self {
        self.required_variables.push(var.into());
        self
    }

    /// Add an optional variable
    pub fn optional_variable(&mut self, var: impl Into<String>) -> &mut Self {
        self.optional_variables.push(var.into());
        self
    }

    /// Add a clause
    pub fn add_clause(&mut self, clause: Clause) -> &mut Self {
        self.clauses.push(clause);
        self
    }

    /// Validate that all required variables are present in context
    pub fn validate_context(&self, context: &TemplateContext) -> Result<(), Vec<String>> {
        let missing: Vec<String> = self
            .required_variables
            .iter()
            .filter(|var| !context.variables.contains_key(*var))
            .cloned()
            .collect();

        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}

/// Generated contract document (生成された契約書)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedContract {
    /// Template used
    pub template_id: String,
    /// Template type
    pub template_type: TemplateType,
    /// Generated content (Japanese)
    pub content_ja: String,
    /// Generated content (English, if available)
    pub content_en: Option<String>,
    /// Generation timestamp
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Variables used
    pub variables: HashMap<String, VariableValue>,
    /// Included clauses
    pub included_clauses: Vec<String>,
}

impl GeneratedContract {
    /// Create a new generated contract
    pub fn new(
        template_id: impl Into<String>,
        template_type: TemplateType,
        content_ja: impl Into<String>,
        variables: HashMap<String, VariableValue>,
    ) -> Self {
        Self {
            template_id: template_id.into(),
            template_type,
            content_ja: content_ja.into(),
            content_en: None,
            generated_at: chrono::Utc::now(),
            variables,
            included_clauses: Vec::new(),
        }
    }

    /// Set English content
    pub fn with_english_content(mut self, content: impl Into<String>) -> Self {
        self.content_en = Some(content.into());
        self
    }

    /// Add included clause
    pub fn with_clause(mut self, clause_id: impl Into<String>) -> Self {
        self.included_clauses.push(clause_id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_type_names() {
        assert_eq!(TemplateType::Employment.japanese_name(), "雇用契約書");
        assert_eq!(TemplateType::NDA.english_name(), "Non-Disclosure Agreement");
    }

    #[test]
    fn test_variable_value_as_string() {
        assert_eq!(
            VariableValue::String("test".to_string()).as_string(),
            "test"
        );
        assert_eq!(VariableValue::Integer(42).as_string(), "42");
        assert_eq!(VariableValue::Boolean(true).as_string(), "true");
    }

    #[test]
    fn test_variable_value_is_truthy() {
        assert!(VariableValue::String("test".to_string()).is_truthy());
        assert!(!VariableValue::String("".to_string()).is_truthy());
        assert!(VariableValue::Integer(1).is_truthy());
        assert!(!VariableValue::Integer(0).is_truthy());
    }

    #[test]
    fn test_template_context() {
        let mut context = TemplateContext::new();
        context.set_string("name", "山田太郎");
        context.set_integer("age", 30);
        context.set_boolean("is_active", true);

        assert_eq!(context.get("name").unwrap().as_string(), "山田太郎");
        assert!(context.is_truthy("is_active"));
        assert!(!context.is_truthy("nonexistent"));
    }

    #[test]
    fn test_clause_creation() {
        let clause = Clause::new("article1", "第1条", "契約の目的")
            .with_risk_level(RiskLevel::Low)
            .with_category(ClauseCategory::General);

        assert_eq!(clause.id, "article1");
        assert_eq!(clause.risk_level, RiskLevel::Low);
        assert!(!clause.is_optional);
    }

    #[test]
    fn test_template_validation() {
        let mut template = ContractTemplate::new(
            "employment_basic",
            "雇用契約書",
            TemplateType::Employment,
            "{{employee_name}}を雇用します",
        );
        template.require_variable("employee_name");
        template.require_variable("employer_name");

        let mut context = TemplateContext::new();
        context.set_string("employee_name", "山田太郎");

        // Should fail - missing employer_name
        let result = template.validate_context(&context);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), vec!["employer_name"]);

        // Should succeed
        context.set_string("employer_name", "株式会社ABC");
        assert!(template.validate_context(&context).is_ok());
    }
}
