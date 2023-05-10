use std::cmp::max;
use std::time::Duration;

use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{info, warn};

use crate::parse::basic_titles::Record;

/*
DEFINE TABLE title SCHEMALESS;
 */

pub const MIN_CHUNK_SIZE: usize = 10000;
pub const CREATE_BATCH_SIZE: usize = 100;

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
            genres: v.genres.split(',').map(|x| x.to_owned()).collect::<Vec<String>>(),
        }
    }
}

pub async fn import_basic_titles(titles: Vec<BasicTitle>) {
    if titles.len() == 0 {
        warn!("Titles is empty!");
        return;
    }

    let mut chunk_index = 0;
    let mut handles = Vec::new();
    let num_cpus: usize = *super::utils::NUM_CPUS;
    let chunk_size = max(MIN_CHUNK_SIZE, titles.len() / num_cpus);

    let db = super::utils::init_db_client().await;
    let delete_all_titles = "DELETE title";
    if let Err(err) = db.query(delete_all_titles).await {
        warn!(chunk_index, "Query failed: {} -> {:#?}", delete_all_titles, err);
    }

    if titles.len() > chunk_size {
        info!(num_cpus, chunk_size, "Running in chunks mode");
        for chunk in titles.chunks(chunk_size) {
            let mut chunk_titles = Vec::new();
            for x in chunk {
                chunk_titles.push(x.clone());
            }
            let handle = tokio::spawn(async move {
                import_basic_titles_chunk(chunk_index, chunk_titles).await;
            });
            handles.push(handle);
            chunk_index += 1;
        }
        loop {
            let mut done = true;
            for handle in handles.iter() {
                if !handle.is_finished() {
                    done = false;
                    break;
                }
            }
            if done {
                break;
            } else {
                sleep(Duration::from_secs(10)).await;
            }
        }
    } else {
        import_basic_titles_chunk(chunk_index, titles).await;
    }
}

async fn import_basic_titles_chunk(chunk_index: usize, titles: Vec<BasicTitle>) {
    let db = super::utils::init_db_client().await;

    let mut count = 0;

    /* batching with query str is much slower than the create way
    let mut query: String = String::with_capacity(CREATE_BATCH_SIZE * 1024);

    for title in titles {
        let id = title.tconst.clone();
        let content = surrealdb::sql::to_value(title).unwrap().to_string();
        query.push_str(format!("CREATE title:{id} CONTENT {content} RETURN NONE;\n").as_str());
        count += 1;
        if count >= CREATE_BATCH_SIZE {
            info!(count, "Creating basic titles");
            if let Err(err) = db.query(&query).await {
                warn!(chunk_index, "Created title failed: {} {} -> {:#?}", id, query, err);
            }
            query.clear();
            count = 0;
        }
    }
     */

    let mut last_info_time = std::time::SystemTime::now();

    for title in titles {
        let id = title.tconst.clone();
        let created: Result<BasicTitle, surrealdb::Error> = db.create(("title", &id))
            .content(&title)
            .await;
        if let Err(err) = created {
            warn!(chunk_index, "Created title failed: {} {:#?} -> {:#?}", id, &title, err);
        }
        let time = std::time::SystemTime::now();
        count += 1;
        if time > last_info_time + std::time::Duration::new(1, 0) {
            info!(chunk_index, count, "Imported basic titles");
            count = 0;
            last_info_time = time;
        }
    }
}
