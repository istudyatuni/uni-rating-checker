#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
/// Errors mapping used in this app for better error handling
pub enum Error {
    /// not found competition in rating
    NoMatchingCompetition,
    /// returned data is empty
    NoRatingReturned(String),
    /// error when fetching programs
    CannotFetchPrograms(String),
    /// error when sending message
    SendMessageError(Option<String>),
    /// we can't send message, e.g. bot blocked by user
    CannotSendMessage(Option<String>),
    /// we cannot get updates from telegram
    CannotGetUpdates(Option<String>),

    // wrappers
    DbError(rusqlite::Error),
    RequestError(reqwest::Error),
    DecodeJsonError(serde_json::Error),
    UrlParseError(url::ParseError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::NoMatchingCompetition => "no matching competition".to_string(),
            Self::NoRatingReturned(program_id) => {
                format!("no rating returned for program {program_id}")
            }
            Self::CannotFetchPrograms(message) => format!("Cannot fetch programs: {}", message),
            Self::SendMessageError(description) => format!(
                "Error sending message: {}",
                description
                    .clone()
                    .unwrap_or_else(|| "error has no description".to_string())
            ),
            Self::CannotSendMessage(description) => format!(
                "Cannot send message: {}",
                description
                    .clone()
                    .unwrap_or_else(|| "error has no description".to_string())
            ),
            Self::CannotGetUpdates(description) => format!(
                "Cannot get updates: {}",
                description
                    .clone()
                    .unwrap_or_else(|| "error has no description".to_string())
            ),
            Self::DbError(e) => e.to_string(),
            Self::RequestError(e) => e.to_string(),
            Self::DecodeJsonError(e) => e.to_string(),
            Self::UrlParseError(e) => e.to_string(),
        };
        write!(f, "{name}")
    }
}

impl std::error::Error for Error {}
