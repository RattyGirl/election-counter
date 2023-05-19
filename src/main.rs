use std::path::PathBuf;

use clap::{Parser, Subcommand};
use election_counter::{BLT_Format, GoogleSheetFormat};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(value_name = "Ballot file")]
    file: PathBuf,

    #[arg(long)]
    sheets: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut blt_info = match cli.sheets {
        true => BLT_Format::from(GoogleSheetFormat::read_from_file(cli.file)),
        false => BLT_Format::read_from_file(cli.file),
    };
    blt_info.cleanup();
    stv(blt_info.clone());
}

fn stv(blt: BLT_Format) {

}
