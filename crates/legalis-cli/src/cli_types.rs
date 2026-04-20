//! CLI operation and value enum types.

use crate::{cloud, plugin, tutorial};
use clap::Subcommand;

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

/// Team operation types.
#[derive(Subcommand)]
pub enum TeamOperation {
    /// Create a new team workspace
    CreateWorkspace {
        /// Workspace name
        #[arg(short, long)]
        name: String,

        /// Workspace description
        #[arg(short, long)]
        description: Option<String>,

        /// Team members (comma-separated usernames)
        #[arg(short, long)]
        members: Option<String>,

        /// Output file for workspace config (defaults to workspace.toml)
        #[arg(short, long)]
        output: Option<String>,
    },

    /// List team workspaces
    ListWorkspaces {
        /// Show verbose information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Join a team workspace
    JoinWorkspace {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// Invitation token
        #[arg(short, long)]
        token: Option<String>,
    },

    /// Leave a team workspace
    LeaveWorkspace {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// Sync command history with team
    SyncHistory {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// Direction (push, pull, both)
        #[arg(short, long, default_value = "both")]
        direction: SyncDirection,

        /// Dry run (show what would be synced)
        #[arg(long)]
        dry_run: bool,
    },

    /// Show shared command history
    ShowHistory {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// Number of commands to show
        #[arg(short, long, default_value = "20")]
        limit: usize,

        /// Filter by user
        #[arg(short, long)]
        user: Option<String>,
    },

    /// Start a collaborative session
    StartSession {
        /// Session name
        #[arg(short, long)]
        name: String,

        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// Session description
        #[arg(short, long)]
        description: Option<String>,

        /// Maximum number of participants
        #[arg(short, long)]
        max_participants: Option<usize>,
    },

    /// Join a collaborative session
    JoinSession {
        /// Session ID or name
        #[arg(short, long)]
        session: String,

        /// Read-only mode
        #[arg(long)]
        readonly: bool,
    },

    /// Leave a collaborative session
    LeaveSession {
        /// Session ID or name
        #[arg(short, long)]
        session: String,
    },

    /// List active collaborative sessions
    ListSessions {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: Option<String>,

        /// Show all sessions (including inactive)
        #[arg(short, long)]
        all: bool,
    },

    /// Send a notification to team members
    Notify {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// Message to send
        #[arg(short, long)]
        message: String,

        /// Specific users to notify (comma-separated)
        #[arg(short, long)]
        users: Option<String>,

        /// Notification priority (low, normal, high)
        #[arg(short, long, default_value = "normal")]
        priority: NotificationPriority,
    },

    /// List team notifications
    ListNotifications {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: Option<String>,

        /// Show only unread notifications
        #[arg(short, long)]
        unread: bool,

        /// Number of notifications to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },

    /// Mark notifications as read
    MarkRead {
        /// Notification IDs (comma-separated)
        #[arg(short, long)]
        ids: String,
    },

    /// Manage role-based access control
    ManageAccess {
        /// Access control operation
        #[command(subcommand)]
        operation: AccessOperation,
    },
}

/// Access control operation types.
#[derive(Subcommand)]
pub enum AccessOperation {
    /// Grant access to a user
    Grant {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// User to grant access to
        #[arg(short, long)]
        user: String,

        /// Role to assign (owner, admin, write, read)
        #[arg(short, long)]
        role: TeamRole,
    },

    /// Revoke access from a user
    Revoke {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// User to revoke access from
        #[arg(short, long)]
        user: String,

        /// Skip confirmation prompt
        #[arg(short, long)]
        yes: bool,
    },

    /// List access permissions
    List {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// Show verbose information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Update user role
    Update {
        /// Workspace ID or name
        #[arg(short, long)]
        workspace: String,

        /// User to update
        #[arg(short, long)]
        user: String,

        /// New role to assign
        #[arg(short, long)]
        role: TeamRole,
    },
}

/// Team role options.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum TeamRole {
    /// Owner (full access, can delete workspace)
    Owner,
    /// Admin (can manage members and access)
    Admin,
    /// Write (can modify and execute commands)
    Write,
    /// Read (can view only)
    Read,
}

/// Notification priority options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum NotificationPriority {
    /// Low priority
    Low,
    /// Normal priority
    #[default]
    Normal,
    /// High priority
    High,
}

/// Performance profiling operation types.
#[derive(Subcommand)]
pub enum PerfOperation {
    /// Start a new profiling session
    Start {
        /// Session name (optional)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// Stop the current profiling session
    Stop {
        /// Generate report immediately
        #[arg(short, long)]
        report: bool,
    },

    /// Record a command execution
    Record {
        /// Command name
        #[arg(short, long)]
        command: String,

        /// Command arguments
        #[arg(short, long)]
        args: Vec<String>,

        /// Duration in milliseconds
        #[arg(short, long)]
        duration: u64,

        /// Memory used in bytes
        #[arg(short, long)]
        memory: Option<u64>,
    },

    /// List all profiling sessions
    List {
        /// Show verbose information
        #[arg(short, long)]
        verbose: bool,
    },

    /// Generate a performance report
    Report {
        /// Session ID (defaults to last session)
        #[arg(short, long)]
        session: Option<String>,

        /// Output file (defaults to stdout)
        #[arg(short, long)]
        output: Option<String>,

        /// Output format
        #[arg(short, long, default_value = "text")]
        format: PerfReportFormat,
    },

    /// Show performance statistics
    Stats {
        /// Session ID (defaults to last session)
        #[arg(short, long)]
        session: Option<String>,

        /// Command filter
        #[arg(short, long)]
        command: Option<String>,
    },

    /// Detect performance bottlenecks
    Bottlenecks {
        /// Session ID (defaults to last session)
        #[arg(short, long)]
        session: Option<String>,

        /// Minimum severity (low, medium, high, critical)
        #[arg(short, long)]
        min_severity: Option<PerfSeverity>,
    },

    /// Get optimization suggestions
    Optimize {
        /// Session ID (defaults to last session)
        #[arg(short, long)]
        session: Option<String>,

        /// Minimum impact (low, medium, high)
        #[arg(short, long)]
        min_impact: Option<PerfImpact>,
    },

    /// Enable global performance profiling
    Enable,

    /// Disable global performance profiling
    Disable,

    /// Show profiling status
    Status,
}

/// Performance report format options.
#[derive(Clone, Debug, Default, clap::ValueEnum)]
pub enum PerfReportFormat {
    /// Text format
    #[default]
    Text,
    /// JSON format
    Json,
    /// HTML format
    Html,
    /// Markdown format
    Markdown,
}

/// Performance severity filter.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum PerfSeverity {
    /// Low severity
    Low,
    /// Medium severity
    Medium,
    /// High severity
    High,
    /// Critical severity
    Critical,
}

/// Performance impact filter.
#[derive(Clone, Debug, clap::ValueEnum)]
pub enum PerfImpact {
    /// Low impact
    Low,
    /// Medium impact
    Medium,
    /// High impact
    High,
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
