use api::itmo::get_rating;
use api::tg::send_message;
use db::sqlite::DB;

mod api;
mod db;
mod model;

const PROGRAM_ID: &str = "15840";
const CASE_NUMBER: &str = env!("CASE_NUMBER");
// me, now hardcoded
pub const CHAT_ID: &str = "687545186";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::new("test.db".to_string())?;

    let competition = get_rating(PROGRAM_ID.to_string(), CASE_NUMBER.to_string()).await?;

    match db.select_competition(CHAT_ID.to_string()) {
        Ok(old_competition) => {
            if competition != old_competition && competition.is_some() {
                if let Some(competition) = competition {
                    // send message if new competition differs from old (or new, when old == None)
                    send_message(&competition, CHAT_ID.to_string()).await?;
                    db.insert_competition(&competition, CHAT_ID.to_string())?;
                }
            }
        }
        Err(e) => {
            eprintln!("cannot select competition: {e}")
        }
    };

    Ok(())
}
