use tracing::{info};
use clap::Parser;

use imdb_importer::args::{Args, Command};
use imdb_importer::parse::basic_titles::parse_basic_titles;
use imdb_importer::import::basic_titles::{import_basic_titles, BasicTitle};

/*
DEFINE TABLE title SCHEMALESS;
 */

pub const YEAR: u16 = 2000;
pub const MAX_TITLE_COUNT: usize = usize::MAX;


#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    match args.command {
        Command::ImportBasicTitles { path } => {
            info!(path = path.as_path().to_str(), "Parsing basic titles");

            /* Tried with datafusion, the loading seems quite slow somehow
            let df = parse_basic_titles(path.as_path()).await.unwrap();
            let count = df.count().await.unwrap();
            info!(count, "Got basic titles");
             */

            let titles: Vec<BasicTitle> = parse_basic_titles(path.as_path(), MAX_TITLE_COUNT, |record| record.start_year == YEAR).unwrap();
            import_basic_titles(titles).await
        },
    }
}
