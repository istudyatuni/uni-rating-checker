#![allow(unused)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Response<T> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: T,
}

pub type GetUpdatesResponse = Response<Vec<Update>>;
pub type SendMessageResponse = Response<Message>;

#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i32,
    pub message: Message,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub from: User,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub username: Option<String>,
}
