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
    pub snils: Option<String>,
    pub exam_scores: Option<f64>,
}
