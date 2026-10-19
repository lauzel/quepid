//! User Repository Port
//!
//! Defines the interface for user persistence operations.

use crate::domain::entities::User;
use crate::domain::value_objects::EmailAddress;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

/// Trait defining user repository operations.
///
/// This trait is implemented in the `quepid-infra` crate for different
/// database backends (MySQL, PostgreSQL, SQLite).
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Find a user by ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String>;

    /// Find a user by email address.
    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>, String>;

    /// Find multiple users by IDs.
    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>, String>;

    /// Create a new user.
    async fn create(&self, user: User) -> Result<User, String>;

    /// Update an existing user.
    async fn update(&self, user: User) -> Result<User, String>;

    /// Delete a user by ID.
    async fn delete(&self, id: Uuid) -> Result<bool, String>;

    /// List users with optional filters.
    async fn list(
        &self,
        page: Option<i64>,
        per_page: Option<i64>,
        search: Option<String>,
        admin: Option<bool>,
    ) -> Result<(Vec<User>, i64), String>;

    /// Check if an email address is available (not taken by another user).
    async fn is_email_available(&self, email: &EmailAddress, exclude_id: Option<Uuid>) -> Result<bool, String>;

    /// Get the total number of users.
    async fn count(&self) -> Result<i64, String>;

    /// Get users by team membership.
    async fn find_by_team_id(&self, team_id: Uuid) -> Result<Vec<User>, String>;

    /// Get users who are team owners.
    async fn find_team_owners(&self) -> Result<Vec<User>, String>;
}

/// Shared user repository implementation that can be used for testing.
#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
    inner: Arc<dyn UserRepository>,
}

impl UserRepositoryImpl {
    /// Create a new UserRepositoryImpl with the given implementation.
    pub fn new(inner: Arc<dyn UserRepository>) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        self.inner.find_by_id(id).await
    }

    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>, String> {
        self.inner.find_by_email(email).await
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>, String> {
        self.inner.find_by_ids(ids).await
    }

    async fn create(&self, user: User) -> Result<User, String> {
        self.inner.create(user).await
    }

    async fn update(&self, user: User) -> Result<User, String> {
        self.inner.update(user).await
    }

    async fn delete(&self, id: Uuid) -> Result<bool, String> {
        self.inner.delete(id).await
    }

    async fn list(
        &self,
        page: Option<i64>,
        per_page: Option<i64>,
        search: Option<String>,
        admin: Option<bool>,
    ) -> Result<(Vec<User>, i64), String> {
        self.inner.list(page, per_page, search, admin).await
    }

    async fn is_email_available(&self, email: &EmailAddress, exclude_id: Option<Uuid>) -> Result<bool, String> {
        self.inner.is_email_available(email, exclude_id).await
    }

    async fn count(&self) -> Result<i64, String> {
        self.inner.count().await
    }

    async fn find_by_team_id(&self, team_id: Uuid) -> Result<Vec<User>, String> {
        self.inner.find_by_team_id(team_id).await
    }

    async fn find_team_owners(&self) -> Result<Vec<User>, String> {
        self.inner.find_team_owners().await
    }
}

/// Mock user repository for testing.
#[cfg(test)]
pub struct MockUserRepository;

#[cfg(test)]
#[async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(&self, _id: Uuid) -> Result<Option<User>, String> {
        Ok(None)
    }

    async fn find_by_email(&self, _email: &EmailAddress) -> Result<Option<User>, String> {
        Ok(None)
    }

    async fn find_by_ids(&self, _ids: &[Uuid]) -> Result<Vec<User>, String> {
        Ok(Vec::new())
    }

    async fn create(&self, _user: User) -> Result<User, String> {
        Err("Mock not implemented".to_string())
    }

    async fn update(&self, _user: User) -> Result<User, String> {
        Err("Mock not implemented".to_string())
    }

    async fn delete(&self, _id: Uuid) -> Result<bool, String> {
        Ok(false)
    }

    async fn list(
        &self,
        _page: Option<i64>,
        _per_page: Option<i64>,
        _search: Option<String>,
        _admin: Option<bool>,
    ) -> Result<(Vec<User>, i64), String> {
        Ok((Vec::new(), 0))
    }

    async fn is_email_available(&self, _email: &EmailAddress, _exclude_id: Option<Uuid>) -> Result<bool, String> {
        Ok(true)
    }

    async fn count(&self) -> Result<i64, String> {
        Ok(0)
    }

    async fn find_by_team_id(&self, _team_id: Uuid) -> Result<Vec<User>, String> {
        Ok(Vec::new())
    }

    async fn find_team_owners(&self) -> Result<Vec<User>, String> {
        Ok(Vec::new())
    }
}
