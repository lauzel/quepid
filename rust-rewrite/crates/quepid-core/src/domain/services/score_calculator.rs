//! Score Calculator
//!
//! Calculates scores for cases based on ratings.

use crate::domain::value_objects::ScoreValue;

/// Score Calculator service
///
/// Calculates various types of scores for queries and cases.
#[derive(Debug, Clone, Default)]
pub struct ScoreCalculator;

impl ScoreCalculator {
    /// Calculate the average score for a set of ratings.
    ///
    /// # Arguments
    ///
    /// * `ratings` - Vector of rating values (0-5 scale)
    ///
    /// # Returns
    ///
    /// The average rating as a ScoreValue (0-100 scale)
    pub fn calculate_average(ratings: &[i32]) -> ScoreValue {
        if ratings.is_empty() {
            return ScoreValue::new(0.0).unwrap();
        }

        let sum: i32 = ratings.iter().sum();
        let average = sum as f64 / ratings.len() as f64;
        
        // Convert from 0-5 scale to 0-100 scale
        let score = (average / 5.0) * 100.0;
        
        ScoreValue::new(score).unwrap()
    }

    /// Calculate the weighted average score.
    ///
    /// # Arguments
    ///
    /// * `ratings` - Vector of rating values
    /// * `weights` - Vector of weights for each rating
    ///
    /// # Returns
    ///
    /// The weighted average as a ScoreValue
    pub fn calculate_weighted(ratings: &[i32], weights: &[f64]) -> Result<ScoreValue, String> {
        if ratings.len() != weights.len() {
            return Err("Ratings and weights must have the same length".to_string());
        }

        if ratings.is_empty() {
            return Ok(ScoreValue::new(0.0).unwrap());
        }

        let weighted_sum: f64 = ratings
            .iter()
            .zip(weights.iter())
            .map(|(rating, weight)| *rating as f64 * weight)
            .sum();

        let total_weight: f64 = weights.iter().sum();
        
        if total_weight == 0.0 {
            return Err("Total weight cannot be zero".to_string());
        }

        let average = weighted_sum / total_weight;
        let score = (average / 5.0) * 100.0;
        
        Ok(ScoreValue::new(score).unwrap())
    }

    /// Calculate the percentile score.
    ///
    /// # Arguments
    ///
    /// * `score` - The score to calculate percentile for
    /// * `all_scores` - All scores to compare against
    ///
    /// # Returns
    ///
    /// The percentile (0-100) representing how the score compares to all scores
    pub fn calculate_percentile(score: f64, all_scores: &[f64]) -> f64 {
        if all_scores.is_empty() {
            return 0.0;
        }

        let count = all_scores.len() as f64;
        let below = all_scores.iter().filter(|&&s| s < score).count() as f64;
        
        (below / count) * 100.0
    }

    /// Calculate scores for multiple queries.
    ///
    /// # Arguments
    ///
    /// * `query_ratings` - Map of query IDs to their ratings
    ///
    /// # Returns
    ///
    /// Map of query IDs to their calculated scores
    pub fn calculate_query_scores(
        query_ratings: &std::collections::HashMap<String, Vec<i32>>,
    ) -> std::collections::HashMap<String, ScoreValue> {
        query_ratings
            .iter()
            .map(|(query_id, ratings)| (query_id.clone(), Self::calculate_average(ratings)))
            .collect()
    }

    /// Calculate the overall case score from query scores.
    ///
    /// # Arguments
    ///
    /// * `query_scores` - Map of query IDs to their scores
    ///
    /// # Returns
    ///
    /// The average score across all queries
    pub fn calculate_case_score(query_scores: &std::collections::HashMap<String, ScoreValue>) -> ScoreValue {
        if query_scores.is_empty() {
            return ScoreValue::new(0.0).unwrap();
        }

        let sum: f64 = query_scores.values().map(|s| s.value()).sum();
        let average = sum / query_scores.len() as f64;
        
        ScoreValue::new(average).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_average_empty() {
        let score = ScoreCalculator::calculate_average(&[]);
        assert_eq!(score.value(), 0.0);
    }

    #[test]
    fn test_calculate_average_single() {
        let score = ScoreCalculator::calculate_average(&[5]);
        assert_eq!(score.value(), 100.0);
    }

    #[test]
    fn test_calculate_average_multiple() {
        let score = ScoreCalculator::calculate_average(&[1, 2, 3, 4, 5]);
        assert!((score.value() - 60.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_average_all_five() {
        let score = ScoreCalculator::calculate_average(&[5, 5, 5, 5]);
        assert_eq!(score.value(), 100.0);
    }

    #[test]
    fn test_calculate_average_all_one() {
        let score = ScoreCalculator::calculate_average(&[1, 1, 1, 1]);
        assert_eq!(score.value(), 20.0);
    }

    #[test]
    fn test_calculate_weighted() {
        let ratings = vec![1, 2, 3, 4, 5];
        let weights = vec![0.1, 0.2, 0.3, 0.2, 0.2];
        
        let score = ScoreCalculator::calculate_weighted(&ratings, &weights).unwrap();
        // (1*0.1 + 2*0.2 + 3*0.3 + 4*0.2 + 5*0.2) = 0.1 + 0.4 + 0.9 + 0.8 + 1.0 = 3.2
        // 3.2 / 5.0 * 100 = 64.0
        assert!((score.value() - 64.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_percentile() {
        let scores = vec![10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0, 90.0, 100.0];
        
        // Score of 50 should be at 50th percentile
        assert_eq!(ScoreCalculator::calculate_percentile(50.0, &scores), 50.0);
        
        // Score of 100 should be at 100th percentile
        assert_eq!(ScoreCalculator::calculate_percentile(100.0, &scores), 100.0);
        
        // Score of 10 should be at 10th percentile (actually 0th since it's the minimum)
        assert_eq!(ScoreCalculator::calculate_percentile(10.0, &scores), 0.0);
    }

    #[test]
    fn test_calculate_query_scores() {
        let mut query_ratings = std::collections::HashMap::new();
        query_ratings.insert("q1".to_string(), vec![5, 5, 5]);
        query_ratings.insert("q2".to_string(), vec![1, 2, 3]);
        
        let scores = ScoreCalculator::calculate_query_scores(&query_ratings);
        
        assert_eq!(scores["q1"].value(), 100.0);
        assert!((scores["q2"].value() - 40.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_case_score() {
        let mut query_scores = std::collections::HashMap::new();
        query_scores.insert("q1".to_string(), ScoreValue::new(100.0).unwrap());
        query_scores.insert("q2".to_string(), ScoreValue::new(50.0).unwrap());
        query_scores.insert("q3".to_string(), ScoreValue::new(0.0).unwrap());
        
        let case_score = ScoreCalculator::calculate_case_score(&query_scores);
        
        // (100 + 50 + 0) / 3 = 50
        assert!((case_score.value() - 50.0).abs() < 0.001);
    }
}
