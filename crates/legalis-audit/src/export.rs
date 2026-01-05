//! Export functionality for audit trails.

use crate::{AuditError, AuditRecord, AuditResult, ComplianceReport};
use printpdf::*;
use rust_xlsxwriter::{Format, Workbook};
use serde_json::{Value, json};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Exports audit records to CSV format.
pub fn to_csv<W: Write>(records: &[AuditRecord], writer: &mut W) -> AuditResult<()> {
    // Write header
    writeln!(
        writer,
        "id,timestamp,event_type,actor_type,statute_id,subject_id,result_type,record_hash"
    )?;

    // Write records
    for record in records {
        let event_type = format!("{:?}", record.event_type);
        let actor_type = match &record.actor {
            crate::Actor::System { component } => format!("System({})", component),
            crate::Actor::User { user_id, role } => format!("User({}, {})", user_id, role),
            crate::Actor::External { system_id } => format!("External({})", system_id),
        };
        let result_type = match &record.result {
            crate::DecisionResult::Deterministic { .. } => "Deterministic",
            crate::DecisionResult::RequiresDiscretion { .. } => "RequiresDiscretion",
            crate::DecisionResult::Void { .. } => "Void",
            crate::DecisionResult::Overridden { .. } => "Overridden",
        };

        writeln!(
            writer,
            "{},{},{},{},{},{},{},{}",
            record.id,
            record.timestamp.to_rfc3339(),
            event_type,
            actor_type,
            record.statute_id,
            record.subject_id,
            result_type,
            record.record_hash
        )?;
    }

    Ok(())
}

/// Exports audit records to JSON-LD format.
pub fn to_jsonld(records: &[AuditRecord]) -> AuditResult<Value> {
    let context = json!({
        "@vocab": "http://schema.org/",
        "audit": "http://legalis.example.org/audit#",
        "id": "@id",
        "type": "@type",
        "AuditRecord": "audit:AuditRecord",
        "DecisionEvent": "audit:DecisionEvent",
        "timestamp": {
            "@id": "audit:timestamp",
            "@type": "http://www.w3.org/2001/XMLSchema#dateTime"
        },
        "actor": "audit:actor",
        "statute": "audit:statute",
        "subject": "audit:subject",
        "result": "audit:result",
        "previousHash": "audit:previousHash",
        "recordHash": "audit:recordHash"
    });

    let graph: Vec<Value> = records
        .iter()
        .map(|record| {
            let actor = match &record.actor {
                crate::Actor::System { component } => json!({
                    "@type": "audit:SystemActor",
                    "component": component
                }),
                crate::Actor::User { user_id, role } => json!({
                    "@type": "audit:UserActor",
                    "userId": user_id,
                    "role": role
                }),
                crate::Actor::External { system_id } => json!({
                    "@type": "audit:ExternalActor",
                    "systemId": system_id
                }),
            };

            let result = match &record.result {
                crate::DecisionResult::Deterministic {
                    effect_applied,
                    parameters,
                } => json!({
                    "@type": "audit:DeterministicResult",
                    "effectApplied": effect_applied,
                    "parameters": parameters
                }),
                crate::DecisionResult::RequiresDiscretion {
                    issue,
                    narrative_hint,
                    assigned_to,
                } => json!({
                    "@type": "audit:DiscretionaryResult",
                    "issue": issue,
                    "narrativeHint": narrative_hint,
                    "assignedTo": assigned_to
                }),
                crate::DecisionResult::Void { reason } => json!({
                    "@type": "audit:VoidResult",
                    "reason": reason
                }),
                crate::DecisionResult::Overridden {
                    original_result: _,
                    new_result: _,
                    justification,
                } => json!({
                    "@type": "audit:OverriddenResult",
                    "justification": justification
                }),
            };

            json!({
                "@type": "AuditRecord",
                "@id": format!("urn:uuid:{}", record.id),
                "timestamp": record.timestamp.to_rfc3339(),
                "eventType": format!("{:?}", record.event_type),
                "actor": actor,
                "statute": record.statute_id,
                "subject": format!("urn:uuid:{}", record.subject_id),
                "result": result,
                "previousHash": record.previous_hash,
                "recordHash": record.record_hash
            })
        })
        .collect();

    Ok(json!({
        "@context": context,
        "@graph": graph
    }))
}

/// Exports audit records to JSON format.
pub fn to_json(records: &[AuditRecord]) -> AuditResult<Value> {
    Ok(serde_json::to_value(records)?)
}

/// Exports audit records to Excel format.
pub fn to_excel<P: AsRef<Path>>(records: &[AuditRecord], path: P) -> AuditResult<()> {
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // Add header format
    let header_format = Format::new().set_bold();

    // Write headers
    worksheet
        .write_string_with_format(0, 0, "ID", &header_format)
        .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
    worksheet
        .write_string_with_format(0, 1, "Timestamp", &header_format)
        .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
    worksheet
        .write_string_with_format(0, 2, "Event Type", &header_format)
        .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
    worksheet
        .write_string_with_format(0, 3, "Actor", &header_format)
        .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
    worksheet
        .write_string_with_format(0, 4, "Statute ID", &header_format)
        .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
    worksheet
        .write_string_with_format(0, 5, "Subject ID", &header_format)
        .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
    worksheet
        .write_string_with_format(0, 6, "Result Type", &header_format)
        .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
    worksheet
        .write_string_with_format(0, 7, "Record Hash", &header_format)
        .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;

    // Write data rows
    for (idx, record) in records.iter().enumerate() {
        let row = (idx + 1) as u32;

        let event_type = format!("{:?}", record.event_type);
        let actor_str = match &record.actor {
            crate::Actor::System { component } => format!("System({})", component),
            crate::Actor::User { user_id, role } => format!("User({}, {})", user_id, role),
            crate::Actor::External { system_id } => format!("External({})", system_id),
        };
        let result_type = match &record.result {
            crate::DecisionResult::Deterministic { .. } => "Deterministic",
            crate::DecisionResult::RequiresDiscretion { .. } => "RequiresDiscretion",
            crate::DecisionResult::Void { .. } => "Void",
            crate::DecisionResult::Overridden { .. } => "Overridden",
        };

        worksheet
            .write_string(row, 0, record.id.to_string())
            .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
        worksheet
            .write_string(row, 1, record.timestamp.to_rfc3339())
            .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
        worksheet
            .write_string(row, 2, &event_type)
            .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
        worksheet
            .write_string(row, 3, &actor_str)
            .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
        worksheet
            .write_string(row, 4, &record.statute_id)
            .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
        worksheet
            .write_string(row, 5, record.subject_id.to_string())
            .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
        worksheet
            .write_string(row, 6, result_type)
            .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
        worksheet
            .write_string(row, 7, &record.record_hash)
            .map_err(|e| AuditError::ExportError(format!("Excel write error: {}", e)))?;
    }

    workbook
        .save(path)
        .map_err(|e| AuditError::ExportError(format!("Failed to save Excel file: {}", e)))?;

    Ok(())
}

/// Exports a compliance report to PDF format.
#[allow(clippy::too_many_arguments)]
pub fn to_pdf<P: AsRef<Path>>(
    records: &[AuditRecord],
    report: &ComplianceReport,
    path: P,
    title: &str,
) -> AuditResult<()> {
    // Create PDF document
    let (doc, page1, layer1) = PdfDocument::new(
        title,
        Mm(210.0), // A4 width
        Mm(297.0), // A4 height
        "Layer 1",
    );

    let current_layer = doc.get_page(page1).get_layer(layer1);

    // Add fonts
    let font_bold = doc
        .add_builtin_font(BuiltinFont::HelveticaBold)
        .map_err(|e| AuditError::ExportError(format!("PDF font error: {}", e)))?;
    let font = doc
        .add_builtin_font(BuiltinFont::Helvetica)
        .map_err(|e| AuditError::ExportError(format!("PDF font error: {}", e)))?;

    let mut y_position = Mm(280.0);

    // Title
    current_layer.use_text(title, 18.0, Mm(20.0), y_position, &font_bold);
    y_position -= Mm(15.0);

    // Report summary
    current_layer.use_text(
        "Compliance Report Summary",
        14.0,
        Mm(20.0),
        y_position,
        &font_bold,
    );
    y_position -= Mm(10.0);

    current_layer.use_text(
        format!("Generated: {}", report.generated_at.to_rfc3339()),
        10.0,
        Mm(20.0),
        y_position,
        &font,
    );
    y_position -= Mm(7.0);

    current_layer.use_text(
        format!("Total Decisions: {}", report.total_decisions),
        10.0,
        Mm(20.0),
        y_position,
        &font,
    );
    y_position -= Mm(7.0);

    current_layer.use_text(
        format!("Automatic Decisions: {}", report.automatic_decisions),
        10.0,
        Mm(20.0),
        y_position,
        &font,
    );
    y_position -= Mm(7.0);

    current_layer.use_text(
        format!(
            "Discretionary Decisions: {}",
            report.discretionary_decisions
        ),
        10.0,
        Mm(20.0),
        y_position,
        &font,
    );
    y_position -= Mm(7.0);

    current_layer.use_text(
        format!("Human Overrides: {}", report.human_overrides),
        10.0,
        Mm(20.0),
        y_position,
        &font,
    );
    y_position -= Mm(7.0);

    current_layer.use_text(
        format!(
            "Integrity Verified: {}",
            if report.integrity_verified {
                "Yes"
            } else {
                "No"
            }
        ),
        10.0,
        Mm(20.0),
        y_position,
        &font,
    );
    y_position -= Mm(15.0);

    // Recent records section
    current_layer.use_text(
        format!("Recent Records (showing up to 20 of {})", records.len()),
        12.0,
        Mm(20.0),
        y_position,
        &font_bold,
    );
    y_position -= Mm(10.0);

    // Table headers
    current_layer.use_text("Timestamp", 9.0, Mm(20.0), y_position, &font_bold);
    current_layer.use_text("Statute ID", 9.0, Mm(70.0), y_position, &font_bold);
    current_layer.use_text("Result", 9.0, Mm(120.0), y_position, &font_bold);
    y_position -= Mm(7.0);

    // List records (up to 20)
    for record in records.iter().take(20) {
        if y_position < Mm(20.0) {
            break; // Prevent writing off the page
        }

        let result_str = match &record.result {
            crate::DecisionResult::Deterministic { .. } => "Deterministic",
            crate::DecisionResult::RequiresDiscretion { .. } => "Discretionary",
            crate::DecisionResult::Void { .. } => "Void",
            crate::DecisionResult::Overridden { .. } => "Overridden",
        };

        current_layer.use_text(
            record.timestamp.format("%Y-%m-%d %H:%M").to_string(),
            8.0,
            Mm(20.0),
            y_position,
            &font,
        );
        current_layer.use_text(&record.statute_id, 8.0, Mm(70.0), y_position, &font);
        current_layer.use_text(result_str, 8.0, Mm(120.0), y_position, &font);
        y_position -= Mm(6.0);
    }

    // Save PDF
    doc.save(&mut BufWriter::new(std::fs::File::create(path).map_err(
        |e| AuditError::ExportError(format!("Failed to create PDF file: {}", e)),
    )?))
    .map_err(|e| AuditError::ExportError(format!("Failed to save PDF: {}", e)))?;

    Ok(())
}

/// Exports a compliance report to HTML format.
pub fn to_html(
    records: &[AuditRecord],
    report: &ComplianceReport,
    title: &str,
) -> AuditResult<String> {
    let mut html = String::new();

    // HTML header
    html.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
    html.push_str("    <meta charset=\"UTF-8\">\n");
    html.push_str(
        "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n",
    );
    html.push_str(&format!("    <title>{}</title>\n", title));
    html.push_str("    <style>\n");
    html.push_str("        body { font-family: Arial, sans-serif; margin: 20px; background-color: #f5f5f5; }\n");
    html.push_str("        .container { max-width: 1200px; margin: 0 auto; background-color: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }\n");
    html.push_str(
        "        h1 { color: #333; border-bottom: 3px solid #4CAF50; padding-bottom: 10px; }\n",
    );
    html.push_str("        h2 { color: #555; margin-top: 30px; }\n");
    html.push_str("        .summary { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 20px; margin: 20px 0; }\n");
    html.push_str("        .summary-card { background-color: #f9f9f9; padding: 20px; border-radius: 5px; border-left: 4px solid #4CAF50; }\n");
    html.push_str(
        "        .summary-card h3 { margin: 0 0 10px 0; color: #666; font-size: 14px; }\n",
    );
    html.push_str(
        "        .summary-card .value { font-size: 28px; font-weight: bold; color: #333; }\n",
    );
    html.push_str("        table { width: 100%; border-collapse: collapse; margin-top: 20px; }\n");
    html.push_str("        th { background-color: #4CAF50; color: white; padding: 12px; text-align: left; }\n");
    html.push_str("        td { padding: 10px; border-bottom: 1px solid #ddd; }\n");
    html.push_str("        tr:hover { background-color: #f5f5f5; }\n");
    html.push_str("        .badge { display: inline-block; padding: 4px 8px; border-radius: 3px; font-size: 12px; font-weight: bold; }\n");
    html.push_str("        .badge-success { background-color: #4CAF50; color: white; }\n");
    html.push_str("        .badge-warning { background-color: #ff9800; color: white; }\n");
    html.push_str("        .badge-danger { background-color: #f44336; color: white; }\n");
    html.push_str("        .badge-info { background-color: #2196F3; color: white; }\n");
    html.push_str("        .timestamp { color: #666; font-size: 14px; }\n");
    html.push_str("    </style>\n");
    html.push_str("</head>\n<body>\n");
    html.push_str("    <div class=\"container\">\n");

    // Title
    html.push_str(&format!("        <h1>{}</h1>\n", title));
    html.push_str(&format!(
        "        <p class=\"timestamp\">Generated: {}</p>\n",
        report.generated_at.to_rfc3339()
    ));

    // Summary cards
    html.push_str("        <h2>Compliance Summary</h2>\n");
    html.push_str("        <div class=\"summary\">\n");

    html.push_str("            <div class=\"summary-card\">\n");
    html.push_str("                <h3>Total Decisions</h3>\n");
    html.push_str(&format!(
        "                <div class=\"value\">{}</div>\n",
        report.total_decisions
    ));
    html.push_str("            </div>\n");

    html.push_str("            <div class=\"summary-card\">\n");
    html.push_str("                <h3>Automatic Decisions</h3>\n");
    html.push_str(&format!(
        "                <div class=\"value\">{}</div>\n",
        report.automatic_decisions
    ));
    html.push_str("            </div>\n");

    html.push_str("            <div class=\"summary-card\">\n");
    html.push_str("                <h3>Discretionary Decisions</h3>\n");
    html.push_str(&format!(
        "                <div class=\"value\">{}</div>\n",
        report.discretionary_decisions
    ));
    html.push_str("            </div>\n");

    html.push_str("            <div class=\"summary-card\">\n");
    html.push_str("                <h3>Human Overrides</h3>\n");
    html.push_str(&format!(
        "                <div class=\"value\">{}</div>\n",
        report.human_overrides
    ));
    html.push_str("            </div>\n");

    html.push_str("            <div class=\"summary-card\">\n");
    html.push_str("                <h3>Integrity Status</h3>\n");
    html.push_str(&format!(
        "                <div class=\"value\">{}</div>\n",
        if report.integrity_verified {
            "✓ Verified"
        } else {
            "✗ Failed"
        }
    ));
    html.push_str("            </div>\n");

    html.push_str("        </div>\n");

    // Recent records table
    html.push_str(&format!(
        "        <h2>Recent Records ({} total)</h2>\n",
        records.len()
    ));
    html.push_str("        <table>\n");
    html.push_str("            <thead>\n");
    html.push_str("                <tr>\n");
    html.push_str("                    <th>Timestamp</th>\n");
    html.push_str("                    <th>Event Type</th>\n");
    html.push_str("                    <th>Statute ID</th>\n");
    html.push_str("                    <th>Actor</th>\n");
    html.push_str("                    <th>Result</th>\n");
    html.push_str("                </tr>\n");
    html.push_str("            </thead>\n");
    html.push_str("            <tbody>\n");

    // Show up to 50 most recent records
    for record in records.iter().take(50) {
        let event_type = format!("{:?}", record.event_type);
        let actor_str = match &record.actor {
            crate::Actor::System { component } => format!("System: {}", component),
            crate::Actor::User { user_id, role } => format!("User: {} ({})", user_id, role),
            crate::Actor::External { system_id } => format!("External: {}", system_id),
        };
        let (result_str, badge_class) = match &record.result {
            crate::DecisionResult::Deterministic { .. } => ("Deterministic", "badge-success"),
            crate::DecisionResult::RequiresDiscretion { .. } => ("Discretionary", "badge-warning"),
            crate::DecisionResult::Void { .. } => ("Void", "badge-danger"),
            crate::DecisionResult::Overridden { .. } => ("Overridden", "badge-info"),
        };

        html.push_str("                <tr>\n");
        html.push_str(&format!(
            "                    <td>{}</td>\n",
            record.timestamp.format("%Y-%m-%d %H:%M:%S")
        ));
        html.push_str(&format!("                    <td>{}</td>\n", event_type));
        html.push_str(&format!(
            "                    <td>{}</td>\n",
            record.statute_id
        ));
        html.push_str(&format!("                    <td>{}</td>\n", actor_str));
        html.push_str(&format!(
            "                    <td><span class=\"badge {}\">{}</span></td>\n",
            badge_class, result_str
        ));
        html.push_str("                </tr>\n");
    }

    html.push_str("            </tbody>\n");
    html.push_str("        </table>\n");

    html.push_str("    </div>\n");
    html.push_str("</body>\n</html>");

    Ok(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Actor, DecisionContext, DecisionResult, EventType};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn test_csv_export() {
        let records = vec![AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )];

        let mut output = Vec::new();
        to_csv(&records, &mut output).unwrap();
        let csv = String::from_utf8(output).unwrap();

        assert!(csv.contains("id,timestamp"));
        assert!(csv.contains("test-statute"));
    }

    #[test]
    fn test_jsonld_export() {
        let records = vec![AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )];

        let jsonld = to_jsonld(&records).unwrap();
        assert!(jsonld.get("@context").is_some());
        assert!(jsonld.get("@graph").is_some());
    }

    #[test]
    fn test_json_export() {
        let records = vec![AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )];

        let json = to_json(&records).unwrap();
        assert!(json.is_array());
    }

    #[test]
    fn test_html_export() {
        let records = vec![AuditRecord::new(
            EventType::AutomaticDecision,
            Actor::System {
                component: "test".to_string(),
            },
            "test-statute".to_string(),
            Uuid::new_v4(),
            DecisionContext::default(),
            DecisionResult::Deterministic {
                effect_applied: "test".to_string(),
                parameters: HashMap::new(),
            },
            None,
        )];

        let report = crate::ComplianceReport {
            total_decisions: 1,
            automatic_decisions: 1,
            discretionary_decisions: 0,
            human_overrides: 0,
            integrity_verified: true,
            generated_at: chrono::Utc::now(),
        };

        let html = to_html(&records, &report, "Test Audit Report").unwrap();
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Test Audit Report"));
        assert!(html.contains("Total Decisions"));
        assert!(html.contains("test-statute"));
    }
}
