use crate::db::sqlite::{cache_key, DB};
use crate::model::error::Error as CrateError;
use crate::model::itmo::{
    Competition, ErrorResponse, ProgramsGroup, ProgramsResponse, RatingResponse,
};

const API_PREFIX: &str = "https://abitlk.itmo.ru/api/v1";
const API_KEY: &str = "9e2eee80b266b31c8d65f1dd3992fa26eb8b4c118ca9633550889a8ff2cac429";

/// Get competition in rating from itmo.ru
pub async fn get_rating_competition(
    db: &DB,
    degree: &str,
    program_id: &str,
    case_number: &str,
) -> Result<Option<Competition>, CrateError> {
    let key = cache_key(degree, program_id);
    let raw_json = if let Some(cached) = db.select_cache(&key)? {
        cached
    } else {
        let response = reqwest::get(format!(
            "{API_PREFIX}/{API_KEY}/rating/{degree}/budget?program_id={program_id}"
        ))
        .await;

        match response {
            Ok(response) => match response.text().await {
                Ok(text) => match db.insert_cache(&key, &text) {
                    Ok(_) => text,
                    Err(e) => return Err(e),
                },
                Err(e) => return Err(CrateError::RequestError(e)),
            },
            Err(e) => return Err(CrateError::RequestError(e)),
        }
    };

    let rating_response: RatingResponse = match serde_json::from_str(&raw_json) {
        Ok(r) => r,
        Err(e) => return Err(CrateError::DecodeJsonError(e)),
    };
    if rating_response.result.general_competition.is_empty() {
        return Err(CrateError::NoRatingReturned(program_id.to_string()));
    }

    match find_score(rating_response, case_number) {
        None => Err(CrateError::NoMatchingCompetition),
        competition => Ok(competition),
    }
}

fn find_score(response: RatingResponse, case_number: &str) -> Option<Competition> {
    if !response.ok {
        return None;
    }

    response
        .result
        .general_competition
        .iter()
        .find(|c| -> bool {
            if let Some(c) = &c.case_number {
                c == case_number
            } else {
                false
            }
        })
        .cloned()
}

async fn get_programs() -> Result<Vec<ProgramsGroup>, CrateError> {
    let params = [
        ("degree", "master".to_string()),
        // enough for now
        ("limit", 100.to_string()),
        ("page", 1.to_string()),
    ];
    let url = match reqwest::Url::parse_with_params(&format!("{API_PREFIX}/programs/list"), &params)
    {
        Ok(u) => u,
        Err(e) => return Err(CrateError::UrlParseError(e)),
    };
    let response = match reqwest::get(url).await {
        Ok(r) => r,
        Err(e) => return Err(CrateError::RequestError(e)),
    };
    if !response.status().is_success() {
        return match response.json::<ErrorResponse>().await {
            Ok(r) => Err(CrateError::CannotFetchPrograms(r.message)),
            Err(e) => Err(CrateError::RequestError(e)),
        };
    }

    match response.json::<ProgramsResponse>().await {
        Ok(data) => Ok(data.result.groups),
        Err(e) => Err(CrateError::RequestError(e)),
    }
}

pub async fn load_programs(db: &DB) -> Result<(), CrateError> {
    let groups = get_programs().await?;
    for group in &groups {
        for program in &group.programs {
            db.insert_program("itmo", &program.isu_id.to_string(), &program.title_ru)?;
        }
    }
    Ok(())
}
