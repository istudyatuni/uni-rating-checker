use super::itmo::Competition;

#[derive(Debug)]
pub struct DbResultItem {
    pub tg_chat_id: String,
    pub program_id: String,
    pub degree: String,
    pub competition: Competition,
}
