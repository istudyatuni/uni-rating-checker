use model::itmo::{Competition, RatingResponse};
use model::tg::SendMessageResponse;

mod model;

const PROGRAM_ID: &str = "15840";
const PROGRAM_NAME: &str = "Разработка программного обеспечения";

const ITMO_API_PREFIX: &str = "https://abitlk.itmo.ru/api/v1/9e2eee80b266b31c8d65f1dd3992fa26eb8b4c118ca9633550889a8ff2cac429";
const SNILS: &str = env!("SNILS");

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";
const TOKEN: &str = env!("TG_TOKEN");
// me
const CHAT_ID: i32 = 687545186;

async fn get_rating() -> Result<Option<Competition>, Box<dyn std::error::Error>> {
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

async fn send_message(competition: Competition) -> Result<(), Box<dyn std::error::Error>> {
    let text = format!(
        "*{}*\nПозиция {}\nВсего баллов {}\nБалл ВИ {}",
        PROGRAM_NAME,
        competition.position,
        competition.total_scores,
        competition.exam_scores.unwrap_or(0f64)
    );
    let params = [
        ("chat_id", CHAT_ID.to_string()),
        ("text", text),
        ("parse_mode", "MarkdownV2".to_string()),
    ];

    let url = reqwest::Url::parse_with_params(&format!("{TG_API_PREFIX}{TOKEN}/sendMessage"), &params)?;
    let response: SendMessageResponse = reqwest::get(url).await?.json().await?;

    if !response.ok {
        eprintln!("Error: {}", response.description.unwrap_or_else(|| "error has no description".to_string()))
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let competition = get_rating().await?;

    if let Some(competition) = competition {
        send_message(competition).await?;
    }

    Ok(())
}
