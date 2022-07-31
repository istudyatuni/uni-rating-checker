use api::itmo::get_rating;
use api::tg::send_message;

mod api;
mod model;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let competition = get_rating().await?;

    if let Some(competition) = competition {
        send_message(competition).await?;
    }

    Ok(())
}
