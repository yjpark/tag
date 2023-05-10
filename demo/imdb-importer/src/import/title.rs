use serde::{Deserialize, Serialize};

use crate::parse::title::Record;

use super::utils::{Row, create_rows, split_by_comma};
use super::{table};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BasicTitle {
    pub tconst: String,
    pub title_type: String,
    pub primary_title: String,
    pub original_title: String,
    pub is_adult: bool,
    pub start_year: u16,
    pub end_year: u16,
    pub runtime_minutes: u16,
    pub genres: Vec<String>,
}

impl Row for BasicTitle {
    fn get_id(&self) -> String {
        self.tconst.clone()
    }
}

impl From<Record> for BasicTitle {
    fn from(v: Record) -> Self {
        Self {
            tconst: v.tconst,
            title_type: v.title_type,
            primary_title: v.primary_title,
            original_title: v.original_title,
            is_adult: v.is_adult,
            start_year: v.start_year,
            end_year: v.end_year,
            runtime_minutes: v.runtime_minutes,
            genres: split_by_comma(&v.genres),
        }
    }
}

pub async fn import_basic_titles(rows: Vec<BasicTitle>) {
    create_rows(table::TITLE, rows, |_row| None).await
}
