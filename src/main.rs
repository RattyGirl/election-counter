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
    let blt = BLT_Format::read_from_file(cli.file);

    stv(blt.clone());


    // let mut blt_info = match cli.sheets {
    //     true => BLT_Format::from(GoogleSheetFormat::read_from_file(cli.file)),
    //     false => BLT_Format::read_from_file(cli.file),
    // };
    // blt_info.cleanup();
    // stv(blt_info.clone());
}

fn stv(mut blt: BLT_Format) {
    blt.remove_withdrawals();
    blt.initial_count();
    stage(&blt);
    println!("{}", blt.info());
}

fn stage(blt: &BLT_Format) {
    //has max elected?

    //can anyone be elected
    if let Some(candidate) = blt.can_someone_be_elected() {
        println!("yes");
    }
    //exclude
}