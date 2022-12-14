use serde_json::json;

use super::{LOGS_CHAT_ID, TG_API_PREFIX, TOKEN};
use crate::api::messages;
use crate::api::CLIENT;
use crate::model::error::{Error as CrateError, Result};
use crate::model::itmo::Competition;
use crate::model::tg::{ErrorResponse, SendMessageResponse};

pub async fn send_competition_message(
    competition: &Competition,
    chat_id: &str,
    program_name: &str,
) -> Result<()> {
    let text = if let Some(case_number) = &competition.case_number {
        format!(
            "*{}*\n_{}_\nПозиция {}\nВсего баллов {}\nБалл ВИ {}",
            program_name,
            case_number,
            competition.position,
            competition.total_scores,
            competition.exam_scores.unwrap_or(0f64)
        )
    } else {
        // this should never be called
        let msg =
            format!("case_number is None\nchat_id: `{chat_id}`\nprogram_name: `{program_name}`");
        send_log(&msg).await?;
        messages::error_occurred.to_string()
    };
    send_message(&text, chat_id).await
}

pub async fn send_incorrect_command_message(command: &str, chat_id: &str) -> Result<()> {
    let text = format!(
        "{}\n{}",
        messages::incorrect_command_header,
        match command {
            "/watch" => messages::watch_command,
            "/unwatch" => messages::unwatch_command,
            _ => "",
        }
    );
    send_message(&text, chat_id).await
}

pub async fn send_message(text: &str, chat_id: &str) -> Result<()> {
    let text = &text.replace('-', "\\-").replace('.', "\\.");

    let data = json!({
        "chat_id": chat_id,
        "text": text,
        "parse_mode": "MarkdownV2",
    });

    let response = CLIENT
        .post(format!("{TG_API_PREFIX}{TOKEN}/sendMessage"))
        .body(reqwest::Body::from(data.to_string()))
        .header("content-type", "application/json")
        .send();
    let response = match response.await {
        Ok(r) => r,
        Err(e) => return Err(CrateError::RequestError(e)),
    };
    if !response.status().is_success() {
        let is_client_error = response.status().is_client_error();
        return match response.json::<ErrorResponse>().await {
            Ok(error) => match is_client_error {
                true => Err(CrateError::CannotSendMessage(error.description)),
                false => Err(CrateError::SendMessageError(error.description)),
            },
            Err(e) => Err(CrateError::RequestError(e)),
        };
    }

    match response.json::<SendMessageResponse>().await {
        Ok(error) => match error.ok {
            true => Ok(()),
            false => Err(CrateError::SendMessageError(error.description)),
        },
        Err(e) => Err(CrateError::RequestError(e)),
    }
}

pub async fn send_log(text: &str) -> Result<()> {
    if let Err(e) = send_message(text, LOGS_CHAT_ID).await {
        eprintln!("failed to send log: {text}\nerror: {e}")
    }
    Ok(())
}
