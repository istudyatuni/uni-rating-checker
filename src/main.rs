use tokio::time;

use crate::model::error::Error as CrateError;
use api::common::handle_competition;
use api::itmo::load_programs;
use api::tg::handle_updates;
#[cfg(feature = "migrate")]
use api::{messages, tg::send_message};
use db::sqlite::DB;

mod api;
mod db;
mod model;

const TEN_MIN_IN_SEC: i32 = 10 * 60;

fn init_db() -> Result<DB, CrateError> {
    #[cfg(feature = "prod")]
    let db_path: String = if let Some(home_dir) = dirs::home_dir() {
        format!("{}/itmo.db", home_dir.display())
    } else {
        eprintln!("no $HOME for storing database file, using /");
        "/itmo.db".to_string()
    };
    #[cfg(not(feature = "prod"))]
    let db_path = "test.db".to_string();

    DB::new(&db_path)
}

async fn check_rating_updates(db: &DB) -> Result<(), CrateError> {
    // select registered watchers from 'results'
    match db.select_all_competitions() {
        Ok(competitions) => {
            for c in competitions {
                if let Some(case_number) = c.competition.case_number {
                    handle_competition(
                        db,
                        &c.tg_chat_id,
                        &c.degree,
                        &case_number,
                        &c.program_id,
                        false,
                    )
                    .await?
                }
            }
        }
        Err(e) => eprintln!("Error selecting all competitions: {e}"),
    }

    Ok(())
}

#[cfg(feature = "migrate")]
async fn migrate(db: &DB) -> Result<(), CrateError> {
    for chat_id in db.select_uniq_chats()? {
        if let Err(e) = send_message(messages::migrate, &chat_id).await {
            eprintln!("Cannot send migrate message to {chat_id}: {e}");
        }
    }
    Ok(())
}

#[cfg_attr(feature = "migrate", allow(unreachable_code))]
#[tokio::main]
async fn main() -> Result<(), CrateError> {
    let db = match init_db() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error init database: {e}");
            return Err(e);
        }
    };

    #[cfg(feature = "migrate")]
    {
        migrate(&db).await?;
        return Ok(());
    }

    match load_programs(&db).await {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Error loading programs: {e}");
        }
    }

    let mut offset = 0;
    let mut sec_counter = 0;
    loop {
        offset = match handle_updates(&db, offset).await {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Error handling telegram updates: {e}");
                return Err(e);
            }
        };

        if sec_counter == 0 {
            db.purge_cache()?;
            match check_rating_updates(&db).await {
                Ok(_) => (),
                Err(e) => eprintln!("Error checking rating updates: {e}"),
            }
        }
        sec_counter = (sec_counter + 1) % TEN_MIN_IN_SEC;

        time::sleep(time::Duration::from_secs(1)).await;
    }
}
