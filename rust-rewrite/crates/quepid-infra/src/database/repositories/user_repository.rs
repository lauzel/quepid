//! User Repository Implementation
//!
//! SeaORM-based implementation of the UserRepository trait.

use async_trait::async_trait;
use quepid_core::domain::entities::User;
use quepid_core::domain::value_objects::EmailAddress;
use quepid_core::ports::repositories::UserRepository as UserRepositoryTrait;
use sea_orm::{EntityTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait};
use std::sync::Arc;
use uuid::Uuid;

use super::super::entities::user as user_entity;
use super::super::connection::DatabaseConnection;

/// User repository implementation using SeaORM.
#[derive(Debug, Clone)]
pub struct UserRepository {
    connection: Arc<DatabaseConnection>,
}

impl UserRepository {
    /// Create a new UserRepository with the given database connection.
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }
}

#[async_trait]
impl UserRepositoryTrait for UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        let conn = self.connection.get().await;

        let user = user_entity::Entity::find_by_id(id)
            .one(&conn)
            .await
            .map_err(|e| format!("Failed to find user by id: {e}"))?;

        Ok(user.map(|u| u.to_domain()))
    }

    async fn find_by_email(&self, email: &EmailAddress) -> Result<Option<User>, String> {
        let conn = self.connection.get().await;

        let user = user_entity::Entity::find()
            .filter(user_entity::Column::Email.eq(email.as_str()))
            .one(&conn)
            .await
            .map_err(|e| format!("Failed to find user by email: {e}"))?;

        Ok(user.map(|u| u.to_domain()))
    }

    async fn find_by_ids(&self, ids: &[Uuid]) -> Result<Vec<User>, String> {
        let conn = self.connection.get().await;

        let users = user_entity::Entity::find()
            .filter(user_entity::Column::Id.is_in(ids))
            .all(&conn)
            .await
            .map_err(|e| format!("Failed to find users by ids: {e}"))?;

        Ok(users.into_iter().map(|u| u.to_domain()).collect())
    }

    async fn create(&self, user: User) -> Result<User, String> {
        let conn = self.connection.get().await;

        let active_model = user_entity::ActiveModel::from_domain(&user);

        let created = active_model
            .insert(&conn)
            .await
            .map_err(|e| format!("Failed to create user: {e}"))?;

        Ok(created.to_domain())
    }

    async fn update(&self, user: User) -> Result<User, String> {
        let conn = self.connection.get().await;

        let active_model = user_entity::ActiveModel::from_domain(&user);

        let updated = active_model
            .update(&conn)
            .await
            .map_err(|e| format!("Failed to update user: {e}"))?;

        Ok(updated.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<bool, String> {
        let conn = self.connection.get().await;

        let result = user_entity::Entity::delete_by_id(id)
            .exec(&conn)
            .await
            .map_err(|e| format!("Failed to delete user: {e}"))?;

        Ok(result.rows_affected > 0)
    }

    async fn list(
        &self,
        page: Option<i64>,
        per_page: Option<i64>,
        search: Option<String>,
        admin: Option<bool>,
    ) -> Result<(Vec<User>, i64), String> {
        let conn = self.connection.get().await;

        let mut query = user_entity::Entity::find();

        if let Some(search) = search {
            query = query.filter(
                user_entity::Column::Name
                    .contains(&search)
                    .or(user_entity::Column::Email.contains(&search)),
            );
        }

        if let Some(admin) = admin {
            query = query.filter(user_entity::Column::Admin.eq(admin));
        }

        // Get total count
        let total = query
            .clone()
            .count(&conn)
            .await
            .map_err(|e| format!("Failed to count users: {e}"))?;

        // Apply pagination
        let page = page.unwrap_or(1);
        let per_page = per_page.unwrap_or(20);
        let offset = (page - 1) * per_page;

        let users = query
            .offset(offset)
            .limit(per_page)
            .order_by(user_entity::Column::CreatedAt, QueryOrder::Desc)
            .all(&conn)
            .await
            .map_err(|e| format!("Failed to list users: {e}"))?;

        Ok((users.into_iter().map(|u| u.to_domain()).collect(), total))
    }

    async fn is_email_available(&self, email: &EmailAddress, exclude_id: Option<Uuid>) -> Result<bool, String> {
        let conn = self.connection.get().await;

        let mut query = user_entity::Entity::find().filter(user_entity::Column::Email.eq(email.as_str()));

        if let Some(id) = exclude_id {
            query = query.filter(user_entity::Column::Id.ne(id));
        }

        let exists = query
            .count(&conn)
            .await
            .map_err(|e| format!("Failed to check email availability: {e}"))?;

        Ok(exists == 0)
    }

    async fn count(&self) -> Result<i64, String> {
        let conn = self.connection.get().await;

        let count = user_entity::Entity::find()
            .count(&conn)
            .await
            .map_err(|e| format!("Failed to count users: {e}"))?;

        Ok(count)
    }

    async fn find_by_team_id(&self, team_id: Uuid) -> Result<Vec<User>, String> {
        // This requires the team_members join table
        // For now, return empty - will be implemented with proper join
        Ok(Vec::new())
    }

    async fn find_team_owners(&self) -> Result<Vec<User>, String> {
        // This requires querying the teams table
        // For now, return empty - will be implemented with proper join
        Ok(Vec::new())
    }
}

// Implement From<&User> for user_entity::ActiveModel
impl From<&User> for user_entity::ActiveModel {
    fn from(user: &User) -> Self {
        Self {
            id: sea_orm::ActiveValue::Set(user.id),
            email: sea_orm::ActiveValue::Set(user.email.to_string()),
            name: sea_orm::ActiveValue::Set(user.name.clone()),
            password_digest: sea_orm::ActiveValue::Set(user.password_digest.clone()),
            admin: sea_orm::ActiveValue::Set(user.admin),
            confirmed_at: sea_orm::ActiveValue::Set(user.confirmed_at),
            created_at: sea_orm::ActiveValue::Set(user.created_at),
            updated_at: sea_orm::ActiveValue::Set(user.updated_at),
            last_sign_in_at: sea_orm::ActiveValue::Set(user.last_sign_in_at),
            current_sign_in_ip: sea_orm::ActiveValue::Set(user.current_sign_in_ip.clone()),
            last_sign_in_ip: sea_orm::ActiveValue::Set(user.last_sign_in_ip.clone()),
            sign_in_count: sea_orm::ActiveValue::Set(user.sign_in_count),
            time_zone: sea_orm::ActiveValue::Set(user.time_zone.clone()),
            locale: sea_orm::ActiveValue::Set(user.locale.clone()),
            encrypted_otp_secret: sea_orm::ActiveValue::NotSet,
            encrypted_otp_secret_iv: sea_orm::ActiveValue::NotSet,
            encrypted_otp_secret_salt: sea_orm::ActiveValue::NotSet,
            consumption_preference: sea_orm::ActiveValue::NotSet,
            failed_attempts: sea_orm::ActiveValue::NotSet,
            unlock_token: sea_orm::ActiveValue::NotSet,
            locked_at: sea_orm::ActiveValue::NotSet,
            authentication_token: sea_orm::ActiveValue::NotSet,
        }
    }
}
