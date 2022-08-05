use crate::db::sqlite::DB;
use crate::model::error::Error as CrateError;
use crate::model::itmo::Competition;
use crate::model::tg::{ErrorResponse, GetUpdatesResponse, MessageRequest, SendMessageResponse};

use super::common::handle_competition;
use super::messages;

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";

#[cfg(feature = "prod")]
const TOKEN: &str = env!("TG_TOKEN_PROD");
#[cfg(not(feature = "prod"))]
const TOKEN: &str = env!("TG_TOKEN");

const LOGS_CHAT_ID: &str = "-1001625263132";

pub async fn send_competition_message(
    competition: &Competition,
    chat_id: &str,
    program_name: &str,
) -> Result<(), CrateError> {
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

async fn send_incorrect_command_message(command: &str, chat_id: &str) -> Result<(), CrateError> {
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

pub async fn send_message(text: &str, chat_id: &str) -> Result<(), CrateError> {
    let text = &text.replace('-', "\\-").replace('.', "\\.");

    let params = [
        ("chat_id", chat_id),
        ("text", text),
        ("parse_mode", "MarkdownV2"),
    ];

    let url_path = format!("{TG_API_PREFIX}{TOKEN}/sendMessage");
    let url = match reqwest::Url::parse_with_params(&url_path, &params) {
        Ok(u) => u,
        Err(e) => return Err(CrateError::UrlParseError(e)),
    };
    let response = match reqwest::get(url).await {
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

pub async fn send_log(text: &str) -> Result<(), CrateError> {
    if let Err(e) = send_message(text, LOGS_CHAT_ID).await {
        eprintln!("failed to send log: {text}\nerror: {e}")
    }
    Ok(())
}

async fn get_updates(offset: i64) -> Result<GetUpdatesResponse, CrateError> {
    let params = [("offset", &offset.to_string())];
    let url_path = format!("{TG_API_PREFIX}{TOKEN}/getUpdates");
    let url = match reqwest::Url::parse_with_params(&url_path, &params) {
        Ok(u) => u,
        Err(e) => return Err(CrateError::UrlParseError(e)),
    };

    let response = match reqwest::get(url).await {
        Ok(r) => r,
        Err(e) => return Err(CrateError::RequestError(e)),
    };
    if !response.status().is_success() {
        return match response.json::<ErrorResponse>().await {
            Ok(error) => {
                let to_return = CrateError::CannotGetUpdates(error.description);
                send_log(&to_return.to_string()).await?;
                Err(to_return)
            }
            Err(e) => Err(CrateError::RequestError(e)),
        };
    }

    match response.json::<GetUpdatesResponse>().await {
        Ok(r) => Ok(r),
        Err(e) => Err(CrateError::RequestError(e)),
    }
}

/// Get and handle updates for TG bot
pub async fn handle_updates(db: &DB, offset: i64) -> Result<i64, CrateError> {
    let data = get_updates(offset).await?;

    // just want to know
    let updates_count = data.result.len();
    if updates_count > 2 {
        let msg = format!("{updates_count} updates in a row");
        if (send_log(&msg).await).is_err() {
            println!("{msg}");
        }
    }

    let mut max_update_id = 0;
    for update in data.result {
        if update.update_id > max_update_id {
            max_update_id = update.update_id;
        }

        if let Some(message) = update.message {
            if let Some(text) = message.text {
                let chat_id = message.from.id.to_string();
                match MessageRequest::from(text) {
                    Some(request) => {
                        if let Err(e) = handle_message_request(db, request, &chat_id).await {
                            if let CrateError::CannotSendMessage(description) = e {
                                // send message if it was a client error, for example, bot blocked by user
                                send_log(&format!(
                                    "cannot send message to chat {chat_id}:\n{}",
                                    description.unwrap_or_default()
                                ))
                                .await?
                            } else {
                                return Err(e);
                            }
                        }
                    }
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
) -> Result<(), CrateError> {
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
            if result.is_err() {
                send_message(messages::rating_not_found, chat_id).await?;
            }
        }
        MessageRequest::Unwatch(args) => {
            db.delete_competition(
                &args.case_number,
                chat_id,
                &args.program_id,
                &args.degree.to_string(),
            )?;
            db.insert_deleted_chat(chat_id)?;
            send_message(messages::done, chat_id).await?;
        }
        MessageRequest::UnwatchAll => {
            db.delete_competition_by_user(chat_id)?;
            db.insert_deleted_chat(chat_id)?;
            send_message(messages::done, chat_id).await?;
        }
        MessageRequest::IncorrectCommand(command) => {
            send_incorrect_command_message(&command, chat_id).await?
        }
        MessageRequest::Help => send_message(messages::help, chat_id).await?,
        MessageRequest::Start => send_message(messages::start, chat_id).await?,
        MessageRequest::Statistics => {
            send_log(&format!(
                "statistics:\n{} unique watchers\n{} deleted chats \\(may overlap the number of watchers\\)",
                db.select_statistics()?,
                db.select_deleted_chats()?
            ))
            .await?;
            send_message(messages::easter_egg, chat_id).await?
        }
        MessageRequest::About => send_message(messages::about, chat_id).await?,
    };
    Ok(())
}
