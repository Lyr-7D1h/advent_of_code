use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author = "Lyr", version = "0.1", about = "Advent of Code")]
pub struct Args {
    #[arg()]
    pub part: String,

    #[arg()]
    pub input: PathBuf,

    #[arg(short, long)]
    pub performance: bool,
}
