use rusqlite::Connection;

use crate::model::db::DbResultItem;
use crate::model::error::Error as CrateError;
use crate::model::itmo::{Competition, Program};

const INIT_DB_SQL: &str = include_str!("./sql/init.sql");

const SELECT_COMPETITION_SQL: &str = include_str!("./sql/select/competition.sql");
const SELECT_ALL_COMPETITIONS_SQL: &str = include_str!("./sql/select/all_competitions.sql");
const INSERT_COMPETITION_SQL: &str = include_str!("./sql/insert/competition.sql");
const UPDATE_COMPETITION_SQL: &str = include_str!("./sql/update/competition.sql");
const DELETE_COMPETITION_SQL: &str = include_str!("./sql/delete/competition.sql");
const DELETE_COMPETITION_BY_USER_SQL: &str = include_str!("./sql/delete/competitions_by_user.sql");

const SELECT_PROGRAM_SQL: &str = include_str!("./sql/select/program.sql");
const INSERT_PROGRAM_SQL: &str = include_str!("./sql/insert/program.sql");

const SELECT_CACHE_SQL: &str = include_str!("./sql/select/cache.sql");
const INSERT_CACHE_SQL: &str = include_str!("./sql/insert/cache.sql");
const PURGE_CACHE_SQL: &str = include_str!("./sql/delete/all_cache.sql");

const SELECT_STATISTICS_CHATS_SQL: &str = include_str!("./sql/select/statistics_chats.sql");

#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    fn prepare(&self) -> Result<(), CrateError> {
        match self.conn.execute_batch(INIT_DB_SQL) {
            Ok(_) => Ok(()),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn new(path: &str) -> Result<Self, CrateError> {
        let conn = match Connection::open(path) {
            Ok(c) => c,
            Err(e) => return Err(CrateError::DbError(e)),
        };
        let db = Self { conn };
        db.prepare()?;
        Ok(db)
    }
    pub fn select_competition(
        &self,
        tg_chat_id: &str,
        case_number: &str,
        degree: &str,
        program_id: &str,
    ) -> Result<Option<Competition>, CrateError> {
        let mut statement = match self.conn.prepare(SELECT_COMPETITION_SQL) {
            Ok(s) => s,
            Err(e) => return Err(CrateError::DbError(e)),
        };
        let result = statement.query_row(
            &[
                (":tg_chat_id", &tg_chat_id),
                (":case_number", &case_number),
                (":program_id", &program_id),
                (":degree", &degree),
            ],
            |row| {
                Ok(Competition {
                    case_number: Some(case_number.to_string()),
                    position: row.get(0)?,
                    priority: row.get(1)?,
                    total_scores: row.get(2)?,
                    exam_scores: row.get(3)?,
                })
            },
        );
        match result {
            Ok(competition) => Ok(Some(competition)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn select_all_competitions(&self) -> Result<Vec<DbResultItem>, CrateError> {
        let mut statement = match self.conn.prepare(SELECT_ALL_COMPETITIONS_SQL) {
            Ok(s) => s,
            Err(e) => return Err(CrateError::DbError(e)),
        };
        let result = statement.query_map((), |row| {
            Ok(DbResultItem {
                tg_chat_id: row.get(0)?,
                degree: row.get(2)?,
                program_id: row.get(3)?,
                competition: Competition {
                    case_number: row.get(1)?,
                    position: row.get(4)?,
                    priority: row.get(5)?,
                    total_scores: row.get(6)?,
                    exam_scores: row.get(7)?,
                },
            })
        });

        match result {
            Ok(items) => Ok(items
                .into_iter()
                .filter_map(|c| if let Ok(c) = c { Some(c) } else { None })
                .collect()),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(vec![]),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn insert_competition(
        &self,
        competition: &Competition,
        tg_chat_id: &str,
        program_id: &str,
        degree: &str,
    ) -> Result<(), CrateError> {
        if competition.case_number.is_none() {
            eprintln!("trying insert, but case_number is none");
            return Ok(());
        }
        match self.conn.execute(
            INSERT_COMPETITION_SQL,
            (
                tg_chat_id,
                &competition.case_number,
                degree,
                program_id,
                competition.position,
                competition.priority,
                competition.total_scores,
                competition.exam_scores,
            ),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn update_competition(
        &self,
        competition: &Competition,
        tg_chat_id: &str,
        program_id: &str,
        degree: &str,
    ) -> Result<(), CrateError> {
        match self.conn.execute(
            UPDATE_COMPETITION_SQL,
            (
                tg_chat_id,
                &competition.case_number,
                program_id,
                degree,
                competition.position,
                competition.priority,
                competition.total_scores,
                competition.exam_scores,
            ),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn delete_competition(
        &self,
        case_number: &str,
        tg_chat_id: &str,
        program_id: &str,
        degree: &str,
    ) -> Result<(), CrateError> {
        match self.conn.execute(
            DELETE_COMPETITION_SQL,
            (tg_chat_id, case_number, program_id, degree),
        ) {
            Ok(_) => Ok(()),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn delete_competition_by_user(&self, tg_chat_id: &str) -> Result<(), CrateError> {
        match self
            .conn
            .execute(DELETE_COMPETITION_BY_USER_SQL, (tg_chat_id,))
        {
            Ok(_) => Ok(()),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn select_program(
        &self,
        uni: &str,
        program_id: &str,
    ) -> Result<Option<Program>, CrateError> {
        let mut statement = match self.conn.prepare(SELECT_PROGRAM_SQL) {
            Ok(s) => s,
            Err(e) => return Err(CrateError::DbError(e)),
        };

        let result = statement.query_row(&[(":id", program_id), (":uni", uni)], |row| {
            Ok(Program {
                isu_id: row.get(0)?,
                title_ru: row.get(1)?,
            })
        });

        match result {
            Ok(program) => Ok(Some(program)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn insert_program(
        &self,
        uni: &str,
        program_id: &str,
        program_name: &str,
    ) -> Result<(), CrateError> {
        match self
            .conn
            .execute(INSERT_PROGRAM_SQL, (program_id, uni, program_name))
        {
            Ok(_) => Ok(()),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn select_cache(&self, key: &str) -> Result<Option<String>, CrateError> {
        let mut statement = match self.conn.prepare(SELECT_CACHE_SQL) {
            Ok(s) => s,
            Err(e) => return Err(CrateError::DbError(e)),
        };
        let result = statement.query_row(&[(":key", &key)], |row| row.get(0));
        match result {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn insert_cache(&self, key: &str, value: &str) -> Result<(), CrateError> {
        match self.conn.execute(INSERT_CACHE_SQL, (key, value)) {
            Ok(_) => Ok(()),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn purge_cache(&self) -> Result<(), CrateError> {
        match self.conn.execute(PURGE_CACHE_SQL, ()) {
            Ok(_) => Ok(()),
            Err(e) => Err(CrateError::DbError(e)),
        }
    }
    pub fn select_statistics(&self) -> Result<i32, CrateError> {
        let mut statement = match self.conn.prepare(SELECT_STATISTICS_CHATS_SQL) {
            Ok(s) => s,
            Err(e) => return Err(CrateError::DbError(e)),
        };
        let result = statement.query_row((), |row| row.get(0));
        match result {
            Ok(count) => Ok(count),
            Err(_) => Ok(0),
        }
    }
}

pub fn cache_key(degree: &str, program_id: &str) -> String {
    format!("{degree}:{program_id}")
}
