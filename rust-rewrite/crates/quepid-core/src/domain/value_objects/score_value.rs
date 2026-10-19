//! Score Value Object
//!
//! Represents a score value between 0.0 and 100.0.

use std::fmt;
use std::str::FromStr;

/// Score value object.
///
/// Scores in Quepid are represented as percentages (0-100).
/// This ensures that all scores are within valid bounds.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct ScoreValue(f64);

impl ScoreValue {
    /// Minimum valid score value
    pub const MIN: f64 = 0.0;

    /// Maximum valid score value
    pub const MAX: f64 = 100.0;

    /// Create a new score value after validation.
    ///
    /// # Arguments
    ///
    /// * `value` - The score value (must be between 0.0 and 100.0)
    ///
    /// # Errors
    ///
    /// Returns an error if the value is outside the valid range.
    pub fn new(value: f64) -> Result<Self, String> {
        if value < Self::MIN {
            return Err(format!(
                "Score value must be at least {}, got {}",
                Self::MIN, value
            ));
        }

        if value > Self::MAX {
            return Err(format!(
                "Score value must be at most {}, got {}",
                Self::MAX, value
            ));
        }

        // Check for NaN or infinity
        if !value.is_finite() {
            return Err("Score value must be finite".to_string());
        }

        Ok(Self(value))
    }

    /// Create a new score value without validation.
    ///
    /// # Safety
    ///
    /// This is unsafe because it doesn't validate the value.
    /// Only use this when you're certain the value is valid.
    pub unsafe fn new_unchecked(value: f64) -> Self {
        Self(value)
    }

    /// Get the score value as f64.
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Check if the score is perfect (100).
    pub fn is_perfect(&self) -> bool {
        self.0 == Self::MAX
    }

    /// Check if the score is zero.
    pub fn is_zero(&self) -> bool {
        self.0 == Self::MIN
    }

    /// Check if the score is passing (>= 70).
    pub fn is_passing(&self) -> bool {
        self.0 >= 70.0
    }

    /// Check if the score is failing (< 70).
    pub fn is_failing(&self) -> bool {
        self.0 < 70.0
    }

    /// Get the score as a percentage string.
    pub fn as_percentage(&self) -> String {
        format!("{:.1}%", self.0)
    }

    /// Round to the nearest integer.
    pub fn round(&self) -> i32 {
        self.0.round() as i32
    }

    /// Convert to 0-5 scale (for ratings).
    pub fn to_rating_scale(&self) -> f64 {
        (self.0 / 100.0) * 5.0
    }

    /// Convert from 0-5 scale to score.
    pub fn from_rating_scale(rating: f64) -> Result<Self, String> {
        let score = (rating / 5.0) * 100.0;
        Self::new(score)
    }
}

impl FromStr for ScoreValue {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s
            .trim()
            .trim_end_matches('%')
            .parse::<f64>()
            .map_err(|_| format!("Invalid score value: {s}"))?;

        Self::new(value)
    }
}

impl fmt::Display for ScoreValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

impl From<ScoreValue> for f64 {
    fn from(score: ScoreValue) -> f64 {
        score.0
    }
}

impl From<ScoreValue> for i32 {
    fn from(score: ScoreValue) -> i32 {
        score.0.round() as i32
    }
}

impl std::ops::Add for ScoreValue {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        // Clamp to MAX to avoid overflow
        let sum = self.0 + rhs.0;
        Self(sum.min(Self::MAX))
    }
}

impl std::ops::Sub for ScoreValue {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        // Clamp to MIN to avoid underflow
        let diff = self.0 - rhs.0;
        Self(diff.max(Self::MIN))
    }
}

impl std::ops::Mul<f64> for ScoreValue {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let product = self.0 * rhs;
        Self(product.clamp(Self::MIN, Self::MAX))
    }
}

impl std::ops::Div<f64> for ScoreValue {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let quotient = self.0 / rhs;
        Self(quotient.clamp(Self::MIN, Self::MAX))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_valid_scores() {
        assert!(ScoreValue::new(0.0).is_ok());
        assert!(ScoreValue::new(50.0).is_ok());
        assert!(ScoreValue::new(100.0).is_ok());
        assert!(ScoreValue::new(0.001).is_ok());
        assert!(ScoreValue::new(99.999).is_ok());
    }

    #[test]
    fn test_create_invalid_scores() {
        assert!(ScoreValue::new(-0.1).is_err());
        assert!(ScoreValue::new(100.1).is_err());
        assert!(ScoreValue::new(f64::NAN).is_err());
        assert!(ScoreValue::new(f64::INFINITY).is_err());
        assert!(ScoreValue::new(f64::NEG_INFINITY).is_err());
    }

    #[test]
    fn test_score_properties() {
        let zero = ScoreValue::new(0.0).unwrap();
        let fifty = ScoreValue::new(50.0).unwrap();
        let hundred = ScoreValue::new(100.0).unwrap();

        assert!(zero.is_zero());
        assert!(!zero.is_perfect());
        assert!(!zero.is_passing());
        assert!(zero.is_failing());

        assert!(!fifty.is_zero());
        assert!(!fifty.is_perfect());
        assert!(fifty.is_passing());
        assert!(!fifty.is_failing());

        assert!(!hundred.is_zero());
        assert!(hundred.is_perfect());
        assert!(hundred.is_passing());
        assert!(!hundred.is_failing());
    }

    #[test]
    fn test_score_operations() {
        let score1 = ScoreValue::new(50.0).unwrap();
        let score2 = ScoreValue::new(30.0).unwrap();

        assert_eq!((score1 + score2).value(), 80.0);
        assert_eq!((score1 - score2).value(), 20.0);
        assert_eq!((score1 * 2.0).value(), 100.0);
        assert_eq!((score1 / 2.0).value(), 25.0);
    }

    #[test]
    fn test_score_conversions() {
        let score = ScoreValue::new(75.0).unwrap();

        assert_eq!(score.value(), 75.0);
        assert_eq!(score.round(), 75);
        assert_eq!(score.to_rating_scale(), 3.75);
        assert_eq!(ScoreValue::from_rating_scale(3.75).unwrap().value(), 75.0);
    }

    #[test]
    fn test_score_display() {
        let score = ScoreValue::new(75.5).unwrap();

        assert_eq!(format!("{score}"), "75.50");
        assert_eq!(score.as_percentage(), "75.5%");
    }

    #[test]
    fn test_score_from_str() {
        assert_eq!("75".parse::<ScoreValue>().unwrap().value(), 75.0);
        assert_eq!("75.5".parse::<ScoreValue>().unwrap().value(), 75.5);
        assert_eq!("75%".parse::<ScoreValue>().unwrap().value(), 75.0);
        assert!("101".parse::<ScoreValue>().is_err());
        assert!("-1".parse::<ScoreValue>().is_err());
    }

    #[test]
    fn test_score_arithmetic_clamping() {
        let score1 = ScoreValue::new(80.0).unwrap();
        let score2 = ScoreValue::new(60.0).unwrap();

        // Addition clamps at MAX
        assert_eq!((score1 + score2).value(), 100.0);

        // Subtraction clamps at MIN
        let zero = ScoreValue::new(0.0).unwrap();
        assert_eq!((zero - score1).value(), 0.0);

        // Multiplication clamps
        assert_eq!((score1 * 2.0).value(), 100.0);
        assert_eq!((score1 * 0.5).value(), 40.0);
    }
}
