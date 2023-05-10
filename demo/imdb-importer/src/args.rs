use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    //#[command(arg_required_else_help = true)]
    #[command()]
    ImportBasicTitles {
        #[arg(long, default_value = "data/title.basics.tsv")]
        path: PathBuf, 
    }
}
