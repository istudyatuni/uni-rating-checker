use crate::db::sqlite::DB;

use super::itmo::get_rating_competition;
use super::tg::send_competition_message;

/// If competition not exists in DB, send message
///
/// If exists, send message if competition in DB differs from new
pub async fn handle_competition(
    db: &DB,
    chat_id: &str,
    case_number: &str,
    program_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let competition = get_rating_competition(program_id, case_number).await?;

    match db.select_competition(chat_id, case_number, program_id) {
        Ok(old_competition) => {
            if competition != old_competition && competition.is_some() {
                if let Some(competition) = competition {
                    let program = db.select_program("itmo", program_id)?;
                    let program_name = if let Some(program) = program {
                        program.title_ru
                    } else {
                        "Названия нет".to_string()
                    };

                    // send message if new competition differs from old (or new, when old == None)
                    send_competition_message(&competition, chat_id, &program_name).await?;

                    // insert if competition is new, update if is old
                    if old_competition.is_none() {
                        db.insert_competition(&competition, chat_id, program_id)?;
                    } else {
                        db.update_competition(&competition, chat_id, program_id)?;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("cannot select competition: {e}")
        }
    };
    Ok(())
}
