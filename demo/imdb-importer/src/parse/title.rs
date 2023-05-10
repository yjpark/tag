use std::{path::Path, error::Error};

use serde::{Deserialize, Serialize};

use super::utils::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Record {
    pub tconst: String,
    #[serde(rename = "titleType")]
    pub title_type: String,
    #[serde(rename = "primaryTitle")]
    pub primary_title: String,
    #[serde(rename = "originalTitle")]
    pub original_title: String,
    #[serde(rename = "isAdult", deserialize_with = "bool_from_int")]
    pub is_adult: bool,
    #[serde(rename = "startYear", deserialize_with = "parse_option_int")]
    pub start_year: u16,
    #[serde(rename = "endYear", deserialize_with = "parse_option_int")]
    pub end_year: u16,
    #[serde(rename = "runtimeMinutes", deserialize_with = "parse_option_int")]
    pub runtime_minutes: u16,
    pub genres: String,
}

pub fn parse_basic_titles<T, F>(path: &Path, max_count: usize, check: F) -> Result<Vec<T>, Box<dyn Error>>
    where
        T: From<Record>,
        F: Fn(&Record) -> bool,
{
    parse_csv_records("imdb_title", path, max_count, check)
}
