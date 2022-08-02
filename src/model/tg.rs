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

#[derive(Debug, PartialEq)]
pub enum Degree {
    Bachelor,
    Master,
    Postgraduate,
}

impl Degree {
    pub fn from(name: &str) -> Option<Self> {
        match name {
            "bachelor" => Some(Self::Bachelor),
            "master" => Some(Self::Master),
            "postgraduate" => Some(Self::Postgraduate),
            _ => None,
        }
    }
}

impl std::fmt::Display for Degree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Degree::Bachelor => "bachelor",
            Degree::Master => "master",
            Degree::Postgraduate => "postgraduate",
        };
        write!(f, "{name}")
    }
}

#[derive(Debug)]
pub struct Watch {
    pub uni: String,
    pub degree: Degree,
    pub program_id: String,
    pub case_number: String,
}

pub enum MessageRequest {
    Help,
    Start,
    Watch(Watch),
    IncorrectCommand(String),
}

impl MessageRequest {
    pub fn from(text: String) -> Option<Self> {
        let text: Vec<String> = text.split(' ').map(|w| w.to_string()).collect();
        if text.is_empty() {
            return None;
        }

        let command = text[0].as_str();
        match command {
            "/watch" => {
                // to correctly get text[2]
                // waiting for let-chain
                if text.len() <= 3 {
                } else if let Some(degree) = Degree::from(&text[2]) {
                    if text.len() == 5 && degree == Degree::Master {
                        return Some(Self::Watch(Watch {
                            // TODO: use if will add more universities. also validate it
                            // uni: text[1].clone(),
                            uni: "itmo".to_string(),
                            degree,
                            program_id: text[3].clone(),
                            case_number: text[4].clone(),
                        }));
                    }
                }

                Some(Self::IncorrectCommand(text[0].clone()))
            }
            "/help" => Some(Self::Help),
            "/start" => Some(Self::Start),
            _ => None,
        }
    }
}
