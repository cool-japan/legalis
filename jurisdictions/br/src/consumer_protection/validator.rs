//! CDC Validation Functions

use super::error::{CdcError, CdcResult};
use super::types::*;
use chrono::NaiveDate;

/// Validate withdrawal right exercise (Art. 49)
///
/// Consumers have 7 calendar days to withdraw from distance/doorstep purchases.
pub fn validate_withdrawal(
    contract_date: NaiveDate,
    withdrawal_date: NaiveDate,
    contract_type: ContractType,
) -> CdcResult<()> {
    // Check if withdrawal right applies
    if !contract_type.has_withdrawal_right() {
        return Err(CdcError::ValidationError {
            message: "Direito de arrependimento não aplicável a este tipo de contrato".to_string(),
        });
    }

    // Calculate days elapsed
    let days_elapsed = (withdrawal_date - contract_date).num_days();

    if days_elapsed < 0 {
        return Err(CdcError::ValidationError {
            message: "Data de desistência anterior à contratação".to_string(),
        });
    }

    if days_elapsed > 7 {
        return Err(CdcError::WithdrawalDenied {
            reason: "Prazo de 7 dias expirado".to_string(),
            days_since_purchase: days_elapsed as u32,
        });
    }

    Ok(())
}

/// Check if clause is abusive (Art. 51)
///
/// Returns the abusive clause type if detected.
pub fn check_abusive_clause(clause_text: &str) -> Option<AbusiveClauseType> {
    let text_lower = clause_text.to_lowercase();

    // Check for liability exclusion patterns
    if text_lower.contains("não se responsabiliza")
        || text_lower.contains("isenta de responsabilidade")
        || text_lower.contains("exclui responsabilidade")
    {
        return Some(AbusiveClauseType::LiabilityExclusion);
    }

    // Check for mandatory arbitration
    if text_lower.contains("arbitragem obrigatória")
        || text_lower.contains("compulsoriamente submetido à arbitragem")
    {
        return Some(AbusiveClauseType::MandatoryArbitration);
    }

    // Check for unilateral modification
    if text_lower.contains("pode alterar unilateralmente")
        || text_lower.contains("modificar a qualquer momento sem aviso")
    {
        return Some(AbusiveClauseType::UnilateralModification);
    }

    // Check for unilateral termination
    if text_lower.contains("rescindir a qualquer momento sem aviso")
        || text_lower.contains("cancelar sem justificativa")
    {
        return Some(AbusiveClauseType::UnilateralTermination);
    }

    // Check for excessive penalty
    if text_lower.contains("multa de 100%") || text_lower.contains("perda total dos valores") {
        return Some(AbusiveClauseType::PenaltyViolation);
    }

    None
}

/// Validate product warranty claim (Art. 26)
///
/// - Non-durable goods: 30 days
/// - Durable goods: 90 days
pub fn validate_warranty_claim(
    product: &Product,
    purchase_date: NaiveDate,
    claim_date: NaiveDate,
) -> CdcResult<()> {
    let warranty_days = product.warranty_period_days();
    let days_elapsed = (claim_date - purchase_date).num_days();

    if days_elapsed < 0 {
        return Err(CdcError::ValidationError {
            message: "Data da reclamação anterior à compra".to_string(),
        });
    }

    if days_elapsed > warranty_days as i64 {
        return Err(CdcError::WarrantyViolation {
            description: format!(
                "Prazo de garantia de {} dias expirado (decorridos {} dias)",
                warranty_days, days_elapsed
            ),
            days_remaining: None,
        });
    }

    Ok(())
}

/// Determine provider liability for product defect (Art. 12)
pub fn determine_product_liability(
    provider: &Provider,
    defect: &ProductDefect,
    exclusion: Option<&LiabilityExclusion>,
) -> ProviderLiability {
    // Check for exclusion grounds (Art. 12, §3)
    if let Some(exclusion_ground) = exclusion {
        match exclusion_ground {
            LiabilityExclusion::NotPlacedInMarket
            | LiabilityExclusion::NoDefect
            | LiabilityExclusion::ExclusiveFault { .. } => {
                return ProviderLiability {
                    is_liable: false,
                    liability_type: LiabilityType::Strict,
                    exclusion_ground: Some(exclusion_ground.clone()),
                };
            }
        }
    }

    // Determine liability type based on provider type
    let liability_type = match provider.provider_type {
        ProviderType::Manufacturer | ProviderType::Importer => LiabilityType::Solidary,
        ProviderType::Retailer | ProviderType::Distributor => LiabilityType::Subsidiary,
        ProviderType::ServiceProvider => LiabilityType::Strict,
    };

    // All providers face strict liability for defects (responsabilidade objetiva)
    let _ = defect; // Used for completeness, defect type doesn't change liability

    ProviderLiability {
        is_liable: true,
        liability_type,
        exclusion_ground: None,
    }
}

/// Validate recall compliance (Art. 10)
pub fn validate_recall(recall: &Recall) -> CdcResult<()> {
    if !recall.autoridades_notificadas {
        return Err(CdcError::RecallFailure {
            description: "Autoridades competentes não foram notificadas (Art. 10, §1º)".to_string(),
        });
    }

    Ok(())
}

/// Validate information adequacy (Art. 6, III and Art. 31)
pub fn validate_product_information(
    has_portuguese_info: bool,
    has_price_info: bool,
    has_warranty_info: bool,
    has_risk_info: bool,
    product_has_risk: bool,
) -> CdcResult<()> {
    if !has_portuguese_info {
        return Err(CdcError::InformationInadequacy {
            missing_info: "Informações devem estar em português (Art. 31)".to_string(),
        });
    }

    if !has_price_info {
        return Err(CdcError::InformationInadequacy {
            missing_info: "Preço não informado claramente (Art. 31)".to_string(),
        });
    }

    if !has_warranty_info {
        return Err(CdcError::InformationInadequacy {
            missing_info: "Informações de garantia não fornecidas (Art. 31)".to_string(),
        });
    }

    if product_has_risk && !has_risk_info {
        return Err(CdcError::InformationInadequacy {
            missing_info: "Riscos do produto não informados adequadamente (Art. 9º)".to_string(),
        });
    }

    Ok(())
}

/// Check for tied sale (venda casada - Art. 39, I)
pub fn check_tied_sale(
    primary_product: &str,
    required_secondary: Option<&str>,
    can_purchase_separately: bool,
) -> CdcResult<()> {
    if let Some(secondary) = required_secondary
        && !can_purchase_separately
    {
        return Err(CdcError::TiedSale {
            description: format!(
                "Condicionamento da venda de '{}' à aquisição de '{}'",
                primary_product, secondary
            ),
        });
    }
    Ok(())
}

/// Validate debt collection practices (Art. 42)
pub fn validate_collection_practice(
    exposed_to_ridicule: bool,
    threatened: bool,
    contacted_at_work: bool,
    work_contact_authorized: bool,
    contacted_after_hours: bool,
) -> CdcResult<()> {
    if exposed_to_ridicule {
        return Err(CdcError::CollectionAbuse {
            description: "Consumidor exposto ao ridículo (Art. 42)".to_string(),
        });
    }

    if threatened {
        return Err(CdcError::CollectionAbuse {
            description: "Consumidor ameaçado durante cobrança (Art. 42)".to_string(),
        });
    }

    if contacted_at_work && !work_contact_authorized {
        return Err(CdcError::CollectionAbuse {
            description: "Cobrança no local de trabalho sem autorização (Art. 42)".to_string(),
        });
    }

    if contacted_after_hours {
        return Err(CdcError::CollectionAbuse {
            description: "Cobrança em horário impróprio (Art. 42)".to_string(),
        });
    }

    Ok(())
}

/// Calculate warranty extension for complaint filed in time (Art. 26, §2)
///
/// The warranty period is suspended during complaint resolution.
pub fn calculate_warranty_suspension(
    original_warranty_end: NaiveDate,
    complaint_date: NaiveDate,
    resolution_date: NaiveDate,
) -> NaiveDate {
    if complaint_date > original_warranty_end {
        return original_warranty_end; // Complaint after warranty
    }

    let suspension_days = (resolution_date - complaint_date).num_days();
    if suspension_days > 0 {
        original_warranty_end + chrono::Duration::days(suspension_days)
    } else {
        original_warranty_end
    }
}

/// Comprehensive CDC compliance check
pub fn validate_cdc_compliance(
    contract: &ConsumerContract,
    product_info_adequate: bool,
    no_abusive_clauses: bool,
    warranty_terms_clear: bool,
) -> ConsumerCompliance {
    let mut compliance = ConsumerCompliance {
        compliant: true,
        information_adequate: product_info_adequate,
        no_abusive_clauses,
        liability_proper: true,
        withdrawal_respected: true,
        warranty_compliant: warranty_terms_clear,
        issues: Vec::new(),
        recommendations: Vec::new(),
    };

    // Check information adequacy
    if !product_info_adequate {
        compliance.compliant = false;
        compliance
            .issues
            .push("Informações inadequadas (Art. 6, III)".to_string());
        compliance
            .recommendations
            .push("Fornecer informações claras em português".to_string());
    }

    // Check abusive clauses
    if !no_abusive_clauses {
        compliance.compliant = false;
        compliance
            .issues
            .push("Cláusulas abusivas detectadas (Art. 51)".to_string());
        compliance
            .recommendations
            .push("Remover cláusulas abusivas do contrato".to_string());
    }

    // Check withdrawal right
    if contract.has_withdrawal_right() {
        compliance.recommendations.push(format!(
            "Informar direito de arrependimento até {}",
            contract.withdrawal_deadline()
        ));
    }

    // Check warranty terms
    if !warranty_terms_clear {
        compliance.compliant = false;
        compliance
            .issues
            .push("Termos de garantia não claros (Art. 26)".to_string());
        compliance.recommendations.push(
            "Especificar garantia legal: 30 dias (não duráveis) ou 90 dias (duráveis)".to_string(),
        );
    }

    // Add hypervulnerable consumer warning
    if contract.consumer.is_hypervulnerable {
        compliance
            .recommendations
            .push("Atenção: consumidor hipervulnerável - proteção reforçada aplicável".to_string());
    }

    compliance
}

/// Get consumer remedy options for defective product (Art. 18)
pub fn get_product_remedy_options() -> Vec<&'static str> {
    vec![
        "Substituição do produto por outro da mesma espécie",
        "Restituição imediata da quantia paga, monetariamente atualizada",
        "Abatimento proporcional do preço",
    ]
}

/// Get consumer remedy options for defective service (Art. 20)
pub fn get_service_remedy_options() -> Vec<&'static str> {
    vec![
        "Reexecução do serviço, sem custo adicional",
        "Restituição imediata da quantia paga, monetariamente atualizada",
        "Abatimento proporcional do preço",
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_withdrawal_valid() {
        let contract_date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let withdrawal_date = NaiveDate::from_ymd_opt(2024, 1, 5).expect("valid date");

        let result = validate_withdrawal(contract_date, withdrawal_date, ContractType::Distance);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_withdrawal_expired() {
        let contract_date = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let withdrawal_date = NaiveDate::from_ymd_opt(2024, 1, 15).expect("valid date");

        let result = validate_withdrawal(contract_date, withdrawal_date, ContractType::Distance);
        assert!(result.is_err());
    }

    #[test]
    fn test_check_abusive_clause_liability() {
        let clause = "O fornecedor não se responsabiliza por quaisquer danos";
        let result = check_abusive_clause(clause);
        assert_eq!(result, Some(AbusiveClauseType::LiabilityExclusion));
    }

    #[test]
    fn test_check_abusive_clause_arbitration() {
        let clause = "Qualquer disputa será compulsoriamente submetido à arbitragem";
        let result = check_abusive_clause(clause);
        assert_eq!(result, Some(AbusiveClauseType::MandatoryArbitration));
    }

    #[test]
    fn test_check_normal_clause() {
        let clause = "Este contrato é regido pelas leis brasileiras";
        let result = check_abusive_clause(clause);
        assert!(result.is_none());
    }

    #[test]
    fn test_validate_warranty_claim_valid() {
        let product = Product {
            nome_pt: "Televisor".to_string(),
            description: "TV LED".to_string(),
            is_durable: true,
            is_essential: false,
            category: ProductCategory::Electronics,
        };

        let purchase = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let claim = NaiveDate::from_ymd_opt(2024, 2, 15).expect("valid date");

        let result = validate_warranty_claim(&product, purchase, claim);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_warranty_claim_expired() {
        let product = Product {
            nome_pt: "Leite".to_string(),
            description: "Leite integral".to_string(),
            is_durable: false,
            is_essential: true,
            category: ProductCategory::Food,
        };

        let purchase = NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date");
        let claim = NaiveDate::from_ymd_opt(2024, 3, 1).expect("valid date");

        let result = validate_warranty_claim(&product, purchase, claim);
        assert!(result.is_err());
    }

    #[test]
    fn test_determine_liability_manufacturer() {
        let provider = Provider {
            cnpj: "11222333000181".to_string(),
            nome_pt: "Fabricante SA".to_string(),
            nome_fantasia: None,
            provider_type: ProviderType::Manufacturer,
            atividade_principal: "Fabricação".to_string(),
        };

        let defect = ProductDefect::Manufacturing {
            description: "Peça defeituosa".to_string(),
        };

        let liability = determine_product_liability(&provider, &defect, None);
        assert!(liability.is_liable);
        assert_eq!(liability.liability_type, LiabilityType::Solidary);
    }

    #[test]
    fn test_determine_liability_with_exclusion() {
        let provider = Provider {
            cnpj: "11222333000181".to_string(),
            nome_pt: "Fabricante SA".to_string(),
            nome_fantasia: None,
            provider_type: ProviderType::Manufacturer,
            atividade_principal: "Fabricação".to_string(),
        };

        let defect = ProductDefect::Manufacturing {
            description: "Suposto defeito".to_string(),
        };

        let exclusion = LiabilityExclusion::NoDefect;
        let liability = determine_product_liability(&provider, &defect, Some(&exclusion));
        assert!(!liability.is_liable);
    }

    #[test]
    fn test_validate_tied_sale() {
        let result = check_tied_sale("Celular", Some("Seguro"), false);
        assert!(result.is_err());

        let result = check_tied_sale("Celular", Some("Seguro"), true);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_collection_abuse() {
        let result = validate_collection_practice(true, false, false, false, false);
        assert!(result.is_err());

        let result = validate_collection_practice(false, false, false, false, false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_warranty_suspension() {
        let warranty_end = NaiveDate::from_ymd_opt(2024, 4, 1).expect("valid date");
        let complaint = NaiveDate::from_ymd_opt(2024, 3, 15).expect("valid date");
        let resolution = NaiveDate::from_ymd_opt(2024, 3, 25).expect("valid date");

        let new_end = calculate_warranty_suspension(warranty_end, complaint, resolution);
        assert!(new_end > warranty_end);
    }

    #[test]
    fn test_compliance_check() {
        let contract = ConsumerContract {
            contract_type: ContractType::Distance,
            data_contratacao: NaiveDate::from_ymd_opt(2024, 1, 1).expect("valid date"),
            consumer: Consumer::individual("12345678909", "Test"),
            provider: Provider {
                cnpj: "11222333000181".to_string(),
                nome_pt: "Loja Ltda".to_string(),
                nome_fantasia: None,
                provider_type: ProviderType::Retailer,
                atividade_principal: "Varejo".to_string(),
            },
            valor_centavos: 10000,
            fora_estabelecimento: true,
        };

        let compliance = validate_cdc_compliance(&contract, true, true, true);
        assert!(compliance.compliant);
    }

    #[test]
    fn test_product_remedy_options() {
        let options = get_product_remedy_options();
        assert_eq!(options.len(), 3);
        assert!(options[0].contains("Substituição"));
    }
}
