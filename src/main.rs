use tokio::time;

use api::itmo::load_programs;
use api::{common::handle_competition, tg::handle_updates};
use db::sqlite::DB;

mod api;
mod db;
mod model;

const TEN_MIN_IN_SEC: i32 = 10 * 60;

fn init_db() -> Result<DB, Box<dyn std::error::Error>> {
    #[cfg(feature = "prod")]
    let db_path: String = if let Some(home_dir) = dirs::home_dir() {
        format!("{}/itmo.db", home_dir.display().to_string())
    } else {
        eprintln!("no $HOME for storing database file, using /");
        "/itmo.db".to_string()
    };
    #[cfg(not(feature = "prod"))]
    let db_path = "test.db".to_string();

    Ok(DB::new(&db_path)?)
}

async fn check_rating_updates(db: &DB) -> Result<(), Box<dyn std::error::Error>> {
    // select registered watchers from 'results'
    for c in db.select_all_competitions()? {
        if let Some(case_number) = c.competition.case_number {
            handle_competition(
                &db,
                &c.tg_chat_id,
                &c.degree,
                &case_number,
                &c.program_id,
                false,
            )
            .await?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = init_db()?;

    load_programs(&db).await.unwrap();

    let mut offset = 0;
    let mut sec_counter = 0;
    loop {
        offset = handle_updates(&db, offset).await.unwrap();

        if sec_counter == 0 {
            check_rating_updates(&db).await?;
        }
        sec_counter = (sec_counter + 1) % TEN_MIN_IN_SEC;

        time::sleep(time::Duration::from_secs(1)).await;
    }
}
