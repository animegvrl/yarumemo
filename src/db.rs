use serde::{Serialize};
use rusqlite::{Connection, Result};

#[derive(Debug, Serialize)]
pub struct Task {
    id: i32,
    done: i32,
    task: String,
    priority: i32,
}

impl Task {
    pub fn create(db_conn: &Connection, task: &str) -> Result<Task, rusqlite::Error> {
        let mut stmt = db_conn.prepare("
            INSERT INTO tasks(task)
            VALUES (?1)
            RETURNING id, done, task, priority;
        ")?;

        Ok(
            stmt.query_one([&task], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    done: row.get(1)?,
                    task: row.get(2)?,
                    priority: row.get(3)?,
                })
            })?
        )
    }
    pub fn query_all(db_conn: &Connection) -> Result<Vec<Task>, rusqlite::Error> {
        let mut stmt = db_conn.prepare("
            SELECT id, done, task, priority
            FROM tasks
            ORDER BY priority DESC;
        ")?;

        stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                done: row.get(1)?,
                task: row.get(2)?,
                priority: row.get(3)?,
            })
        })?.collect()
    }
    pub fn update_task(db_conn: &Connection, id: &i32, task: &str) -> Result<(), rusqlite::Error> {
        let mut stmt = db_conn.prepare("
            UPDATE tasks
            SET task = (?1)
            WHERE id = (?2);
        ")?;

        stmt.execute((&task, &id))?;
        Ok(())
    }
    pub fn update_priority(
            db_conn: &Connection,
            id: &i32,
            priority: &i32,
    ) -> Result<(), rusqlite::Error> {
        let mut stmt = db_conn.prepare("
            UPDATE tasks
            SET priority = (?1)
            WHERE id = (?2);
        ")?;

        stmt.execute((&priority, &id))?;
        Ok(())
    }
    pub fn update_done(db_conn: &Connection, id: &i32, done: &i32) -> Result<(), rusqlite::Error> {
        let mut stmt = db_conn.prepare("
            UPDATE tasks
            SET done = (?1)
            WHERE id = (?2);
        ")?;

        stmt.execute((&done, &id))?;
        Ok(())
    }
    pub fn delete(db_conn: &Connection, id: &i32) -> Result<(), rusqlite::Error> {
        let mut stmt = db_conn.prepare("
            DELETE FROM tasks
            WHERE id = (?1);
        ")?;

        stmt.execute([&id])?;
        Ok(())
    }
}
