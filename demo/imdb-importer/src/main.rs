use std::collections::HashSet;
use std::fs::File;
use std::io::{BufWriter, Write, BufReader, BufRead};

use imdb_importer::import::utils::split_by_comma;
use tracing::{info, warn};
use clap::Parser;

use imdb_importer::args::{Args, Command};
use imdb_importer::parse::title::parse_basic_titles;
use imdb_importer::import::title::{import_basic_titles, BasicTitle};
use imdb_importer::parse::name::parse_basic_names;
use imdb_importer::import::name::{import_basic_names, BasicName};
use imdb_importer::parse::principal::parse_title_principals;
use imdb_importer::import::principal::{import_title_principals, TitlePrincipal};

/*
DEFINE TABLE title SCHEMALESS;
 */

pub const YEAR: u16 = 2000;
pub const MAX_TITLE_COUNT: usize = usize::MAX;
pub const MAX_NAME_COUNT: usize = usize::MAX;
pub const MAX_PRINCIPAL_COUNT: usize = usize::MAX;


#[tokio::main(flavor = "multi_thread", worker_threads = 16)]
async fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    match args.command {
        Command::ImportBasicTitles { path } => {
            info!(path = path.as_path().to_str(), "parsing basic titles");

            let titles: Vec<BasicTitle> = parse_basic_titles(path.as_path(), MAX_TITLE_COUNT, |record| record.start_year == YEAR).unwrap();
            write_year_title_ids(YEAR, &titles);
            import_basic_titles(titles).await
        },
        Command::ImportBasicNames { path } => {
            info!(path = path.as_path().to_str(), "parsing basic names");

            let mut title_ids = HashSet::new();
            title_ids.extend(read_year_title_ids(YEAR));

            let names: Vec<BasicName> = parse_basic_names(path.as_path(), MAX_NAME_COUNT, |record| {
                let titles = split_by_comma(&record.known_for_titles);
                for title in titles.iter() {
                    if title_ids.contains(title) {
                        return true;
                    }
                }
                false
            }).unwrap();
            import_basic_names(names).await
        },
        Command::ImportTitlePrincipals { path } => {
            info!(path = path.as_path().to_str(), "parsing title principals");

            let mut title_ids = HashSet::new();
            title_ids.extend(read_year_title_ids(YEAR));

            let principals: Vec<TitlePrincipal> = parse_title_principals(path.as_path(), MAX_PRINCIPAL_COUNT, |record| {
                title_ids.contains(record.tconst.as_str())
            }).unwrap();
            import_title_principals(principals).await
        },
    }
}

fn write_year_title_ids(year: u16, titles: &Vec<BasicTitle>) {
    let file = File::create(format!("data/title.{}.id.txt", year)).unwrap();
    let mut writer = BufWriter::new(file);
    for title in titles.iter() {
        writeln!(writer, "{}", title.tconst).unwrap();
    }
    writer.flush().unwrap();
}

fn read_year_title_ids(year: u16) -> Vec<String> {
    let path = format!("data/title.{}.id.txt", year);
    info!(year, path, "loading year titles ids");
    let file = File::open(path).unwrap();
    let mut ids = Vec::new();
    for line in BufReader::new(file).lines() {
        if let Ok(id) = line {
            ids.push(id);
        }
    }
    info!(year = YEAR, count = ids.len(), "read year title ids");
    ids
}
