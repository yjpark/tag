use serde::{Deserialize, Serialize};

use crate::parse::name::Record;

use super::utils::{Row, create_rows, split_by_comma};
use super::{table, relation};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BasicName {
    pub nconst: String,
    pub primary_name: String,
    pub birth_year: u16,
    pub death_year: u16,
    pub primary_profession: Vec<String>,
    #[serde(skip)]
    known_for_titles: Vec<String>,
}

impl Row for BasicName {
    fn get_id(&self) -> String {
        self.nconst.clone()
    }
}

impl From<Record> for BasicName {
    fn from(v: Record) -> Self {
        Self {
            nconst: v.nconst,
            primary_name: v.primary_name,
            birth_year: v.birth_year,
            death_year: v.death_year,
            primary_profession: split_by_comma(&v.primary_profession),
            known_for_titles: split_by_comma(&v.known_for_titles),
        }
    }
}

pub async fn import_basic_names(rows: Vec<BasicName>) {
    create_rows(table::NAME, rows, |row| {
        if row.known_for_titles.len() == 0 {
            None
        } else {
            Some(
                row.known_for_titles.iter().map(|x| {
                    format!("RELATE {}:{} -> {} -> {}:{};",
                        table::NAME, row.nconst, relation::KNOWN_FOR, table::TITLE, x)
                }).collect::<Vec<String>>().join("\n")
            )
        }
    }).await
}
