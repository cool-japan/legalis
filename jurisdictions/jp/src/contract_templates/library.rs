//! Contract Clause Library (契約条項ライブラリ)
//!
//! This module provides a library of pre-built contract clauses.

use super::types::{Clause, ClauseCategory, RiskLevel};

/// Clause library providing standard contract clauses
pub struct ClauseLibrary {
    clauses: Vec<Clause>,
}

impl ClauseLibrary {
    /// Create a new clause library with standard clauses
    pub fn new() -> Self {
        let mut library = Self {
            clauses: Vec::new(),
        };

        library.register_standard_clauses();
        library
    }

    /// Register standard clauses
    fn register_standard_clauses(&mut self) {
        // General clauses
        self.add_clause(Self::purpose_clause());
        self.add_clause(Self::effective_date_clause());
        self.add_clause(Self::term_clause());
        self.add_clause(Self::termination_clause());
        self.add_clause(Self::amendment_clause());
        self.add_clause(Self::entire_agreement_clause());
        self.add_clause(Self::governing_law_clause());
        self.add_clause(Self::jurisdiction_clause());

        // Employment-specific clauses
        self.add_clause(Self::working_hours_clause());
        self.add_clause(Self::wage_payment_clause());
        self.add_clause(Self::probation_period_clause());
        self.add_clause(Self::resignation_notice_clause());

        // Confidentiality clauses
        self.add_clause(Self::confidentiality_obligation_clause());
        self.add_clause(Self::confidentiality_exceptions_clause());
        self.add_clause(Self::return_of_materials_clause());

        // Liability clauses
        self.add_clause(Self::liability_limitation_clause());
        self.add_clause(Self::indemnification_clause());
        self.add_clause(Self::force_majeure_clause());
    }

    /// Add a clause to the library
    pub fn add_clause(&mut self, clause: Clause) {
        self.clauses.push(clause);
    }

    /// Get a clause by ID
    pub fn get_clause(&self, id: &str) -> Option<&Clause> {
        self.clauses.iter().find(|c| c.id == id)
    }

    /// Get all clauses by category
    pub fn get_clauses_by_category(&self, category: ClauseCategory) -> Vec<&Clause> {
        self.clauses
            .iter()
            .filter(|c| c.category == category)
            .collect()
    }

    // ========================================================================
    // GENERAL CLAUSES
    // ========================================================================

    fn purpose_clause() -> Clause {
        Clause::new(
            "purpose",
            "第1条（目的）",
            "本契約は、{{party_a}}（以下「甲」という。）と{{party_b}}（以下「乙」という。）との間における{{purpose}}に関する事項を定めることを目的とする。",
        )
        .with_english_title("Article 1 (Purpose)")
        .with_english_content("This Agreement sets forth the terms and conditions regarding {{purpose}} between {{party_a}} (hereinafter \"Party A\") and {{party_b}} (hereinafter \"Party B\").")
        .with_category(ClauseCategory::General)
        .with_risk_level(RiskLevel::Low)
    }

    fn effective_date_clause() -> Clause {
        Clause::new(
            "effective_date",
            "第2条（契約の効力発生日）",
            "本契約は、{{effective_date}}をもって効力を生じるものとする。",
        )
        .with_english_title("Article 2 (Effective Date)")
        .with_english_content("This Agreement shall become effective on {{effective_date}}.")
        .with_category(ClauseCategory::General)
        .with_risk_level(RiskLevel::Low)
    }

    fn term_clause() -> Clause {
        Clause::new(
            "term",
            "第3条（契約期間）",
            "本契約の有効期間は、{{start_date}}から{{end_date}}までとする。{{#if auto_renewal}}ただし、期間満了の{{renewal_notice_days}}日前までに当事者のいずれかから書面による解約の申し出がない場合は、同一条件でさらに{{renewal_period}}間自動的に更新されるものとし、以後も同様とする。{{/if}}",
        )
        .with_english_title("Article 3 (Term)")
        .with_english_content("The term of this Agreement shall be from {{start_date}} to {{end_date}}. {{#if auto_renewal}}Unless either party gives written notice of termination at least {{renewal_notice_days}} days prior to expiration, this Agreement shall automatically renew for an additional {{renewal_period}} under the same terms.{{/if}}")
        .with_category(ClauseCategory::General)
        .with_risk_level(RiskLevel::Low)
    }

    fn termination_clause() -> Clause {
        Clause::new(
            "termination",
            "第4条（解除）",
            "各当事者は、相手方が本契約に定める義務に違反し、催告後{{cure_period}}日以内に是正されない場合、本契約を解除することができる。",
        )
        .with_english_title("Article 4 (Termination)")
        .with_english_content("Either party may terminate this Agreement if the other party breaches any obligation under this Agreement and fails to cure such breach within {{cure_period}} days after notice.")
        .with_category(ClauseCategory::Termination)
        .with_risk_level(RiskLevel::Medium)
    }

    fn amendment_clause() -> Clause {
        Clause::new(
            "amendment",
            "第5条（契約の変更）",
            "本契約の変更は、当事者双方の書面による合意によってのみ行うことができる。",
        )
        .with_english_title("Article 5 (Amendment)")
        .with_english_content(
            "This Agreement may be amended only by written agreement of both parties.",
        )
        .with_category(ClauseCategory::General)
        .with_risk_level(RiskLevel::Low)
    }

    fn entire_agreement_clause() -> Clause {
        Clause::new(
            "entire_agreement",
            "第6条（完全合意）",
            "本契約は、本件に関する当事者間の完全なる合意を構成し、本契約締結前の一切の口頭又は書面による合意、了解及び表明に優先する。",
        )
        .with_english_title("Article 6 (Entire Agreement)")
        .with_english_content("This Agreement constitutes the entire agreement between the parties and supersedes all prior oral or written agreements, understandings, and representations.")
        .with_category(ClauseCategory::General)
        .with_risk_level(RiskLevel::Low)
    }

    fn governing_law_clause() -> Clause {
        Clause::new(
            "governing_law",
            "第7条（準拠法）",
            "本契約は、日本法に準拠し、日本法に従って解釈されるものとする。",
        )
        .with_english_title("Article 7 (Governing Law)")
        .with_english_content("This Agreement shall be governed by and construed in accordance with the laws of Japan.")
        .with_category(ClauseCategory::DisputeResolution)
        .with_risk_level(RiskLevel::Low)
    }

    fn jurisdiction_clause() -> Clause {
        Clause::new(
            "jurisdiction",
            "第8条（管轄裁判所）",
            "本契約に関する一切の紛争については、{{jurisdiction_court}}を第一審の専属的合意管轄裁判所とする。",
        )
        .with_english_title("Article 8 (Jurisdiction)")
        .with_english_content("The {{jurisdiction_court}} shall have exclusive jurisdiction over any disputes arising under this Agreement.")
        .with_category(ClauseCategory::DisputeResolution)
        .with_risk_level(RiskLevel::Medium)
    }

    // ========================================================================
    // EMPLOYMENT CLAUSES
    // ========================================================================

    fn working_hours_clause() -> Clause {
        Clause::new(
            "working_hours",
            "第9条（勤務時間）",
            "乙の勤務時間は、{{start_time}}から{{end_time}}までとし、休憩時間は{{break_minutes}}分とする。",
        )
        .with_english_title("Article 9 (Working Hours)")
        .with_english_content("Party B's working hours shall be from {{start_time}} to {{end_time}}, with a {{break_minutes}}-minute break.")
        .with_category(ClauseCategory::WorkingConditions)
        .with_risk_level(RiskLevel::Low)
    }

    fn wage_payment_clause() -> Clause {
        Clause::new(
            "wage_payment",
            "第10条（賃金）",
            "甲は、乙に対し、基本給として月額{{base_salary}}円を支払う。賃金は、毎月{{payment_day}}日に、乙の指定する銀行口座に振り込む方法により支払う。",
        )
        .with_english_title("Article 10 (Wages)")
        .with_english_content("Party A shall pay Party B a monthly base salary of {{base_salary}} yen. Wages shall be paid on the {{payment_day}}th of each month by bank transfer to Party B's designated account.")
        .with_category(ClauseCategory::Payment)
        .with_risk_level(RiskLevel::Low)
    }

    fn probation_period_clause() -> Clause {
        Clause::new(
            "probation_period",
            "第11条（試用期間）",
            "乙については、雇用開始日から{{probation_months}}ヶ月間を試用期間とする。試用期間中又は試用期間満了時に、乙が従業員として不適格と認められた場合、甲は本契約を解約することができる。",
        )
        .with_english_title("Article 11 (Probation Period)")
        .with_english_content("Party B shall be subject to a probation period of {{probation_months}} months from the employment start date. Party A may terminate this Agreement during or at the end of the probation period if Party B is deemed unsuitable.")
        .with_category(ClauseCategory::WorkingConditions)
        .with_risk_level(RiskLevel::Medium)
        .optional()
    }

    fn resignation_notice_clause() -> Clause {
        Clause::new(
            "resignation_notice",
            "第12条（退職）",
            "乙が自己の都合により退職しようとするときは、退職予定日の少なくとも{{notice_days}}日前までに、甲に対して書面で通知するものとする。",
        )
        .with_english_title("Article 12 (Resignation)")
        .with_english_content("If Party B wishes to resign, Party B shall provide written notice to Party A at least {{notice_days}} days prior to the intended resignation date.")
        .with_category(ClauseCategory::Termination)
        .with_risk_level(RiskLevel::Low)
    }

    // ========================================================================
    // CONFIDENTIALITY CLAUSES
    // ========================================================================

    fn confidentiality_obligation_clause() -> Clause {
        Clause::new(
            "confidentiality_obligation",
            "第13条（秘密保持義務）",
            "当事者は、本契約に関連して相手方から開示された秘密情報を、第三者に開示又は漏洩してはならず、本契約の目的以外に使用してはならない。この義務は、本契約終了後も{{confidentiality_years}}年間継続する。",
        )
        .with_english_title("Article 13 (Confidentiality Obligation)")
        .with_english_content("Each party shall not disclose confidential information received from the other party to any third party and shall not use such information for any purpose other than the purpose of this Agreement. This obligation shall survive for {{confidentiality_years}} years after termination of this Agreement.")
        .with_category(ClauseCategory::Confidentiality)
        .with_risk_level(RiskLevel::High)
    }

    fn confidentiality_exceptions_clause() -> Clause {
        Clause::new(
            "confidentiality_exceptions",
            "第14条（秘密情報の例外）",
            "前条の秘密情報には、次の各号に該当する情報は含まれない。\n(1) 開示時に既に公知であった情報\n(2) 開示後に受領者の責めによらず公知となった情報\n(3) 開示時に既に受領者が保有していた情報\n(4) 第三者から秘密保持義務を負うことなく正当に入手した情報\n(5) 秘密情報によらず独自に開発した情報",
        )
        .with_english_title("Article 14 (Exceptions to Confidential Information)")
        .with_english_content("The confidential information in the preceding article shall not include:\n(1) Information that was publicly known at the time of disclosure\n(2) Information that becomes publicly known after disclosure through no fault of the recipient\n(3) Information already possessed by the recipient at the time of disclosure\n(4) Information rightfully obtained from a third party without confidentiality obligations\n(5) Information independently developed without reference to confidential information")
        .with_category(ClauseCategory::Confidentiality)
        .with_risk_level(RiskLevel::Low)
    }

    fn return_of_materials_clause() -> Clause {
        Clause::new(
            "return_of_materials",
            "第15条（資料等の返還）",
            "当事者は、本契約終了時又は相手方の要求があった場合、相手方から受領した秘密情報及びこれを含む一切の資料を速やかに返還又は廃棄し、その写しを保持してはならない。",
        )
        .with_english_title("Article 15 (Return of Materials)")
        .with_english_content("Upon termination of this Agreement or upon request, each party shall promptly return or destroy all confidential information and materials received from the other party and shall not retain any copies.")
        .with_category(ClauseCategory::Confidentiality)
        .with_risk_level(RiskLevel::Medium)
    }

    // ========================================================================
    // LIABILITY CLAUSES
    // ========================================================================

    fn liability_limitation_clause() -> Clause {
        Clause::new(
            "liability_limitation",
            "第16条（責任の制限）",
            "本契約に基づく損害賠償責任は、債務不履行、不法行為その他請求原因の如何を問わず、直接かつ現実に生じた通常の損害に限るものとし、間接損害、特別損害、逸失利益については責任を負わないものとする。ただし、当事者の故意又は重過失による場合はこの限りではない。",
        )
        .with_english_title("Article 16 (Limitation of Liability)")
        .with_english_content("Liability for damages under this Agreement shall be limited to direct and actual ordinary damages, regardless of the cause of action, and shall not include indirect damages, special damages, or lost profits, except in cases of willful misconduct or gross negligence.")
        .with_category(ClauseCategory::Liability)
        .with_risk_level(RiskLevel::High)
    }

    fn indemnification_clause() -> Clause {
        Clause::new(
            "indemnification",
            "第17条（補償）",
            "各当事者は、自己の責めに帰すべき事由により相手方に損害を与えた場合、相手方に対し、当該損害を賠償するものとする。",
        )
        .with_english_title("Article 17 (Indemnification)")
        .with_english_content("Each party shall indemnify the other party for any damages caused by such party's fault.")
        .with_category(ClauseCategory::Liability)
        .with_risk_level(RiskLevel::Medium)
    }

    fn force_majeure_clause() -> Clause {
        Clause::new(
            "force_majeure",
            "第18条（不可抗力）",
            "天災地変、戦争、暴動、法令の制定改廃、公権力による命令処分その他当事者の責めに帰することのできない事由により本契約の履行が不能となった場合、当事者は当該不履行につき責任を負わないものとする。",
        )
        .with_english_title("Article 18 (Force Majeure)")
        .with_english_content("Neither party shall be liable for failure to perform its obligations under this Agreement due to acts of God, war, riots, changes in laws, government orders, or other causes beyond its reasonable control.")
        .with_category(ClauseCategory::Liability)
        .with_risk_level(RiskLevel::Low)
    }
}

impl Default for ClauseLibrary {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_creation() {
        let library = ClauseLibrary::new();
        assert!(!library.clauses.is_empty());
    }

    #[test]
    fn test_get_clause_by_id() {
        let library = ClauseLibrary::new();
        let clause = library.get_clause("purpose");
        assert!(clause.is_some());
        assert_eq!(clause.unwrap().id, "purpose");
    }

    #[test]
    fn test_get_clauses_by_category() {
        let library = ClauseLibrary::new();
        let confidentiality_clauses =
            library.get_clauses_by_category(ClauseCategory::Confidentiality);
        assert!(!confidentiality_clauses.is_empty());
        assert!(
            confidentiality_clauses
                .iter()
                .all(|c| c.category == ClauseCategory::Confidentiality)
        );
    }

    #[test]
    fn test_clause_has_japanese_and_english() {
        let library = ClauseLibrary::new();
        let clause = library.get_clause("purpose").unwrap();
        assert!(!clause.title_ja.is_empty());
        assert!(clause.title_en.is_some());
        assert!(!clause.content_ja.is_empty());
        assert!(clause.content_en.is_some());
    }

    #[test]
    fn test_risk_levels_assigned() {
        let library = ClauseLibrary::new();
        let high_risk = library.get_clause("liability_limitation").unwrap();
        assert_eq!(high_risk.risk_level, RiskLevel::High);

        let low_risk = library.get_clause("purpose").unwrap();
        assert_eq!(low_risk.risk_level, RiskLevel::Low);
    }
}
