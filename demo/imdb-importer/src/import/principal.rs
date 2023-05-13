use tracing::warn;
use serde::{Deserialize, Serialize};

use crate::parse::principal::Record;

use super::utils::{Row, create_rows};
use super::{table, relation};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct TitlePrincipal {
    pub tconst: String,
    pub ordering: u16,
    pub nconst: String,
    pub category: String,
    pub job: Option<String>,
    pub characters: Vec<String>,
}

impl Row for TitlePrincipal {
    fn get_id(&self) -> String {
        format!("{}-{}-{}", self.tconst, self.ordering, self.nconst)
    }
}

impl From<Record> for TitlePrincipal {
    fn from(v: Record) -> Self {
        Self {
            tconst: v.tconst,
            ordering: v.ordering,
            nconst: v.nconst,
            category: v.category,
            job: v.job,
            characters: v.characters,
        }
    }
}

pub async fn import_title_principals(rows: Vec<TitlePrincipal>) {
    create_rows(relation::HAS_PRINCIPAL, rows, |row| {
        match serde_json::to_string(row) {
            Ok(v) => {
                Some(
                    format!("RELATE {}:{} -> {} -> {}:{} CONTENT {};",
                        table::TITLE, row.tconst,
                        relation::HAS_PRINCIPAL,
                        table::NAME, row.nconst,
                        v
                    )
                )
            },
            Err(err) => {
                warn!(tconst = row.tconst, ordering = row.ordering, nconst = row.nconst, "convert to json failed: {}", err.to_string());
                None
            },
        }
    }, true).await
}

