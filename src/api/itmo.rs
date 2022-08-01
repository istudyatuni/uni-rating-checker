use crate::db::sqlite::DB;
use crate::model::itmo::{
    Competition, ErrorResponse, ProgramsGroup, ProgramsResponse, RatingResponse,
};

const API_PREFIX: &str = "https://abitlk.itmo.ru/api/v1";
const API_KEY: &str = "9e2eee80b266b31c8d65f1dd3992fa26eb8b4c118ca9633550889a8ff2cac429";

/// Get competition in rating from itmo.ru
pub async fn get_rating_competition(
    program_id: &str,
    case_number: &str,
) -> Result<Option<Competition>, Box<dyn std::error::Error>> {
    let rating_response: RatingResponse = reqwest::get(format!(
        "{API_PREFIX}/{API_KEY}/rating/master/budget?program_id={program_id}"
    ))
    .await?
    .json()
    .await?;

    match find_score(rating_response, case_number) {
        None => eprintln!("no matching competition"),
        competition => return Ok(competition),
    }

    Ok(None)
}

fn find_score(response: RatingResponse, case_number: &str) -> Option<Competition> {
    if !response.ok {
        return None;
    }

    let filtered_competition = response
        .result
        .general_competition
        .iter()
        .filter(|c| {
            if let Some(c) = &c.case_number {
                c == case_number
            } else {
                false
            }
        })
        .collect::<Vec<&Competition>>();
    if filtered_competition.len() == 1 {
        return Some(filtered_competition[0].clone());
    }

    None
}

async fn get_programs() -> Result<Vec<ProgramsGroup>, Box<dyn std::error::Error>> {
    let params = [
        ("degree", "master".to_string()),
        // enough for now
        ("limit", 100.to_string()),
        ("page", 1.to_string()),
    ];
    let url = reqwest::Url::parse_with_params(&format!("{API_PREFIX}/programs/list"), &params)?;
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        let error: ErrorResponse = response.json().await?;
        eprintln!("Cannot fetch programs: {}", error.message);
        return Ok(vec![]);
    }

    let data: ProgramsResponse = response.json().await?;

    Ok(data.result.groups)
}

pub async fn load_programs(db: &DB) -> Result<(), Box<dyn std::error::Error>> {
    let groups = get_programs().await?;
    for group in &groups {
        for program in &group.programs {
            db.insert_program("itmo", &program.isu_id.to_string(), &program.title_ru)?;
        }
    }
    Ok(())
}
