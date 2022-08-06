use std::time::{Duration, Instant};

use crate::model::error::Error as CrateError;
use api::itmo::load_programs;
use api::{common::handle_competition, tg::handle_updates};
use db::sqlite::DB;

mod api;
mod db;
mod model;

const SLEEP_DURATION: Duration = Duration::from_secs(1);
const TEN_MIN: Duration = Duration::from_secs(10 * 60);

#[tokio::main]
async fn main() -> Result<(), CrateError> {
    let db = match init_db() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error init database: {e}");
            return Err(e);
        }
    };

    if let Err(e) = load_programs(&db).await {
        eprintln!("Error loading programs: {e}");
    }

    check_rating_updates_wrapper(&db).await?;

    let mut offset = 0;
    let mut timer = Instant::now();
    loop {
        offset = match handle_updates(&db, offset).await {
            Ok(o) => o,
            Err(e) => {
                eprintln!("Error handling telegram updates: {e}");
                return Err(e);
            }
        };

        if timer.elapsed() >= TEN_MIN {
            check_rating_updates_wrapper(&db).await?;
            timer = Instant::now();
        }

        tokio::time::sleep(SLEEP_DURATION).await;
    }
}

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

async fn check_rating_updates_wrapper(db: &DB) -> Result<(), CrateError> {
    db.purge_cache()?;
    if let Err(e) = check_rating_updates(db).await {
        eprintln!("Error checking rating updates: {e}")
    }
    Ok(())
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
