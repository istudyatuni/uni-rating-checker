use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub message: String,
    pub result: T,
}

pub type RatingResponse = Response<RatingResult>;
pub type ProgramsResponse = Response<ProgramsResult>;
pub type ErrorResponse = Response<Option<()>>;

// Rating

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

// Programs

#[derive(Debug, Deserialize)]
pub struct ProgramsResult {
    pub groups: Vec<ProgramsGroup>,
}

#[derive(Debug, Deserialize)]
pub struct ProgramsGroup {
    pub name: String,
    pub programs: Vec<Program>,
}

#[derive(Debug, Deserialize)]
pub struct Program {
    pub isu_id: i32,
}
