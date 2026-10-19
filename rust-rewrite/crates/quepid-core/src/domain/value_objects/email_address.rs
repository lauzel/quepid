//! Email Address Value Object
//!
//! Represents a validated email address.

use std::fmt;
use std::str::FromStr;
use validator::Validate;

/// Email address value object.
///
/// This ensures that all email addresses in the system are valid
/// and properly formatted.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Create a new email address after validation.
    ///
    /// # Errors
    ///
    /// Returns an error if the email address is invalid.
    pub fn new(email: impl Into<String>) -> Result<Self, String> {
        let email = email.into();
        Self::validate(&email)?;
        Ok(Self(email))
    }

    /// Create a new email address without validation.
    ///
    /// # Safety
    ///
    /// This is unsafe because it doesn't validate the email address.
    /// Only use this when you're certain the email is valid.
    pub unsafe fn new_unchecked(email: impl Into<String>) -> Self {
        Self(email.into())
    }

    /// Get the email address as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert to owned string.
    pub fn to_string(&self) -> String {
        self.0.clone()
    }

    /// Validate an email address string.
    pub fn validate(email: &str) -> Result<(), String> {
        // Basic validation
        if email.is_empty() {
            return Err("Email address cannot be empty".to_string());
        }

        if email.len() > 254 {
            return Err("Email address cannot exceed 254 characters".to_string());
        }

        // Split into local and domain parts
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return Err("Email address must contain exactly one '@' symbol".to_string());
        }

        let local = parts[0];
        let domain = parts[1];

        // Validate local part
        if local.is_empty() || local.len() > 64 {
            return Err("Local part must be between 1 and 64 characters".to_string());
        }

        // Validate domain part
        if domain.is_empty() || domain.len() > 255 {
            return Err("Domain part must be between 1 and 255 characters".to_string());
        }

        // Check for valid characters
        if !email.chars().all(|c| c.is_ascii() && !c.is_control()) {
            return Err("Email address must contain only ASCII characters".to_string());
        }

        // Check for dots at start or end
        if local.starts_with('.') || local.ends_with('.') {
            return Err("Local part cannot start or end with a dot".to_string());
        }

        if domain.starts_with('.') || domain.ends_with('.') {
            return Err("Domain part cannot start or end with a dot".to_string());
        }

        // Check for consecutive dots
        if local.contains("..") || domain.contains("..") {
            return Err("Email address cannot contain consecutive dots".to_string());
        }

        Ok(())
    }
}

impl FromStr for EmailAddress {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<EmailAddress> for String {
    fn from(email: EmailAddress) -> String {
        email.0
    }
}

impl PartialEq<str> for EmailAddress {
    fn eq(&self, other: &str) -> bool {
        self.0 == other
    }
}

impl PartialEq<&str> for EmailAddress {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_email_addresses() {
        let valid_emails = vec![
            "user@example.com",
            "first.last@sub.domain.com",
            "user+tag@example.com",
            "user.name@example.co.uk",
            "user@example-domain.com",
        ];

        for email in valid_emails {
            assert!(EmailAddress::new(email).is_ok(), "Failed for: {email}");
        }
    }

    #[test]
    fn test_invalid_email_addresses() {
        let invalid_emails = vec![
            ("", "empty"),
            ("@example.com", "missing local"),
            ("user@", "missing domain"),
            ("user@example", "missing tld"),
            ("user..name@example.com", "consecutive dots"),
            (".user@example.com", "starts with dot"),
            ("user.@example.com", "ends with dot"),
            ("user@example..com", "domain consecutive dots"),
            ("user@.example.com", "domain starts with dot"),
            ("user@example.com.", "domain ends with dot"),
        ];

        for (email, reason) in invalid_emails {
            assert!(
                EmailAddress::new(email).is_err(),
                "Should have failed for: {email} ({reason})"
            );
        }
    }

    #[test]
    fn test_email_address_operations() {
        let email = EmailAddress::new("user@example.com").unwrap();

        assert_eq!(email.as_str(), "user@example.com");
        assert_eq!(email.to_string(), "user@example.com");
        assert_eq!(format!("{email}"), "user@example.com");
        assert_eq!(email, "user@example.com");
        assert_eq!(&email, "user@example.com");
    }

    #[test]
    fn test_email_address_equality() {
        let email1 = EmailAddress::new("user@example.com").unwrap();
        let email2 = EmailAddress::new("user@example.com").unwrap();
        let email3 = EmailAddress::new("other@example.com").unwrap();

        assert_eq!(email1, email2);
        assert_ne!(email1, email3);
    }
}
