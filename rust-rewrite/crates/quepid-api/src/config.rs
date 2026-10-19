//! API Configuration
//!
//! Configuration for the API server.

use quepid_infra::config::DatabaseBackend;
use serde::Deserialize;
use std::path::PathBuf;

/// Application configuration.
///
/// This struct contains all configuration needed to run the Quepid API server.
/// Configuration can be loaded from environment variables or a configuration file.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Server configuration
    pub server: ServerConfig,
    /// Database configuration
    pub database: DatabaseConfig,
    /// Authentication configuration
    pub auth: AuthConfig,
    /// External services configuration
    pub external: ExternalServicesConfig,
    /// Feature flags
    pub features: FeatureFlags,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            auth: AuthConfig::default(),
            external: ExternalServicesConfig::default(),
            features: FeatureFlags::default(),
        }
    }
}

impl AppConfig {
    /// Load configuration from environment variables.
    pub fn from_env() -> Result<Self, String> {
        let mut config = config::Config::builder()
            .add_source(config::Environment::default().prefix("QUEPID_"))
            .build()
            .map_err(|e| format!("Failed to build config: {e}"))?;

        config
            .try_deserialize()
            .map_err(|e| format!("Failed to deserialize config: {e}"))
    }

    /// Load configuration from a file.
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, String> {
        let mut config = config::Config::builder()
            .add_source(config::File::from(path.as_ref()))
            .add_source(config::Environment::default().prefix("QUEPID_"))
            .build()
            .map_err(|e| format!("Failed to build config: {e}"))?;

        config
            .try_deserialize()
            .map_err(|e| format!("Failed to deserialize config: {e}"))
    }
}

/// Server configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    /// Host to bind to
    pub host: String,
    /// Port to bind to
    pub port: u16,
    /// Path to static assets
    pub public_path: Option<PathBuf>,
    /// Enable CORS
    pub cors_enabled: bool,
    /// Allowed origins for CORS
    pub cors_origins: Vec<String>,
    /// Rate limit requests per minute
    pub rate_limit: Option<u64>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 3000,
            public_path: None,
            cors_enabled: true,
            cors_origins: vec!["*".to_string()],
            rate_limit: None,
        }
    }
}

/// Database configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    /// Database backend type
    pub backend: DatabaseBackend,
    /// Connection string
    pub connection_string: String,
    /// Maximum connection pool size
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connect_timeout: u64,
    /// Query timeout in seconds
    pub query_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            backend: DatabaseBackend::MySql,
            connection_string: "mysql://root:password@localhost/quepid".to_string(),
            max_connections: 10,
            connect_timeout: 30,
            query_timeout: 30,
        }
    }
}

/// Authentication configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    /// JWT secret key
    pub jwt_secret: String,
    /// JWT expiration in hours
    pub jwt_expiration_hours: u64,
    /// Enable OAuth2
    pub oauth_enabled: bool,
    /// OAuth2 providers
    pub oauth_providers: Vec<OAuthProviderConfig>,
    /// Enable OpenID Connect
    pub oidc_enabled: bool,
    /// OpenID Connect configuration
    pub oidc_config: Option<OIDCConfig>,
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            jwt_secret: "change-me-in-production".to_string(),
            jwt_expiration_hours: 24,
            oauth_enabled: false,
            oauth_providers: Vec::new(),
            oidc_enabled: false,
            oidc_config: None,
        }
    }
}

/// OAuth2 provider configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct OAuthProviderConfig {
    /// Provider name
    pub name: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Authorization URL
    pub auth_url: String,
    /// Token URL
    pub token_url: String,
    /// User info URL
    pub user_info_url: String,
    /// Callback URL
    pub callback_url: String,
}

/// OpenID Connect configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct OIDCConfig {
    /// Issuer URL
    pub issuer: String,
    /// Client ID
    pub client_id: String,
    /// Client secret
    pub client_secret: String,
    /// Discovery endpoint
    pub discovery_endpoint: String,
    /// Callback URL
    pub callback_url: String,
}

/// External services configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct ExternalServicesConfig {
    /// Search engine configurations
    pub search_engines: Vec<SearchEngineConfig>,
    /// LLM judge configurations
    pub llm_judges: Vec<LLMJudgeConfig>,
    /// Email service configuration
    pub email: EmailConfig,
}

impl Default for ExternalServicesConfig {
    fn default() -> Self {
        Self {
            search_engines: Vec::new(),
            llm_judges: Vec::new(),
            email: EmailConfig::default(),
        }
    }
}

/// Search engine configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct SearchEngineConfig {
    /// Engine name
    pub name: String,
    /// Engine type (solr, opensearch, elasticsearch, custom)
    pub engine_type: String,
    /// Base URL
    pub base_url: String,
    /// Authentication token (if needed)
    pub auth_token: Option<String>,
}

/// LLM judge configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct LLMJudgeConfig {
    /// Judge name
    pub name: String,
    /// Provider (openai, anthropic, google, ollama)
    pub provider: String,
    /// API key
    pub api_key: String,
    /// Model to use
    pub model: String,
    /// Base URL (for custom providers)
    pub base_url: Option<String>,
}

/// Email configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct EmailConfig {
    /// Enable email sending
    pub enabled: bool,
    /// SMTP host
    pub smtp_host: String,
    /// SMTP port
    pub smtp_port: u16,
    /// SMTP username
    pub smtp_username: String,
    /// SMTP password
    pub smtp_password: String,
    /// From address
    pub from_address: String,
    /// Use TLS
    pub use_tls: bool,
}

impl Default for EmailConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            smtp_host: "localhost".to_string(),
            smtp_port: 587,
            smtp_username: "".to_string(),
            smtp_password: "".to_string(),
            from_address: "noreply@quepid.com".to_string(),
            use_tls: true,
        }
    }
}

/// Feature flags.
#[derive(Debug, Clone, Deserialize)]
pub struct FeatureFlags {
    /// Enable registration
    pub registration_enabled: bool,
    /// Enable AI judging
    pub ai_judging_enabled: bool,
    /// Enable WebSocket
    pub websocket_enabled: bool,
    /// Enable background jobs
    pub jobs_enabled: bool,
    /// Enable analytics
    pub analytics_enabled: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            registration_enabled: true,
            ai_judging_enabled: true,
            websocket_enabled: true,
            jobs_enabled: true,
            analytics_enabled: true,
        }
    }
}
