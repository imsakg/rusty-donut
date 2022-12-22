// #![allow(unused)]

use clap::Parser;

use crate::prelude::*;
use std::fs::read_dir;

mod error;
mod prelude;
mod utils;

fn main() -> Result<()> {
    let args = Cli::parse();

    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry:?}");
    }

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    Ok(())
}
