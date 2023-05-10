use std::{path::Path, error::Error};

use serde::{Deserialize, Serialize};

use super::utils::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub nconst: String,
    #[serde(rename = "primaryName")]
    pub primary_name: String,
    #[serde(rename = "birthYear", deserialize_with = "parse_option_int")]
    pub birth_year: u16,
    #[serde(rename = "deathYear", deserialize_with = "parse_option_int")]
    pub death_year: u16,
    #[serde(rename = "primaryProfession")]
    pub primary_profession: String,
    #[serde(rename = "knownForTitles")]
    pub known_for_titles: String,
}

pub fn parse_basic_names<T, F>(path: &Path, max_count: usize, check: F) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: From<Record>,
        F: Fn(&Record) -> bool,
{
    parse_csv_records("imdb_name", path, max_count, check)
}
