use crate::model::itmo::Competition;
use crate::model::tg::SendMessageResponse;

const PROGRAM_NAME: &str = "Разработка программного обеспечения";

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";
const TOKEN: &str = env!("TG_TOKEN");
// me
const CHAT_ID: i32 = 687545186;

pub async fn send_message(competition: Competition) -> Result<(), Box<dyn std::error::Error>> {
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

    let url =
        reqwest::Url::parse_with_params(&format!("{TG_API_PREFIX}{TOKEN}/sendMessage"), &params)?;
    let response: SendMessageResponse = reqwest::get(url).await?.json().await?;

    if !response.ok {
        eprintln!(
            "Error: {}",
            response
                .description
                .unwrap_or_else(|| "error has no description".to_string())
        )
    }

    Ok(())
}
