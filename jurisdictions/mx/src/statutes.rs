//! `Statute`-based models of major Mexican federal legislation.
//!
//! This module lifts the domain types and validators implemented across the
//! `legalis-mx` crate (civil, company, competition, criminal, data protection,
//! intellectual property, labour and tax law) into the jurisdiction-neutral
//! [`legalis_core::Statute`] abstraction. Each builder encodes a *real* statutory
//! provision of Mexican federal law — the authoritative ley, its publication year
//! in the *Diario Oficial de la Federación* and the operative rule — as an
//! [`Effect`] together with a meaningful [`Condition`] precondition wherever the
//! underlying law turns on a quantifiable trigger (a statutory period, a monetary
//! threshold expressed as an attribute, an ownership percentage or a status flag).
//!
//! Mexico is a *civil-law* jurisdiction in the Romano-Germanic tradition; its
//! federal statutes are codified instruments enacted by the Congreso de la Unión.
//! The provisions modelled here are deliberately the load-bearing rules of each
//! ley (the jornada máxima, the IVA rate, the LFPDPPP breach-notice duty, and so
//! on) rather than incidental clauses.
//!
//! The modelled statutes can be rendered as `legalis-dsl` source text via
//! [`statutes_as_dsl`], so the Mexican rule-set can be inspected, diffed,
//! formatted and consumed by the DSL tooling (LSP, documentation generation,
//! structural diffing) on the same footing as every other jurisdiction.
//!
//! # Coverage
//!
//! | Builder | Ley |
//! |---------|-----|
//! | [`codigo_civil_federal_statute`] | Código Civil Federal (1928), art. 1794/1796 |
//! | [`lgsm_statute`] | Ley General de Sociedades Mercantiles (1934), art. 6/89 |
//! | [`ley_federal_trabajo_statute`] | Ley Federal del Trabajo (1970), arts. 61 & 90 |
//! | [`lfpdppp_statute`] | LFPDPPP (2010), art. 20 |
//! | [`lfce_statute`] | Ley Federal de Competencia Económica (2014), art. 53 |
//! | [`codigo_penal_federal_statute`] | Código Penal Federal (1931), art. 367 |
//! | [`ley_iva_statute`] | Ley del Impuesto al Valor Agregado (1978), art. 1 |
//! | [`ley_isr_statute`] | Ley del Impuesto sobre la Renta (2013), art. 9 |
//!
//! # Disclaimer
//!
//! These models are simplified abstractions for computational reasoning and are
//! provided for educational and informational purposes only. They are not legal
//! advice; consult a licensed Mexican attorney (*abogado/a con cédula
//! profesional*).

use legalis_core::{ComparisonOp, Condition, DurationUnit, Effect, EffectType, Statute};

/// Código Civil Federal (1928), arts. 1794 & 1796 — validity and binding force of
/// contracts.
///
/// Under article 1794 a contract requires consent (*consentimiento*) and an object
/// that can be the subject of the contract (*objeto que pueda ser materia del
/// contrato*) to exist. Article 1796 establishes that contracts are perfected by
/// mere consent and from that moment bind the parties not only to what is expressly
/// agreed but also to the consequences that, by their nature, conform to good faith,
/// usage and the law (*pacta sunt servanda*). Modelled here as the obligation that
/// arises once valid consent has been given.
///
/// Real source: Código Civil Federal, arts. 1794 y 1796 (DOF 1928).
#[must_use]
pub fn codigo_civil_federal_statute() -> Statute {
    Statute::new(
        "MX-CCF-1928-ART1796",
        "Fuerza Obligatoria de los Contratos (Código Civil Federal, art. 1796)",
        Effect::new(
            EffectType::Obligation,
            "Un contrato perfeccionado por el consentimiento obliga a las partes al \
             cumplimiento de lo pactado y a las consecuencias conformes a la buena fe, \
             el uso y la ley",
        )
        .with_parameter("codigo", "Codigo Civil Federal")
        .with_parameter("year", "1928")
        .with_parameter("articulo", "1796"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "consentimiento_otorgado".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MX")
}

/// Ley General de Sociedades Mercantiles (1934), arts. 6 & 89 — minimum content of
/// the constitutive act and incorporation of a sociedad anónima.
///
/// Article 6 requires the constitutive instrument (*escritura constitutiva*) of any
/// mercantile company to state, among other particulars, the corporate name, object,
/// domicile, capital and the way the capital is represented. Article 89 governs the
/// sociedad anónima (S.A.), requiring at least two shareholders each subscribing at
/// least one share and the full subscription of the share capital. Modelled here as
/// the obligation to formalise the company once the minimum number of shareholders
/// is met.
///
/// Real source: Ley General de Sociedades Mercantiles, arts. 6 y 89 (DOF 1934).
#[must_use]
pub fn lgsm_statute() -> Statute {
    Statute::new(
        "MX-LGSM-1934-ART89",
        "Constitución de la Sociedad Anónima (LGSM, art. 89)",
        Effect::new(
            EffectType::Obligation,
            "Una sociedad anónima debe constituirse mediante escritura pública con al \
             menos dos accionistas que suscriban una acción cada uno y el íntegro del \
             capital social suscrito",
        )
        .with_parameter("ley", "Ley General de Sociedades Mercantiles")
        .with_parameter("year", "1934")
        .with_parameter("articulo", "89")
        .with_parameter("min_accionistas", "2"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "numero_accionistas".to_string(),
        value: "2".to_string(),
    })
    .with_jurisdiction("MX")
}

/// Ley Federal del Trabajo (1970), arts. 61 & 90 — jornada máxima y salario mínimo.
///
/// Article 61 fixes the maximum length of the working day (*jornada*): eight hours
/// for the day shift (*jornada diurna*), seven hours for the night shift and seven
/// and a half hours for the mixed shift. Article 90 defines the minimum wage
/// (*salario mínimo*) as the lowest amount a worker must receive in cash for a
/// working day, sufficient to satisfy the normal material, social and cultural needs
/// of a head of family, below which no remuneration may fall. The constitutional
/// basis for both is article 123, Apartado A (fracciones I and VI) of the
/// Constitution. Modelled here as the day-shift jornada limit of eight hours, the
/// load-bearing protective rule of the working-day regime.
///
/// Real source: Ley Federal del Trabajo, arts. 61 y 90 (DOF 1970); Const. art. 123-A.
#[must_use]
pub fn ley_federal_trabajo_statute() -> Statute {
    Statute::new(
        "MX-LFT-1970-ART61",
        "Jornada Máxima y Salario Mínimo (Ley Federal del Trabajo, arts. 61 y 90)",
        Effect::new(
            EffectType::Prohibition,
            "La duración de la jornada diurna no debe exceder de ocho horas, salvo las \
             horas extraordinarias permitidas por la ley, y la retribución no puede ser \
             inferior al salario mínimo general vigente",
        )
        .with_parameter("ley", "Ley Federal del Trabajo")
        .with_parameter("year", "1970")
        .with_parameter("articulo", "61")
        .with_parameter("articulo_salario", "90")
        .with_parameter("max_horas_jornada_diurna", "8"),
    )
    .with_precondition(Condition::Duration {
        operator: ComparisonOp::LessOrEqual,
        value: 8,
        unit: DurationUnit::Days,
    })
    .with_jurisdiction("MX")
}

/// Ley Federal de Protección de Datos Personales en Posesión de los Particulares
/// (2010), art. 20 — notificación de vulneraciones de seguridad.
///
/// Article 20 of the LFPDPPP requires that breaches of security occurring at any
/// stage of processing which materially affect the patrimonial or moral rights of
/// data owners (*titulares*) be reported to the affected owners immediately by the
/// responsible party (*responsable*), so that they may take steps to protect their
/// rights. The Act, published in the Diario Oficial de la Federación on 5 July 2010,
/// is enforced by the INAI and is built around the ARCO rights (acceso,
/// rectificación, cancelación, oposición).
///
/// Real source: Ley Federal de Protección de Datos Personales en Posesión de los
/// Particulares, art. 20 (DOF 05/07/2010).
#[must_use]
pub fn lfpdppp_statute() -> Statute {
    Statute::new(
        "MX-LFPDPPP-2010",
        "Notificación de Vulneraciones de Seguridad (LFPDPPP, art. 20)",
        Effect::new(
            EffectType::Obligation,
            "El responsable debe informar de forma inmediata a los titulares afectados \
             las vulneraciones de seguridad que afecten significativamente sus derechos \
             patrimoniales o morales",
        )
        .with_parameter("ley", "LFPDPPP")
        .with_parameter("year", "2010")
        .with_parameter("articulo", "20")
        .with_parameter("autoridad", "INAI"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "vulneracion_seguridad".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MX")
}

/// Ley Federal de Competencia Económica (2014), art. 53 — prácticas monopólicas
/// absolutas.
///
/// Article 53 declares unlawful and absolutely void (*ilícitas y nulas de pleno
/// derecho*) the contracts, arrangements or combinations between competing economic
/// agents (*prácticas monopólicas absolutas*) whose object or effect is to fix
/// prices, restrict output, divide markets, rig bids or exchange information for any
/// of those purposes. The current LFCE was published in the Diario Oficial de la
/// Federación on 23 May 2014 and is enforced by the COFECE.
///
/// Real source: Ley Federal de Competencia Económica, art. 53 (DOF 23/05/2014).
#[must_use]
pub fn lfce_statute() -> Statute {
    Statute::new(
        "MX-LFCE-2014-ART53",
        "Prácticas Monopólicas Absolutas (LFCE, art. 53)",
        Effect::new(
            EffectType::Prohibition,
            "Los acuerdos entre agentes económicos competidores para fijar precios, \
             restringir la oferta, dividir mercados o coordinar posturas en licitaciones \
             son prácticas monopólicas absolutas, ilícitas y nulas de pleno derecho",
        )
        .with_parameter("ley", "Ley Federal de Competencia Economica")
        .with_parameter("year", "2014")
        .with_parameter("articulo", "53")
        .with_parameter("autoridad", "COFECE"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "agentes_competidores".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MX")
}

/// Código Penal Federal (1931), arts. 367 & 370 — delito de robo.
///
/// Article 367 defines theft (*robo*) as the taking by a person, without right and
/// without the consent of the person who may lawfully give it, of a movable thing
/// belonging to another. Article 370 graduates the penalty by reference to the value
/// of the thing stolen, providing escalating terms of imprisonment as the value
/// exceeds successive multiples of the daily minimum wage. Modelled here as the
/// status change to criminal liability once the unlawful taking is established.
///
/// Real source: Código Penal Federal, arts. 367 y 370 (DOF 1931).
#[must_use]
pub fn codigo_penal_federal_statute() -> Statute {
    Statute::new(
        "MX-CPF-1931-ART367",
        "Delito de Robo (Código Penal Federal, art. 367)",
        Effect::new(
            EffectType::StatusChange,
            "Comete el delito de robo quien se apodera de una cosa ajena mueble, sin \
             derecho y sin consentimiento de quien puede disponer de ella conforme a la \
             ley, incurriendo en responsabilidad penal",
        )
        .with_parameter("codigo", "Codigo Penal Federal")
        .with_parameter("year", "1931")
        .with_parameter("articulo", "367"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "apoderamiento_sin_derecho".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MX")
}

/// Ley del Impuesto al Valor Agregado (1978), art. 1 — causación del IVA.
///
/// Article 1 of the Ley del IVA obliges natural and juridical persons who, within
/// national territory, alienate goods, render independent services, grant the
/// temporary use or enjoyment of goods, or import goods or services, to pay
/// value-added tax. The general rate is 16 % (*tasa general del 16 %*), applied to
/// the value of the act or activity. Modelled here as the obligation to charge and
/// remit IVA at the standard rate when a taxable act or activity is carried out.
///
/// Real source: Ley del Impuesto al Valor Agregado, art. 1 (DOF 29/12/1978).
#[must_use]
pub fn ley_iva_statute() -> Statute {
    Statute::new(
        "MX-LIVA-1978-ART1",
        "Causación del Impuesto al Valor Agregado (Ley del IVA, art. 1)",
        Effect::new(
            EffectType::Obligation,
            "Las personas que enajenen bienes, presten servicios independientes, otorguen \
             el uso o goce temporal de bienes o importen bienes o servicios en territorio \
             nacional deben trasladar y enterar el IVA a la tasa general del 16 %",
        )
        .with_parameter("ley", "Ley del Impuesto al Valor Agregado")
        .with_parameter("year", "1978")
        .with_parameter("articulo", "1")
        .with_parameter("tasa_general_pct", "16"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "acto_o_actividad_gravado".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MX")
}

/// Ley del Impuesto sobre la Renta (2013), art. 9 — ISR de personas morales.
///
/// Article 9 of the Ley del ISR provides that legal entities (*personas morales*)
/// must compute and pay income tax by applying the rate of 30 % to their taxable
/// result for the financial year (*resultado fiscal*), arrived at by reducing
/// accumulable income by authorised deductions and employee profit sharing, and then
/// applying prior-year losses. The current Ley del ISR was published in the Diario
/// Oficial de la Federación on 11 December 2013. Modelled here as the obligation to
/// pay corporate income tax at the 30 % rate once a positive taxable result exists.
///
/// Real source: Ley del Impuesto sobre la Renta, art. 9 (DOF 11/12/2013).
#[must_use]
pub fn ley_isr_statute() -> Statute {
    Statute::new(
        "MX-LISR-2013-ART9",
        "Impuesto sobre la Renta de Personas Morales (Ley del ISR, art. 9)",
        Effect::new(
            EffectType::MonetaryTransfer,
            "Las personas morales deben calcular y enterar el impuesto sobre la renta \
             aplicando la tasa del 30 % al resultado fiscal obtenido en el ejercicio",
        )
        .with_parameter("ley", "Ley del Impuesto sobre la Renta")
        .with_parameter("year", "2013")
        .with_parameter("articulo", "9")
        .with_parameter("tasa_personas_morales_pct", "30"),
    )
    .with_precondition(Condition::AttributeEquals {
        key: "resultado_fiscal_positivo".to_string(),
        value: "true".to_string(),
    })
    .with_jurisdiction("MX")
}

/// Returns every modelled Mexican federal statute.
#[must_use]
pub fn all_statutes() -> Vec<Statute> {
    vec![
        codigo_civil_federal_statute(),
        lgsm_statute(),
        ley_federal_trabajo_statute(),
        lfpdppp_statute(),
        lfce_statute(),
        codigo_penal_federal_statute(),
        ley_iva_statute(),
        ley_isr_statute(),
    ]
}

/// Renders every modelled Mexican federal statute as `legalis-dsl` source text.
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
        assert!(!statutes.is_empty(), "MX must model at least one statute");
        assert_eq!(statutes.len(), 8, "MX must model exactly 8 statutes");

        let dsl = statutes_as_dsl();
        assert!(!dsl.is_empty(), "DSL export must not be empty");

        // Every modelled statute's id must appear in the rendered DSL, proving the
        // printer handled each one (covers the range of condition kinds the MX
        // adapters use: Duration and AttributeEquals).
        for statute in &statutes {
            assert!(
                dsl.contains(statute.id.as_str()),
                "statute {} missing from DSL export",
                statute.id
            );
        }
    }
}
