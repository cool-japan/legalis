//! CLI command implementations.

pub mod analysis;
pub mod cloud_team_perf;
pub mod core;
pub mod registry_operations;
pub mod registry_plugin_config;
pub mod script_ai_workflow;

// Re-export all public handler functions for backwards-compatibility.
pub use analysis::{
    handle_batch_export, handle_batch_format, handle_batch_lint, handle_batch_verify,
    handle_benchmark, handle_builder_wizard, handle_debug, handle_diff_viewer, handle_explain,
    handle_graph, handle_migrate, handle_profile, handle_registry_browser,
    handle_resolve_conflicts, handle_sim_tune, handle_trace,
};
pub use cloud_team_perf::{
    handle_perf_bottlenecks, handle_perf_disable, handle_perf_enable, handle_perf_list,
    handle_perf_optimize, handle_perf_report, handle_perf_start, handle_perf_stats,
    handle_perf_status, handle_perf_stop, handle_team_access_grant, handle_team_access_list,
    handle_team_access_revoke, handle_team_access_update, handle_team_create_workspace,
    handle_team_list_notifications, handle_team_list_sessions, handle_team_list_workspaces,
    handle_team_mark_read, handle_team_notify, handle_team_show_history, handle_team_start_session,
    handle_team_sync_history,
};
pub use core::{
    handle_audit, handle_complexity, handle_convert, handle_diff, handle_export, handle_format,
    handle_import, handle_init, handle_lint, handle_lod, handle_parse, handle_port,
    handle_simulate, handle_verify, handle_viz, handle_watch,
};
pub use registry_operations::{
    handle_add, handle_clean, handle_doctor, handle_install, handle_list, handle_new,
    handle_outdated, handle_publish, handle_repl, handle_search, handle_test, handle_uninstall,
    handle_update, handle_validate,
};
pub use registry_plugin_config::{
    handle_config_activate, handle_config_diff, handle_config_init, handle_config_profiles,
    handle_config_show, handle_config_validate, handle_plugin_disable, handle_plugin_enable,
    handle_plugin_info, handle_plugin_install, handle_plugin_list, handle_plugin_uninstall,
    handle_plugin_update, handle_registry_diff, handle_registry_login, handle_registry_logout,
    handle_registry_pull, handle_registry_push, handle_registry_sync,
};
pub use script_ai_workflow::{
    handle_ai_complete, handle_ai_help, handle_ai_intent, handle_ai_parse, handle_ai_suggest,
    handle_cloud_aws, handle_cloud_azure, handle_cloud_configure, handle_cloud_gcp,
    handle_cloud_list, handle_cloud_provision, handle_cloud_status, handle_dashboard,
    handle_script_builtin, handle_script_info, handle_script_install, handle_script_list,
    handle_script_new, handle_script_run, handle_script_uninstall, handle_script_validate,
    handle_workflow_info, handle_workflow_list_templates, handle_workflow_new, handle_workflow_run,
    handle_workflow_validate,
};

// Shared internal helpers used by multiple submodules.
use anyhow::{Context, Result};
use legalis_core::Statute;
use legalis_dsl::LegalDslParser;
use std::fs;

/// Parses multiple statute files.
pub(crate) fn parse_statutes(inputs: &[String]) -> Result<Vec<Statute>> {
    let parser = LegalDslParser::new();
    let mut statutes = Vec::new();

    for input in inputs {
        let content = fs::read_to_string(input)
            .with_context(|| format!("Failed to read input file: {}", input))?;

        let statute = parser
            .parse_statute(&content)
            .map_err(|e| anyhow::anyhow!("Parse error in {}: {}", input, e))?;

        statutes.push(statute);
    }

    Ok(statutes)
}

/// Converts a statute to DSL string representation.
pub(crate) fn statute_to_dsl(statute: &Statute) -> String {
    let mut dsl = format!("STATUTE {}: \"{}\" {{\n", statute.id, statute.title);

    if let Some(ref jur) = statute.jurisdiction {
        dsl.push_str(&format!("    JURISDICTION \"{}\"\n", jur));
    }
    if statute.version > 1 {
        dsl.push_str(&format!("    VERSION {}\n", statute.version));
    }
    if let Some(eff) = statute.temporal_validity.effective_date {
        dsl.push_str(&format!("    EFFECTIVE \"{}\"\n", eff.format("%Y-%m-%d")));
    }
    if let Some(exp) = statute.temporal_validity.expiry_date {
        dsl.push_str(&format!("    EXPIRES \"{}\"\n", exp.format("%Y-%m-%d")));
    }

    if !statute.preconditions.is_empty() {
        let conditions: Vec<String> = statute.preconditions.iter().map(condition_to_dsl).collect();
        dsl.push_str(&format!("    WHEN {}\n", conditions.join(" AND ")));
    }

    dsl.push_str(&format!(
        "    THEN {:?} \"{}\"\n",
        statute.effect.effect_type, statute.effect.description
    ));

    if let Some(ref discretion) = statute.discretion_logic {
        dsl.push_str(&format!("    DISCRETION \"{}\"\n", discretion));
    }

    dsl.push('}');
    dsl
}

/// Converts a condition to DSL string representation.
pub(crate) fn condition_to_dsl(condition: &legalis_core::Condition) -> String {
    use legalis_core::Condition;

    match condition {
        Condition::Age { operator, value } => {
            format!("AGE {} {}", operator, value)
        }
        Condition::Income { operator, value } => {
            format!("INCOME {} {}", operator, value)
        }
        Condition::And(left, right) => {
            format!(
                "({} AND {})",
                condition_to_dsl(left),
                condition_to_dsl(right)
            )
        }
        Condition::Or(left, right) => {
            format!(
                "({} OR {})",
                condition_to_dsl(left),
                condition_to_dsl(right)
            )
        }
        Condition::Not(inner) => {
            format!("NOT {}", condition_to_dsl(inner))
        }
        Condition::AttributeEquals { key, value } => {
            format!("HAS \"{}\" = \"{}\"", key, value)
        }
        Condition::HasAttribute { key } => {
            format!("HAS \"{}\"", key)
        }
        Condition::ResidencyDuration { operator, months } => {
            format!("RESIDENCY {} {} months", operator, months)
        }
        Condition::Geographic {
            region_type,
            region_id,
        } => {
            format!("REGION {:?} \"{}\"", region_type, region_id)
        }
        Condition::Custom { description } => {
            format!("CUSTOM \"{}\"", description)
        }
        _ => format!("{:?}", condition),
    }
}
