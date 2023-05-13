use std::{path::Path, error::Error};

use serde::{Deserialize, Serialize};

use super::utils::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub tconst: String,
    pub ordering: u16,
    pub nconst: String,
    pub category: String,
    #[serde(deserialize_with = "parse_option_string")]
    pub job: Option<String>,
    #[serde(deserialize_with = "parse_option_string_array")]
    pub characters: Vec<String>,
}

pub fn parse_title_principals<T, F>(path: &Path, max_count: usize, check: F) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: From<Record>,
        F: Fn(&Record) -> bool,
{
    parse_csv_records("imdb_principals", path, max_count, check)
}
