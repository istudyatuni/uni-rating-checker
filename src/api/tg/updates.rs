use super::{send_incorrect_command_message, send_log, send_message, TG_API_PREFIX, TOKEN};
use crate::api::common::handle_competition;
use crate::api::messages;
use crate::db::sqlite::DB;
use crate::model::error::Error as CrateError;
use crate::model::tg::{ErrorResponse, GetUpdatesResponse, MessageRequest};

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
