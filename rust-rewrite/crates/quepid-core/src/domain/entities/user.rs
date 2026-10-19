//! User Entity
//!
//! Represents a user in the Quepid system.

use crate::domain::value_objects::EmailAddress;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// User entity representing a person who can authenticate and use Quepid.
///
/// A User:
/// - Can own Cases, Books, Scorers, etc.
/// - Can be a member of Teams
/// - Can have admin privileges
/// - Has authentication credentials
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier for the user
    pub id: Uuid,
    /// User's email address (unique)
    pub email: EmailAddress,
    /// User's display name
    pub name: String,
    /// Hashed password
    pub password_digest: String,
    /// Whether the user has admin privileges
    pub admin: bool,
    /// Whether the user has confirmed their email
    pub confirmed_at: Option<DateTime<Utc>>,
    /// When the user was created
    pub created_at: DateTime<Utc>,
    /// When the user was last updated
    pub updated_at: DateTime<Utc>,
    /// Last login timestamp
    pub last_sign_in_at: Option<DateTime<Utc>>,
    /// Current sign in IP address
    pub current_sign_in_ip: Option<String>,
    /// Last sign in IP address
    pub last_sign_in_ip: Option<String>,
    /// Number of times user has signed in
    pub sign_in_count: i32,
    /// User's time zone
    pub time_zone: Option<String>,
    /// User's preferred locale
    pub locale: Option<String>,
}

impl User {
    /// Create a new user with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `email` - User's email address
    /// * `name` - User's display name
    /// * `password_digest` - Hashed password
    /// * `admin` - Whether the user has admin privileges
    ///
    /// # Returns
    ///
    /// A new User instance with current timestamps.
    pub fn new(
        email: EmailAddress,
        name: impl Into<String>,
        password_digest: impl Into<String>,
        admin: bool,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            name: name.into(),
            password_digest: password_digest.into(),
            admin,
            confirmed_at: None,
            created_at: now,
            updated_at: now,
            last_sign_in_at: None,
            current_sign_in_ip: None,
            last_sign_in_ip: None,
            sign_in_count: 0,
            time_zone: None,
            locale: None,
        }
    }

    /// Check if the user is confirmed.
    pub fn is_confirmed(&self) -> bool {
        self.confirmed_at.is_some()
    }

    /// Mark the user as confirmed.
    pub fn confirm(&mut self) {
        self.confirmed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Update the user's last sign in information.
    pub fn update_last_sign_in(&mut self, ip_address: Option<String>) {
        self.last_sign_in_at = Some(Utc::now());
        self.last_sign_in_ip = self.current_sign_in_ip.take();
        self.current_sign_in_ip = ip_address;
        self.sign_in_count += 1;
        self.updated_at = Utc::now();
    }

    /// Update the user's profile information.
    pub fn update_profile(
        &mut self,
        name: Option<String>,
        email: Option<EmailAddress>,
        time_zone: Option<String>,
        locale: Option<String>,
    ) {
        if let Some(name) = name {
            self.name = name;
        }
        if let Some(email) = email {
            self.email = email;
        }
        if let Some(time_zone) = time_zone {
            self.time_zone = Some(time_zone);
        }
        if let Some(locale) = locale {
            self.locale = Some(locale);
        }
        self.updated_at = Utc::now();
    }

    /// Update the user's password.
    pub fn update_password(&mut self, new_password_digest: impl Into<String>) {
        self.password_digest = new_password_digest.into();
        self.updated_at = Utc::now();
    }

    /// Check if the user is an admin.
    pub fn is_admin(&self) -> bool {
        self.admin
    }

    /// Set admin status.
    pub fn set_admin(&mut self, admin: bool) {
        self.admin = admin;
        self.updated_at = Utc::now();
    }
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "User(id: {}, email: {}, name: {}, admin: {})",
            self.id, self.email, self.name, self.admin
        )
    }
}

/// User creation parameters (for registration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub email: EmailAddress,
    pub name: String,
    pub password: String,
    pub admin: bool,
}

/// User update parameters (for profile updates)
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<EmailAddress>,
    pub password: Option<String>,
    pub time_zone: Option<String>,
    pub locale: Option<String>,
    pub admin: Option<bool>,
}

/// User response (for API responses, excludes sensitive data)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: EmailAddress,
    pub name: String,
    pub admin: bool,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_sign_in_at: Option<DateTime<Utc>>,
    pub sign_in_count: i32,
    pub time_zone: Option<String>,
    pub locale: Option<String>,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            name: user.name.clone(),
            admin: user.admin,
            confirmed_at: user.confirmed_at,
            created_at: user.created_at,
            updated_at: user.updated_at,
            last_sign_in_at: user.last_sign_in_at,
            sign_in_count: user.sign_in_count,
            time_zone: user.time_zone.clone(),
            locale: user.locale.clone(),
        }
    }
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self::from(&user)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_user() {
        let email = EmailAddress::new("user@example.com").unwrap();
        let user = User::new(email.clone(), "Test User", "hashed_password", false);

        assert_eq!(user.email, email);
        assert_eq!(user.name, "Test User");
        assert_eq!(user.password_digest, "hashed_password");
        assert!(!user.admin);
        assert!(!user.is_confirmed());
        assert!(user.is_admin() == false);
    }

    #[test]
    fn test_user_confirmation() {
        let email = EmailAddress::new("user@example.com").unwrap();
        let mut user = User::new(email, "Test User", "password", false);

        assert!(!user.is_confirmed());

        user.confirm();

        assert!(user.is_confirmed());
    }

    #[test]
    fn test_user_sign_in() {
        let email = EmailAddress::new("user@example.com").unwrap();
        let mut user = User::new(email, "Test User", "password", false);

        assert_eq!(user.sign_in_count, 0);
        assert!(user.last_sign_in_at.is_none());

        user.update_last_sign_in(Some("192.168.1.1".to_string()));

        assert_eq!(user.sign_in_count, 1);
        assert!(user.last_sign_in_at.is_some());
        assert_eq!(user.current_sign_in_ip, Some("192.168.1.1".to_string()));
    }

    #[test]
    fn test_user_admin() {
        let email = EmailAddress::new("user@example.com").unwrap();
        let mut user = User::new(email, "Test User", "password", false);

        assert!(!user.is_admin());

        user.set_admin(true);

        assert!(user.is_admin());
    }

    #[test]
    fn test_user_response() {
        let email = EmailAddress::new("user@example.com").unwrap();
        let user = User::new(email.clone(), "Test User", "password", true);

        let response = UserResponse::from(user);

        assert_eq!(response.email, email);
        assert_eq!(response.name, "Test User");
        assert!(response.admin);
    }
}
