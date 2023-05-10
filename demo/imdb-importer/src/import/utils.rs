use tracing::{info, warn};
use std::cmp::max;
use std::fmt::Debug;
use std::time::Duration;
use lazy_static::lazy_static;

use tokio::time::sleep;

use serde::{Deserialize, Serialize};
use surrealdb::opt::auth::Root;

use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Ws, Client};

pub const MIN_CHUNK_SIZE: usize = 10000;
pub const CREATE_BATCH_SIZE: usize = 100;

pub const ENV_DB_URL: &'static str = "IMDB_DB_URL";
pub const ENV_DB_USERNAME: &'static str = "IMDB_DB_USERNAME";
pub const ENV_DB_PASSWORD: &'static str = "IMDB_DB_PASSWORD";
pub const ENV_DB_NAMESPACE: &'static str = "IMDB_DB_NAMESPACE";
pub const ENV_DB_DATABASE: &'static str = "IMDB_DB_DATABASE";

pub const DEFAULT_DB_URL: &'static str = "10.0.1.9:8000";
pub const DEFAULT_DB_USERNAME: &'static str = "root";
pub const DEFAULT_DB_PASSWORD: &'static str = "root";
pub const DEFAULT_DB_NAMESPACE: &'static str = "tag";
pub const DEFAULT_DB_DATABASE: &'static str = "imdb";

lazy_static! {
    pub static ref NUM_CPUS: usize = num_cpus::get();
    pub static ref DB_URL: String = std::env::var(ENV_DB_URL).unwrap_or(DEFAULT_DB_URL.to_owned());
    pub static ref DB_USERNAME: String = std::env::var(ENV_DB_USERNAME).unwrap_or(DEFAULT_DB_USERNAME.to_owned());
    pub static ref DB_PASSWORD: String = std::env::var(ENV_DB_PASSWORD).unwrap_or(DEFAULT_DB_PASSWORD.to_owned());
    pub static ref DB_NAMESPACE: String = std::env::var(DEFAULT_DB_NAMESPACE).unwrap_or(DEFAULT_DB_NAMESPACE.to_owned());
    pub static ref DB_DATABASE: String = std::env::var(ENV_DB_DATABASE).unwrap_or(DEFAULT_DB_DATABASE.to_owned());
}

pub trait Row : Serialize + Sync + Send + Debug + Clone {
    fn get_id(&self) -> String;
}

pub fn split_by_comma(str: &str) -> Vec<String> {
    str.split(',').map(|x| x.to_owned()).collect::<Vec<String>>()
}

pub async fn init_db_client() -> Surreal<Client> {
    info!(db_url = DB_URL.as_str(), "connecting to SurrealDB");
    let db = Surreal::new::<Ws>(DB_URL.as_str()).await.unwrap();
    info!(db_url = DB_URL.as_str(), "connected to SurrealDB");

    db.signin(Root {
        username: DB_USERNAME.as_str(),
        password: DB_PASSWORD.as_str(),
    }).await.unwrap();
    db.use_ns(DB_NAMESPACE.as_str()).use_db(DB_DATABASE.as_str()).await.unwrap();
    info!(username = DB_USERNAME.as_str(), namespace = DB_NAMESPACE.as_str(), database = DB_DATABASE.as_str(), "signed in and use namespace and database succeeded");
    db
}

pub async fn create_rows_chunk<R>
    (table: &str, chunk_index: usize, rows: Vec<R>, extra: fn(&R) -> Option<String>)
    where 
        R: Row + for<'de> Deserialize<'de>,
{
    let db = super::utils::init_db_client().await;

    let total = rows.len();

    let mut count = 0;

    let mut last_info_time = std::time::SystemTime::now();

    for row in rows {
        let id = row.get_id();
        let created: Result<R, surrealdb::Error> = db.create((table, &id))
            .content(&row)
            .await;
        if let Err(err) = created {
            warn!(table, chunk_index, "create row failed: {} {:#?} -> {:#?}", id, &row, err);
        }
        if let Some(extra_query) = extra(&row) {
            if let Err(err) = db.query(&extra_query).await {
                warn!(table, chunk_index, "execute query failed: {} -> {:#?}", &extra_query, err);
            }
        }
        let time = std::time::SystemTime::now();
        count += 1;
        if time > last_info_time + std::time::Duration::new(1, 0) {
            info!(total, table, chunk_index, count, "rows created");
            count = 0;
            last_info_time = time;
        }
    }
}

pub async fn create_rows<R>
    (table: &'static str, rows: Vec<R>, extra: fn(&R) -> Option<String>)
    where
        R: 'static + Row + for<'de> Deserialize<'de>,
{
    if rows.len() == 0 {
        warn!(table, "rows is empty!");
        return;
    }
    let total = rows.len();
    let mut chunk_index = 0;
    let mut handles = Vec::new();
    let num_cpus: usize = *NUM_CPUS;
    let chunk_size = max(MIN_CHUNK_SIZE, rows.len() / num_cpus);

    let db = super::utils::init_db_client().await;
    let define_table = format!("DEFINE {table} DROP SCHEMALESS");
    if let Err(err) = db.query(&define_table).await {
        warn!(table, chunk_index, "create table failed: {} -> {:#?}", &define_table, err);
    }

    if rows.len() > chunk_size {
        info!(total, num_cpus, chunk_size, "running in chunks mode");
        for chunk in rows.chunks(chunk_size) {
            let mut chunk_rows: Vec<R> = Vec::new();
            for x in chunk {
                chunk_rows.push(x.clone());
            }
            let handle = tokio::spawn(async move {
                create_rows_chunk(table, chunk_index, chunk_rows, extra).await;
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
        info!(total, "running in single mode");
        create_rows_chunk(table, chunk_index, rows, extra).await;
    }
}

