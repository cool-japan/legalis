//! Legalis-CLI: Command-line interface for Legalis-RS.
//!
//! This crate provides a CLI tool for:
//! - Parsing and validating legal DSL files
//! - Running verification checks
//! - Generating visualizations
//! - Exporting to various formats

pub mod ai;
pub mod batch;
pub mod cache;
pub mod cloud;
pub mod commands;
pub mod config;
pub mod debug;
pub mod error_suggestions;
pub mod interactive;
pub mod parallel;
pub mod plugin;
pub mod profile;
pub mod progress;
pub mod scripting;
pub mod theme;
pub mod tui;
pub mod tutorial;
pub mod workflow;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{Shell, generate};
use clap_mangen::Man;

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

    /// Interactive mode (guided input)
    #[arg(short, long)]
    pub interactive: bool,

    /// Color theme (default, dark, light, monokai, solarized)
    #[arg(long, default_value = "default")]
    pub theme: ColorTheme,

    /// Disable emoji in output
    #[arg(long)]
    pub no_emoji: bool,

    /// Terminal width for output formatting (auto-detected if not specified)
    #[arg(long)]
    pub width: Option<usize>,

    /// Enable pager for long outputs
    #[arg(long)]
    pub pager: bool,

    /// Structured logging mode (json, logfmt)
    #[arg(long)]
    pub log_format: Option<LogFormat>,

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

/// Color theme options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum ColorTheme {
    /// Default color scheme
    #[default]
    Default,
    /// Dark background optimized
    Dark,
    /// Light background optimized
    Light,
    /// Monokai color scheme
    Monokai,
    /// Solarized color scheme
    Solarized,
    /// No colors (plain text)
    None,
}

/// Structured logging format options.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum LogFormat {
    /// JSON structured logging
    Json,
    /// Logfmt key=value format
    Logfmt,
    /// Compact format for machines
    Compact,
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

    /// Generate man pages
    ManPage {
        /// Output directory for man pages (generates all if specified, otherwise outputs to stdout)
        #[arg(short, long)]
        output: Option<String>,
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

    /// Interactive tutorials for learning Legalis
    Tutorial {
        /// Specific tutorial topic (if not specified, shows selection menu)
        #[arg(short, long)]
        topic: Option<TutorialTopicArg>,
    },

    /// Explain a statute in natural language
    Explain {
        /// Input statute file
        #[arg(short, long)]
        input: String,

        /// Detail level (basic, detailed, verbose)
        #[arg(long, default_value = "detailed")]
        detail: ExplainDetail,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Trace condition evaluation path
    Trace {
        /// Input statute file
        #[arg(short, long)]
        input: String,

        /// Test case file with input variables
        #[arg(short, long)]
        test_case: String,

        /// Output format
        #[arg(long, default_value = "text")]
        trace_format: TraceFormat,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Benchmark verification or simulation performance
    Benchmark {
        /// Input statute file(s)
        #[arg(short, long)]
        input: Vec<String>,

        /// Benchmark type (verify, simulate, all)
        #[arg(long, default_value = "all")]
        bench_type: BenchmarkType,

        /// Number of iterations
        #[arg(long, default_value = "100")]
        iterations: usize,

        /// Population size for simulation benchmarks
        #[arg(short, long, default_value = "1000")]
        population: usize,

        /// Output file for results (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Migrate statutes between versions
    Migrate {
        /// Input statute file
        #[arg(short, long)]
        input: String,

        /// Source DSL version
        #[arg(long)]
        from_version: String,

        /// Target DSL version
        #[arg(long)]
        to_version: String,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// Dry run (show migration plan without executing)
        #[arg(long)]
        dry_run: bool,
    },

    /// Generate dependency graphs
    Graph {
        /// Input statute file(s) or directory
        #[arg(short, long)]
        input: Vec<String>,

        /// Graph type (dependency, reference, call)
        #[arg(long, default_value = "dependency")]
        graph_type: GraphType,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Graph format (dot, mermaid, json)
        #[arg(long, default_value = "dot")]
        graph_format: GraphFormat,
    },

    /// Launch interactive statute builder wizard
    BuilderWizard {
        /// Skip the wizard and show help
        #[arg(long)]
        help_only: bool,
    },

    /// Launch interactive diff viewer with accept/reject
    DiffViewer {
        /// First file to compare
        #[arg(short, long)]
        old: String,

        /// Second file to compare
        #[arg(short, long)]
        new: String,
    },

    /// Launch interactive simulation parameter tuning
    SimTune {
        /// Input statute file(s)
        #[arg(short, long)]
        input: Vec<String>,
    },

    /// Resolve statute conflicts interactively
    ResolveConflicts {
        /// Statute files with conflicts
        #[arg(short, long)]
        input: Vec<String>,
    },

    /// Browse registry with TUI dashboard
    RegistryBrowser {
        /// Registry directory path (defaults to current directory)
        #[arg(short, long, default_value = ".")]
        registry: String,

        /// Start in search mode
        #[arg(long)]
        search: bool,
    },

    /// Execute batch operations on multiple statutes
    Batch {
        /// Batch operation to perform
        #[command(subcommand)]
        operation: BatchOperation,
    },

    /// Profile CPU and memory usage
    Profile {
        /// Input statute file(s)
        #[arg(short, long)]
        input: Vec<String>,

        /// Profile type (cpu, memory, all)
        #[arg(long, default_value = "all")]
        profile_type: ProfileType,

        /// Number of iterations for profiling
        #[arg(long, default_value = "100")]
        iterations: usize,

        /// Output file for profile results (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// Generate flamegraph (requires flamegraph to be installed)
        #[arg(long)]
        flamegraph: bool,

        /// Output directory for flamegraph files (Linux only)
        #[cfg(target_os = "linux")]
        #[arg(long, default_value = "./profiles")]
        flamegraph_dir: String,
    },

    /// Debug step-through evaluation
    Debug {
        /// Input statute file
        #[arg(short, long)]
        input: String,

        /// Test case file with input variables
        #[arg(short, long)]
        test_case: String,

        /// Enable interactive mode (step through evaluation)
        #[arg(long)]
        interactive: bool,

        /// Show memory allocations
        #[arg(long)]
        show_memory: bool,

        /// Show timing breakdown
        #[arg(long)]
        show_timing: bool,

        /// Output file for debug trace (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Registry operations (push, pull, diff, sync, login, logout)
    Registry {
        /// Registry operation to perform
        #[command(subcommand)]
        operation: RegistryOperation,
    },

    /// Plugin management (install, uninstall, list, enable, disable)
    Plugin {
        /// Plugin operation to perform
        #[command(subcommand)]
        operation: PluginOperation,
    },

    /// Configuration management (validate, diff, profiles)
    Config {
        /// Config operation to perform
        #[command(subcommand)]
        operation: ConfigOperation,
    },

    /// Script management and execution (Lua scripting)
    Script {
        /// Script operation to perform
        #[command(subcommand)]
        operation: ScriptOperation,
    },

    /// AI-powered CLI features
    Ai {
        /// AI operation to perform
        #[command(subcommand)]
        operation: AiOperation,
    },

    /// Launch interactive TUI dashboard
    Dashboard {
        /// Enable vim-style keyboard shortcuts
        #[arg(long)]
        vim_keys: bool,

        /// Disable mouse support
        #[arg(long)]
        no_mouse: bool,
    },

    /// Workflow automation operations
    Workflow {
        /// Workflow operation to perform
        #[command(subcommand)]
        operation: WorkflowOperation,
    },

    /// Cloud integration operations (AWS, Azure, GCP)
    Cloud {
        /// Cloud operation to perform
        #[command(subcommand)]
        operation: CloudOperation,
    },
}

/// Batch operation types.
#[derive(Subcommand)]
pub enum BatchOperation {
    /// Verify multiple statute files in parallel
    Verify {
        /// Input directory or file pattern (e.g., "statutes/*.ldsl")
        #[arg(short, long)]
        input: String,

        /// Fail on warnings
        #[arg(long)]
        strict: bool,

        /// Number of parallel workers (defaults to CPU count)
        #[arg(short, long)]
        workers: Option<usize>,

        /// Resume from previous run (uses journal file)
        #[arg(long)]
        resume: bool,

        /// Journal file for tracking progress
        #[arg(long, default_value = ".batch_journal.json")]
        journal: String,
    },

    /// Format multiple statute files in parallel
    Format {
        /// Input directory or file pattern
        #[arg(short, long)]
        input: String,

        /// Format style
        #[arg(long, default_value = "default")]
        style: FormatStyle,

        /// Modify files in place
        #[arg(long)]
        inplace: bool,

        /// Number of parallel workers
        #[arg(short, long)]
        workers: Option<usize>,

        /// Resume from previous run
        #[arg(long)]
        resume: bool,

        /// Journal file for tracking progress
        #[arg(long, default_value = ".batch_journal.json")]
        journal: String,
    },

    /// Lint multiple statute files in parallel
    Lint {
        /// Input directory or file pattern
        #[arg(short, long)]
        input: String,

        /// Fix auto-fixable issues
        #[arg(long)]
        fix: bool,

        /// Fail on warnings
        #[arg(long)]
        strict: bool,

        /// Number of parallel workers
        #[arg(short, long)]
        workers: Option<usize>,

        /// Resume from previous run
        #[arg(long)]
        resume: bool,

        /// Journal file for tracking progress
        #[arg(long, default_value = ".batch_journal.json")]
        journal: String,
    },

    /// Export multiple statutes to a different format
    Export {
        /// Input directory or file pattern
        #[arg(short, long)]
        input: String,

        /// Output directory
        #[arg(short, long)]
        output: String,

        /// Export format
        #[arg(long)]
        export_format: ExportFormat,

        /// Number of parallel workers
        #[arg(short, long)]
        workers: Option<usize>,

        /// Resume from previous run
        #[arg(long)]
        resume: bool,

        /// Journal file for tracking progress
        #[arg(long, default_value = ".batch_journal.json")]
        journal: String,
    },
}

/// Registry operation types.
#[derive(Subcommand)]
pub enum RegistryOperation {
    /// Push a statute to a remote registry
    Push {
        /// Input statute file
        #[arg(short, long)]
        input: String,

        /// Registry URL (defaults to configured registry)
        #[arg(short, long)]
        registry: Option<String>,

        /// Tags to associate with the statute
        #[arg(short, long)]
        tags: Vec<String>,

        /// Visibility (public, private)
        #[arg(long, default_value = "public")]
        visibility: RegistryVisibility,

        /// Dry run (show what would be pushed without actually pushing)
        #[arg(long)]
        dry_run: bool,

        /// Force push (overwrite existing statute)
        #[arg(long)]
        force: bool,
    },

    /// Pull a statute from a remote registry
    Pull {
        /// Statute ID to pull
        #[arg(short, long)]
        statute_id: String,

        /// Registry URL (defaults to configured registry)
        #[arg(short, long)]
        registry: Option<String>,

        /// Output directory for pulled statute
        #[arg(short, long, default_value = "./statutes")]
        output: String,

        /// Specific version to pull (defaults to latest)
        #[arg(short, long)]
        version: Option<String>,

        /// Force pull (overwrite existing local statute)
        #[arg(long)]
        force: bool,
    },

    /// Compare local statute with remote registry version
    Diff {
        /// Local statute file
        #[arg(short, long)]
        local: String,

        /// Statute ID in registry (defaults to ID from local file)
        #[arg(short, long)]
        statute_id: Option<String>,

        /// Registry URL (defaults to configured registry)
        #[arg(short, long)]
        registry: Option<String>,

        /// Output format
        #[arg(long, default_value = "text")]
        diff_format: DiffFormat,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// Synchronize local statutes with registry
    Sync {
        /// Directory containing local statutes
        #[arg(short, long, default_value = "./statutes")]
        directory: String,

        /// Registry URL (defaults to configured registry)
        #[arg(short, long)]
        registry: Option<String>,

        /// Sync direction (pull, push, both)
        #[arg(long, default_value = "pull")]
        direction: SyncDirection,

        /// Conflict resolution strategy (local, remote, ask)
        #[arg(long, default_value = "ask")]
        conflict: ConflictResolution,

        /// Dry run (show what would be synced without actually syncing)
        #[arg(long)]
        dry_run: bool,
    },

    /// Log in to a registry
    Login {
        /// Registry URL
        #[arg(short, long)]
        registry: String,

        /// Username (will prompt if not provided)
        #[arg(short, long)]
        username: Option<String>,

        /// Password (will prompt securely if not provided)
        #[arg(short, long)]
        password: Option<String>,

        /// API token (alternative to username/password)
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Log out from a registry
    Logout {
        /// Registry URL (logs out from all registries if not specified)
        #[arg(short, long)]
        registry: Option<String>,

        /// Clear all stored credentials
        #[arg(long)]
        all: bool,
    },
}

/// Plugin operation types.
#[derive(Subcommand)]
pub enum PluginOperation {
    /// Install a plugin from a directory or archive
    Install {
        /// Plugin source directory or archive path
        #[arg(short, long)]
        source: String,

        /// Force reinstall if already installed
        #[arg(long)]
        force: bool,
    },

    /// Uninstall a plugin by name
    Uninstall {
        /// Plugin name to uninstall
        #[arg(short, long)]
        name: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// List all installed plugins
    List {
        /// Show detailed plugin information
        #[arg(short, long)]
        verbose: bool,

        /// Filter by plugin type (command, hook, formatter, linter, extension)
        #[arg(short, long)]
        plugin_type: Option<PluginTypeFilter>,
    },

    /// Show detailed information about a plugin
    Info {
        /// Plugin name
        #[arg(short, long)]
        name: String,
    },

    /// Enable a plugin
    Enable {
        /// Plugin name to enable
        #[arg(short, long)]
        name: String,
    },

    /// Disable a plugin
    Disable {
        /// Plugin name to disable
        #[arg(short, long)]
        name: String,
    },

    /// Update a plugin to the latest version
    Update {
        /// Plugin name to update (updates all if not specified)
        #[arg(short, long)]
        name: Option<String>,
    },
}

/// Plugin type filter options.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum PluginTypeFilter {
    /// Command plugins
    Command,
    /// Hook plugins
    Hook,
    /// Formatter plugins
    Formatter,
    /// Linter plugins
    Linter,
    /// Extension plugins
    Extension,
}

impl From<PluginTypeFilter> for plugin::PluginType {
    fn from(f: PluginTypeFilter) -> Self {
        match f {
            PluginTypeFilter::Command => plugin::PluginType::Command,
            PluginTypeFilter::Hook => plugin::PluginType::Hook,
            PluginTypeFilter::Formatter => plugin::PluginType::Formatter,
            PluginTypeFilter::Linter => plugin::PluginType::Linter,
            PluginTypeFilter::Extension => plugin::PluginType::Extension,
        }
    }
}

/// Config operation types.
#[derive(Subcommand)]
pub enum ConfigOperation {
    /// Validate the current configuration
    Validate {
        /// Configuration file path (defaults to current config)
        #[arg(short, long)]
        config: Option<String>,

        /// Show detailed validation information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show differences between two configurations
    Diff {
        /// First configuration file
        #[arg(short, long)]
        config1: String,

        /// Second configuration file or profile name
        #[arg(short = '2', long)]
        config2: String,

        /// Treat config2 as a profile name instead of a file
        #[arg(long)]
        profile: bool,
    },

    /// List all available profiles
    Profiles {
        /// Configuration file path (defaults to current config)
        #[arg(short, long)]
        config: Option<String>,
    },

    /// Activate a profile
    Activate {
        /// Profile name to activate
        #[arg(short, long)]
        profile: String,

        /// Configuration file to update
        #[arg(short, long)]
        config: Option<String>,
    },

    /// Show the current configuration
    Show {
        /// Configuration file path (defaults to current config)
        #[arg(short, long)]
        config: Option<String>,

        /// Apply a profile before showing
        #[arg(short, long)]
        profile: Option<String>,

        /// Output format (toml, json, yaml)
        #[arg(short, long, default_value = "toml")]
        format: ConfigShowFormat,
    },

    /// Initialize user-level configuration
    Init {
        /// Force overwrite existing configuration
        #[arg(long)]
        force: bool,
    },
}

/// Config show format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum ConfigShowFormat {
    /// TOML format
    #[default]
    Toml,
    /// JSON format
    Json,
    /// YAML format
    Yaml,
}

/// Script operation types.
#[derive(Subcommand)]
pub enum ScriptOperation {
    /// Execute a Lua script
    Run {
        /// Script name or path to script file
        #[arg(short, long)]
        script: String,

        /// Arguments to pass to the script
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// Enable debug mode
        #[arg(short, long)]
        debug: bool,
    },

    /// List all available scripts
    List {
        /// Show detailed script information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show information about a script
    Info {
        /// Script name
        #[arg(short, long)]
        name: String,
    },

    /// Install a script from a directory
    Install {
        /// Script source directory
        #[arg(short, long)]
        source: String,
    },

    /// Uninstall a script
    Uninstall {
        /// Script name to uninstall
        #[arg(short, long)]
        name: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Create a new script from a template
    New {
        /// Script name
        #[arg(short, long)]
        name: String,

        /// Script template type
        #[arg(short, long, default_value = "basic")]
        template: ScriptTemplate,

        /// Output directory (defaults to current directory)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// List built-in script library
    Builtin {
        /// Show script code
        #[arg(short, long)]
        show_code: bool,
    },

    /// Validate a script without executing it
    Validate {
        /// Script file path
        #[arg(short, long)]
        script: String,
    },
}

/// AI operation types.
#[derive(Subcommand)]
pub enum AiOperation {
    /// Parse natural language command into CLI syntax
    Parse {
        /// Natural language input (e.g., "verify my statute file")
        #[arg(trailing_var_arg = true)]
        input: Vec<String>,
    },

    /// Recognize command intent from natural language
    Intent {
        /// Natural language query
        #[arg(trailing_var_arg = true)]
        query: Vec<String>,
    },

    /// Get AI-powered assistance for a command
    Assist {
        /// Help query or topic
        #[arg(trailing_var_arg = true)]
        query: Vec<String>,
    },

    /// Get suggested next commands based on history
    Suggest {
        /// Previous command (leave empty for general suggestions)
        #[arg(short, long)]
        previous: Option<String>,
    },

    /// Get autocomplete suggestions
    Complete {
        /// Partial command input
        #[arg(trailing_var_arg = true)]
        input: Vec<String>,
    },
}

/// Workflow operation types.
#[derive(Subcommand)]
pub enum WorkflowOperation {
    /// Execute a workflow from a definition file
    Run {
        /// Workflow file path (YAML format)
        #[arg(short, long)]
        file: String,

        /// Override workflow variables (key=value format)
        #[arg(short, long)]
        vars: Vec<String>,

        /// Dry run (show what would be executed without actually executing)
        #[arg(long)]
        dry_run: bool,

        /// Continue workflow execution even if tasks fail
        #[arg(long)]
        continue_on_error: bool,
    },

    /// List all available workflow templates
    ListTemplates {
        /// Show detailed template information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Generate a new workflow from a template
    New {
        /// Template name (see list-templates for available templates)
        #[arg(short, long)]
        template: String,

        /// Output file path
        #[arg(short, long)]
        output: String,

        /// Override template variables (key=value format)
        #[arg(short, long)]
        vars: Vec<String>,
    },

    /// Validate a workflow definition file
    Validate {
        /// Workflow file path
        #[arg(short, long)]
        file: String,

        /// Show detailed validation information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Show information about a workflow file
    Info {
        /// Workflow file path
        #[arg(short, long)]
        file: String,
    },
}

/// Cloud operation types.
#[derive(Subcommand)]
pub enum CloudOperation {
    /// Check status of cloud CLI tools
    Status,

    /// Execute AWS CLI command
    Aws {
        /// AWS CLI command arguments
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// AWS profile to use
        #[arg(long)]
        profile: Option<String>,

        /// AWS region to use
        #[arg(long)]
        region: Option<String>,
    },

    /// Execute Azure CLI command
    Azure {
        /// Azure CLI command arguments
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// Azure subscription ID
        #[arg(long)]
        subscription: Option<String>,
    },

    /// Execute GCP gcloud command
    Gcp {
        /// gcloud command arguments
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,

        /// GCP project ID
        #[arg(long)]
        project: Option<String>,

        /// GCP zone
        #[arg(long)]
        zone: Option<String>,
    },

    /// Provision cloud resources from definition file
    Provision {
        /// Resource definition file (YAML format)
        #[arg(short, long)]
        file: String,

        /// Cloud provider (aws, azure, gcp)
        #[arg(short, long)]
        provider: CloudProviderArg,

        /// Dry run (show what would be provisioned)
        #[arg(long)]
        dry_run: bool,
    },

    /// List cloud resources
    List {
        /// Cloud provider (aws, azure, gcp)
        #[arg(short, long)]
        provider: CloudProviderArg,

        /// Resource type (compute, storage, database, function, etc.)
        #[arg(short, long)]
        resource_type: String,

        /// AWS profile (for AWS only)
        #[arg(long)]
        profile: Option<String>,

        /// AWS region (for AWS only)
        #[arg(long)]
        region: Option<String>,

        /// Azure subscription (for Azure only)
        #[arg(long)]
        subscription: Option<String>,

        /// GCP project (for GCP only)
        #[arg(long)]
        project: Option<String>,
    },

    /// Configure cloud provider
    Configure {
        /// Cloud provider to configure
        #[arg(short, long)]
        provider: CloudProviderArg,

        /// Configuration in key=value format
        #[arg(short, long)]
        config: Vec<String>,
    },
}

/// Cloud provider argument for CLI.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum CloudProviderArg {
    /// Amazon Web Services
    Aws,
    /// Microsoft Azure
    Azure,
    /// Google Cloud Platform
    Gcp,
}

impl From<CloudProviderArg> for cloud::CloudProvider {
    fn from(arg: CloudProviderArg) -> Self {
        match arg {
            CloudProviderArg::Aws => cloud::CloudProvider::Aws,
            CloudProviderArg::Azure => cloud::CloudProvider::Azure,
            CloudProviderArg::Gcp => cloud::CloudProvider::Gcp,
        }
    }
}

/// Script template options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum ScriptTemplate {
    /// Basic script template
    #[default]
    Basic,
    /// Batch processing template
    Batch,
    /// Report generation template
    Report,
    /// Data transformation template
    Transform,
}

/// Tutorial topic argument for CLI.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum TutorialTopicArg {
    /// Introduction to Legalis
    Introduction,
    /// Parsing & validating DSL files
    Parsing,
    /// Creating statutes from templates
    Creating,
    /// Verification & testing
    Verification,
    /// Visualization techniques
    Visualization,
    /// Export formats & interoperability
    Exporting,
    /// Using the statute registry
    Registry,
    /// Advanced features
    Advanced,
}

impl From<TutorialTopicArg> for tutorial::TutorialTopic {
    fn from(arg: TutorialTopicArg) -> Self {
        match arg {
            TutorialTopicArg::Introduction => tutorial::TutorialTopic::Introduction,
            TutorialTopicArg::Parsing => tutorial::TutorialTopic::ParsingBasics,
            TutorialTopicArg::Creating => tutorial::TutorialTopic::CreatingStatutes,
            TutorialTopicArg::Verification => tutorial::TutorialTopic::Verification,
            TutorialTopicArg::Visualization => tutorial::TutorialTopic::Visualization,
            TutorialTopicArg::Exporting => tutorial::TutorialTopic::Exporting,
            TutorialTopicArg::Registry => tutorial::TutorialTopic::RegistryUsage,
            TutorialTopicArg::Advanced => tutorial::TutorialTopic::Advanced,
        }
    }
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

/// Explain detail level options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum ExplainDetail {
    /// Basic explanation (brief overview)
    Basic,
    /// Detailed explanation (default, includes conditions and outcomes)
    #[default]
    Detailed,
    /// Verbose explanation (full detail with examples)
    Verbose,
}

/// Trace output format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum TraceFormat {
    /// Human-readable text format
    #[default]
    Text,
    /// JSON format with full trace data
    Json,
    /// Tree diagram format (ASCII)
    Tree,
    /// Mermaid flowchart format
    Mermaid,
}

/// Benchmark type options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum BenchmarkType {
    /// Benchmark verification only
    Verify,
    /// Benchmark simulation only
    Simulate,
    /// Benchmark both verification and simulation
    #[default]
    All,
}

/// Graph type options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum GraphType {
    /// Dependency graph (statute dependencies)
    #[default]
    Dependency,
    /// Reference graph (cross-references between statutes)
    Reference,
    /// Call graph (condition evaluation flow)
    Call,
}

/// Graph output format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum GraphFormat {
    /// GraphViz DOT format
    #[default]
    Dot,
    /// Mermaid diagram format
    Mermaid,
    /// JSON format with graph data
    Json,
    /// SVG image format
    Svg,
}

/// Profile type options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum ProfileType {
    /// Profile CPU usage only
    Cpu,
    /// Profile memory usage only
    Memory,
    /// Profile both CPU and memory
    #[default]
    All,
}

/// Registry visibility options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum RegistryVisibility {
    /// Public visibility (anyone can view)
    #[default]
    Public,
    /// Private visibility (only authorized users)
    Private,
}

/// Sync direction options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum SyncDirection {
    /// Pull from registry to local
    #[default]
    Pull,
    /// Push from local to registry
    Push,
    /// Synchronize both ways
    Both,
}

/// Conflict resolution strategy.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum ConflictResolution {
    /// Keep local version
    Local,
    /// Use remote version
    Remote,
    /// Ask user for each conflict
    #[default]
    Ask,
}

/// Generates shell completions and writes them to stdout.
pub fn generate_completions(shell: Shell) {
    let mut cmd = Cli::command();
    generate(shell, &mut cmd, "legalis", &mut std::io::stdout());
}

/// Generates man page and writes it to stdout.
pub fn generate_man_page() -> std::io::Result<()> {
    let cmd = Cli::command();
    let man = Man::new(cmd);
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;
    std::io::Write::write_all(&mut std::io::stdout(), &buffer)?;
    Ok(())
}

/// Generates man pages for all subcommands to a directory.
pub fn generate_all_man_pages(output_dir: &std::path::Path) -> std::io::Result<()> {
    let cmd = Cli::command();

    // Create output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Generate main man page
    let man = Man::new(cmd.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer)?;
    let man_path = output_dir.join("legalis.1");
    std::fs::write(man_path, buffer)?;

    // Generate man pages for subcommands
    for subcmd in cmd.get_subcommands() {
        let name = subcmd.get_name();
        let subcmd_name = format!("legalis-{}", name);
        let man = Man::new(subcmd.clone()).title(&subcmd_name);
        let mut buffer = Vec::new();
        man.render(&mut buffer)?;
        let man_path = output_dir.join(format!("{}.1", subcmd_name));
        std::fs::write(man_path, buffer)?;
    }

    Ok(())
}
