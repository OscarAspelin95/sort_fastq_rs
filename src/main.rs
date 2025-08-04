mod sort;
mod utils;

mod args;
use args::Args;

use clap::Parser;
use log::LevelFilter;
use sort::sort_fastq;

use simple_logger::SimpleLogger;

fn main() {
    let args = Args::parse();

    let level = match args.debug {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    SimpleLogger::new()
        .with_level(level)
        .init()
        .expect("Failed to initialize SimpleLogger.");

    sort_fastq(&args);
}
