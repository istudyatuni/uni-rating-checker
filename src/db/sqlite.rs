use rusqlite::{Connection, Result};

use crate::model::db::DbResultItem;
use crate::model::itmo::{Competition, Program};

const INIT_DB_SQL: &str = include_str!("./sql/init.sql");

const SELECT_COMPETITION_SQL: &str = include_str!("./sql/select_competition.sql");
const SELECT_ALL_COMPETITIONS_SQL: &str = include_str!("./sql/select_all_competitions.sql");
const INSERT_COMPETITION_SQL: &str = include_str!("./sql/insert_competition.sql");
const UPDATE_COMPETITION_SQL: &str = include_str!("./sql/update_competition.sql");

const SELECT_PROGRAM_SQL: &str = include_str!("./sql/select_program.sql");
const INSERT_PROGRAM_SQL: &str = include_str!("./sql/insert_program.sql");

#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    fn prepare(&self) -> Result<()> {
        self.conn.execute_batch(INIT_DB_SQL)?;
        Ok(())
    }
    pub fn new(path: &str) -> Result<Self> {
        let db = Self {
            conn: Connection::open(path)?,
        };
        db.prepare()?;
        Ok(db)
    }
    pub fn select_competition(
        &self,
        tg_chat_id: &str,
        case_number: &str,
        degree: &str,
        program_id: &str,
    ) -> Result<Option<Competition>> {
        let mut statement = self.conn.prepare(SELECT_COMPETITION_SQL)?;
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
            Err(e) => Err(e),
        }
    }
    pub fn select_all_competitions(&self) -> Result<Vec<DbResultItem>> {
        let mut statement = self.conn.prepare(SELECT_ALL_COMPETITIONS_SQL)?;
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
            Err(e) => Err(e),
        }
    }
    pub fn insert_competition(
        &self,
        competition: &Competition,
        tg_chat_id: &str,
        program_id: &str,
        degree: &str,
    ) -> Result<()> {
        if competition.case_number.is_none() {
            eprintln!("trying insert, but case_number is none");
            return Ok(());
        }
        self.conn.execute(
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
        )?;
        Ok(())
    }
    pub fn update_competition(
        &self,
        competition: &Competition,
        tg_chat_id: &str,
        program_id: &str,
        degree: &str,
    ) -> Result<()> {
        self.conn.execute(
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
        )?;
        Ok(())
    }
    pub fn select_program(&self, uni: &str, program_id: &str) -> Result<Option<Program>> {
        let mut statement = self.conn.prepare(SELECT_PROGRAM_SQL)?;

        let result = statement.query_row(&[(":id", program_id), (":uni", uni)], |row| {
            Ok(Program {
                isu_id: row.get(0)?,
                title_ru: row.get(1)?,
            })
        });

        match result {
            Ok(program) => Ok(Some(program)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }
    pub fn insert_program(&self, uni: &str, program_id: &str, program_name: &str) -> Result<()> {
        self.conn
            .execute(INSERT_PROGRAM_SQL, (program_id, uni, program_name))?;
        Ok(())
    }
}
