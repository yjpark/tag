use lazy_static::lazy_static;

use surrealdb::opt::auth::Root;
use tracing::{info};

use surrealdb::Surreal;
use surrealdb::engine::remote::ws::{Ws, Client};

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


pub async fn init_db_client() -> Surreal<Client> {
    info!(db_url = DB_URL.as_str(), "Connecting to SurrealDB");
    let db = Surreal::new::<Ws>(DB_URL.as_str()).await.unwrap();
    info!(db_url = DB_URL.as_str(), "Connected to SurrealDB");

    db.signin(Root {
        username: DB_USERNAME.as_str(),
        password: DB_PASSWORD.as_str(),
    }).await.unwrap();
    db.use_ns(DB_NAMESPACE.as_str()).use_db(DB_DATABASE.as_str()).await.unwrap();
    info!(username = DB_USERNAME.as_str(), namespace = DB_NAMESPACE.as_str(), database = DB_DATABASE.as_str(), "Signed in and use namespace and database succeeded");
    db
}