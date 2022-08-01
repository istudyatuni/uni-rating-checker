use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: T,
}

pub type GetUpdatesResponse = Response<Vec<Update>>;
pub type SendMessageResponse = Response<Message>;
pub type ErrorResponse = Response<Option<()>>;

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i32,
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub from: User,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
}

// Bot commands

#[derive(Debug)]
pub struct Watch {
    pub uni: String,
    pub program_id: String,
    pub case_number: String,
}

pub enum MessageRequest {
    Watch(Watch),
    None,
}

impl MessageRequest {
    pub fn from(text: String) -> Self {
        let text: Vec<String> = text.split(' ').map(|w| w.to_string()).collect();
        if text.len() == 4 && text[0] == "/watch" {
            Self::Watch(Watch {
                // TODO: use if will add more universities. also validate it
                // uni: text[1].clone(),
                uni: "itmo".to_string(),
                program_id: text[2].clone(),
                case_number: text[3].clone(),
            })
        } else {
            Self::None
        }
    }
}
