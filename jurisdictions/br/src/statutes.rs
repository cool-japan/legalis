//! `legalis-core` Statute models for major Brazilian federal legislation (Direito brasileiro).
//!
//! This module expresses landmark provisions of Brazilian law as
//! [`legalis_core::Statute`] values, so they can be reasoned over by the core
//! engine (conflict analysis, forward chaining, temporal validity) and rendered
//! as `legalis-dsl` source text via [`statutes_as_dsl`].
//!
//! Each builder is grounded in a real, currently-in-force Lei / Decreto-Lei and
//! cites it in its doc comment. The provisions span the law areas implemented by
//! this crate: Código Civil, CLT (trabalho), CDC (consumidor), LGPD (proteção de
//! dados), Lei das S.A. (societário), Lei de Falências (insolvência), Política
//! Nacional do Meio Ambiente (ambiental), and the Código Tributário Nacional.
//!
//! Modelos de Statute do legalis-core para a legislação federal brasileira.

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Código Civil (Lei nº 10.406/2002), Art. 186 c/c Art. 927 — responsabilidade
/// civil por ato ilícito (tort liability): whoever, by voluntary act or omission,
/// negligence or imprudence, violates a right and causes damage to another is
/// obliged to repair it.
#[must_use]
pub fn civil_code_tort_liability_statute() -> Statute {
    Statute::new(
        "BR-CC-2002-ART186",
        "Responsabilidade Civil por Ato Ilícito (Lei nº 10.406/2002, Art. 186 e 927)",
        Effect::new(
            EffectType::Obligation,
            "Quem, por ação ou omissão voluntária, negligência ou imprudência, \
             violar direito e causar dano a outrem comete ato ilícito e fica \
             obrigado a repará-lo (dever de indenizar)",
        )
        .with_parameter("article", "186")
        .with_parameter("reparation", "integral"),
    )
    .with_precondition(Condition::attribute_equals("unlawful_act", "true"))
    .with_jurisdiction("BR")
}

/// CLT — Consolidação das Leis do Trabalho (Decreto-Lei nº 5.452/1943), Art. 58 —
/// jornada normal de trabalho: the normal working day must not exceed 8 hours,
/// capped at 44 hours per week, unless a different limit is expressly set.
#[must_use]
pub fn clt_working_hours_statute() -> Statute {
    Statute::new(
        "BR-CLT-1943-ART58",
        "Jornada Normal de Trabalho (Decreto-Lei nº 5.452/1943 - CLT, Art. 58)",
        Effect::new(
            EffectType::Prohibition,
            "A duração normal do trabalho não pode exceder 8 horas diárias e \
             44 horas semanais, salvo limite legal expressamente fixado",
        )
        .with_parameter("max_daily_hours", "8")
        .with_parameter("max_weekly_hours", "44"),
    )
    .with_precondition(Condition::attribute_equals(
        "employment_relationship",
        "true",
    ))
    .with_jurisdiction("BR")
}

/// Código de Defesa do Consumidor (Lei nº 8.078/1990), Art. 49 — direito de
/// arrependimento: the consumer may withdraw from a contract entered into away
/// from business premises (e.g. by telephone or at home) within 7 days.
#[must_use]
pub fn cdc_withdrawal_right_statute() -> Statute {
    Statute::new(
        "BR-CDC-1990-ART49",
        "Direito de Arrependimento (Lei nº 8.078/1990 - CDC, Art. 49)",
        Effect::new(
            EffectType::Grant,
            "O consumidor pode desistir do contrato firmado fora do estabelecimento \
             comercial, no prazo de 7 dias a contar da assinatura ou do recebimento \
             do produto, com restituição integral dos valores pagos",
        )
        .with_parameter("cooling_off_days", "7"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::LessOrEqual,
        value: 7,
        unit: DurationUnit::Days,
    })
    .with_jurisdiction("BR")
}

/// LGPD — Lei Geral de Proteção de Dados Pessoais (Lei nº 13.709/2018), Art. 7 —
/// hipóteses de tratamento: the processing of personal data may only be carried
/// out upon a lawful legal basis, the first being the consent of the data subject.
#[must_use]
pub fn lgpd_lawful_processing_statute() -> Statute {
    Statute::new(
        "BR-LGPD-2018-ART7",
        "Bases Legais para Tratamento de Dados Pessoais (Lei nº 13.709/2018 - LGPD, Art. 7)",
        Effect::new(
            EffectType::Obligation,
            "O tratamento de dados pessoais somente pode ser realizado mediante \
             hipótese legal autorizadora; na ausência de outra base, exige-se o \
             consentimento livre, informado e inequívoco do titular",
        )
        .with_parameter("legal_bases", "10")
        .with_parameter("authority", "ANPD"),
    )
    .with_precondition(Condition::attribute_equals("data_subject_consent", "true"))
    .with_jurisdiction("BR")
}

/// Lei das Sociedades por Ações (Lei nº 6.404/1976), Art. 109 — direitos
/// essenciais do acionista: certain shareholder rights, such as sharing in the
/// company's profits, may not be removed by the bylaws or the general meeting.
#[must_use]
pub fn corporate_shareholder_rights_statute() -> Statute {
    Statute::new(
        "BR-LSA-1976-ART109",
        "Direitos Essenciais do Acionista (Lei nº 6.404/1976 - Lei das S.A., Art. 109)",
        Effect::new(
            EffectType::Grant,
            "Nem o estatuto social nem a assembleia geral podem privar o acionista \
             dos direitos essenciais de participar dos lucros sociais, fiscalizar a \
             gestão e retirar-se da sociedade nos casos previstos em lei",
        )
        .with_parameter("article", "109"),
    )
    .with_precondition(Condition::percentage(
        ComparisonOp::GreaterThan,
        0,
        "share_ownership",
    ))
    .with_jurisdiction("BR")
}

/// Lei de Recuperação Judicial e Falências (Lei nº 11.101/2005), Art. 6 —
/// suspensão das execuções: the granting of judicial reorganization suspends, as a
/// rule, the course of actions and enforcement proceedings against the debtor for
/// a stay period of 180 days.
#[must_use]
pub fn bankruptcy_automatic_stay_statute() -> Statute {
    Statute::new(
        "BR-LREF-2005-ART6",
        "Suspensão das Execuções na Recuperação Judicial (Lei nº 11.101/2005, Art. 6)",
        Effect::new(
            EffectType::Prohibition,
            "O deferimento do processamento da recuperação judicial suspende o curso \
             das execuções ajuizadas contra o devedor pelo prazo (stay period) de \
             180 dias, vedando atos de constrição sobre bens essenciais à atividade",
        )
        .with_parameter("stay_period_days", "180"),
    )
    .with_precondition(Condition::attribute_equals(
        "judicial_reorganization_granted",
        "true",
    ))
    .with_jurisdiction("BR")
}

/// Política Nacional do Meio Ambiente (Lei nº 6.938/1981), Art. 14, §1º —
/// responsabilidade objetiva ambiental: the polluter is obliged, regardless of
/// fault, to indemnify or repair damage caused to the environment and to third
/// parties affected by its activity (princípio do poluidor-pagador).
#[must_use]
pub fn environmental_strict_liability_statute() -> Statute {
    Statute::new(
        "BR-PNMA-1981-ART14",
        "Responsabilidade Objetiva por Dano Ambiental (Lei nº 6.938/1981 - PNMA, Art. 14)",
        Effect::new(
            EffectType::Obligation,
            "O poluidor é obrigado, independentemente da existência de culpa, a \
             indenizar ou reparar os danos causados ao meio ambiente e a terceiros \
             afetados por sua atividade (princípio do poluidor-pagador)",
        )
        .with_parameter("liability_regime", "objective")
        .with_parameter("article", "14"),
    )
    .with_precondition(Condition::attribute_equals("environmental_damage", "true"))
    .with_jurisdiction("BR")
}

/// Código Tributário Nacional (Lei nº 5.172/1966), Art. 113 e 142 — obrigação
/// tributária principal: the principal tax obligation arises with the occurrence
/// of the taxable event and has as its object the payment of the tax, constituted
/// by the administrative act of assessment (lançamento).
#[must_use]
pub fn tax_principal_obligation_statute() -> Statute {
    Statute::new(
        "BR-CTN-1966-ART113",
        "Obrigação Tributária Principal (Lei nº 5.172/1966 - CTN, Art. 113 e 142)",
        Effect::new(
            EffectType::MonetaryTransfer,
            "A obrigação tributária principal surge com a ocorrência do fato gerador, \
             tem por objeto o pagamento do tributo e é constituída pelo lançamento, \
             ato administrativo vinculado e obrigatório",
        )
        .with_parameter("obligation_type", "principal")
        .with_parameter("constituting_act", "lancamento"),
    )
    .with_precondition(Condition::attribute_equals(
        "taxable_event_occurred",
        "true",
    ))
    .with_jurisdiction("BR")
}

/// Returns every modelled Brazilian statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        civil_code_tort_liability_statute(),
        clt_working_hours_statute(),
        cdc_withdrawal_right_statute(),
        lgpd_lawful_processing_statute(),
        corporate_shareholder_rights_statute(),
        bankruptcy_automatic_stay_statute(),
        environmental_strict_liability_statute(),
        tax_principal_obligation_statute(),
    ]
}

/// Renders every modelled Brazilian statute as `legalis-dsl` source text.
///
/// Each statute is emitted as a `STATUTE … { WHEN … THEN … }` block by
/// [`legalis_dsl::format_statutes`].
#[must_use]
pub fn statutes_as_dsl() -> String {
    legalis_dsl::format_statutes(&all_statutes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statutes_render_as_valid_dsl() {
        let statutes = all_statutes();
        assert!(!statutes.is_empty(), "BR must model at least one statute");
        assert_eq!(statutes.len(), 8, "BR must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving
        // the printer handled each one across the range of effect/condition
        // kinds the BR adapters use.
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
