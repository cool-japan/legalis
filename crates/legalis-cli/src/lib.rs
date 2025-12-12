//! Legalis-CLI: Command-line interface for Legalis-RS.
//!
//! This crate provides a CLI tool for:
//! - Parsing and validating legal DSL files
//! - Running verification checks
//! - Generating visualizations
//! - Exporting to various formats

pub mod commands;

use clap::{Parser, Subcommand};

/// Legalis-RS Command Line Interface
#[derive(Parser)]
#[command(name = "legalis")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Increase verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Output format
    #[arg(short, long, default_value = "text")]
    pub format: OutputFormat,

    #[command(subcommand)]
    pub command: Commands,
}

/// Output format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
    Yaml,
}

/// Available commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Parse a legal DSL file
    Parse {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Verify statutes for logical consistency
    Verify {
        /// Input file(s) to verify
        #[arg(short, long)]
        input: Vec<String>,

        /// Fail on warnings
        #[arg(long)]
        strict: bool,
    },

    /// Generate visualization
    Viz {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Visualization format (dot, mermaid)
        #[arg(long, default_value = "mermaid")]
        viz_format: VizFormat,
    },

    /// Export statute to different formats
    Export {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Export format
        #[arg(long)]
        export_format: ExportFormat,
    },

    /// Start the API server
    Serve {
        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(short, long, default_value = "3000")]
        port: u16,
    },

    /// Initialize a new Legalis project
    Init {
        /// Project directory
        #[arg(default_value = ".")]
        path: String,
    },
}

/// Visualization format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum VizFormat {
    /// GraphViz DOT format
    Dot,
    /// Mermaid diagram format
    #[default]
    Mermaid,
}

/// Export format options.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum ExportFormat {
    /// JSON format
    Json,
    /// YAML format
    Yaml,
    /// Solidity smart contract
    Solidity,
}
