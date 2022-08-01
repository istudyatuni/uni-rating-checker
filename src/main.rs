use api::itmo::get_rating;
use api::tg::{send_message, CHAT_ID};
use db::sqlite::DB;

mod api;
mod db;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::new("test.db".to_string())?;

    let competition = get_rating().await?;

    match db.select_competition(CHAT_ID.to_string()) {
        Ok(old_competition) => {
            if competition != old_competition && competition.is_some() {
                if let Some(competition) = competition {
                    // send message if new competition differs from old (or new, when old == None)
                    send_message(&competition).await?;
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
