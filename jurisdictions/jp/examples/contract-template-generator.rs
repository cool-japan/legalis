//! Contract Template Generator Example
//!
//! 契約テンプレート生成器の例
//!
//! This example demonstrates the contract template generation system,
//! showing how to create various types of contracts using templates
//! with variable substitution and conditional clauses.

use legalis_jp::contract_templates::*;

fn main() {
    println!("=== Contract Template Generator Example ===\n");
    println!("契約テンプレート生成器の例 - Contract Template Generation System\n");

    // Part 1: Employment Contract
    part1_employment_contract();

    // Part 2: Non-Disclosure Agreement (NDA)
    part2_nda_template();

    // Part 3: Service Agreement
    part3_service_agreement();

    // Part 4: Using the Clause Library
    part4_clause_library();
}

/// Part 1: Generate an employment contract (雇用契約書)
fn part1_employment_contract() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 1: EMPLOYMENT CONTRACT (雇用契約書)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Create template engine
    let mut engine = TemplateEngine::new();

    // Define employment contract template
    let employment_template = r#"
雇用契約書

{{employer_name}}（以下「甲」という。）と{{employee_name}}（以下「乙」という。）は、以下のとおり雇用契約を締結する。

第1条（雇用）
甲は乙を{{position}}として雇用し、乙はこれを承諾する。

第2条（雇用期間）
雇用期間は{{start_date}}から{{end_date}}までとする。

{{#if has_probation}}
第3条（試用期間）
雇用開始日から{{probation_months}}ヶ月間を試用期間とする。
{{/if}}

第{{article_num}}条（勤務時間）
勤務時間は{{work_start_time}}から{{work_end_time}}までとし、休憩時間は{{break_minutes}}分とする。

第{{next_article}}条（賃金）
甲は乙に対し、基本給として月額{{base_salary}}円を支払う。
賃金は毎月{{payment_day}}日に乙の指定する銀行口座に振り込む。

{{#unless is_part_time}}
第{{final_article}}条（フルタイム雇用）
本契約はフルタイム雇用であり、週{{working_days}}日の勤務を原則とする。
{{/unless}}

{{contract_date}}
甲: {{employer_name}}
乙: {{employee_name}}
"#;

    let mut template = ContractTemplate::new(
        "employment_fulltime",
        "雇用契約書（正社員）",
        TemplateType::Employment,
        employment_template,
    );

    // Define required variables
    template.require_variable("employer_name");
    template.require_variable("employee_name");
    template.require_variable("position");
    template.require_variable("start_date");
    template.require_variable("end_date");
    template.require_variable("work_start_time");
    template.require_variable("work_end_time");
    template.require_variable("break_minutes");
    template.require_variable("base_salary");
    template.require_variable("payment_day");
    template.require_variable("contract_date");

    engine.register_template(template);

    // Create context with employee data
    let mut context = TemplateContext::new();
    context.set_string("employer_name", "株式会社テクノロジーソリューションズ");
    context.set_string("employee_name", "山田太郎");
    context.set_string("position", "ソフトウェアエンジニア");
    context.set_string("start_date", "2024年4月1日");
    context.set_string("end_date", "2025年3月31日");
    context.set_boolean("has_probation", true);
    context.set_integer("probation_months", 3);
    context.set_string("work_start_time", "9:00");
    context.set_string("work_end_time", "18:00");
    context.set_integer("break_minutes", 60);
    context.set_integer("base_salary", 400_000);
    context.set_integer("payment_day", 25);
    context.set_boolean("is_part_time", false);
    context.set_integer("working_days", 5);
    context.set_integer("article_num", 4);
    context.set_integer("next_article", 5);
    context.set_integer("final_article", 6);
    context.set_string("contract_date", "2024年3月15日");

    // Render the contract
    match engine.render("employment_fulltime", &context) {
        Ok(contract) => {
            println!("✅ Contract Generated Successfully!\n");
            println!("{}", contract.content_ja);
            println!("\n📋 Contract Details:");
            println!("   Template: {}", contract.template_id);
            println!("   Type: {}", contract.template_type.japanese_name());
            println!(
                "   Generated: {}",
                contract.generated_at.format("%Y-%m-%d %H:%M:%S")
            );
            println!("   Variables: {} used", contract.variables.len());
        }
        Err(e) => {
            eprintln!("❌ Error generating contract: {}", e);
        }
    }

    println!("\n");
}

/// Part 2: Generate an NDA template (秘密保持契約書)
fn part2_nda_template() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 2: NON-DISCLOSURE AGREEMENT (秘密保持契約書)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut engine = TemplateEngine::new();

    let nda_template = r#"
秘密保持契約書

{{party_a}}（以下「甲」という。）と{{party_b}}（以下「乙」という。）は、{{purpose}}に関連して、以下のとおり秘密保持契約を締結する。

第1条（秘密情報の定義）
本契約において「秘密情報」とは、{{information_types}}に関する一切の情報をいう。

第2条（秘密保持義務）
当事者は、相手方から開示された秘密情報を第三者に開示又は漏洩してはならず、本契約の目的以外に使用してはならない。

第3条（秘密保持期間）
本条の義務は、本契約終了後も{{confidentiality_years}}年間継続するものとする。

{{#if is_mutual}}
第4条（相互秘密保持）
本契約は相互秘密保持契約であり、双方の当事者が秘密情報を開示及び受領する。
{{/if}}

第5条（例外）
次の各号に該当する情報は秘密情報に含まれない。
(1) 開示時に既に公知であった情報
(2) 開示後に受領者の責めによらず公知となった情報
(3) 開示時に既に受領者が保有していた情報

第6条（契約期間）
本契約の有効期間は、{{effective_date}}から{{expiration_date}}までとする。

{{contract_date}}
甲: {{party_a}}
乙: {{party_b}}
"#;

    let mut template = ContractTemplate::new(
        "nda_mutual",
        "秘密保持契約書",
        TemplateType::NDA,
        nda_template,
    );

    template.require_variable("party_a");
    template.require_variable("party_b");
    template.require_variable("purpose");
    template.require_variable("information_types");
    template.require_variable("confidentiality_years");
    template.require_variable("effective_date");
    template.require_variable("expiration_date");
    template.require_variable("contract_date");

    engine.register_template(template);

    let mut context = TemplateContext::new();
    context.set_string("party_a", "株式会社ABC");
    context.set_string("party_b", "株式会社XYZ");
    context.set_string("purpose", "新製品の共同開発");
    context.set_string("information_types", "技術情報、営業情報、顧客情報");
    context.set_integer("confidentiality_years", 5);
    context.set_boolean("is_mutual", true);
    context.set_string("effective_date", "2024年4月1日");
    context.set_string("expiration_date", "2027年3月31日");
    context.set_string("contract_date", "2024年3月20日");

    match engine.render("nda_mutual", &context) {
        Ok(contract) => {
            println!("✅ NDA Generated Successfully!\n");
            println!("{}", contract.content_ja);
            println!("\n📋 NDA Type: Mutual (相互NDA)");
            println!(
                "   Confidentiality Period: {} years",
                context.get("confidentiality_years").unwrap().as_string()
            );
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    println!("\n");
}

/// Part 3: Service Agreement (業務委託契約書)
fn part3_service_agreement() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 3: SERVICE AGREEMENT (業務委託契約書)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let mut engine = TemplateEngine::new();

    let service_template = r#"
業務委託契約書

{{client}}（以下「甲」という。）と{{contractor}}（以下「乙」という。）は、{{service_description}}の業務委託に関し、以下のとおり契約を締結する。

第1条（業務内容）
乙は甲に対し、以下の業務を提供する。
{{service_details}}

第2条（契約期間）
本契約の期間は、{{start_date}}から{{end_date}}までとする。

第3条（報酬）
甲は乙に対し、本業務の対価として{{total_amount}}円を支払う。
{{#if is_installment}}
支払いは{{installment_count}}回に分割し、{{installment_schedule}}に従って支払う。
{{/if}}

第4条（業務の遂行）
乙は、善良なる管理者の注意をもって業務を遂行するものとする。

{{#if has_deliverables}}
第5条（成果物）
乙は、{{delivery_date}}までに{{deliverables}}を甲に納入する。
{{/if}}

第6条（知的財産権）
本業務により生じた成果物の著作権その他の知的財産権は、{{ip_owner}}に帰属するものとする。

{{contract_date}}
甲: {{client}}
乙: {{contractor}}
"#;

    let mut template = ContractTemplate::new(
        "service_agreement_dev",
        "業務委託契約書（ソフトウェア開発）",
        TemplateType::ServiceAgreement,
        service_template,
    );

    template.require_variable("client");
    template.require_variable("contractor");
    template.require_variable("service_description");
    template.require_variable("service_details");
    template.require_variable("start_date");
    template.require_variable("end_date");
    template.require_variable("total_amount");
    template.require_variable("ip_owner");
    template.require_variable("contract_date");

    engine.register_template(template);

    let mut context = TemplateContext::new();
    context.set_string("client", "株式会社クライアント");
    context.set_string("contractor", "フリーランスエンジニア 佐藤花子");
    context.set_string("service_description", "ウェブアプリケーション開発");
    context.set_string(
        "service_details",
        "・要件定義\n・設計\n・実装\n・テスト\n・納品",
    );
    context.set_string("start_date", "2024年5月1日");
    context.set_string("end_date", "2024年8月31日");
    context.set_integer("total_amount", 2_000_000);
    context.set_boolean("is_installment", true);
    context.set_integer("installment_count", 4);
    context.set_string("installment_schedule", "毎月末");
    context.set_boolean("has_deliverables", true);
    context.set_string("delivery_date", "2024年8月31日");
    context.set_string(
        "deliverables",
        "完成したウェブアプリケーション及びソースコード",
    );
    context.set_string("ip_owner", "甲（クライアント）");
    context.set_string("contract_date", "2024年4月15日");

    match engine.render("service_agreement_dev", &context) {
        Ok(contract) => {
            println!("✅ Service Agreement Generated!\n");
            println!("{}", contract.content_ja);
            println!("\n📋 Service Type: Software Development");
            println!(
                "   Total Amount: ¥{}",
                context.get("total_amount").unwrap().as_string()
            );
            println!(
                "   Duration: {} to {}",
                context.get("start_date").unwrap().as_string(),
                context.get("end_date").unwrap().as_string()
            );
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
        }
    }

    println!("\n");
}

/// Part 4: Using the Clause Library (条項ライブラリの使用)
fn part4_clause_library() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("PART 4: CLAUSE LIBRARY (条項ライブラリ)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    let library = ClauseLibrary::new();

    println!("📚 Standard Clause Library");
    println!("Available standard clauses for contract generation:\n");

    // Show confidentiality clauses
    println!("🔒 Confidentiality Clauses:");
    let confidentiality = library.get_clauses_by_category(ClauseCategory::Confidentiality);
    for clause in &confidentiality {
        println!("   • {} ({})", clause.title_ja, clause.id);
        println!("     Risk Level: {:?}", clause.risk_level);
    }

    println!();

    // Show liability clauses
    println!("⚖️  Liability Clauses:");
    let liability = library.get_clauses_by_category(ClauseCategory::Liability);
    for clause in &liability {
        println!("   • {} ({})", clause.title_ja, clause.id);
        println!("     Risk Level: {:?}", clause.risk_level);
    }

    println!();

    // Show dispute resolution clauses
    println!("⚡ Dispute Resolution Clauses:");
    let dispute = library.get_clauses_by_category(ClauseCategory::DisputeResolution);
    for clause in &dispute {
        println!("   • {} ({})", clause.title_ja, clause.id);
        if let Some(title_en) = &clause.title_en {
            println!("     English: {}", title_en);
        }
    }

    println!();

    // Demonstrate clause rendering
    println!("📝 Example: Purpose Clause with Variables\n");

    let engine = TemplateEngine::new();
    if let Some(purpose_clause) = library.get_clause("purpose") {
        let mut context = TemplateContext::new();
        context.set_string("party_a", "甲社");
        context.set_string("party_b", "乙社");
        context.set_string("purpose", "ソフトウェアライセンス");

        match engine.render_template_string(&purpose_clause.content_ja, &context) {
            Ok(rendered) => {
                println!("Title: {}", purpose_clause.title_ja);
                println!("Content:\n{}", rendered);
            }
            Err(e) => {
                eprintln!("Error rendering clause: {}", e);
            }
        }
    }

    println!("\n✅ Clause Library Demo Complete!");
}
