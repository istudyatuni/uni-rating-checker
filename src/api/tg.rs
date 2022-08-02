use crate::db::sqlite::DB;
use crate::model::itmo::Competition;
use crate::model::tg::{ErrorResponse, GetUpdatesResponse, MessageRequest, SendMessageResponse};

use super::common::handle_competition;
use super::messages;

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";
const TOKEN: &str = env!("TG_TOKEN");
const LOGS_CHAT_ID: &str = "-1001625263132";

pub async fn send_competition_message(
    competition: &Competition,
    chat_id: &str,
    program_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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
        send_message(&msg, LOGS_CHAT_ID).await?;
        messages::error_occurred.to_string()
    };
    send_message(&text, chat_id).await
}

async fn send_incorrect_command_message(
    command: &str,
    chat_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let text = format!(
        "{}\n{}",
        messages::incorrect_command_header,
        match command {
            "/watch" => messages::watch_command,
            _ => "",
        }
    );
    send_message(&text, chat_id).await
}

pub async fn send_message(text: &str, chat_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let text = &text.replace('-', "\\-").replace('.', "\\.");

    let params = [
        ("chat_id", chat_id),
        ("text", text),
        ("parse_mode", "MarkdownV2"),
    ];

    let url =
        reqwest::Url::parse_with_params(&format!("{TG_API_PREFIX}{TOKEN}/sendMessage"), &params)?;
    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        let error: ErrorResponse = response.json().await?;
        let msg = "Cannot send message request";
        if let Some(description) = error.description {
            eprintln!("{msg}: {description}");
        }
        return Err(Box::from(msg));
    }

    let data: SendMessageResponse = response.json().await?;

    if !data.ok {
        eprintln!(
            "Cannot send message: {}",
            data.description
                .unwrap_or_else(|| "error has no description".to_string())
        )
    }

    Ok(())
}

async fn get_updates(offset: i32) -> Result<GetUpdatesResponse, Box<dyn std::error::Error>> {
    let params = [("offset", &offset.to_string())];
    let url =
        reqwest::Url::parse_with_params(&format!("{TG_API_PREFIX}{TOKEN}/getUpdates"), &params)?;

    let response = reqwest::get(url).await?;
    if !response.status().is_success() {
        let error: ErrorResponse = response.json().await?;
        let text = format!(
            "cannot get updates\nerror: `{}`",
            error.description.unwrap_or_default()
        );
        send_message(&text, LOGS_CHAT_ID).await?;
        return Err(Box::from("cannot get updates"));
    }

    Ok(response.json().await?)
}

/// Get and handle updates for TG bot
pub async fn handle_updates(db: &DB, offset: i32) -> Result<i32, Box<dyn std::error::Error>> {
    let data = get_updates(offset).await?;

    let mut max_update_id = 0;
    for update in data.result {
        if update.update_id > max_update_id {
            max_update_id = update.update_id;
        }

        if let Some(message) = update.message {
            if let Some(text) = message.text {
                let chat_id = message.from.id.to_string();
                match MessageRequest::from(text) {
                    Some(request) => handle_message_request(db, request, &chat_id).await?,
                    None => send_message(messages::unknown_message, &chat_id).await?,
                }
            }
        }
    }

    Ok(max_update_id + 1)
}

async fn handle_message_request(
    db: &DB,
    request: MessageRequest,
    chat_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    match request {
        MessageRequest::Watch(args) => {
            let result = handle_competition(
                db,
                chat_id,
                &args.degree.to_string(),
                &args.case_number,
                &args.program_id,
                true,
            )
            .await;
            match result {
                Ok(_) => (),
                Err(_) => {
                    send_message(messages::rating_not_found, chat_id).await?;
                }
            }
        }
        MessageRequest::IncorrectCommand(command) => {
            send_incorrect_command_message(&command, chat_id).await?
        }
        MessageRequest::Help => send_message(messages::help, chat_id).await?,
        MessageRequest::Start => send_message(messages::start, chat_id).await?,
        MessageRequest::About => send_message(messages::about, chat_id).await?,
    };
    Ok(())
}
