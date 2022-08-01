use tokio::time;

use api::itmo::get_programs;
use api::tg::handle_updates;
use db::sqlite::DB;

mod api;
mod db;
mod model;

async fn load_programs(db: &DB) -> Result<(), Box<dyn std::error::Error>> {
    let groups = get_programs().await?;
    for group in &groups {
        for program in &group.programs {
            db.insert_program("itmo", &program.isu_id.to_string(), &program.title_ru)?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = DB::new("test.db")?;

    load_programs(&db).await.unwrap();

    let mut offset = 0;
    loop {
        offset = handle_updates(&db, offset).await.unwrap();
        time::sleep(time::Duration::from_secs(1)).await;
    }
}
