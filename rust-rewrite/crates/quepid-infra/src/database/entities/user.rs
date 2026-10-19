//! User Entity
//!
//! SeaORM entity definition for the User table.

use chrono::{DateTime, Utc};
use sea_orm::{entity::prelude::*, ActiveValue};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User entity for SeaORM.
#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub password_digest: String,
    pub admin: bool,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub current_sign_in_ip: Option<String>,
    pub last_sign_in_ip: Option<String>,
    pub sign_in_count: i32,
    pub time_zone: Option<String>,
    pub locale: Option<String>,
    pub encrypted_otp_secret: Option<String>,
    pub encrypted_otp_secret_iv: Option<String>,
    pub encrypted_otp_secret_salt: Option<String>,
    pub consumption_preference: Option<String>,
    pub failed_attempts: i32,
    pub unlock_token: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub authentication_token: Option<String>,
}

/// Active model for User updates.
#[derive(Clone, Debug, PartialEq, DeriveIntoActiveModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct ActiveModel {
    pub id: ActiveValue<Uuid>,
    pub email: ActiveValue<String>,
    pub name: ActiveValue<String>,
    pub password_digest: ActiveValue<String>,
    pub admin: ActiveValue<bool>,
    pub confirmed_at: ActiveValue<Option<DateTime<Utc>>>,
    pub created_at: ActiveValue<DateTime<Utc>>,
    pub updated_at: ActiveValue<DateTime<Utc>>,
    pub last_sign_in_at: ActiveValue<Option<DateTime<Utc>>>,
    pub current_sign_in_ip: ActiveValue<Option<String>>,
    pub last_sign_in_ip: ActiveValue<Option<String>>,
    pub sign_in_count: ActiveValue<i32>,
    pub time_zone: ActiveValue<Option<String>>,
    pub locale: ActiveValue<Option<String>>,
    pub encrypted_otp_secret: ActiveValue<Option<String>>,
    pub encrypted_otp_secret_iv: ActiveValue<Option<String>>,
    pub encrypted_otp_secret_salt: ActiveValue<Option<String>>,
    pub consumption_preference: ActiveValue<Option<String>>,
    pub failed_attempts: ActiveValue<i32>,
    pub unlock_token: ActiveValue<Option<String>>,
    pub locked_at: ActiveValue<Option<DateTime<Utc>>>,
    pub authentication_token: ActiveValue<Option<String>>,
}

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "users"
    }
}

/// User entity wrapper with additional methods.
impl Model {
    /// Convert to domain User.
    pub fn to_domain(&self) -> quepid_core::domain::entities::User {
        use quepid_core::domain::value_objects::EmailAddress;

        quepid_core::domain::entities::User {
            id: self.id,
            email: EmailAddress::new_unchecked(self.email.clone()),
            name: self.name.clone(),
            password_digest: self.password_digest.clone(),
            admin: self.admin,
            confirmed_at: self.confirmed_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
            last_sign_in_at: self.last_sign_in_at,
            current_sign_in_ip: self.current_sign_in_ip.clone(),
            last_sign_in_ip: self.last_sign_in_ip.clone(),
            sign_in_count: self.sign_in_count,
            time_zone: self.time_zone.clone(),
            locale: self.locale.clone(),
        }
    }

    /// Create from domain User.
    pub fn from_domain(user: &quepid_core::domain::entities::User) -> Self {
        Self {
            id: user.id,
            email: user.email.to_string(),
            name: user.name.clone(),
            password_digest: user.password_digest.clone(),
            admin: user.admin,
            confirmed_at: user.confirmed_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
            last_sign_in_at: user.last_sign_in_at,
            current_sign_in_ip: user.current_sign_in_ip.clone(),
            last_sign_in_ip: user.last_sign_in_ip.clone(),
            sign_in_count: user.sign_in_count,
            time_zone: user.time_zone.clone(),
            locale: user.locale.clone(),
            encrypted_otp_secret: None,
            encrypted_otp_secret_iv: None,
            encrypted_otp_secret_salt: None,
            consumption_preference: None,
            failed_attempts: 0,
            unlock_token: None,
            locked_at: None,
            authentication_token: None,
        }
    }
}

/// Relation definitions for User.
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::team::Entity")]
    Teams,
    #[sea_orm(has_many = "super::case::Entity")]
    Cases,
    #[sea_orm(has_many = "super::book::Entity")]
    Books,
    #[sea_orm(has_many = "super::scorer::Entity")]
    Scorers,
    #[sea_orm(has_many = "super::api_key::Entity")]
    ApiKeys,
    #[sea_orm(has_many = "super::ai_judge::Entity")]
    AiJudges,
}

impl Related<super::team::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Teams.def()
    }
}

impl Related<super::case::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cases.def()
    }
}

impl Related<super::book::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Books.def()
    }
}

impl Related<super::scorer::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Scorers.def()
    }
}

impl Related<super::api_key::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiKeys.def()
    }
}

impl Related<super::ai_judge::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AiJudges.def()
    }
}
