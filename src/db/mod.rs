#![allow(unused)]

use rusqlite::{Connection, Result};

use crate::model::itmo::Competition;

const INIT_DB_SQL: &str = r#"CREATE TABLE IF NOT EXISTS results (
	tg_chat_id TEXT NOT NULL,
	case_number TEXT NOT NULL,
	position INTEGER,
	priority INTEGER,
	total_scores INTEGER,
	exam_scores INTEGER
)"#;
const SELECT_COMPETITION_SQL: &str =
    "SELECT case_number, position, priority, total_scores, exam_scores
FROM results
WHERE tg_chat_id = :tg_chat_id";
const INSERT_COMPETITION_SQL: &str = r#"INSERT INTO results
	(tg_chat_id, case_number, position, priority, total_scores, exam_scores)
VALUES (?1, ?2, ?3, ?4, ?5, ?6)"#;

#[derive(Debug)]
pub struct DB {
    conn: Connection,
}

impl DB {
    fn prepare(&self) -> Result<()> {
        self.conn.execute(INIT_DB_SQL, ())?;
        Ok(())
    }
    pub fn new(path: String) -> Result<Self> {
        let db = Self {
            conn: Connection::open(path)?,
        };
        db.prepare()?;
        Ok(db)
    }
    pub fn select_competition(&self, tg_chat_id: String) -> Result<Option<Competition>> {
        let mut statement = self.conn.prepare(SELECT_COMPETITION_SQL)?;
        let rows: Vec<_> = statement
            .query_map(&[(":tg_chat_id", &tg_chat_id)], |row| {
                Ok(Competition {
                    case_number: row.get(0)?,
                    position: row.get(1)?,
                    priority: row.get(2)?,
                    total_scores: row.get(3)?,
                    exam_scores: row.get(4)?,
                })
            })?
            .into_iter()
            .filter_map(|c| if let Ok(c) = c { Some(c) } else { None })
            .collect();

        if rows.is_empty() {
            return Ok(None);
        }

        Ok(Some(rows[0].clone()))
    }
    pub fn insert_competition(&self, competition: &Competition, tg_chat_id: String) -> Result<()> {
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
}
