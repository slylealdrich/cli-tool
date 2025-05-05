use std::usize;

use rusqlite::OptionalExtension;
use rusqlite::{Connection, Result, fallible_iterator::FallibleIterator, params};

use crate::Animal;
use crate::AnimalType;

pub struct Data {
    conn: Connection,
}

impl Data {
    pub fn new() -> Self {
        let conn = Connection::open("./cli-tool.db").unwrap();
        Self { conn }
    }

    pub fn init_db(&self) -> Result<usize> {
        let conn = &self.conn;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS animal (
                id INTEGER PRIMARY KEY,
                type TEXT NOT NULL CHECK( type IN ('Cow', 'Chicken', 'Horse', 'Pig') ),
                name TEXT NOT NULL UNIQUE
            )",
            (),
        )
    }

    pub fn select_all_animals(&self) -> Result<Vec<Animal>> {
        let conn = &self.conn;
        let mut statement = conn.prepare("SELECT * FROM animal")?;
        let rows = statement.query([])?;
        rows.map(|row| {
            Ok(Animal {
                id: row.get(0)?,
                animal_type: row.get(1)?,
                name: row.get(2)?,
            })
        })
        .collect()
    }

    pub fn select_animal_by_name(&self, name: &String) -> Result<Option<Animal>> {
        let conn = &self.conn;
        let mut statement = conn.prepare("SELECT * FROM animal WHERE name = ?1")?;

        statement
            .query_row(params![name], |row| {
                Ok(Animal {
                    id: row.get(0)?,
                    animal_type: row.get(1)?,
                    name: row.get(2)?,
                })
            })
            .optional()
    }

    pub fn select_animals_by_animal_type(&self, animal_type: &String) -> Result<Vec<Animal>> {
        let conn = &self.conn;
        let mut statement = conn.prepare("SELECT * FROM animal WHERE type = ?1")?;
        let rows = statement.query(params![animal_type])?;
        rows.map(|row| {
            Ok(Animal {
                id: row.get(0)?,
                animal_type: row.get(1)?,
                name: row.get(2)?,
            })
        })
        .collect()
    }

    pub fn insert_animal(&self, animal_type: &AnimalType, name: &String) -> Result<usize> {
        let conn = &self.conn;

        conn.execute(
            "INSERT INTO animal (type, name) VALUES (?1, ?2)",
            params![animal_type.to_string(), name],
        )
    }

    pub fn delete_animal_by_name(&self, name: &String) -> Result<usize> {
        let conn = &self.conn;

        conn.execute("DELETE FROM animal WHERE name=?1", params![name])
    }
}
