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
    pub update_id: i64,
    pub message: Option<Message>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub from: User,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
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
            Self::Bachelor => "bachelor",
            Self::Master => "master",
            Self::Postgraduate => "postgraduate",
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

impl Watch {
    pub fn new(uni: &str, degree: &str, program_id: &str, case_number: &str) -> Option<Self> {
        Degree::from(degree).map(|degree| Self {
            uni: uni.to_string(),
            degree,
            program_id: program_id.to_string(),
            case_number: case_number.to_string(),
        })
    }
}

pub enum MessageRequest {
    About,
    Help,
    Start,
    Statistics,
    Unwatch(Watch),
    UnwatchAll,
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
            "/watch" | "/unwatch" => {
                let incorrect_command = Some(Self::IncorrectCommand(command.to_string()));

                if text.len() < 5 {
                    if text.len() == 2 && text[1] == "all" {
                        return Some(Self::UnwatchAll);
                    }
                    return incorrect_command;
                }

                // waiting for let-chain
                if let Some(watch) = Watch::new("itmo", &text[2], &text[3], &text[4]) {
                    if watch.degree == Degree::Master {
                        return match command {
                            "/watch" => Some(Self::Watch(watch)),
                            "/unwatch" => Some(Self::Unwatch(watch)),
                            _ => incorrect_command,
                        };
                    }
                }

                incorrect_command
            }
            "/about" => Some(Self::About),
            "/help" => Some(Self::Help),
            "/start" => Some(Self::Start),
            "/stats" => Some(Self::Statistics),
            _ => None,
        }
    }
}
