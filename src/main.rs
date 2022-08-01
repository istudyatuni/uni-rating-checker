use tokio::time;

use api::itmo::load_programs;
use api::tg::handle_updates;
use db::sqlite::DB;

mod api;
mod db;
mod model;

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
