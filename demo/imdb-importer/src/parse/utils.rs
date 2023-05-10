use tracing::{info, warn};
use std::{path::Path, error::Error};

use serde::de::{self, Unexpected};
use serde::{Deserialize};

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

pub fn parse_csv_records<R, T, F>(kind: &str, path: &Path, max_count: usize, check: F) -> Result<Vec<T>, Box<dyn Error>>
    where
        R: for<'de> serde::Deserialize<'de>,
        T: From<R>,
        F: Fn(&R) -> bool,
{
    info!(kind, path = path.to_str(), "start importing csv records");  
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(path)?;

    let mut result: Vec<T> = Vec::new();
    let mut last_len = 0;
    let mut last_info_time = std::time::SystemTime::now();

    for title in reader.deserialize() {
        let parse_result: Result<R, _> = title;
        match parse_result {
            Ok(record) => {
                if check(&record) {
                    result.push(record.into());
                }
            },
            Err(err) => {
                warn!(kind, index = result.len(), error = err.to_string(), "parsing csv record failed");
                continue;
            }
        }
        let time = std::time::SystemTime::now();
        if time > last_info_time + std::time::Duration::new(1, 0) {
            let count = result.len() - last_len;
            info!(kind, count, total = result.len(), "parsing csv record");
            last_len = result.len();
            last_info_time = time;
        }
        if result.len() >= max_count {
            info!(kind, count = result.len(), "stop parsing csv records due to max count reached");
            break;
        }
    }
    info!(kind, path = path.to_str(), total = result.len(), "parsing csv records finished");  
    Ok(result)
}
