use std::path::PathBuf;

use clap::{Parser, Subcommand};
use election_counter::BLT_Format;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "FILE")]
    file: PathBuf,
    
    #[arg(long)]
    sheets: bool,
}

fn main() {
    let cli = Cli::parse();

    let blt_info = BLT_Format::read_from_file(cli.file);
}
