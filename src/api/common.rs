use super::itmo::get_rating_competition;
use super::tg::{send_competition_message, send_log};
use crate::db::sqlite::DB;
use crate::model::error::Result;

/// If competition not exists in DB, send message
///
/// If exists, send message if competition in DB differs from new
pub async fn handle_competition(
    db: &DB,
    chat_id: &str,
    degree: &str,
    case_number: &str,
    program_id: &str,
    is_user_request: bool,
) -> Result<()> {
    let competition = get_rating_competition(db, degree, program_id, case_number).await?;

    match db.select_competition(chat_id, case_number, degree, program_id) {
        Ok(old_competition) => {
            if let Some(competition) = competition {
                let program_name = get_program_name(db, program_id)?;

                let mut should_send_message = false;

                // update if competition is old (competition != old_competition)
                // insert if is new (when old == None, on first user request)
                if let Some(old_competition) = old_competition {
                    if competition != old_competition {
                        db.update_competition(&competition, chat_id, program_id, degree)?;
                        should_send_message = true;
                    }
                } else {
                    db.insert_competition(&competition, chat_id, program_id, degree)?;
                }

                // send if it's user request or record in db was updated
                if is_user_request || should_send_message {
                    if let Err(e) =
                        send_competition_message(&competition, chat_id, &program_name).await
                    {
                        send_log(&format!("cannot send competition to chat {chat_id}:\n{e}"))
                            .await?;
                    }
                }
            }
        }
        Err(e) => return Err(e),
    };
    Ok(())
}

pub fn get_program_name(db: &DB, program_id: &str) -> Result<String> {
    let program = db.select_program("itmo", program_id)?;
    let program_name = if let Some(program) = program {
        program.title_ru
    } else {
        "Названия нет".to_string()
    };
    Ok(program_name)
}
