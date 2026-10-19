//! Database Connection
//!
//! Manages database connections for SeaORM.

use sea_orm::{Database, DatabaseConnection, DbErr};
use std::sync::Arc;
use tokio::sync::OnceCell;

/// Database connection manager.
///
/// This struct holds a connection pool and provides methods to get connections.
#[derive(Debug, Clone)]
pub struct DatabaseConnection {
    pool: Arc<OnceCell<DatabaseConnection>>,
    connection_string: String,
}

impl DatabaseConnection {
    /// Create a new database connection manager.
    ///
    /// # Arguments
    ///
    /// * `connection_string` - The database connection URL
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::env;
    ///
    /// let connection_string = env::var("DATABASE_URL")
    ///     .unwrap_or("mysql://user:password@localhost/quepid".to_string());
    /// let db = DatabaseConnection::new(connection_string);
    /// ```
    pub fn new(connection_string: impl Into<String>) -> Self {
        Self {
            pool: Arc::new(OnceCell::new()),
            connection_string: connection_string.into(),
        }
    }

    /// Get a database connection from the pool.
    ///
    /// This will establish the connection on first use and reuse it for
    /// subsequent calls.
    pub async fn get(&self) -> &DatabaseConnection {
        self.pool
            .get_or_init(|| async {
                Self::create_connection(&self.connection_string).await
            })
            .await
    }

    /// Create a new database connection.
    async fn create_connection(connection_string: &str) -> DatabaseConnection {
        Database::connect(connection_string)
            .await
            .expect("Failed to connect to database")
    }

    /// Get the connection string.
    pub fn connection_string(&self) -> &str {
        &self.connection_string
    }

    /// Create a connection manager from environment variables.
    ///
    /// Uses `DATABASE_URL` environment variable by default.
    pub fn from_env() -> Self {
        let connection_string = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "mysql://root:password@localhost/quepid".to_string());

        Self::new(connection_string)
    }
}

impl Default for DatabaseConnection {
    fn default() -> Self {
        Self::from_env()
    }
}

/// Database configuration for different backends.
#[derive(Debug, Clone)]
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

/// Database backend types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DatabaseBackend {
    /// MySQL database
    MySql,
    /// PostgreSQL database
    Postgres,
    /// SQLite database
    Sqlite,
}

impl std::fmt::Display for DatabaseBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseBackend::MySql => write!(f, "mysql"),
            DatabaseBackend::Postgres => write!(f, "postgres"),
            DatabaseBackend::Sqlite => write!(f, "sqlite"),
        }
    }
}

impl DatabaseBackend {
    /// Detect the backend from a connection string.
    pub fn from_connection_string(connection_string: &str) -> Self {
        if connection_string.starts_with("mysql://") || connection_string.starts_with("mysql2://") {
            DatabaseBackend::MySql
        } else if connection_string.starts_with("postgres://") || connection_string.starts_with("postgresql://") {
            DatabaseBackend::Postgres
        } else if connection_string.starts_with("sqlite://") || connection_string.contains(".sqlite") {
            DatabaseBackend::Sqlite
        } else {
            // Default to MySQL for backward compatibility
            DatabaseBackend::MySql
        }
    }
}

/// Create a database connection with configuration.
pub async fn create_database_connection(config: &DatabaseConfig) -> Result<DatabaseConnection, DbErr> {
    let mut connection_options = sea_orm::ConnectOptions::new(config.connection_string.clone());
    connection_options.max_connections(config.max_connections);
    connection_options.connect_timeout(std::time::Duration::from_secs(config.connect_timeout));
    connection_options.query_timeout(std::time::Duration::from_secs(config.query_timeout));

    Database::connect(connection_options).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_backend_detection() {
        assert_eq!(
            DatabaseBackend::from_connection_string("mysql://user:pass@host/db"),
            DatabaseBackend::MySql
        );
        assert_eq!(
            DatabaseBackend::from_connection_string("postgres://user:pass@host/db"),
            DatabaseBackend::Postgres
        );
        assert_eq!(
            DatabaseBackend::from_connection_string("sqlite://path/to/db.sqlite"),
            DatabaseBackend::Sqlite
        );
    }

    #[test]
    fn test_database_connection_creation() {
        let connection_string = "mysql://user:pass@localhost/quepid";
        let db = DatabaseConnection::new(connection_string);

        assert_eq!(db.connection_string(), connection_string);
    }
}
