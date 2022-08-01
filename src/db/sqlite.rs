use rusqlite::{Connection, Result};

use crate::model::itmo::Competition;

const INIT_DB_SQL: &str = include_str!("./sql/init.sql");
const SELECT_COMPETITION_SQL: &str = include_str!("./sql/select_competition.sql");
const INSERT_COMPETITION_SQL: &str = include_str!("./sql/insert_competition.sql");
const UPDATE_COMPETITION_SQL: &str = include_str!("./sql/update_competition.sql");
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
    ) -> Result<Option<Competition>> {
        let mut statement = self.conn.prepare(SELECT_COMPETITION_SQL)?;
        let rows: Vec<_> = statement
            .query_map(
                &[(":tg_chat_id", &tg_chat_id), (":case_number", &case_number)],
                |row| {
                    Ok(Competition {
                        case_number: case_number.to_string(),
                        position: row.get(0)?,
                        priority: row.get(1)?,
                        total_scores: row.get(2)?,
                        exam_scores: row.get(3)?,
                    })
                },
            )?
            .into_iter()
            .filter_map(|c| if let Ok(c) = c { Some(c) } else { None })
            .collect();

        if rows.is_empty() {
            return Ok(None);
        }

        Ok(Some(rows[0].clone()))
    }
    pub fn insert_competition(&self, competition: &Competition, tg_chat_id: &str) -> Result<()> {
        self.conn.execute(
            INSERT_COMPETITION_SQL,
            (
                tg_chat_id,
                &competition.case_number,
                competition.position,
                competition.priority,
                competition.total_scores,
                competition.exam_scores,
            ),
        )?;
        Ok(())
    }
    pub fn update_competition(&self, competition: &Competition, tg_chat_id: &str) -> Result<()> {
        self.conn.execute(
            UPDATE_COMPETITION_SQL,
            (
                tg_chat_id,
                &competition.case_number,
                competition.position,
                competition.priority,
                competition.total_scores,
                competition.exam_scores,
            ),
        )?;
        Ok(())
    }
    pub fn insert_program(&self, uni: &str, program_id: i32, program_name: &str) -> Result<()> {
        self.conn
            .execute(INSERT_PROGRAM_SQL, (program_id, uni, program_name))?;
        Ok(())
    }
}
