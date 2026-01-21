//! Securities Law Errors (Securities Act 1933, Securities Exchange Act 1934)

#![allow(missing_docs)]

use thiserror::Error;

/// Errors related to US securities regulation
#[derive(Debug, Clone, Error, PartialEq)]
pub enum SecuritiesError {
    // ============================================================================
    // Registration Errors (Securities Act 1933)
    // ============================================================================
    /// Security not registered with SEC (Section 5 violation)
    #[error(
        "Security '{security_name}' is not registered with the SEC. Sale of unregistered securities violates Section 5 of the Securities Act of 1933. Registration or valid exemption required."
    )]
    NotRegistered { security_name: String },

    /// Registration statement not yet effective
    #[error(
        "Registration statement filed on {filing_date} but not yet effective. Cannot offer or sell securities before effectiveness. See Securities Act Section 5(a)."
    )]
    RegistrationNotEffective { filing_date: String },

    /// Gun-jumping violation (offers before registration filing)
    #[error(
        "Gun-jumping violation: {violation_type}. Pre-filing period restrictions violated. See Securities Act Section 5. No offers permitted before registration filing except as allowed under Securities Act Rules 163/163A."
    )]
    GunJumping { violation_type: String },

    /// Prospectus delivery requirement not met
    #[error(
        "Failed to deliver prospectus to purchaser. Securities Act Section 5(b)(2) requires delivery of statutory prospectus. Violation may result in rescission rights."
    )]
    ProspectusNotDelivered,

    // ============================================================================
    // Exemption Errors
    // ============================================================================
    /// No valid exemption from registration
    #[error(
        "No valid exemption from registration. Security offering of ${offering_amount} does not qualify for any exemption under Regulation D, Regulation A, Section 4(a)(2), or other exemptions."
    )]
    NoValidExemption { offering_amount: f64 },

    /// Regulation D violation
    #[error(
        "Regulation D violation: {details}. Rule {rule} requirements not satisfied. {consequence}"
    )]
    RegulationDViolation {
        rule: String,
        details: String,
        consequence: String,
    },

    /// General solicitation in Rule 506(b) offering
    #[error(
        "General solicitation prohibited in Rule 506(b) offering. General solicitation detected: {solicitation_type}. Use Rule 506(c) if general solicitation needed."
    )]
    GeneralSolicitationProhibited { solicitation_type: String },

    /// Non-accredited investor in 506(c) offering
    #[error(
        "Rule 506(c) requires all purchasers to be accredited investors. Investor '{investor_name}' is not accredited. Verification method: {verification_method}."
    )]
    NonAccreditedIn506C {
        investor_name: String,
        verification_method: String,
    },

    /// Regulation A offering limit exceeded
    #[error(
        "Regulation A {tier} offering limit exceeded. Maximum: ${max_amount}, Offered: ${offered_amount}. Tier 1 max: $20M, Tier 2 max: $75M."
    )]
    RegulationALimitExceeded {
        tier: String,
        max_amount: f64,
        offered_amount: f64,
    },

    /// Regulation Crowdfunding limit exceeded
    #[error(
        "Regulation Crowdfunding offering limit exceeded. Maximum $5 million in 12-month period. Current offering: ${current_amount}, Prior offerings: ${prior_amount}."
    )]
    CrowdfundingLimitExceeded {
        current_amount: f64,
        prior_amount: f64,
    },

    /// Investment limit exceeded for crowdfunding investor
    #[error(
        "Crowdfunding investment limit exceeded for investor '{investor_name}'. Annual income/net worth: ${financial_threshold}, Investment limit: ${limit}, Attempted investment: ${attempted}."
    )]
    CrowdfundingInvestmentLimitExceeded {
        investor_name: String,
        financial_threshold: f64,
        limit: f64,
        attempted: f64,
    },

    /// Form D not filed
    #[error(
        "Form D notice filing not submitted within 15 days of first sale. Regulation D requires Form D filing. File immediately to maintain exemption."
    )]
    FormDNotFiled,

    /// Integration violation (multiple offerings treated as one)
    #[error(
        "Integration of offerings: Current offering integrated with prior offering from {prior_date}. Combined offerings exceed exemption limits or violate exemption requirements."
    )]
    IntegrationViolation { prior_date: String },

    // ============================================================================
    // Accredited Investor Errors
    // ============================================================================
    /// Accredited investor verification insufficient
    #[error(
        "Accredited investor verification insufficient for '{investor_name}'. Method used: {method}. Rule 506(c) requires reasonable steps to verify. Acceptable methods: income documentation, net worth documentation, third-party verification, professional certifications."
    )]
    InsufficientAccreditedVerification {
        investor_name: String,
        method: String,
    },

    /// Investor does not meet accredited investor criteria
    #[error(
        "Investor '{investor_name}' does not meet accredited investor criteria. Income: {income:?}, Net worth: {net_worth:?}. Required: $200k income (individual) or $300k (joint) for 2 years, OR $1M net worth (excluding primary residence)."
    )]
    NotAccreditedInvestor {
        investor_name: String,
        income: Option<f64>,
        net_worth: Option<f64>,
    },

    // ============================================================================
    // Disclosure Errors
    // ============================================================================
    /// Materially misleading disclosure
    #[error(
        "Material misstatement or omission in {document_type}: {details}. See Securities Act Section 12(a)(2). Material misstatements may result in liability for rescission or damages."
    )]
    MaterialMisstatement {
        document_type: String,
        details: String,
    },

    /// Omission of material fact
    #[error(
        "Omission of material fact: {fact_description}. Rule 10b-5 and Section 11 require disclosure of all material facts. Reasonable investor would consider this important."
    )]
    MaterialOmission { fact_description: String },

    /// Missing risk factors
    #[error(
        "Risk factors section incomplete or missing. Item 503(c) of Regulation S-K requires disclosure of material risks. Missing: {missing_risks}"
    )]
    IncompleteRiskFactors { missing_risks: String },

    // ============================================================================
    // Exchange Act Errors (Periodic Reporting)
    // ============================================================================
    /// Late filing of periodic report
    #[error(
        "Periodic report {form_type} for period ending {period_end} not filed by deadline {deadline}. Exchange Act Section 13 requires timely filing. File immediately and consider NT filing."
    )]
    LatePeriodicReportFiling {
        form_type: String,
        period_end: String,
        deadline: String,
    },

    /// Missing Exchange Act registration
    #[error(
        "Company must register under Exchange Act Section 12. Thresholds exceeded: {threshold_violated}. Assets > $10M and either (1) 2,000+ shareholders or (2) 500+ non-accredited shareholders."
    )]
    MissingExchangeActRegistration { threshold_violated: String },

    /// Failure to file Form 8-K for material event
    #[error(
        "Form 8-K not filed for material event: {event_description}. Exchange Act requires 8-K filing within 4 business days of material events. Item {item_number} triggered."
    )]
    MissingForm8K {
        event_description: String,
        item_number: String,
    },

    // ============================================================================
    // Insider Trading Errors
    // ============================================================================
    /// Insider trading violation (Rule 10b-5)
    #[error(
        "Suspected insider trading by {person} on {date}. Traded {security} while in possession of material non-public information: {information}. Violates Exchange Act Section 10(b) and Rule 10b-5. Criminal and civil penalties apply."
    )]
    InsiderTrading {
        person: String,
        date: String,
        security: String,
        information: String,
    },

    /// Tipping violation
    #[error(
        "Tipping of material non-public information by {tipper} to {tippee}. Information: {information}. Dirks v. SEC establishes tipper/tippee liability. Both may face SEC enforcement."
    )]
    TippingViolation {
        tipper: String,
        tippee: String,
        information: String,
    },

    /// Failure to file Form 4 (insider transaction)
    #[error(
        "Form 4 not filed within 2 business days of transaction by {insider_name} on {transaction_date}. Section 16(a) requires timely reporting of insider transactions."
    )]
    LateForm4Filing {
        insider_name: String,
        transaction_date: String,
    },

    /// Section 16(b) short-swing profit violation
    #[error(
        "Section 16(b) short-swing profit violation by {insider_name}. Purchase on {purchase_date}, sale on {sale_date} (within 6 months). Profit: ${profit}. Corporation entitled to disgorgement."
    )]
    ShortSwingProfit {
        insider_name: String,
        purchase_date: String,
        sale_date: String,
        profit: f64,
    },

    // ============================================================================
    // Blue Sky Law Errors
    // ============================================================================
    /// State registration required
    #[error(
        "Security offering must be registered in {state}. Not a covered security under NSMIA. State blue sky registration or exemption required before offering to {state} residents."
    )]
    StateRegistrationRequired { state: String },

    /// Notice filing not submitted
    #[error(
        "Notice filing required in {state} for covered security. File Form U-1 or state equivalent with required fees and consent to service of process."
    )]
    NoticeFilingRequired { state: String },

    /// State investment limit exceeded
    #[error(
        "State {state} investment limit exceeded. {state} law limits investments to ${limit} per investor for {exemption_type} offerings."
    )]
    StateInvestmentLimitExceeded {
        state: String,
        limit: f64,
        exemption_type: String,
    },

    // ============================================================================
    // Rule 144 Errors (Resale Restrictions)
    // ============================================================================
    /// Rule 144 holding period not satisfied
    #[error(
        "Rule 144 holding period not satisfied. Acquired on {acquisition_date}, {days_held} days held. Required: {required_days} days for {issuer_type} issuer."
    )]
    HoldingPeriodNotSatisfied {
        acquisition_date: String,
        days_held: u32,
        required_days: u32,
        issuer_type: String,
    },

    /// Rule 144 volume limitation exceeded
    #[error(
        "Rule 144 volume limitation exceeded. Shares to be sold: {shares_to_sell}, Greater of 1% of outstanding or 4-week average trading volume: {volume_limit}."
    )]
    VolumeLimitationExceeded {
        shares_to_sell: u64,
        volume_limit: u64,
    },

    /// Form 144 not filed
    #[error(
        "Form 144 notice required for sale of {shares} shares. Rule 144 requires Form 144 filing concurrent with sell order if selling more than 5,000 shares or value exceeds $50,000 in 3 months."
    )]
    Form144Required { shares: u64 },

    /// Public information requirement not met
    #[error(
        "Public information requirement not satisfied. Reporting company must have filed all required Exchange Act reports for past 12 months. Non-reporting company must make specified information publicly available."
    )]
    PublicInformationNotAvailable,

    // ============================================================================
    // Rule 144A Errors (QIB Resales)
    // ============================================================================
    /// Purchaser not a QIB
    #[error(
        "Purchaser '{purchaser_name}' not a qualified institutional buyer (QIB). Rule 144A requires QIB status. QIBs must own/invest at least $100 million in securities ($10M for broker-dealers)."
    )]
    NotQualifiedInstitutionalBuyer { purchaser_name: String },

    /// Rule 144A information requirement not met
    #[error(
        "Rule 144A information requirement not satisfied. For non-reporting foreign issuers, must provide: (1) brief description of business, (2) most recent balance sheet and P&L, (3) similar information for prior 2 years if available."
    )]
    Rule144AInformationNotProvided,

    // ============================================================================
    // Proxy and Tender Offer Errors
    // ============================================================================
    /// Proxy statement violation
    #[error(
        "Proxy statement violation: {violation_type}. Schedule 14A requirements not met. {details}"
    )]
    ProxyStatementViolation {
        violation_type: String,
        details: String,
    },

    /// Tender offer violation
    #[error(
        "Tender offer violation: {violation_type}. Williams Act (Sections 13(d), 14(d)) requirements not satisfied. {details}"
    )]
    TenderOfferViolation {
        violation_type: String,
        details: String,
    },

    /// 13D/13G filing required
    #[error(
        "Schedule 13D or 13G filing required. Beneficial ownership of {percentage}% exceeds 5% threshold. Must file within 10 days (13D) or 45 days after calendar year-end (13G for passive investors)."
    )]
    BeneficialOwnershipReportingRequired { percentage: f64 },

    // ============================================================================
    // Investment Company Act Errors
    // ============================================================================
    /// Inadvertent investment company
    #[error(
        "Entity may be an inadvertent investment company under Investment Company Act Section 3(a)(1). {reason}. Must either register as investment company or restructure to avoid 40 Act status."
    )]
    InadvertentInvestmentCompany { reason: String },

    /// 3(c)(1) exemption exceeded
    #[error(
        "Investment Company Act Section 3(c)(1) exemption threshold exceeded. Number of beneficial owners: {owners}, Maximum: 100. Cannot rely on 3(c)(1) exemption."
    )]
    Section3C1ExceededOwners { owners: usize },

    /// 3(c)(7) exemption not qualified
    #[error(
        "Section 3(c)(7) exemption requires all owners to be qualified purchasers. Investor '{investor_name}' does not meet qualified purchaser standard ($5M investments for individuals, $25M for entities)."
    )]
    NotQualifiedPurchaser { investor_name: String },

    // ============================================================================
    // JOBS Act Errors (Emerging Growth Companies)
    // ============================================================================
    /// EGC status improperly claimed
    #[error(
        "Emerging growth company (EGC) status improperly claimed. {disqualification_reason}. EGC status lost when: annual revenues ≥ $1.235B, non-convertible debt > $1B, deemed large accelerated filer, or 5 years since IPO."
    )]
    EgcStatusImproperlyClaimed { disqualification_reason: String },

    /// Test-the-waters communication violation
    #[error(
        "Test-the-waters communication violates JOBS Act Section 5(d). {details}. Only EGCs can engage in test-the-waters communications with QIBs and institutional accredited investors before/after filing."
    )]
    TestTheWatersViolation { details: String },

    // ============================================================================
    // Sarbanes-Oxley Act Errors
    // ============================================================================
    /// SOX 404 internal controls deficiency
    #[error(
        "Sarbanes-Oxley Section 404 internal control deficiency: {deficiency_description}. Material weakness in internal control over financial reporting. Requires disclosure in Form 10-K."
    )]
    InternalControlDeficiency { deficiency_description: String },

    /// CEO/CFO certification missing
    #[error(
        "CEO/CFO certifications required under SOX Sections 302 and 906 not provided. Certifications must accompany Form 10-K and 10-Q. Criminal penalties for false certification."
    )]
    MissingCertifications,

    /// Audit committee requirement not met
    #[error(
        "Sarbanes-Oxley Section 301 audit committee requirement not satisfied: {violation}. Listed companies must have independent audit committee with financial expert."
    )]
    AuditCommitteeViolation { violation: String },

    // ============================================================================
    // Dodd-Frank Act Errors
    // ============================================================================
    /// Say-on-pay vote not held
    #[error(
        "Dodd-Frank Section 951 say-on-pay vote not held. Public companies must hold advisory vote on executive compensation at least every 3 years."
    )]
    SayOnPayVoteNotHeld,

    /// Conflict minerals disclosure missing
    #[error(
        "Dodd-Frank Section 1502 conflict minerals disclosure required. Must file Form SD if manufacturing products containing tin, tantalum, tungsten, or gold from DRC region."
    )]
    ConflictMineralsDisclosureMissing,

    /// Whistleblower retaliation
    #[error(
        "Dodd-Frank Section 922 whistleblower retaliation prohibited. Action taken against employee '{employee}' for reporting securities law violation. SEC whistleblower protections apply."
    )]
    WhistleblowerRetaliation { employee: String },

    // ============================================================================
    // General Errors
    // ============================================================================
    /// Validation error
    #[error("Securities law validation error: {message}")]
    ValidationError { message: String },

    /// Multiple errors
    #[error("Multiple securities law errors: {errors:?}")]
    MultipleErrors { errors: Vec<String> },

    /// Statute of limitations expired
    #[error(
        "Statute of limitations expired for {claim_type}. Section 10(b) claims: 2 years from discovery, 5 years from violation. Section 11 claims: 1 year from discovery, 3 years from offering."
    )]
    StatuteOfLimitationsExpired { claim_type: String },
}

/// Result type for securities operations
pub type Result<T> = std::result::Result<T, SecuritiesError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_variants_exist() {
        let _error = SecuritiesError::ValidationError {
            message: "test".to_string(),
        };
    }
}
