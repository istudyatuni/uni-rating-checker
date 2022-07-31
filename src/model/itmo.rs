#![allow(unused)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RatingResponse {
    pub ok: bool,
    pub result: RatingResult,
}

#[derive(Debug, Deserialize)]
pub struct RatingResult {
    pub general_competition: Vec<Competition>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Competition {
    pub position: i32,
    pub priority: i32,
    pub total_scores: f64,
    pub case_number: String,
    pub exam_scores: Option<f64>,
}

impl std::cmp::PartialEq for Competition {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.priority == other.priority
            && self.total_scores == other.total_scores
            && self.exam_scores == other.exam_scores
    }
}

impl std::cmp::Eq for Competition {}
