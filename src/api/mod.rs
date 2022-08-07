pub mod common;
pub mod itmo;
pub mod messages;
pub mod tg;

use lazy_static::lazy_static;

lazy_static! {
    /// Global reqwest::Client object
    static ref CLIENT: reqwest::Client = reqwest::Client::new();
}
