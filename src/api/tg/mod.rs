mod send;
mod updates;

pub use send::{send_competition_message, send_incorrect_command_message, send_log, send_message};
pub use updates::handle_updates;

const TG_API_PREFIX: &str = "https://api.telegram.org/bot";

#[cfg(feature = "prod")]
const TOKEN: &str = env!("TG_TOKEN_PROD");
#[cfg(not(feature = "prod"))]
const TOKEN: &str = env!("TG_TOKEN");

const LOGS_CHAT_ID: &str = "-1001625263132";
