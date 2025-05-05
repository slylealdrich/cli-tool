use clap::ValueEnum;
use rusqlite::{types::{FromSql, FromSqlError}, ToSql};
use strum::Display;

#[derive(ValueEnum, Debug, Clone, Display)]
pub enum AnimalType {
    Cow,
    Chicken,
    Horse,
    Pig,
}

impl ToSql for AnimalType {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(self.to_string().into())
    }
}

impl FromSql for AnimalType {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        AnimalType::from_str(value.as_str()?, true).map_err(|_| FromSqlError::InvalidType)
    }
}
