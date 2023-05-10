use tracing::{info, warn};
use std::{path::Path, error::Error};

use serde::de::{self, Unexpected};
use serde::{Deserialize, Serialize};
use datafusion::prelude::*;

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

// https://github.com/serde-rs/serde/issues/1344
pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match u8::deserialize(deserializer)? {
        0 => Ok(false),
        1 => Ok(true),
        other => Err(de::Error::invalid_value(
            Unexpected::Unsigned(other as u64),
            &"zero or one",
        )),
    }
}

pub fn parse_option_int<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "\\N" => Ok(0),
        other => {
            match other.parse::<u16>() {
                Ok(v) => Ok(v),
                Err(_err) =>
                    Err(de::Error::invalid_value(
                        Unexpected::Str(other),
                        &"\\N or integer",
                    )),
            }
        }
    }
}

pub fn parse_basic_titles<T: From<Record>>(path: &Path, max_count: usize, check: fn(&Record) -> bool) -> Result<Vec<T>, Box<dyn Error>> {
    info!(path = path.to_str(), "Start importing IMDB titles");  
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(path)?;

    let mut result: Vec<T> = Vec::new();
    let mut last_len = 0;
    let mut last_info_time = std::time::SystemTime::now();

    for title in reader.deserialize() {
        let parse_result: Result<Record, _> = title;
        match parse_result {
            Ok(record) => {
                if check(&record) {
                    result.push(record.into());
                }
            },
            Err(err) => {
                warn!(index = result.len(), error = err.to_string(), "Parse basic title failed");
                continue;
            }
        }
        let time = std::time::SystemTime::now();
        if time > last_info_time + std::time::Duration::new(1, 0) {
            let count = result.len() - last_len;
            info!(count, total = result.len(), "Parsing basic titles");
            last_len = result.len();
            last_info_time = time;
        }
        if result.len() >= max_count {
            info!(count = result.len(), "Stop parsing due to max count reached");
            break;
        }
    }
    info!(path = path.to_str(), total = result.len(), "Parsing IMDB titles finished");  
    Ok(result)
}

pub async fn parse_basic_titles_datafusion(path: &Path) -> Result<DataFrame, Box<dyn Error>> {
    let ctx = SessionContext::new();
    let options = CsvReadOptions::new()
        .delimiter(b'\t')
        .file_extension(".tsv");
    let df = ctx.read_csv(path.to_str().unwrap(), options).await?;
    Ok(df)
}
