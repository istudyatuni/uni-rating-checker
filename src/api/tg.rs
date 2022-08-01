use crate::model::itmo::Competition;
use crate::model::tg::{ErrorResponse, SendMessageResponse};

const PROGRAM_NAME: &str = "Разработка программного обеспечения";

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";
const TOKEN: &str = env!("TG_TOKEN");

pub async fn send_message(competition: &Competition, chat_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let text = format!(
        "*{}*\n_{}_\nПозиция {}\nВсего баллов {}\nБалл ВИ {}",
        PROGRAM_NAME,
        competition.case_number,
        competition.position,
        competition.total_scores,
        competition.exam_scores.unwrap_or(0f64)
    );
    let text = text.replace('-', "\\-");

    let params = [
        ("chat_id", chat_id),
        ("text", text.clone()),
        ("parse_mode", "MarkdownV2".to_string()),
    ];

    let url =
        reqwest::Url::parse_with_params(&format!("{TG_API_PREFIX}{TOKEN}/sendMessage"), &params)?;
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        let error: ErrorResponse = response.json().await?;
        if let Some(description) = error.description {
            eprintln!("Error when making send message request: {description}");
        }
        return Ok(());
    }

    let data: SendMessageResponse = response.json().await?;

    if !data.ok {
        eprintln!(
            "Error when send message: {}",
            data.description
                .unwrap_or_else(|| "error has no description".to_string())
        )
    }

    Ok(())
}
