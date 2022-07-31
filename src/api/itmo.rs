use crate::model::itmo::{Competition, RatingResponse};

const ITMO_API_PREFIX: &str = "https://abitlk.itmo.ru/api/v1/9e2eee80b266b31c8d65f1dd3992fa26eb8b4c118ca9633550889a8ff2cac429";
const SNILS: &str = env!("SNILS");

const PROGRAM_ID: &str = "15840";

pub async fn get_rating() -> Result<Option<Competition>, Box<dyn std::error::Error>> {
    let rating_response: RatingResponse = reqwest::get(format!(
        "{ITMO_API_PREFIX}/rating/master/budget?program_id={PROGRAM_ID}"
    ))
    .await?
    .json()
    .await?;

    match find_score(rating_response) {
        None => eprintln!("no matching competition"),
        competition => return Ok(competition),
    }

    Ok(None)
}

fn find_score(response: RatingResponse) -> Option<Competition> {
    if !response.ok {
        return None;
    }

    let filtered_competition = response
        .result
        .general_competition
        .iter()
        .filter(|c| {
            if let Some(s) = &c.snils {
                return s == SNILS;
            }
            false
        })
        .collect::<Vec<&Competition>>();
    if filtered_competition.len() == 1 {
        return Some(filtered_competition[0].clone());
    }

    None
}
