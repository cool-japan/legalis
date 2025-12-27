//! Legalis-CLI: Command-line interface for Legalis-RS.
//!
//! This crate provides a CLI tool for:
//! - Parsing and validating legal DSL files
//! - Running verification checks
//! - Generating visualizations
//! - Exporting to various formats

pub mod commands;
pub mod config;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};

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

    /// Path to config file (defaults to legalis.toml or ~/.config/legalis/config.toml)
    #[arg(long)]
    pub config: Option<String>,

    /// Quiet mode (suppress non-error output)
    #[arg(short, long)]
    pub quiet: bool,

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
    Toml,
    Table,
    Csv,
    Html,
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

        /// Dry run (show what would be created without actually creating)
        #[arg(long)]
        dry_run: bool,
    },

    /// Compare two statute files
    Diff {
        /// First statute file
        #[arg(short, long)]
        old: String,

        /// Second statute file
        #[arg(short, long)]
        new: String,

        /// Output format
        #[arg(long, default_value = "text")]
        diff_format: DiffFormat,
    },

    /// Run a simulation on a population
    Simulate {
        /// Statute file(s)
        #[arg(short, long)]
        input: Vec<String>,

        /// Population size
        #[arg(short, long, default_value = "1000")]
        population: usize,

        /// Output file for results
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Generate an audit report
    Audit {
        /// Input statute files
        #[arg(short, long)]
        input: Vec<String>,

        /// Output file for audit report
        #[arg(short, long)]
        output: String,

        /// Include complexity analysis
        #[arg(long)]
        with_complexity: bool,
    },

    /// Analyze complexity of statutes
    Complexity {
        /// Input statute files
        #[arg(short, long)]
        input: Vec<String>,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Port a statute to another jurisdiction
    Port {
        /// Input statute file
        #[arg(short, long)]
        input: String,

        /// Target jurisdiction code (e.g., "JP", "US-CA", "DE")
        #[arg(short, long)]
        target: String,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// Output format
        #[arg(long, default_value = "json")]
        port_format: PortFormat,
    },

    /// Import from external legal DSL format (Catala, Stipula, L4, Akoma Ntoso)
    Import {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Source format (auto-detected if not specified)
        #[arg(long)]
        from: Option<LegalDslFormat>,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// Output format
        #[arg(long, default_value = "json")]
        import_output: ImportOutputFormat,
    },

    /// Convert between legal DSL formats
    Convert {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Source format (auto-detected if not specified)
        #[arg(long)]
        from: Option<LegalDslFormat>,

        /// Target format
        #[arg(long)]
        to: LegalDslFormat,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Export statute to Linked Open Data format (RDF/TTL/JSON-LD)
    Lod {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// RDF output format
        #[arg(long, default_value = "turtle")]
        rdf_format: RdfOutputFormat,

        /// Base URI for generated resources
        #[arg(long, default_value = "https://example.org/legalis/")]
        base_uri: String,
    },

    /// Format (pretty-print) a DSL file
    Format {
        /// Input file path
        #[arg(short, long)]
        input: String,

        /// Output file (defaults to stdout, use --inplace to modify in place)
        #[arg(short, long)]
        output: Option<String>,

        /// Modify the file in place
        #[arg(long)]
        inplace: bool,

        /// Output style (default, compact, verbose)
        #[arg(long, default_value = "default")]
        style: FormatStyle,

        /// Dry run (show what would be written without actually writing)
        #[arg(long)]
        dry_run: bool,
    },

    /// Lint DSL files for style and best practices
    Lint {
        /// Input file(s) to lint
        #[arg(short, long)]
        input: Vec<String>,

        /// Fix auto-fixable issues
        #[arg(long)]
        fix: bool,

        /// Fail on warnings
        #[arg(long)]
        strict: bool,
    },

    /// Watch files for changes and re-run commands
    Watch {
        /// Input file(s) to watch
        #[arg(short, long)]
        input: Vec<String>,

        /// Command to run on changes (verify, lint, test)
        #[arg(short, long, default_value = "verify")]
        command: WatchCommand,
    },

    /// Test statutes with test cases
    Test {
        /// Input statute file(s)
        #[arg(short, long)]
        input: Vec<String>,

        /// Test specification file
        #[arg(short, long)]
        tests: String,

        /// Verbose test output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Create a new statute from a template
    New {
        /// Statute name/ID
        #[arg(short, long)]
        name: String,

        /// Statute template type
        #[arg(short, long, default_value = "basic")]
        template: StatuteTemplate,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Run diagnostics on the installation
    Doctor {
        /// Verbose diagnostic output
        #[arg(short, long)]
        verbose: bool,
    },

    /// Start an interactive REPL (Read-Eval-Print Loop)
    Repl {
        /// Load a file on startup
        #[arg(short, long)]
        load: Option<String>,

        /// Disable colored output
        #[arg(long)]
        no_color: bool,
    },

    /// Search for statutes in a registry
    Search {
        /// Registry directory path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        registry: String,

        /// Search query (statute ID, title, or text)
        #[arg(short, long)]
        query: String,

        /// Filter by jurisdiction
        #[arg(short, long)]
        jurisdiction: Option<String>,

        /// Filter by tags
        #[arg(short, long)]
        tags: Vec<String>,

        /// Maximum number of results
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Publish a statute to a registry
    Publish {
        /// Input statute file
        #[arg(short, long)]
        input: String,

        /// Registry directory path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        registry: String,

        /// Tags to associate with the statute
        #[arg(short, long)]
        tags: Vec<String>,

        /// Dry run (show what would be published without actually publishing)
        #[arg(long)]
        dry_run: bool,
    },

    /// Validate a legal DSL file for format compliance
    Validate {
        /// Input file(s) to validate
        #[arg(short, long)]
        input: Vec<String>,

        /// Format to validate against (auto-detected if not specified)
        #[arg(long)]
        format: Option<LegalDslFormat>,

        /// Strict validation mode
        #[arg(long)]
        strict: bool,
    },

    /// Install a statute from a registry
    Install {
        /// Statute ID to install
        #[arg(short, long)]
        statute_id: String,

        /// Registry directory path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        registry: String,

        /// Output directory for installed statute
        #[arg(short, long, default_value = "./statutes")]
        output: String,

        /// Force reinstall if already installed
        #[arg(long)]
        force: bool,
    },

    /// List installed statutes
    List {
        /// Directory containing installed statutes
        #[arg(short, long, default_value = "./statutes")]
        directory: String,

        /// Show detailed information
        #[arg(long)]
        verbose: bool,
    },

    /// Add a dependency statute
    Add {
        /// Statute ID to add as dependency
        #[arg(short, long)]
        statute_id: String,

        /// Registry directory path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        registry: String,

        /// Project configuration file to update
        #[arg(long, default_value = "legalis.yaml")]
        config: String,
    },

    /// Update installed statutes to latest versions
    Update {
        /// Specific statute ID to update (updates all if not specified)
        #[arg(short, long)]
        statute_id: Option<String>,

        /// Registry directory path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        registry: String,

        /// Check for updates without installing
        #[arg(long)]
        dry_run: bool,
    },

    /// Clean cache and temporary files
    Clean {
        /// Remove all cached data
        #[arg(long)]
        all: bool,

        /// Remove compilation cache
        #[arg(long)]
        cache: bool,

        /// Remove temporary files
        #[arg(long)]
        temp: bool,

        /// Show what would be deleted without actually deleting
        #[arg(long)]
        dry_run: bool,
    },

    /// Check for outdated statutes
    Outdated {
        /// Directory containing installed statutes
        #[arg(short, long, default_value = "./statutes")]
        directory: String,

        /// Registry directory path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        registry: String,

        /// Show all statutes, not just outdated ones
        #[arg(long)]
        all: bool,
    },

    /// Uninstall a statute
    Uninstall {
        /// Statute ID to uninstall
        #[arg(short, long)]
        statute_id: String,

        /// Directory containing installed statutes
        #[arg(short, long, default_value = "./statutes")]
        directory: String,

        /// Remove without confirmation
        #[arg(long)]
        force: bool,

        /// Dry run (show what would be removed without actually removing)
        #[arg(long)]
        dry_run: bool,
    },
}

/// Statute template options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum StatuteTemplate {
    /// Basic statute with age condition
    #[default]
    Basic,
    /// Income-based statute
    Income,
    /// Geographic/regional statute
    Geographic,
    /// Time-based statute with effective dates
    Temporal,
    /// Complex statute with multiple conditions
    Complex,
}

/// Watch command options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum WatchCommand {
    /// Run verification
    #[default]
    Verify,
    /// Run linter
    Lint,
    /// Run tests
    Test,
    /// Run formatting
    Format,
}

/// Port output format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum PortFormat {
    /// JSON format
    #[default]
    Json,
    /// YAML format
    Yaml,
    /// Report format showing compatibility issues
    Report,
}

/// Diff output format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum DiffFormat {
    /// Human-readable text format
    #[default]
    Text,
    /// JSON format
    Json,
    /// Markdown format
    Markdown,
}

/// Visualization format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum VizFormat {
    /// GraphViz DOT format
    Dot,
    /// Mermaid diagram format
    #[default]
    Mermaid,
    /// ASCII tree format (terminal-friendly)
    Ascii,
    /// ASCII box format (terminal-friendly)
    Box,
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

/// Legal DSL format options for interop.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum LegalDslFormat {
    /// Catala (Inria, France)
    Catala,
    /// Stipula (University of Bologna)
    Stipula,
    /// L4 / SLL (Singapore)
    L4,
    /// Akoma Ntoso XML (OASIS)
    AkomaNtoso,
    /// Native Legalis DSL
    Legalis,
}

impl From<LegalDslFormat> for legalis_interop::LegalFormat {
    fn from(f: LegalDslFormat) -> Self {
        match f {
            LegalDslFormat::Catala => legalis_interop::LegalFormat::Catala,
            LegalDslFormat::Stipula => legalis_interop::LegalFormat::Stipula,
            LegalDslFormat::L4 => legalis_interop::LegalFormat::L4,
            LegalDslFormat::AkomaNtoso => legalis_interop::LegalFormat::AkomaNtoso,
            LegalDslFormat::Legalis => legalis_interop::LegalFormat::Legalis,
        }
    }
}

/// Import output format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum ImportOutputFormat {
    /// JSON format
    #[default]
    Json,
    /// YAML format
    Yaml,
    /// Native Legalis DSL format
    Legalis,
}

/// RDF output format options for LOD export.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum RdfOutputFormat {
    /// Turtle format (TTL) - human-readable RDF
    #[default]
    Turtle,
    /// N-Triples format - line-based RDF
    NTriples,
    /// RDF/XML format
    RdfXml,
    /// JSON-LD format - JSON-based RDF
    JsonLd,
}

impl From<RdfOutputFormat> for legalis_lod::RdfFormat {
    fn from(f: RdfOutputFormat) -> Self {
        match f {
            RdfOutputFormat::Turtle => legalis_lod::RdfFormat::Turtle,
            RdfOutputFormat::NTriples => legalis_lod::RdfFormat::NTriples,
            RdfOutputFormat::RdfXml => legalis_lod::RdfFormat::RdfXml,
            RdfOutputFormat::JsonLd => legalis_lod::RdfFormat::JsonLd,
        }
    }
}

/// Format style options for DSL pretty-printing.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum FormatStyle {
    /// Default formatting (4-space indent)
    #[default]
    Default,
    /// Compact formatting (2-space indent, no comments)
    Compact,
    /// Verbose formatting (includes comments, wide lines)
    Verbose,
}

impl From<FormatStyle> for legalis_dsl::PrinterConfig {
    fn from(style: FormatStyle) -> Self {
        match style {
            FormatStyle::Default => legalis_dsl::PrinterConfig::default(),
            FormatStyle::Compact => legalis_dsl::PrinterConfig::compact(),
            FormatStyle::Verbose => legalis_dsl::PrinterConfig::verbose(),
        }
    }
}

/// Generates shell completions and writes them to stdout.
pub fn generate_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "legalis", &mut std::io::stdout());
}
