mod auth;
mod client;
mod commands;
mod config;
mod formatter;
mod useragent;
mod util;
mod version;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "pup", version = version::VERSION, about = "Datadog API CLI (Rust)")]
struct Cli {
    /// Output format (json, table, yaml)
    #[arg(short, long, global = true, default_value = "json")]
    output: String,
    /// Auto-approve destructive operations
    #[arg(short = 'y', long = "yes", global = true)]
    yes: bool,
    /// Enable agent mode
    #[arg(long, global = true)]
    agent: bool,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage monitors
    Monitors {
        #[command(subcommand)]
        action: MonitorActions,
    },
    /// Search and analyze logs
    Logs {
        #[command(subcommand)]
        action: LogActions,
    },
    /// Manage incidents
    Incidents {
        #[command(subcommand)]
        action: IncidentActions,
    },
    /// Manage dashboards
    Dashboards {
        #[command(subcommand)]
        action: DashboardActions,
    },
    /// Query and manage metrics
    Metrics {
        #[command(subcommand)]
        action: MetricActions,
    },
    /// Manage SLOs
    Slos {
        #[command(subcommand)]
        action: SloActions,
    },
    /// Manage synthetics tests
    Synthetics {
        #[command(subcommand)]
        action: SyntheticsActions,
    },
    /// Manage events
    Events {
        #[command(subcommand)]
        action: EventActions,
    },
    /// Manage downtimes
    Downtime {
        #[command(subcommand)]
        action: DowntimeActions,
    },
    /// Manage host tags
    Tags {
        #[command(subcommand)]
        action: TagActions,
    },
    /// Manage users and roles
    Users {
        #[command(subcommand)]
        action: UserActions,
    },
    /// Manage infrastructure hosts
    Infrastructure {
        #[command(subcommand)]
        action: InfraActions,
    },
    /// Search audit logs
    #[command(name = "audit-logs")]
    AuditLogs {
        #[command(subcommand)]
        action: AuditLogActions,
    },
    /// Manage security monitoring
    Security {
        #[command(subcommand)]
        action: SecurityActions,
    },
    /// Manage organizations
    Organizations {
        #[command(subcommand)]
        action: OrgActions,
    },
    /// Manage cloud integrations
    Cloud {
        #[command(subcommand)]
        action: CloudActions,
    },
    /// Manage cases
    Cases {
        #[command(subcommand)]
        action: CaseActions,
    },
    /// Manage service catalog
    #[command(name = "service-catalog")]
    ServiceCatalog {
        #[command(subcommand)]
        action: ServiceCatalogActions,
    },
    /// Manage API keys
    #[command(name = "api-keys")]
    ApiKeys {
        #[command(subcommand)]
        action: ApiKeyActions,
    },
    /// Manage application keys
    #[command(name = "app-keys")]
    AppKeys {
        #[command(subcommand)]
        action: AppKeyActions,
    },
    /// Query usage data
    Usage {
        #[command(subcommand)]
        action: UsageActions,
    },
    /// Manage notebooks
    Notebooks {
        #[command(subcommand)]
        action: NotebookActions,
    },
    /// RUM (Real User Monitoring)
    Rum {
        #[command(subcommand)]
        action: RumActions,
    },
    /// CI/CD visibility
    Cicd {
        #[command(subcommand)]
        action: CicdActions,
    },
    /// Manage on-call teams
    #[command(name = "on-call")]
    OnCall {
        #[command(subcommand)]
        action: OnCallActions,
    },
    /// Fleet automation
    Fleet {
        #[command(subcommand)]
        action: FleetActions,
    },
    /// Data governance
    #[command(name = "data-governance")]
    DataGovernance {
        #[command(subcommand)]
        action: DataGovActions,
    },
    /// Error tracking
    #[command(name = "error-tracking")]
    ErrorTracking {
        #[command(subcommand)]
        action: ErrorTrackingActions,
    },
    /// Code coverage
    #[command(name = "code-coverage")]
    CodeCoverage {
        #[command(subcommand)]
        action: CodeCoverageActions,
    },
    /// HAMR (High Availability Multi-Region)
    Hamr {
        #[command(subcommand)]
        action: HamrActions,
    },
    /// Status pages
    #[command(name = "status-pages")]
    StatusPages {
        #[command(subcommand)]
        action: StatusPageActions,
    },
    /// Manage integrations (Jira, ServiceNow)
    Integrations {
        #[command(subcommand)]
        action: IntegrationActions,
    },
    /// Cloud cost management
    Cost {
        #[command(subcommand)]
        action: CostActions,
    },
    /// Miscellaneous (IP ranges)
    Misc {
        #[command(subcommand)]
        action: MiscActions,
    },
    /// APM (Application Performance Monitoring)
    Apm {
        #[command(subcommand)]
        action: ApmActions,
    },
    /// Manage investigations
    Investigations {
        #[command(subcommand)]
        action: InvestigationActions,
    },
    /// Network monitoring (placeholder)
    Network {
        #[command(subcommand)]
        action: NetworkActions,
    },
    /// Observability pipelines (placeholder)
    #[command(name = "obs-pipelines")]
    ObsPipelines {
        #[command(subcommand)]
        action: ObsPipelinesActions,
    },
    /// Scorecards (placeholder)
    Scorecards {
        #[command(subcommand)]
        action: ScorecardsActions,
    },
    /// Distributed traces (placeholder)
    Traces {
        #[command(subcommand)]
        action: TracesActions,
    },
    /// Agent management (placeholder)
    #[command(name = "agent")]
    Agent {
        #[command(subcommand)]
        action: AgentActions,
    },
    /// Manage command aliases
    Alias {
        #[command(subcommand)]
        action: AliasActions,
    },
    /// Product analytics
    #[command(name = "product-analytics")]
    ProductAnalytics {
        #[command(subcommand)]
        action: ProductAnalyticsActions,
    },
    /// Static analysis
    #[command(name = "static-analysis")]
    StaticAnalysis {
        #[command(subcommand)]
        action: StaticAnalysisActions,
    },
    /// Authentication (OAuth2)
    Auth {
        #[command(subcommand)]
        action: AuthActions,
    },
    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
    /// Show version information
    Version,
    /// Validate configuration
    Test,
}

// ---- Monitors ----
#[derive(Subcommand)]
enum MonitorActions {
    /// List monitors
    List {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        tags: Option<String>,
        #[arg(long, default_value_t = 200)]
        limit: i32,
    },
    /// Get monitor details
    Get { monitor_id: i64 },
    /// Create a monitor from JSON file
    Create { #[arg(long)] file: String },
    /// Update a monitor from JSON file
    Update { monitor_id: i64, #[arg(long)] file: String },
    /// Search monitors
    Search { #[arg(long)] query: Option<String> },
    /// Delete a monitor
    Delete { monitor_id: i64 },
}

// ---- Logs ----
#[derive(Subcommand)]
enum LogActions {
    /// Search logs (forces API key auth)
    Search {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// List logs (alias for search)
    List {
        #[arg(long, default_value = "*")]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// Query logs (alias for search)
    Query {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// Aggregate logs
    Aggregate {
        #[arg(long, default_value = "*")]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
    /// Manage log archives
    Archives {
        #[command(subcommand)]
        action: LogArchiveActions,
    },
    /// Manage custom destinations
    #[command(name = "custom-destinations")]
    CustomDestinations {
        #[command(subcommand)]
        action: LogCustomDestinationActions,
    },
    /// Manage log-based metrics
    Metrics {
        #[command(subcommand)]
        action: LogMetricActions,
    },
    /// Manage log restriction queries
    #[command(name = "restriction-queries")]
    RestrictionQueries {
        #[command(subcommand)]
        action: LogRestrictionQueryActions,
    },
}

#[derive(Subcommand)]
enum LogRestrictionQueryActions {
    /// List restriction queries
    List,
    /// Get restriction query details
    Get { query_id: String },
}

#[derive(Subcommand)]
enum LogArchiveActions {
    /// List log archives
    List,
    /// Get archive details
    Get { archive_id: String },
    /// Delete an archive
    Delete { archive_id: String },
}

#[derive(Subcommand)]
enum LogCustomDestinationActions {
    /// List custom destinations
    List,
    /// Get custom destination details
    Get { destination_id: String },
}

#[derive(Subcommand)]
enum LogMetricActions {
    /// List log-based metrics
    List,
    /// Get metric details
    Get { metric_id: String },
    /// Delete a log-based metric
    Delete { metric_id: String },
}

// ---- Incidents ----
#[derive(Subcommand)]
enum IncidentActions {
    /// List incidents (unstable)
    List {
        #[arg(long, default_value_t = 50)]
        limit: i64,
    },
    /// Get incident details
    Get { incident_id: String },
    /// Manage incident attachments
    Attachments {
        #[command(subcommand)]
        action: IncidentAttachmentActions,
    },
    /// Manage global incident settings
    Settings {
        #[command(subcommand)]
        action: IncidentSettingsActions,
    },
    /// Manage global incident handles
    Handles {
        #[command(subcommand)]
        action: IncidentHandleActions,
    },
    /// Manage postmortem templates
    #[command(name = "postmortem-templates")]
    PostmortemTemplates {
        #[command(subcommand)]
        action: IncidentPostmortemActions,
    },
}

#[derive(Subcommand)]
enum IncidentAttachmentActions {
    /// List incident attachments
    List { incident_id: String },
    /// Delete an incident attachment
    Delete {
        incident_id: String,
        attachment_id: String,
    },
}

#[derive(Subcommand)]
enum IncidentSettingsActions {
    /// Get global incident settings
    Get,
    /// Update global incident settings from JSON file
    Update {
        #[arg(long)]
        file: String,
    },
}

#[derive(Subcommand)]
enum IncidentHandleActions {
    /// List global incident handles
    List,
    /// Create a global incident handle from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a global incident handle from JSON file
    Update {
        #[arg(long)]
        file: String,
    },
    /// Delete a global incident handle
    Delete { handle_id: String },
}

#[derive(Subcommand)]
enum IncidentPostmortemActions {
    /// List postmortem templates
    List,
    /// Get postmortem template details
    Get { template_id: String },
    /// Create a postmortem template from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a postmortem template from JSON file
    Update {
        template_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a postmortem template
    Delete { template_id: String },
}

// ---- Dashboards ----
#[derive(Subcommand)]
enum DashboardActions {
    /// List all dashboards
    List,
    /// Get dashboard details
    Get { id: String },
    /// Create a dashboard from JSON file
    Create { #[arg(long)] file: String },
    /// Update a dashboard from JSON file
    Update { id: String, #[arg(long)] file: String },
    /// Delete a dashboard
    Delete { id: String },
}

// ---- Metrics ----
#[derive(Subcommand)]
enum MetricActions {
    /// List active metrics
    List {
        #[arg(long)]
        filter: Option<String>,
        #[arg(long, default_value = "1h")]
        from: String,
    },
    /// Query metrics (v1 API)
    Search {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
    /// Query metrics (v2 timeseries, alias)
    Query {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
    /// Submit custom metrics from JSON file
    Submit {
        #[arg(long)]
        file: String,
    },
    /// Manage metric metadata
    Metadata {
        #[command(subcommand)]
        action: MetricMetadataActions,
    },
    /// Manage metric tags
    Tags {
        #[command(subcommand)]
        action: MetricTagActions,
    },
}

#[derive(Subcommand)]
enum MetricTagActions {
    /// List tags for a metric
    List { metric_name: String },
}

#[derive(Subcommand)]
enum MetricMetadataActions {
    /// Get metric metadata
    Get { metric_name: String },
    /// Update metric metadata from JSON file
    Update {
        metric_name: String,
        #[arg(long)]
        file: String,
    },
}

// ---- SLOs ----
#[derive(Subcommand)]
enum SloActions {
    /// List all SLOs
    List,
    /// Get SLO details
    Get { id: String },
    /// Create an SLO from JSON file
    Create { #[arg(long)] file: String },
    /// Update an SLO from JSON file
    Update { id: String, #[arg(long)] file: String },
    /// Delete an SLO
    Delete { id: String },
    /// Get SLO status
    Status {
        id: String,
        #[arg(long)]
        from_ts: i64,
        #[arg(long)]
        to_ts: i64,
    },
}

// ---- Synthetics ----
#[derive(Subcommand)]
enum SyntheticsActions {
    /// Manage synthetic tests
    Tests {
        #[command(subcommand)]
        action: SyntheticsTestActions,
    },
    /// Manage synthetic locations
    Locations {
        #[command(subcommand)]
        action: SyntheticsLocationActions,
    },
    /// Manage synthetic suites
    Suites {
        #[command(subcommand)]
        action: SyntheticsSuiteActions,
    },
}

#[derive(Subcommand)]
enum SyntheticsTestActions {
    /// List all tests
    List,
    /// Get test details
    Get { public_id: String },
    /// Search tests
    Search {
        #[arg(long)]
        text: Option<String>,
        #[arg(long, default_value_t = 50)]
        count: i64,
        #[arg(long, default_value_t = 0)]
        start: i64,
    },
}

#[derive(Subcommand)]
enum SyntheticsLocationActions {
    /// List test locations
    List,
}

#[derive(Subcommand)]
enum SyntheticsSuiteActions {
    /// List synthetic suites
    List {
        #[arg(long)]
        query: Option<String>,
    },
    /// Get suite details
    Get { suite_id: String },
    /// Create a suite from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a suite from JSON file
    Update {
        suite_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete suites
    Delete {
        /// Suite IDs to delete
        suite_ids: Vec<String>,
    },
}

// ---- Events ----
#[derive(Subcommand)]
enum EventActions {
    /// List events (v1 API)
    List {
        #[arg(long, default_value_t = 0)]
        start: i64,
        #[arg(long, default_value_t = 0)]
        end: i64,
        #[arg(long)]
        tags: Option<String>,
    },
    /// Search events (v2 API, requires API keys)
    Search {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 100)]
        limit: i32,
    },
    /// Get event details
    Get { event_id: i64 },
}

// ---- Downtime ----
#[derive(Subcommand)]
enum DowntimeActions {
    /// List downtimes
    List,
    /// Get downtime details
    Get { id: String },
    /// Create a downtime from JSON file
    Create { #[arg(long)] file: String },
    /// Cancel a downtime
    Cancel { id: String },
}

// ---- Tags ----
#[derive(Subcommand)]
enum TagActions {
    /// List all host tags
    List,
    /// Get tags for a host
    Get { hostname: String },
    /// Add tags to a host
    Add { hostname: String, tags: Vec<String> },
    /// Update tags for a host
    Update { hostname: String, tags: Vec<String> },
    /// Delete all tags for a host
    Delete { hostname: String },
}

// ---- Users ----
#[derive(Subcommand)]
enum UserActions {
    /// List users
    List,
    /// Get user details
    Get { user_id: String },
    /// Manage roles
    Roles {
        #[command(subcommand)]
        action: UserRoleActions,
    },
}

#[derive(Subcommand)]
enum UserRoleActions {
    /// List roles
    List,
}

// ---- Infrastructure ----
#[derive(Subcommand)]
enum InfraActions {
    /// Manage hosts
    Hosts {
        #[command(subcommand)]
        action: InfraHostActions,
    },
}

#[derive(Subcommand)]
enum InfraHostActions {
    /// List hosts
    List {
        #[arg(long)]
        filter: Option<String>,
        #[arg(long, default_value = "status")]
        sort: String,
        #[arg(long, default_value_t = 100)]
        count: i64,
    },
    /// Get host details
    Get { hostname: String },
}

// ---- Audit Logs ----
#[derive(Subcommand)]
enum AuditLogActions {
    /// List audit logs
    List {
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 100)]
        limit: i32,
    },
    /// Search audit logs
    Search {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 100)]
        limit: i32,
    },
}

// ---- Security ----
#[derive(Subcommand)]
enum SecurityActions {
    /// Manage security rules
    Rules {
        #[command(subcommand)]
        action: SecurityRuleActions,
    },
    /// Search security signals
    Signals {
        #[command(subcommand)]
        action: SecuritySignalActions,
    },
    /// Search security findings
    Findings {
        #[command(subcommand)]
        action: SecurityFindingActions,
    },
    /// Manage content packs
    #[command(name = "content-packs")]
    ContentPacks {
        #[command(subcommand)]
        action: SecurityContentPackActions,
    },
    /// Manage entity risk scores
    #[command(name = "risk-scores")]
    RiskScores {
        #[command(subcommand)]
        action: SecurityRiskScoreActions,
    },
}

#[derive(Subcommand)]
enum SecurityRuleActions {
    /// List rules
    List,
    /// Get rule details
    Get { rule_id: String },
    /// Bulk export rules
    #[command(name = "bulk-export")]
    BulkExport {
        /// Rule IDs to export
        rule_ids: Vec<String>,
    },
}

#[derive(Subcommand)]
enum SecuritySignalActions {
    /// Search signals
    Search {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 100)]
        limit: i32,
    },
}

#[derive(Subcommand)]
enum SecurityFindingActions {
    /// Search findings
    Search {
        #[arg(long)]
        query: Option<String>,
        #[arg(long, default_value_t = 100)]
        limit: i64,
    },
}

#[derive(Subcommand)]
enum SecurityContentPackActions {
    /// List content packs
    List,
    /// Activate a content pack
    Activate { pack_id: String },
    /// Deactivate a content pack
    Deactivate { pack_id: String },
}

#[derive(Subcommand)]
enum SecurityRiskScoreActions {
    /// List entity risk scores
    List {
        #[arg(long)]
        query: Option<String>,
    },
}

// ---- Organizations ----
#[derive(Subcommand)]
enum OrgActions {
    /// List organizations
    List,
    /// Get current organization
    Get,
}

// ---- Cloud ----
#[derive(Subcommand)]
enum CloudActions {
    /// List AWS integrations
    Aws {
        #[command(subcommand)]
        action: CloudSubActions,
    },
    /// List GCP integrations
    Gcp {
        #[command(subcommand)]
        action: CloudSubActions,
    },
    /// List Azure integrations
    Azure {
        #[command(subcommand)]
        action: CloudSubActions,
    },
    /// OCI integration
    Oci {
        #[command(subcommand)]
        action: CloudOciActions,
    },
}

#[derive(Subcommand)]
enum CloudSubActions {
    /// List integrations
    List,
}

#[derive(Subcommand)]
enum CloudOciActions {
    /// Manage OCI tenancies
    Tenancies {
        #[command(subcommand)]
        action: CloudOciTenancyActions,
    },
    /// List OCI products
    Products {
        /// Product keys to query
        product_keys: String,
    },
}

#[derive(Subcommand)]
enum CloudOciTenancyActions {
    /// List OCI tenancies
    List,
    /// Get OCI tenancy details
    Get { tenancy_id: String },
    /// Create an OCI tenancy from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update an OCI tenancy from JSON file
    Update {
        tenancy_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete an OCI tenancy
    Delete { tenancy_id: String },
}

// ---- Cases ----
#[derive(Subcommand)]
enum CaseActions {
    /// Search cases
    Search {
        #[arg(long)]
        query: Option<String>,
        #[arg(long, default_value_t = 50)]
        page_size: i64,
    },
    /// Get case details
    Get { case_id: String },
    /// Create a case from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Archive a case
    Archive { case_id: String },
    /// Unarchive a case
    Unarchive { case_id: String },
    /// Assign a case to a user
    Assign {
        case_id: String,
        #[arg(long)]
        user_id: String,
    },
    /// Update case priority
    #[command(name = "update-priority")]
    UpdatePriority {
        case_id: String,
        #[arg(long)]
        priority: String,
    },
    /// Update case status
    #[command(name = "update-status")]
    UpdateStatus {
        case_id: String,
        #[arg(long)]
        status: String,
    },
    /// Manage projects
    Projects {
        #[command(subcommand)]
        action: CaseProjectActions,
    },
    /// Jira integration for cases
    Jira {
        #[command(subcommand)]
        action: CaseJiraActions,
    },
    /// ServiceNow integration for cases
    Servicenow {
        #[command(subcommand)]
        action: CaseServicenowActions,
    },
}

#[derive(Subcommand)]
enum CaseProjectActions {
    /// List projects
    List,
    /// Get project details
    Get { project_id: String },
    /// Create a project
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        key: String,
    },
    /// Delete a project
    Delete { project_id: String },
    /// Manage notification rules
    #[command(name = "notification-rules")]
    NotificationRules {
        #[command(subcommand)]
        action: CaseNotificationRuleActions,
    },
}

#[derive(Subcommand)]
enum CaseJiraActions {
    /// Create a Jira issue for a case
    #[command(name = "create-issue")]
    CreateIssue {
        case_id: String,
        #[arg(long)]
        file: String,
    },
    /// Link a Jira issue to a case
    Link {
        case_id: String,
        #[arg(long)]
        file: String,
    },
    /// Unlink a Jira issue from a case
    Unlink { case_id: String },
}

#[derive(Subcommand)]
enum CaseServicenowActions {
    /// Create a ServiceNow ticket for a case
    #[command(name = "create-ticket")]
    CreateTicket {
        case_id: String,
        #[arg(long)]
        file: String,
    },
}

#[derive(Subcommand)]
enum CaseNotificationRuleActions {
    /// List notification rules for a project
    List { project_id: String },
    /// Create a notification rule from JSON file
    Create {
        project_id: String,
        #[arg(long)]
        file: String,
    },
    /// Update a notification rule from JSON file
    Update {
        project_id: String,
        rule_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a notification rule
    Delete {
        project_id: String,
        rule_id: String,
    },
}

// ---- Service Catalog ----
#[derive(Subcommand)]
enum ServiceCatalogActions {
    /// List services
    List,
    /// Get service details
    Get { service_name: String },
}

// ---- API Keys ----
#[derive(Subcommand)]
enum ApiKeyActions {
    /// List API keys
    List,
    /// Get API key details
    Get { key_id: String },
    /// Create an API key
    Create { #[arg(long)] name: String },
    /// Delete an API key
    Delete { key_id: String },
}

// ---- App Keys ----
#[derive(Subcommand)]
enum AppKeyActions {
    /// List application keys
    List,
    /// Get application key details
    Get { key_id: String },
    /// Register an application key
    Register { key_id: String },
    /// Unregister an application key
    Unregister { key_id: String },
}

// ---- Usage ----
#[derive(Subcommand)]
enum UsageActions {
    /// Get usage summary
    Summary {
        #[arg(long, default_value = "30d")]
        start: String,
        #[arg(long)]
        end: Option<String>,
    },
    /// Get hourly usage
    Hourly {
        #[arg(long, default_value = "1d")]
        start: String,
        #[arg(long)]
        end: Option<String>,
    },
}

// ---- Notebooks ----
#[derive(Subcommand)]
enum NotebookActions {
    /// List notebooks
    List,
    /// Get notebook details
    Get { notebook_id: i64 },
    /// Create a notebook from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a notebook from JSON file
    Update {
        notebook_id: i64,
        #[arg(long)]
        file: String,
    },
    /// Delete a notebook
    Delete { notebook_id: i64 },
}

// ---- RUM ----
#[derive(Subcommand)]
enum RumActions {
    /// Manage RUM applications
    Apps {
        #[command(subcommand)]
        action: RumAppActions,
    },
    /// List RUM events
    Events {
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 100)]
        limit: i32,
    },
    /// Search RUM sessions
    Sessions {
        #[command(subcommand)]
        action: RumSessionActions,
    },
    /// Manage RUM metrics
    Metrics {
        #[command(subcommand)]
        action: RumMetricActions,
    },
    /// Manage RUM retention filters
    #[command(name = "retention-filters")]
    RetentionFilters {
        #[command(subcommand)]
        action: RumRetentionFilterActions,
    },
    /// Manage RUM replay playlists
    Playlists {
        #[command(subcommand)]
        action: RumPlaylistActions,
    },
    /// Query RUM heatmaps
    Heatmaps {
        #[command(subcommand)]
        action: RumHeatmapActions,
    },
}

#[derive(Subcommand)]
enum RumAppActions {
    /// List RUM apps (requires API keys)
    List,
    /// Get RUM app details (requires API keys)
    Get { app_id: String },
    /// Create a RUM app (requires API keys)
    Create {
        #[arg(long)]
        name: String,
        #[arg(long, name = "type")]
        app_type: Option<String>,
    },
    /// Update a RUM app (requires API keys)
    Update {
        app_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a RUM app (requires API keys)
    Delete { app_id: String },
}

#[derive(Subcommand)]
enum RumSessionActions {
    /// Search RUM sessions
    Search {
        #[arg(long)]
        query: Option<String>,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 100)]
        limit: i32,
    },
    /// List RUM sessions
    List {
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 100)]
        limit: i32,
    },
}

#[derive(Subcommand)]
enum RumMetricActions {
    /// List RUM metrics
    List,
    /// Get RUM metric details
    Get { metric_id: String },
    /// Create a RUM metric from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a RUM metric from JSON file
    Update {
        metric_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a RUM metric
    Delete { metric_id: String },
}

#[derive(Subcommand)]
enum RumRetentionFilterActions {
    /// List retention filters for an app
    List { app_id: String },
    /// Get retention filter details
    Get { app_id: String, filter_id: String },
    /// Create a retention filter from JSON file
    Create {
        app_id: String,
        #[arg(long)]
        file: String,
    },
    /// Update a retention filter from JSON file
    Update {
        app_id: String,
        filter_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a retention filter
    Delete { app_id: String, filter_id: String },
}

#[derive(Subcommand)]
enum RumPlaylistActions {
    /// List RUM replay playlists
    List,
    /// Get RUM replay playlist details
    Get { playlist_id: i32 },
}

#[derive(Subcommand)]
enum RumHeatmapActions {
    /// Query heatmap snapshots for a view
    Query {
        #[arg(long)]
        view_name: String,
    },
}

// ---- CI/CD ----
#[derive(Subcommand)]
enum CicdActions {
    /// CI pipelines
    Pipelines {
        #[command(subcommand)]
        action: CicdPipelineActions,
    },
    /// List CI tests
    Tests {
        #[command(subcommand)]
        action: CicdTestActions,
    },
    /// CI pipeline events
    Events {
        #[command(subcommand)]
        action: CicdEventActions,
    },
    /// DORA metrics
    Dora {
        #[command(subcommand)]
        action: CicdDoraActions,
    },
    /// Flaky tests
    #[command(name = "flaky-tests")]
    FlakyTests {
        #[command(subcommand)]
        action: CicdFlakyTestActions,
    },
}

#[derive(Subcommand)]
enum CicdPipelineActions {
    /// List CI pipelines
    List {
        #[arg(long)]
        query: Option<String>,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// Get CI pipeline details
    Get { pipeline_id: String },
}

#[derive(Subcommand)]
enum CicdTestActions {
    /// List CI test events
    List {
        #[arg(long)]
        query: Option<String>,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// Search CI test events
    Search {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// Aggregate CI test events
    Aggregate {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
}

#[derive(Subcommand)]
enum CicdEventActions {
    /// Search CI pipeline events
    Search {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// Aggregate CI pipeline events
    Aggregate {
        #[arg(long)]
        query: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
}

#[derive(Subcommand)]
enum CicdDoraActions {
    /// Patch a DORA deployment
    #[command(name = "patch-deployment")]
    PatchDeployment {
        deployment_id: String,
        #[arg(long)]
        file: String,
    },
}

#[derive(Subcommand)]
enum CicdFlakyTestActions {
    /// Search flaky tests
    Search {
        #[arg(long)]
        query: Option<String>,
    },
    /// Update flaky tests from JSON file
    Update {
        #[arg(long)]
        file: String,
    },
}

// ---- On-Call ----
#[derive(Subcommand)]
enum OnCallActions {
    /// Manage on-call teams
    Teams {
        #[command(subcommand)]
        action: OnCallTeamActions,
    },
}

#[derive(Subcommand)]
enum OnCallTeamActions {
    /// List teams
    List,
    /// Get team details
    Get { team_id: String },
    /// Create a team
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        handle: String,
    },
    /// Update a team
    Update {
        team_id: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        handle: String,
    },
    /// Delete a team
    Delete { team_id: String },
    /// Manage team memberships
    Memberships {
        #[command(subcommand)]
        action: OnCallMembershipActions,
    },
}

#[derive(Subcommand)]
enum OnCallMembershipActions {
    /// List team memberships
    List {
        team_id: String,
        #[arg(long, default_value_t = 100)]
        page_size: i64,
    },
    /// Add a membership
    Add {
        team_id: String,
        #[arg(long)]
        user_id: String,
        #[arg(long)]
        role: Option<String>,
    },
    /// Update a membership
    Update {
        team_id: String,
        user_id: String,
        #[arg(long)]
        role: String,
    },
    /// Remove a membership
    Remove { team_id: String, user_id: String },
}

// ---- Fleet ----
#[derive(Subcommand)]
enum FleetActions {
    /// Manage fleet agents
    Agents {
        #[command(subcommand)]
        action: FleetAgentActions,
    },
    /// Manage fleet deployments
    Deployments {
        #[command(subcommand)]
        action: FleetDeploymentActions,
    },
    /// Manage fleet schedules
    Schedules {
        #[command(subcommand)]
        action: FleetScheduleActions,
    },
}

#[derive(Subcommand)]
enum FleetAgentActions {
    /// List agents
    List {
        #[arg(long)]
        page_size: Option<i64>,
    },
    /// Get agent details
    Get { agent_key: String },
    /// List agent versions
    Versions,
}

#[derive(Subcommand)]
enum FleetDeploymentActions {
    /// List deployments
    List {
        #[arg(long)]
        page_size: Option<i64>,
    },
    /// Get deployment details
    Get { deployment_id: String },
    /// Cancel a deployment
    Cancel { deployment_id: String },
    /// Configure a deployment from JSON file
    Configure {
        #[arg(long)]
        file: String,
    },
    /// Upgrade a deployment from JSON file
    Upgrade {
        #[arg(long)]
        file: String,
    },
}

#[derive(Subcommand)]
enum FleetScheduleActions {
    /// List schedules
    List,
    /// Get schedule details
    Get { schedule_id: String },
    /// Create a schedule from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a schedule from JSON file
    Update {
        schedule_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a schedule
    Delete { schedule_id: String },
    /// Trigger a schedule
    Trigger { schedule_id: String },
}

// ---- Data Governance ----
#[derive(Subcommand)]
enum DataGovActions {
    /// List scanner rules
    #[command(name = "scanner-rules")]
    ScannerRules {
        #[command(subcommand)]
        action: DataGovScannerActions,
    },
}

#[derive(Subcommand)]
enum DataGovScannerActions {
    /// List rules
    List,
}

// ---- Error Tracking ----
#[derive(Subcommand)]
enum ErrorTrackingActions {
    /// Manage error tracking issues
    Issues {
        #[command(subcommand)]
        action: ErrorTrackingIssueActions,
    },
}

#[derive(Subcommand)]
enum ErrorTrackingIssueActions {
    /// Search issues
    Search {
        #[arg(long)]
        query: Option<String>,
        #[arg(long, default_value_t = 50)]
        limit: i32,
    },
    /// Get issue details
    Get { issue_id: String },
}

// ---- Code Coverage ----
#[derive(Subcommand)]
enum CodeCoverageActions {
    /// Get branch coverage summary
    #[command(name = "branch-summary")]
    BranchSummary {
        #[arg(long)]
        repo: String,
        #[arg(long)]
        branch: String,
    },
    /// Get commit coverage summary
    #[command(name = "commit-summary")]
    CommitSummary {
        #[arg(long)]
        repo: String,
        #[arg(long)]
        commit: String,
    },
}

// ---- HAMR ----
#[derive(Subcommand)]
enum HamrActions {
    /// Manage HAMR connections
    Connections {
        #[command(subcommand)]
        action: HamrConnectionActions,
    },
}

#[derive(Subcommand)]
enum HamrConnectionActions {
    /// Get HAMR org connection
    Get,
    /// Create HAMR org connection from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
}

// ---- Status Pages ----
#[derive(Subcommand)]
enum StatusPageActions {
    /// Manage status pages
    Pages {
        #[command(subcommand)]
        action: StatusPagePageActions,
    },
    /// Manage page components
    Components {
        #[command(subcommand)]
        action: StatusPageComponentActions,
    },
    /// Manage degradations
    Degradations {
        #[command(subcommand)]
        action: StatusPageDegradationActions,
    },
    /// Third-party status pages
    #[command(name = "third-party")]
    ThirdParty {
        #[command(subcommand)]
        action: StatusPageThirdPartyActions,
    },
}

#[derive(Subcommand)]
enum StatusPagePageActions {
    /// List status pages
    List,
    /// Get status page details
    Get { page_id: String },
    /// Create a status page from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a status page from JSON file
    Update {
        page_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a status page
    Delete { page_id: String },
}

#[derive(Subcommand)]
enum StatusPageComponentActions {
    /// List components
    List { page_id: String },
    /// Get component details
    Get {
        page_id: String,
        component_id: String,
    },
    /// Create a component from JSON file
    Create {
        page_id: String,
        #[arg(long)]
        file: String,
    },
    /// Update a component from JSON file
    Update {
        page_id: String,
        component_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a component
    Delete {
        page_id: String,
        component_id: String,
    },
}

#[derive(Subcommand)]
enum StatusPageDegradationActions {
    /// List degradations
    List,
    /// Get degradation details
    Get {
        page_id: String,
        degradation_id: String,
    },
    /// Create a degradation from JSON file
    Create {
        page_id: String,
        #[arg(long)]
        file: String,
    },
    /// Update a degradation from JSON file
    Update {
        page_id: String,
        degradation_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a degradation
    Delete {
        page_id: String,
        degradation_id: String,
    },
}

#[derive(Subcommand)]
enum StatusPageThirdPartyActions {
    /// List third-party status pages
    List,
}

// ---- Integrations ----
#[derive(Subcommand)]
enum IntegrationActions {
    /// Jira integration
    Jira {
        #[command(subcommand)]
        action: JiraActions,
    },
    /// ServiceNow integration
    Servicenow {
        #[command(subcommand)]
        action: ServiceNowActions,
    },
    /// Slack integration
    Slack {
        #[command(subcommand)]
        action: SlackActions,
    },
    /// PagerDuty integration
    Pagerduty {
        #[command(subcommand)]
        action: PagerdutyActions,
    },
    /// Webhooks integration
    Webhooks {
        #[command(subcommand)]
        action: WebhooksActions,
    },
}

#[derive(Subcommand)]
enum JiraActions {
    /// Manage Jira accounts
    Accounts {
        #[command(subcommand)]
        action: JiraAccountActions,
    },
    /// Manage Jira templates
    Templates {
        #[command(subcommand)]
        action: JiraTemplateActions,
    },
}

#[derive(Subcommand)]
enum JiraAccountActions {
    /// List Jira accounts
    List,
    /// Delete a Jira account
    Delete { account_id: String },
}

#[derive(Subcommand)]
enum JiraTemplateActions {
    /// List templates
    List,
    /// Get template details
    Get { template_id: String },
    /// Create a template from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a template from JSON file
    Update {
        template_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a template
    Delete { template_id: String },
}

#[derive(Subcommand)]
enum ServiceNowActions {
    /// Manage ServiceNow instances
    Instances {
        #[command(subcommand)]
        action: ServiceNowInstanceActions,
    },
    /// Manage ServiceNow templates
    Templates {
        #[command(subcommand)]
        action: ServiceNowTemplateActions,
    },
    /// Manage ServiceNow users
    Users {
        #[command(subcommand)]
        action: ServiceNowUserActions,
    },
    /// Manage ServiceNow assignment groups
    #[command(name = "assignment-groups")]
    AssignmentGroups {
        #[command(subcommand)]
        action: ServiceNowAssignmentGroupActions,
    },
    /// Manage ServiceNow business services
    #[command(name = "business-services")]
    BusinessServices {
        #[command(subcommand)]
        action: ServiceNowBusinessServiceActions,
    },
}

#[derive(Subcommand)]
enum ServiceNowInstanceActions {
    /// List ServiceNow instances
    List,
}

#[derive(Subcommand)]
enum ServiceNowUserActions {
    /// List ServiceNow users for an instance
    List { instance_name: String },
}

#[derive(Subcommand)]
enum ServiceNowAssignmentGroupActions {
    /// List ServiceNow assignment groups for an instance
    List { instance_name: String },
}

#[derive(Subcommand)]
enum ServiceNowBusinessServiceActions {
    /// List ServiceNow business services for an instance
    List { instance_name: String },
}

#[derive(Subcommand)]
enum ServiceNowTemplateActions {
    /// List templates
    List,
    /// Get template details
    Get { template_id: String },
    /// Create a template from JSON file
    Create {
        #[arg(long)]
        file: String,
    },
    /// Update a template from JSON file
    Update {
        template_id: String,
        #[arg(long)]
        file: String,
    },
    /// Delete a template
    Delete { template_id: String },
}

#[derive(Subcommand)]
enum SlackActions {
    /// List Slack channels
    List,
}

#[derive(Subcommand)]
enum PagerdutyActions {
    /// List PagerDuty services
    List,
}

#[derive(Subcommand)]
enum WebhooksActions {
    /// List webhooks
    List,
}

// ---- Cost ----
#[derive(Subcommand)]
enum CostActions {
    /// Get projected cost
    Projected,
    /// Get cost by org
    #[command(name = "by-org")]
    ByOrg {
        #[arg(long)]
        start_month: String,
        #[arg(long)]
        end_month: Option<String>,
    },
    /// Get monthly cost attribution
    Attribution {
        #[arg(long)]
        start: String,
        #[arg(long)]
        fields: Option<String>,
    },
}

// ---- Misc ----
#[derive(Subcommand)]
enum MiscActions {
    /// Get IP ranges
    #[command(name = "ip-ranges")]
    IpRanges,
    /// Validate API key / check status
    Status,
}

// ---- APM ----
#[derive(Subcommand)]
enum ApmActions {
    /// Manage APM services
    Services {
        #[command(subcommand)]
        action: ApmServiceActions,
    },
    /// List APM entities
    Entities {
        #[command(subcommand)]
        action: ApmEntityActions,
    },
    /// List APM dependencies
    Dependencies {
        #[command(subcommand)]
        action: ApmDependencyActions,
    },
    /// APM flow map
    #[command(name = "flow-map")]
    FlowMap {
        #[arg(long)]
        query: String,
        #[arg(long, default_value_t = 100)]
        limit: i64,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
}

#[derive(Subcommand)]
enum ApmServiceActions {
    /// List APM services
    List {
        #[arg(long)]
        env: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
    /// Get service stats
    Stats {
        #[arg(long)]
        env: String,
        #[arg(long)]
        from: String,
        #[arg(long)]
        to: String,
    },
    /// List operations for a service
    Operations {
        #[arg(long)]
        service: String,
        #[arg(long)]
        env: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
    /// List resources for a service operation
    Resources {
        #[arg(long)]
        service: String,
        #[arg(long)]
        operation: String,
        #[arg(long)]
        env: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
}

#[derive(Subcommand)]
enum ApmEntityActions {
    /// List APM entities
    List {
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
}

#[derive(Subcommand)]
enum ApmDependencyActions {
    /// List APM service dependencies
    List {
        #[arg(long)]
        env: String,
        #[arg(long, default_value = "1h")]
        from: String,
        #[arg(long, default_value = "now")]
        to: String,
    },
}

// ---- Investigations ----
#[derive(Subcommand)]
enum InvestigationActions {
    /// List investigations
    List {
        #[arg(long, default_value_t = 50)]
        page_limit: i64,
        #[arg(long, default_value_t = 0)]
        page_offset: i64,
    },
    /// Get investigation details
    Get { investigation_id: String },
    /// Trigger an investigation from JSON file
    Trigger {
        #[arg(long)]
        file: String,
    },
}

// ---- Network (placeholder) ----
#[derive(Subcommand)]
enum NetworkActions {
    /// List network devices/monitors
    List,
    /// Network flows
    Flows {
        #[command(subcommand)]
        action: NetworkFlowActions,
    },
    /// Network devices
    Devices {
        #[command(subcommand)]
        action: NetworkDeviceActions,
    },
}

#[derive(Subcommand)]
enum NetworkFlowActions {
    /// List network flows
    List,
}

#[derive(Subcommand)]
enum NetworkDeviceActions {
    /// List network devices
    List,
}

// ---- Obs Pipelines (placeholder) ----
#[derive(Subcommand)]
enum ObsPipelinesActions {
    /// List observability pipelines
    List,
    /// Get observability pipeline details
    Get { pipeline_id: String },
}

// ---- Scorecards (placeholder) ----
#[derive(Subcommand)]
enum ScorecardsActions {
    /// List scorecards
    List,
    /// Get scorecard details
    Get { scorecard_id: String },
}

// ---- Traces (placeholder) ----
#[derive(Subcommand)]
enum TracesActions {
    /// List traces
    List,
}

// ---- Agent (placeholder) ----
#[derive(Subcommand)]
enum AgentActions {
    /// Generate agent schema
    Schema,
    /// Show agent management guide
    Guide,
}

// ---- Alias ----
#[derive(Subcommand)]
enum AliasActions {
    /// List all aliases
    List,
    /// Set an alias
    Set { name: String, command: String },
    /// Delete aliases
    Delete { names: Vec<String> },
    /// Import aliases from a file
    Import {
        #[arg(long)]
        file: String,
    },
}

// ---- Product Analytics ----
#[derive(Subcommand)]
enum ProductAnalyticsActions {
    /// Product analytics events
    Events {
        #[command(subcommand)]
        action: ProductAnalyticsEventActions,
    },
}

#[derive(Subcommand)]
enum ProductAnalyticsEventActions {
    /// Send a product analytics event from JSON file
    Send {
        #[arg(long)]
        file: String,
    },
}

// ---- Static Analysis ----
#[derive(Subcommand)]
enum StaticAnalysisActions {
    /// Manage AST results
    Ast {
        #[command(subcommand)]
        action: StaticAnalysisAstActions,
    },
    /// Manage custom rulesets
    #[command(name = "custom-rulesets")]
    CustomRulesets {
        #[command(subcommand)]
        action: StaticAnalysisCustomRulesetActions,
    },
    /// Manage SCA results
    Sca {
        #[command(subcommand)]
        action: StaticAnalysisScaActions,
    },
    /// Manage coverage
    Coverage {
        #[command(subcommand)]
        action: StaticAnalysisCoverageActions,
    },
}

#[derive(Subcommand)]
enum StaticAnalysisAstActions {
    /// List AST results
    List,
    /// Get AST result details
    Get { id: String },
}

#[derive(Subcommand)]
enum StaticAnalysisCustomRulesetActions {
    /// List custom rulesets
    List,
    /// Get custom ruleset details
    Get { id: String },
}

#[derive(Subcommand)]
enum StaticAnalysisScaActions {
    /// List SCA results
    List,
    /// Get SCA result details
    Get { id: String },
}

#[derive(Subcommand)]
enum StaticAnalysisCoverageActions {
    /// List coverage results
    List,
    /// Get coverage result details
    Get { id: String },
}

// ---- Auth ----
#[derive(Subcommand)]
enum AuthActions {
    /// Login via OAuth2
    Login,
    /// Logout
    Logout,
    /// Show auth status
    Status,
    /// Print access token
    Token,
}

// ---- Main ----

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut cfg = config::Config::from_env()?;

    // Apply flag overrides
    if let Ok(fmt) = cli.output.parse() {
        cfg.output_format = fmt;
    }
    if cli.yes {
        cfg.auto_approve = true;
    }
    cfg.agent_mode = cli.agent || useragent::is_agent_mode();
    if cfg.agent_mode {
        cfg.auto_approve = true;
    }

    match cli.command {
        // --- Monitors ---
        Commands::Monitors { action } => {
            cfg.validate_auth()?;
            match action {
                MonitorActions::List { name, tags, limit } => {
                    commands::monitors::list(&cfg, name, tags, limit).await?;
                }
                MonitorActions::Get { monitor_id } => {
                    commands::monitors::get(&cfg, monitor_id).await?;
                }
                MonitorActions::Create { file } => {
                    commands::monitors::create(&cfg, &file).await?;
                }
                MonitorActions::Update { monitor_id, file } => {
                    commands::monitors::update(&cfg, monitor_id, &file).await?;
                }
                MonitorActions::Search { query } => {
                    commands::monitors::search(&cfg, query).await?;
                }
                MonitorActions::Delete { monitor_id } => {
                    commands::monitors::delete(&cfg, monitor_id).await?;
                }
            }
        }
        // --- Logs ---
        Commands::Logs { action } => {
            cfg.validate_auth()?;
            match action {
                LogActions::Search {
                    query,
                    from,
                    to,
                    limit,
                } => {
                    commands::logs::search(&cfg, query, from, to, limit).await?;
                }
                LogActions::List {
                    query,
                    from,
                    to,
                    limit,
                } => {
                    commands::logs::list(&cfg, query, from, to, limit).await?;
                }
                LogActions::Query {
                    query,
                    from,
                    to,
                    limit,
                } => {
                    commands::logs::query(&cfg, query, from, to, limit).await?;
                }
                LogActions::Aggregate {
                    query,
                    from,
                    to,
                } => {
                    commands::logs::aggregate(&cfg, query, from, to).await?;
                }
                LogActions::Archives { action } => match action {
                    LogArchiveActions::List => commands::logs::archives_list(&cfg).await?,
                    LogArchiveActions::Get { archive_id } => {
                        commands::logs::archives_get(&cfg, &archive_id).await?;
                    }
                    LogArchiveActions::Delete { archive_id } => {
                        commands::logs::archives_delete(&cfg, &archive_id).await?;
                    }
                },
                LogActions::CustomDestinations { action } => match action {
                    LogCustomDestinationActions::List => {
                        commands::logs::custom_destinations_list(&cfg).await?;
                    }
                    LogCustomDestinationActions::Get { destination_id } => {
                        commands::logs::custom_destinations_get(&cfg, &destination_id).await?;
                    }
                },
                LogActions::Metrics { action } => match action {
                    LogMetricActions::List => commands::logs::metrics_list(&cfg).await?,
                    LogMetricActions::Get { metric_id } => {
                        commands::logs::metrics_get(&cfg, &metric_id).await?;
                    }
                    LogMetricActions::Delete { metric_id } => {
                        commands::logs::metrics_delete(&cfg, &metric_id).await?;
                    }
                },
                LogActions::RestrictionQueries { action } => match action {
                    LogRestrictionQueryActions::List => {
                        commands::logs::restriction_queries_list(&cfg).await?;
                    }
                    LogRestrictionQueryActions::Get { query_id } => {
                        commands::logs::restriction_queries_get(&cfg, &query_id).await?;
                    }
                },
            }
        }
        // --- Incidents ---
        Commands::Incidents { action } => {
            cfg.validate_auth()?;
            match action {
                IncidentActions::List { limit } => {
                    commands::incidents::list(&cfg, limit).await?;
                }
                IncidentActions::Get { incident_id } => {
                    commands::incidents::get(&cfg, &incident_id).await?;
                }
                IncidentActions::Attachments { action } => match action {
                    IncidentAttachmentActions::List { incident_id } => {
                        commands::incidents::attachments_list(&cfg, &incident_id).await?;
                    }
                    IncidentAttachmentActions::Delete {
                        incident_id,
                        attachment_id,
                    } => {
                        commands::incidents::attachments_delete(&cfg, &incident_id, &attachment_id)
                            .await?;
                    }
                },
                IncidentActions::Settings { action } => match action {
                    IncidentSettingsActions::Get => {
                        commands::incidents::settings_get(&cfg).await?;
                    }
                    IncidentSettingsActions::Update { file } => {
                        commands::incidents::settings_update(&cfg, &file).await?;
                    }
                },
                IncidentActions::Handles { action } => match action {
                    IncidentHandleActions::List => {
                        commands::incidents::handles_list(&cfg).await?;
                    }
                    IncidentHandleActions::Create { file } => {
                        commands::incidents::handles_create(&cfg, &file).await?;
                    }
                    IncidentHandleActions::Update { file } => {
                        commands::incidents::handles_update(&cfg, &file).await?;
                    }
                    IncidentHandleActions::Delete { handle_id } => {
                        commands::incidents::handles_delete(&cfg, &handle_id).await?;
                    }
                },
                IncidentActions::PostmortemTemplates { action } => match action {
                    IncidentPostmortemActions::List => {
                        commands::incidents::postmortem_templates_list(&cfg).await?;
                    }
                    IncidentPostmortemActions::Get { template_id } => {
                        commands::incidents::postmortem_templates_get(&cfg, &template_id).await?;
                    }
                    IncidentPostmortemActions::Create { file } => {
                        commands::incidents::postmortem_templates_create(&cfg, &file).await?;
                    }
                    IncidentPostmortemActions::Update { template_id, file } => {
                        commands::incidents::postmortem_templates_update(&cfg, &template_id, &file)
                            .await?;
                    }
                    IncidentPostmortemActions::Delete { template_id } => {
                        commands::incidents::postmortem_templates_delete(&cfg, &template_id)
                            .await?;
                    }
                },
            }
        }
        // --- Dashboards ---
        Commands::Dashboards { action } => {
            cfg.validate_auth()?;
            match action {
                DashboardActions::List => commands::dashboards::list(&cfg).await?,
                DashboardActions::Get { id } => commands::dashboards::get(&cfg, &id).await?,
                DashboardActions::Create { file } => {
                    commands::dashboards::create(&cfg, &file).await?;
                }
                DashboardActions::Update { id, file } => {
                    commands::dashboards::update(&cfg, &id, &file).await?;
                }
                DashboardActions::Delete { id } => commands::dashboards::delete(&cfg, &id).await?,
            }
        }
        // --- Metrics ---
        Commands::Metrics { action } => {
            cfg.validate_auth()?;
            match action {
                MetricActions::List { filter, from } => {
                    commands::metrics::list(&cfg, filter, from).await?;
                }
                MetricActions::Search { query, from, to } => {
                    commands::metrics::search(&cfg, query, from, to).await?;
                }
                MetricActions::Query { query, from, to } => {
                    commands::metrics::query(&cfg, query, from, to).await?;
                }
                MetricActions::Submit { file } => {
                    commands::metrics::submit(&cfg, &file).await?;
                }
                MetricActions::Metadata { action } => match action {
                    MetricMetadataActions::Get { metric_name } => {
                        commands::metrics::metadata_get(&cfg, &metric_name).await?;
                    }
                    MetricMetadataActions::Update { metric_name, file } => {
                        commands::metrics::metadata_update(&cfg, &metric_name, &file).await?;
                    }
                },
                MetricActions::Tags { action } => match action {
                    MetricTagActions::List { metric_name } => {
                        commands::metrics::tags_list(&cfg, &metric_name).await?;
                    }
                },
            }
        }
        // --- SLOs ---
        Commands::Slos { action } => {
            cfg.validate_auth()?;
            match action {
                SloActions::List => commands::slos::list(&cfg).await?,
                SloActions::Get { id } => commands::slos::get(&cfg, &id).await?,
                SloActions::Create { file } => commands::slos::create(&cfg, &file).await?,
                SloActions::Update { id, file } => {
                    commands::slos::update(&cfg, &id, &file).await?;
                }
                SloActions::Delete { id } => commands::slos::delete(&cfg, &id).await?,
                SloActions::Status { id, from_ts, to_ts } => {
                    commands::slos::status(&cfg, &id, from_ts, to_ts).await?;
                }
            }
        }
        // --- Synthetics ---
        Commands::Synthetics { action } => {
            cfg.validate_auth()?;
            match action {
                SyntheticsActions::Tests { action } => match action {
                    SyntheticsTestActions::List => commands::synthetics::tests_list(&cfg).await?,
                    SyntheticsTestActions::Get { public_id } => {
                        commands::synthetics::tests_get(&cfg, &public_id).await?;
                    }
                    SyntheticsTestActions::Search { text, count, start } => {
                        commands::synthetics::tests_search(&cfg, text, count, start).await?;
                    }
                },
                SyntheticsActions::Locations { action } => match action {
                    SyntheticsLocationActions::List => {
                        commands::synthetics::locations_list(&cfg).await?;
                    }
                },
                SyntheticsActions::Suites { action } => match action {
                    SyntheticsSuiteActions::List { query } => {
                        commands::synthetics::suites_list(&cfg, query).await?;
                    }
                    SyntheticsSuiteActions::Get { suite_id } => {
                        commands::synthetics::suites_get(&cfg, &suite_id).await?;
                    }
                    SyntheticsSuiteActions::Create { file } => {
                        commands::synthetics::suites_create(&cfg, &file).await?;
                    }
                    SyntheticsSuiteActions::Update { suite_id, file } => {
                        commands::synthetics::suites_update(&cfg, &suite_id, &file).await?;
                    }
                    SyntheticsSuiteActions::Delete { suite_ids } => {
                        commands::synthetics::suites_delete(&cfg, suite_ids).await?;
                    }
                },
            }
        }
        // --- Events ---
        Commands::Events { action } => {
            cfg.validate_auth()?;
            match action {
                EventActions::List { start, end, tags } => {
                    commands::events::list(&cfg, start, end, tags).await?;
                }
                EventActions::Search {
                    query,
                    from,
                    to,
                    limit,
                } => {
                    commands::events::search(&cfg, query, from, to, limit).await?;
                }
                EventActions::Get { event_id } => {
                    commands::events::get(&cfg, event_id).await?;
                }
            }
        }
        // --- Downtime ---
        Commands::Downtime { action } => {
            cfg.validate_auth()?;
            match action {
                DowntimeActions::List => commands::downtime::list(&cfg).await?,
                DowntimeActions::Get { id } => commands::downtime::get(&cfg, &id).await?,
                DowntimeActions::Create { file } => {
                    commands::downtime::create(&cfg, &file).await?;
                }
                DowntimeActions::Cancel { id } => commands::downtime::cancel(&cfg, &id).await?,
            }
        }
        // --- Tags ---
        Commands::Tags { action } => {
            cfg.validate_auth()?;
            match action {
                TagActions::List => commands::tags::list(&cfg).await?,
                TagActions::Get { hostname } => commands::tags::get(&cfg, &hostname).await?,
                TagActions::Add { hostname, tags } => {
                    commands::tags::add(&cfg, &hostname, tags).await?;
                }
                TagActions::Update { hostname, tags } => {
                    commands::tags::update(&cfg, &hostname, tags).await?;
                }
                TagActions::Delete { hostname } => {
                    commands::tags::delete(&cfg, &hostname).await?;
                }
            }
        }
        // --- Users ---
        Commands::Users { action } => {
            cfg.validate_auth()?;
            match action {
                UserActions::List => commands::users::list(&cfg).await?,
                UserActions::Get { user_id } => commands::users::get(&cfg, &user_id).await?,
                UserActions::Roles { action } => match action {
                    UserRoleActions::List => commands::users::roles_list(&cfg).await?,
                },
            }
        }
        // --- Infrastructure ---
        Commands::Infrastructure { action } => {
            cfg.validate_auth()?;
            match action {
                InfraActions::Hosts { action } => match action {
                    InfraHostActions::List {
                        filter,
                        sort,
                        count,
                    } => {
                        commands::infrastructure::hosts_list(&cfg, filter, sort, count).await?;
                    }
                    InfraHostActions::Get { hostname } => {
                        commands::infrastructure::hosts_get(&cfg, &hostname).await?;
                    }
                },
            }
        }
        // --- Audit Logs ---
        Commands::AuditLogs { action } => {
            cfg.validate_auth()?;
            match action {
                AuditLogActions::List { from, to, limit } => {
                    commands::audit_logs::list(&cfg, from, to, limit).await?;
                }
                AuditLogActions::Search {
                    query,
                    from,
                    to,
                    limit,
                } => {
                    commands::audit_logs::search(&cfg, query, from, to, limit).await?;
                }
            }
        }
        // --- Security ---
        Commands::Security { action } => {
            cfg.validate_auth()?;
            match action {
                SecurityActions::Rules { action } => match action {
                    SecurityRuleActions::List => commands::security::rules_list(&cfg).await?,
                    SecurityRuleActions::Get { rule_id } => {
                        commands::security::rules_get(&cfg, &rule_id).await?;
                    }
                    SecurityRuleActions::BulkExport { rule_ids } => {
                        commands::security::rules_bulk_export(&cfg, rule_ids).await?;
                    }
                },
                SecurityActions::Signals { action } => match action {
                    SecuritySignalActions::Search {
                        query,
                        from,
                        to,
                        limit,
                    } => {
                        commands::security::signals_search(&cfg, query, from, to, limit).await?;
                    }
                },
                SecurityActions::Findings { action } => match action {
                    SecurityFindingActions::Search { query, limit } => {
                        commands::security::findings_search(&cfg, query, limit).await?;
                    }
                },
                SecurityActions::ContentPacks { action } => match action {
                    SecurityContentPackActions::List => {
                        commands::security::content_packs_list(&cfg).await?;
                    }
                    SecurityContentPackActions::Activate { pack_id } => {
                        commands::security::content_packs_activate(&cfg, &pack_id).await?;
                    }
                    SecurityContentPackActions::Deactivate { pack_id } => {
                        commands::security::content_packs_deactivate(&cfg, &pack_id).await?;
                    }
                },
                SecurityActions::RiskScores { action } => match action {
                    SecurityRiskScoreActions::List { query } => {
                        commands::security::risk_scores_list(&cfg, query).await?;
                    }
                },
            }
        }
        // --- Organizations ---
        Commands::Organizations { action } => {
            cfg.validate_auth()?;
            match action {
                OrgActions::List => commands::organizations::list(&cfg).await?,
                OrgActions::Get => commands::organizations::get(&cfg).await?,
            }
        }
        // --- Cloud ---
        Commands::Cloud { action } => {
            cfg.validate_auth()?;
            match action {
                CloudActions::Aws { action } => match action {
                    CloudSubActions::List => commands::cloud::aws_list(&cfg).await?,
                },
                CloudActions::Gcp { action } => match action {
                    CloudSubActions::List => commands::cloud::gcp_list(&cfg).await?,
                },
                CloudActions::Azure { action } => match action {
                    CloudSubActions::List => commands::cloud::azure_list(&cfg).await?,
                },
                CloudActions::Oci { action } => match action {
                    CloudOciActions::Tenancies { action } => match action {
                        CloudOciTenancyActions::List => {
                            commands::cloud::oci_tenancies_list(&cfg).await?;
                        }
                        CloudOciTenancyActions::Get { tenancy_id } => {
                            commands::cloud::oci_tenancies_get(&cfg, &tenancy_id).await?;
                        }
                        CloudOciTenancyActions::Create { file } => {
                            commands::cloud::oci_tenancies_create(&cfg, &file).await?;
                        }
                        CloudOciTenancyActions::Update { tenancy_id, file } => {
                            commands::cloud::oci_tenancies_update(&cfg, &tenancy_id, &file).await?;
                        }
                        CloudOciTenancyActions::Delete { tenancy_id } => {
                            commands::cloud::oci_tenancies_delete(&cfg, &tenancy_id).await?;
                        }
                    },
                    CloudOciActions::Products { product_keys } => {
                        commands::cloud::oci_products_list(&cfg, &product_keys).await?;
                    }
                },
            }
        }
        // --- Cases ---
        Commands::Cases { action } => {
            cfg.validate_auth()?;
            match action {
                CaseActions::Search { query, page_size } => {
                    commands::cases::search(&cfg, query, page_size).await?;
                }
                CaseActions::Get { case_id } => commands::cases::get(&cfg, &case_id).await?,
                CaseActions::Create { file } => {
                    commands::cases::create(&cfg, &file).await?;
                }
                CaseActions::Archive { case_id } => {
                    commands::cases::archive(&cfg, &case_id).await?;
                }
                CaseActions::Unarchive { case_id } => {
                    commands::cases::unarchive(&cfg, &case_id).await?;
                }
                CaseActions::Assign { case_id, user_id } => {
                    commands::cases::assign(&cfg, &case_id, &user_id).await?;
                }
                CaseActions::UpdatePriority { case_id, priority } => {
                    commands::cases::update_priority(&cfg, &case_id, &priority).await?;
                }
                CaseActions::UpdateStatus { case_id, status } => {
                    commands::cases::update_status(&cfg, &case_id, &status).await?;
                }
                CaseActions::Projects { action } => match action {
                    CaseProjectActions::List => commands::cases::projects_list(&cfg).await?,
                    CaseProjectActions::Get { project_id } => {
                        commands::cases::projects_get(&cfg, &project_id).await?;
                    }
                    CaseProjectActions::Create { name, key } => {
                        commands::cases::projects_create(&cfg, &name, &key).await?;
                    }
                    CaseProjectActions::Delete { project_id } => {
                        commands::cases::projects_delete(&cfg, &project_id).await?;
                    }
                    CaseProjectActions::NotificationRules { action } => match action {
                        CaseNotificationRuleActions::List { project_id } => {
                            commands::cases::projects_notification_rules_list(&cfg, &project_id)
                                .await?;
                        }
                        CaseNotificationRuleActions::Create { project_id, file } => {
                            commands::cases::projects_notification_rules_create(
                                &cfg,
                                &project_id,
                                &file,
                            )
                            .await?;
                        }
                        CaseNotificationRuleActions::Update {
                            project_id,
                            rule_id,
                            file,
                        } => {
                            commands::cases::projects_notification_rules_update(
                                &cfg,
                                &project_id,
                                &rule_id,
                                &file,
                            )
                            .await?;
                        }
                        CaseNotificationRuleActions::Delete {
                            project_id,
                            rule_id,
                        } => {
                            commands::cases::projects_notification_rules_delete(
                                &cfg,
                                &project_id,
                                &rule_id,
                            )
                            .await?;
                        }
                    },
                },
                CaseActions::Jira { action } => match action {
                    CaseJiraActions::CreateIssue { case_id, file } => {
                        commands::cases::jira_create_issue(&cfg, &case_id, &file).await?;
                    }
                    CaseJiraActions::Link { case_id, file } => {
                        commands::cases::jira_link(&cfg, &case_id, &file).await?;
                    }
                    CaseJiraActions::Unlink { case_id } => {
                        commands::cases::jira_unlink(&cfg, &case_id).await?;
                    }
                },
                CaseActions::Servicenow { action } => match action {
                    CaseServicenowActions::CreateTicket { case_id, file } => {
                        commands::cases::servicenow_create_ticket(&cfg, &case_id, &file).await?;
                    }
                },
            }
        }
        // --- Service Catalog ---
        Commands::ServiceCatalog { action } => {
            cfg.validate_auth()?;
            match action {
                ServiceCatalogActions::List => commands::service_catalog::list(&cfg).await?,
                ServiceCatalogActions::Get { service_name } => {
                    commands::service_catalog::get(&cfg, &service_name).await?;
                }
            }
        }
        // --- API Keys ---
        Commands::ApiKeys { action } => {
            cfg.validate_auth()?;
            match action {
                ApiKeyActions::List => commands::api_keys::list(&cfg).await?,
                ApiKeyActions::Get { key_id } => commands::api_keys::get(&cfg, &key_id).await?,
                ApiKeyActions::Create { name } => {
                    commands::api_keys::create(&cfg, &name).await?;
                }
                ApiKeyActions::Delete { key_id } => {
                    commands::api_keys::delete(&cfg, &key_id).await?;
                }
            }
        }
        // --- App Keys ---
        Commands::AppKeys { action } => {
            cfg.validate_auth()?;
            match action {
                AppKeyActions::List => commands::app_keys::list(&cfg).await?,
                AppKeyActions::Get { key_id } => commands::app_keys::get(&cfg, &key_id).await?,
                AppKeyActions::Register { key_id } => {
                    commands::app_keys::register(&cfg, &key_id).await?;
                }
                AppKeyActions::Unregister { key_id } => {
                    commands::app_keys::unregister(&cfg, &key_id).await?;
                }
            }
        }
        // --- Usage ---
        Commands::Usage { action } => {
            cfg.validate_auth()?;
            match action {
                UsageActions::Summary { start, end } => {
                    commands::usage::summary(&cfg, start, end).await?;
                }
                UsageActions::Hourly { start, end } => {
                    commands::usage::hourly(&cfg, start, end).await?;
                }
            }
        }
        // --- Notebooks ---
        Commands::Notebooks { action } => {
            cfg.validate_auth()?;
            match action {
                NotebookActions::List => commands::notebooks::list(&cfg).await?,
                NotebookActions::Get { notebook_id } => {
                    commands::notebooks::get(&cfg, notebook_id).await?;
                }
                NotebookActions::Create { file } => {
                    commands::notebooks::create(&cfg, &file).await?;
                }
                NotebookActions::Update { notebook_id, file } => {
                    commands::notebooks::update(&cfg, notebook_id, &file).await?;
                }
                NotebookActions::Delete { notebook_id } => {
                    commands::notebooks::delete(&cfg, notebook_id).await?;
                }
            }
        }
        // --- RUM ---
        Commands::Rum { action } => {
            cfg.validate_auth()?;
            match action {
                RumActions::Apps { action } => match action {
                    RumAppActions::List => commands::rum::apps_list(&cfg).await?,
                    RumAppActions::Get { app_id } => commands::rum::apps_get(&cfg, &app_id).await?,
                    RumAppActions::Create { name, app_type } => {
                        commands::rum::apps_create(&cfg, &name, app_type).await?;
                    }
                    RumAppActions::Update { app_id, file } => {
                        commands::rum::apps_update(&cfg, &app_id, &file).await?;
                    }
                    RumAppActions::Delete { app_id } => {
                        commands::rum::apps_delete(&cfg, &app_id).await?;
                    }
                },
                RumActions::Events { from, to, limit } => {
                    commands::rum::events_list(&cfg, from, to, limit).await?;
                }
                RumActions::Sessions { action } => match action {
                    RumSessionActions::Search {
                        query,
                        from,
                        to,
                        limit,
                    } => {
                        commands::rum::sessions_search(&cfg, query, from, to, limit).await?;
                    }
                    RumSessionActions::List { from, to, limit } => {
                        commands::rum::sessions_list(&cfg, from, to, limit).await?;
                    }
                },
                RumActions::Metrics { action } => match action {
                    RumMetricActions::List => commands::rum::metrics_list(&cfg).await?,
                    RumMetricActions::Get { metric_id } => {
                        commands::rum::metrics_get(&cfg, &metric_id).await?;
                    }
                    RumMetricActions::Create { file } => {
                        commands::rum::metrics_create(&cfg, &file).await?;
                    }
                    RumMetricActions::Update { metric_id, file } => {
                        commands::rum::metrics_update(&cfg, &metric_id, &file).await?;
                    }
                    RumMetricActions::Delete { metric_id } => {
                        commands::rum::metrics_delete(&cfg, &metric_id).await?;
                    }
                },
                RumActions::RetentionFilters { action } => match action {
                    RumRetentionFilterActions::List { app_id } => {
                        commands::rum::retention_filters_list(&cfg, &app_id).await?;
                    }
                    RumRetentionFilterActions::Get { app_id, filter_id } => {
                        commands::rum::retention_filters_get(&cfg, &app_id, &filter_id).await?;
                    }
                    RumRetentionFilterActions::Create { app_id, file } => {
                        commands::rum::retention_filters_create(&cfg, &app_id, &file).await?;
                    }
                    RumRetentionFilterActions::Update {
                        app_id,
                        filter_id,
                        file,
                    } => {
                        commands::rum::retention_filters_update(&cfg, &app_id, &filter_id, &file)
                            .await?;
                    }
                    RumRetentionFilterActions::Delete { app_id, filter_id } => {
                        commands::rum::retention_filters_delete(&cfg, &app_id, &filter_id).await?;
                    }
                },
                RumActions::Playlists { action } => match action {
                    RumPlaylistActions::List => commands::rum::playlists_list(&cfg).await?,
                    RumPlaylistActions::Get { playlist_id } => {
                        commands::rum::playlists_get(&cfg, playlist_id).await?;
                    }
                },
                RumActions::Heatmaps { action } => match action {
                    RumHeatmapActions::Query { view_name } => {
                        commands::rum::heatmaps_query(&cfg, &view_name).await?;
                    }
                },
            }
        }
        // --- CI/CD ---
        Commands::Cicd { action } => {
            cfg.validate_auth()?;
            match action {
                CicdActions::Pipelines { action } => match action {
                    CicdPipelineActions::List {
                        query,
                        from,
                        to,
                        limit,
                    } => {
                        commands::cicd::pipelines_list(&cfg, query, from, to, limit).await?;
                    }
                    CicdPipelineActions::Get { pipeline_id } => {
                        commands::cicd::pipelines_get(&cfg, &pipeline_id).await?;
                    }
                },
                CicdActions::Tests { action } => match action {
                    CicdTestActions::List {
                        query,
                        from,
                        to,
                        limit,
                    } => {
                        commands::cicd::tests_list(&cfg, query, from, to, limit).await?;
                    }
                    CicdTestActions::Search {
                        query,
                        from,
                        to,
                        limit,
                    } => {
                        commands::cicd::tests_search(&cfg, query, from, to, limit).await?;
                    }
                    CicdTestActions::Aggregate { query, from, to } => {
                        commands::cicd::tests_aggregate(&cfg, query, from, to).await?;
                    }
                },
                CicdActions::Events { action } => match action {
                    CicdEventActions::Search {
                        query,
                        from,
                        to,
                        limit,
                    } => {
                        commands::cicd::events_search(&cfg, query, from, to, limit).await?;
                    }
                    CicdEventActions::Aggregate { query, from, to } => {
                        commands::cicd::events_aggregate(&cfg, query, from, to).await?;
                    }
                },
                CicdActions::Dora { action } => match action {
                    CicdDoraActions::PatchDeployment {
                        deployment_id,
                        file,
                    } => {
                        commands::cicd::dora_patch_deployment(&cfg, &deployment_id, &file).await?;
                    }
                },
                CicdActions::FlakyTests { action } => match action {
                    CicdFlakyTestActions::Search { query } => {
                        commands::cicd::flaky_tests_search(&cfg, query).await?;
                    }
                    CicdFlakyTestActions::Update { file } => {
                        commands::cicd::flaky_tests_update(&cfg, &file).await?;
                    }
                },
            }
        }
        // --- On-Call ---
        Commands::OnCall { action } => {
            cfg.validate_auth()?;
            match action {
                OnCallActions::Teams { action } => match action {
                    OnCallTeamActions::List => commands::on_call::teams_list(&cfg).await?,
                    OnCallTeamActions::Get { team_id } => {
                        commands::on_call::teams_get(&cfg, &team_id).await?;
                    }
                    OnCallTeamActions::Create { name, handle } => {
                        commands::on_call::teams_create(&cfg, &name, &handle).await?;
                    }
                    OnCallTeamActions::Update {
                        team_id,
                        name,
                        handle,
                    } => {
                        commands::on_call::teams_update(&cfg, &team_id, &name, &handle).await?;
                    }
                    OnCallTeamActions::Delete { team_id } => {
                        commands::on_call::teams_delete(&cfg, &team_id).await?;
                    }
                    OnCallTeamActions::Memberships { action } => match action {
                        OnCallMembershipActions::List { team_id, page_size } => {
                            commands::on_call::memberships_list(&cfg, &team_id, page_size).await?;
                        }
                        OnCallMembershipActions::Add {
                            team_id,
                            user_id,
                            role,
                        } => {
                            commands::on_call::memberships_add(&cfg, &team_id, &user_id, role)
                                .await?;
                        }
                        OnCallMembershipActions::Update {
                            team_id,
                            user_id,
                            role,
                        } => {
                            commands::on_call::memberships_update(&cfg, &team_id, &user_id, &role)
                                .await?;
                        }
                        OnCallMembershipActions::Remove { team_id, user_id } => {
                            commands::on_call::memberships_remove(&cfg, &team_id, &user_id)
                                .await?;
                        }
                    },
                },
            }
        }
        // --- Fleet ---
        Commands::Fleet { action } => {
            cfg.validate_auth()?;
            match action {
                FleetActions::Agents { action } => match action {
                    FleetAgentActions::List { page_size } => {
                        commands::fleet::agents_list(&cfg, page_size).await?;
                    }
                    FleetAgentActions::Get { agent_key } => {
                        commands::fleet::agents_get(&cfg, &agent_key).await?;
                    }
                    FleetAgentActions::Versions => commands::fleet::agents_versions(&cfg).await?,
                },
                FleetActions::Deployments { action } => match action {
                    FleetDeploymentActions::List { page_size } => {
                        commands::fleet::deployments_list(&cfg, page_size).await?;
                    }
                    FleetDeploymentActions::Get { deployment_id } => {
                        commands::fleet::deployments_get(&cfg, &deployment_id).await?;
                    }
                    FleetDeploymentActions::Cancel { deployment_id } => {
                        commands::fleet::deployments_cancel(&cfg, &deployment_id).await?;
                    }
                    FleetDeploymentActions::Configure { file } => {
                        commands::fleet::deployments_configure(&cfg, &file).await?;
                    }
                    FleetDeploymentActions::Upgrade { file } => {
                        commands::fleet::deployments_upgrade(&cfg, &file).await?;
                    }
                },
                FleetActions::Schedules { action } => match action {
                    FleetScheduleActions::List => commands::fleet::schedules_list(&cfg).await?,
                    FleetScheduleActions::Get { schedule_id } => {
                        commands::fleet::schedules_get(&cfg, &schedule_id).await?;
                    }
                    FleetScheduleActions::Create { file } => {
                        commands::fleet::schedules_create(&cfg, &file).await?;
                    }
                    FleetScheduleActions::Update { schedule_id, file } => {
                        commands::fleet::schedules_update(&cfg, &schedule_id, &file).await?;
                    }
                    FleetScheduleActions::Delete { schedule_id } => {
                        commands::fleet::schedules_delete(&cfg, &schedule_id).await?;
                    }
                    FleetScheduleActions::Trigger { schedule_id } => {
                        commands::fleet::schedules_trigger(&cfg, &schedule_id).await?;
                    }
                },
            }
        }
        // --- Data Governance ---
        Commands::DataGovernance { action } => {
            cfg.validate_auth()?;
            match action {
                DataGovActions::ScannerRules { action } => match action {
                    DataGovScannerActions::List => {
                        commands::data_governance::scanner_rules_list(&cfg).await?;
                    }
                },
            }
        }
        // --- Error Tracking ---
        Commands::ErrorTracking { action } => {
            cfg.validate_auth()?;
            match action {
                ErrorTrackingActions::Issues { action } => match action {
                    ErrorTrackingIssueActions::Search { query, limit } => {
                        commands::error_tracking::issues_search(&cfg, query, limit).await?;
                    }
                    ErrorTrackingIssueActions::Get { issue_id } => {
                        commands::error_tracking::issues_get(&cfg, &issue_id).await?;
                    }
                },
            }
        }
        // --- Code Coverage ---
        Commands::CodeCoverage { action } => {
            cfg.validate_auth()?;
            match action {
                CodeCoverageActions::BranchSummary { repo, branch } => {
                    commands::code_coverage::branch_summary(&cfg, repo, branch).await?;
                }
                CodeCoverageActions::CommitSummary { repo, commit } => {
                    commands::code_coverage::commit_summary(&cfg, repo, commit).await?;
                }
            }
        }
        // --- HAMR ---
        Commands::Hamr { action } => {
            cfg.validate_auth()?;
            match action {
                HamrActions::Connections { action } => match action {
                    HamrConnectionActions::Get => commands::hamr::connections_get(&cfg).await?,
                    HamrConnectionActions::Create { file } => {
                        commands::hamr::connections_create(&cfg, &file).await?;
                    }
                },
            }
        }
        // --- Status Pages ---
        Commands::StatusPages { action } => {
            cfg.validate_auth()?;
            match action {
                StatusPageActions::Pages { action } => match action {
                    StatusPagePageActions::List => commands::status_pages::pages_list(&cfg).await?,
                    StatusPagePageActions::Get { page_id } => {
                        commands::status_pages::pages_get(&cfg, &page_id).await?;
                    }
                    StatusPagePageActions::Create { file } => {
                        commands::status_pages::pages_create(&cfg, &file).await?;
                    }
                    StatusPagePageActions::Update { page_id, file } => {
                        commands::status_pages::pages_update(&cfg, &page_id, &file).await?;
                    }
                    StatusPagePageActions::Delete { page_id } => {
                        commands::status_pages::pages_delete(&cfg, &page_id).await?;
                    }
                },
                StatusPageActions::Components { action } => match action {
                    StatusPageComponentActions::List { page_id } => {
                        commands::status_pages::components_list(&cfg, &page_id).await?;
                    }
                    StatusPageComponentActions::Get {
                        page_id,
                        component_id,
                    } => {
                        commands::status_pages::components_get(&cfg, &page_id, &component_id)
                            .await?;
                    }
                    StatusPageComponentActions::Create { page_id, file } => {
                        commands::status_pages::components_create(&cfg, &page_id, &file).await?;
                    }
                    StatusPageComponentActions::Update {
                        page_id,
                        component_id,
                        file,
                    } => {
                        commands::status_pages::components_update(
                            &cfg,
                            &page_id,
                            &component_id,
                            &file,
                        )
                        .await?;
                    }
                    StatusPageComponentActions::Delete {
                        page_id,
                        component_id,
                    } => {
                        commands::status_pages::components_delete(&cfg, &page_id, &component_id)
                            .await?;
                    }
                },
                StatusPageActions::Degradations { action } => match action {
                    StatusPageDegradationActions::List => {
                        commands::status_pages::degradations_list(&cfg).await?;
                    }
                    StatusPageDegradationActions::Get {
                        page_id,
                        degradation_id,
                    } => {
                        commands::status_pages::degradations_get(&cfg, &page_id, &degradation_id)
                            .await?;
                    }
                    StatusPageDegradationActions::Create { page_id, file } => {
                        commands::status_pages::degradations_create(&cfg, &page_id, &file).await?;
                    }
                    StatusPageDegradationActions::Update {
                        page_id,
                        degradation_id,
                        file,
                    } => {
                        commands::status_pages::degradations_update(
                            &cfg,
                            &page_id,
                            &degradation_id,
                            &file,
                        )
                        .await?;
                    }
                    StatusPageDegradationActions::Delete {
                        page_id,
                        degradation_id,
                    } => {
                        commands::status_pages::degradations_delete(
                            &cfg,
                            &page_id,
                            &degradation_id,
                        )
                        .await?;
                    }
                },
                StatusPageActions::ThirdParty { action } => match action {
                    StatusPageThirdPartyActions::List => {
                        commands::status_pages::third_party_list(&cfg).await?;
                    }
                },
            }
        }
        // --- Integrations ---
        Commands::Integrations { action } => {
            cfg.validate_auth()?;
            match action {
                IntegrationActions::Jira { action } => match action {
                    JiraActions::Accounts { action } => match action {
                        JiraAccountActions::List => {
                            commands::integrations::jira_accounts_list(&cfg).await?
                        }
                        JiraAccountActions::Delete { account_id } => {
                            commands::integrations::jira_accounts_delete(&cfg, &account_id).await?;
                        }
                    },
                    JiraActions::Templates { action } => match action {
                        JiraTemplateActions::List => {
                            commands::integrations::jira_templates_list(&cfg).await?
                        }
                        JiraTemplateActions::Get { template_id } => {
                            commands::integrations::jira_templates_get(&cfg, &template_id).await?;
                        }
                        JiraTemplateActions::Create { file } => {
                            commands::integrations::jira_templates_create(&cfg, &file).await?;
                        }
                        JiraTemplateActions::Update { template_id, file } => {
                            commands::integrations::jira_templates_update(
                                &cfg,
                                &template_id,
                                &file,
                            )
                            .await?;
                        }
                        JiraTemplateActions::Delete { template_id } => {
                            commands::integrations::jira_templates_delete(&cfg, &template_id)
                                .await?;
                        }
                    },
                },
                IntegrationActions::Servicenow { action } => match action {
                    ServiceNowActions::Instances { action } => match action {
                        ServiceNowInstanceActions::List => {
                            commands::integrations::servicenow_instances_list(&cfg).await?;
                        }
                    },
                    ServiceNowActions::Templates { action } => match action {
                        ServiceNowTemplateActions::List => {
                            commands::integrations::servicenow_templates_list(&cfg).await?;
                        }
                        ServiceNowTemplateActions::Get { template_id } => {
                            commands::integrations::servicenow_templates_get(&cfg, &template_id)
                                .await?;
                        }
                        ServiceNowTemplateActions::Create { file } => {
                            commands::integrations::servicenow_templates_create(&cfg, &file)
                                .await?;
                        }
                        ServiceNowTemplateActions::Update { template_id, file } => {
                            commands::integrations::servicenow_templates_update(
                                &cfg,
                                &template_id,
                                &file,
                            )
                            .await?;
                        }
                        ServiceNowTemplateActions::Delete { template_id } => {
                            commands::integrations::servicenow_templates_delete(
                                &cfg,
                                &template_id,
                            )
                            .await?;
                        }
                    },
                    ServiceNowActions::Users { action } => match action {
                        ServiceNowUserActions::List { instance_name } => {
                            commands::integrations::servicenow_users_list(&cfg, &instance_name)
                                .await?;
                        }
                    },
                    ServiceNowActions::AssignmentGroups { action } => match action {
                        ServiceNowAssignmentGroupActions::List { instance_name } => {
                            commands::integrations::servicenow_assignment_groups_list(
                                &cfg,
                                &instance_name,
                            )
                            .await?;
                        }
                    },
                    ServiceNowActions::BusinessServices { action } => match action {
                        ServiceNowBusinessServiceActions::List { instance_name } => {
                            commands::integrations::servicenow_business_services_list(
                                &cfg,
                                &instance_name,
                            )
                            .await?;
                        }
                    },
                },
                IntegrationActions::Slack { action } => match action {
                    SlackActions::List => commands::integrations::slack_list(&cfg).await?,
                },
                IntegrationActions::Pagerduty { action } => match action {
                    PagerdutyActions::List => {
                        commands::integrations::pagerduty_list(&cfg).await?;
                    }
                },
                IntegrationActions::Webhooks { action } => match action {
                    WebhooksActions::List => commands::integrations::webhooks_list(&cfg).await?,
                },
            }
        }
        // --- Cost ---
        Commands::Cost { action } => {
            cfg.validate_auth()?;
            match action {
                CostActions::Projected => commands::cost::projected(&cfg).await?,
                CostActions::ByOrg {
                    start_month,
                    end_month,
                } => {
                    commands::cost::by_org(&cfg, start_month, end_month).await?;
                }
                CostActions::Attribution { start, fields } => {
                    commands::cost::attribution(&cfg, start, fields).await?;
                }
            }
        }
        // --- Misc ---
        Commands::Misc { action } => {
            cfg.validate_auth()?;
            match action {
                MiscActions::IpRanges => commands::misc::ip_ranges(&cfg).await?,
                MiscActions::Status => commands::misc::status(&cfg).await?,
            }
        }
        // --- APM ---
        Commands::Apm { action } => {
            cfg.validate_auth()?;
            match action {
                ApmActions::Services { action } => match action {
                    ApmServiceActions::List { env, from, to } => {
                        commands::apm::services_list(&cfg, env, from, to).await?;
                    }
                    ApmServiceActions::Stats { env, from, to } => {
                        commands::apm::services_stats(&cfg, env, from, to).await?;
                    }
                    ApmServiceActions::Operations {
                        service,
                        env,
                        from,
                        to,
                    } => {
                        commands::apm::services_operations(&cfg, service, env, from, to).await?;
                    }
                    ApmServiceActions::Resources {
                        service,
                        operation,
                        env,
                        from,
                        to,
                    } => {
                        commands::apm::services_resources(&cfg, service, operation, env, from, to)
                            .await?;
                    }
                },
                ApmActions::Entities { action } => match action {
                    ApmEntityActions::List { from, to } => {
                        commands::apm::entities_list(&cfg, from, to).await?;
                    }
                },
                ApmActions::Dependencies { action } => match action {
                    ApmDependencyActions::List { env, from, to } => {
                        commands::apm::dependencies_list(&cfg, env, from, to).await?;
                    }
                },
                ApmActions::FlowMap {
                    query,
                    limit,
                    from,
                    to,
                } => {
                    commands::apm::flow_map(&cfg, query, limit, from, to).await?;
                }
            }
        }
        // --- Investigations ---
        Commands::Investigations { action } => {
            cfg.validate_auth()?;
            match action {
                InvestigationActions::List {
                    page_limit,
                    page_offset,
                } => {
                    commands::investigations::list(&cfg, page_limit, page_offset).await?;
                }
                InvestigationActions::Get { investigation_id } => {
                    commands::investigations::get(&cfg, &investigation_id).await?;
                }
                InvestigationActions::Trigger { file } => {
                    commands::investigations::trigger(&cfg, &file).await?;
                }
            }
        }
        // --- Network (placeholder) ---
        Commands::Network { action } => {
            match action {
                NetworkActions::List => commands::network::list()?,
                NetworkActions::Flows { action } => match action {
                    NetworkFlowActions::List => {
                        cfg.validate_auth()?;
                        commands::network::flows_list(&cfg).await?;
                    }
                },
                NetworkActions::Devices { action } => match action {
                    NetworkDeviceActions::List => {
                        cfg.validate_auth()?;
                        commands::network::devices_list(&cfg).await?;
                    }
                },
            }
        }
        // --- Obs Pipelines (placeholder) ---
        Commands::ObsPipelines { action } => match action {
            ObsPipelinesActions::List => commands::obs_pipelines::list()?,
            ObsPipelinesActions::Get { pipeline_id } => {
                commands::obs_pipelines::get(&pipeline_id)?;
            }
        },
        // --- Scorecards (placeholder) ---
        Commands::Scorecards { action } => match action {
            ScorecardsActions::List => commands::scorecards::list()?,
            ScorecardsActions::Get { scorecard_id } => {
                commands::scorecards::get(&scorecard_id)?;
            }
        },
        // --- Traces (placeholder) ---
        Commands::Traces { action } => match action {
            TracesActions::List => commands::traces::list()?,
        },
        // --- Agent (placeholder) ---
        Commands::Agent { action } => match action {
            AgentActions::Schema => commands::agent::schema()?,
            AgentActions::Guide => commands::agent::guide()?,
        },
        // --- Alias ---
        Commands::Alias { action } => match action {
            AliasActions::List => commands::alias::list()?,
            AliasActions::Set { name, command } => commands::alias::set(name, command)?,
            AliasActions::Delete { names } => commands::alias::delete(names)?,
            AliasActions::Import { file } => commands::alias::import(&file)?,
        },
        // --- Product Analytics ---
        Commands::ProductAnalytics { action } => {
            cfg.validate_auth()?;
            match action {
                ProductAnalyticsActions::Events { action } => match action {
                    ProductAnalyticsEventActions::Send { file } => {
                        commands::product_analytics::events_send(&cfg, &file).await?;
                    }
                },
            }
        }
        // --- Static Analysis ---
        Commands::StaticAnalysis { action } => {
            cfg.validate_auth()?;
            match action {
                StaticAnalysisActions::Ast { action } => match action {
                    StaticAnalysisAstActions::List => {
                        commands::static_analysis::ast_list(&cfg).await?;
                    }
                    StaticAnalysisAstActions::Get { id } => {
                        commands::static_analysis::ast_get(&cfg, &id).await?;
                    }
                },
                StaticAnalysisActions::CustomRulesets { action } => match action {
                    StaticAnalysisCustomRulesetActions::List => {
                        commands::static_analysis::custom_rulesets_list(&cfg).await?;
                    }
                    StaticAnalysisCustomRulesetActions::Get { id } => {
                        commands::static_analysis::custom_rulesets_get(&cfg, &id).await?;
                    }
                },
                StaticAnalysisActions::Sca { action } => match action {
                    StaticAnalysisScaActions::List => {
                        commands::static_analysis::sca_list(&cfg).await?;
                    }
                    StaticAnalysisScaActions::Get { id } => {
                        commands::static_analysis::sca_get(&cfg, &id).await?;
                    }
                },
                StaticAnalysisActions::Coverage { action } => match action {
                    StaticAnalysisCoverageActions::List => {
                        commands::static_analysis::coverage_list(&cfg).await?;
                    }
                    StaticAnalysisCoverageActions::Get { id } => {
                        commands::static_analysis::coverage_get(&cfg, &id).await?;
                    }
                },
            }
        }
        // --- Auth ---
        Commands::Auth { action } => match action {
            AuthActions::Login => commands::auth::login(&cfg).await?,
            AuthActions::Logout => commands::auth::logout(&cfg).await?,
            AuthActions::Status => commands::auth::status(&cfg)?,
            AuthActions::Token => commands::auth::token(&cfg)?,
        },
        // --- Utility ---
        Commands::Completions { shell } => {
            clap_complete::generate(
                shell,
                &mut Cli::command(),
                "pup",
                &mut std::io::stdout(),
            );
        }
        Commands::Version => println!("{}", version::build_info()),
        Commands::Test => commands::test::run(&cfg)?,
    }

    Ok(())
}
