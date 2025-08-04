use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
pub struct Args {
    #[arg(short, long, help = "Path to .fastq.gz file.")]
    pub fastq: PathBuf,

    #[arg(short, long, help = "Output file (gzipped fastq).")]
    pub outfile: PathBuf,

    #[arg(short, long, default_value_t = 15, help = "Minimizer kmer size.")]
    pub kmer_size: usize,

    #[arg(
        short,
        long,
        default_value_t = 5,
        help = "Minimizer window size (num consecutive kmers)."
    )]
    pub window_size: usize,

    #[arg(
        long,
        default_value_t = 0.05,
        help = "Max allowed mean read error to keep read."
    )]
    pub max_read_error: f64,

    #[arg(
        long,
        default_value_t = 0.05,
        help = "Max allowed minimizer error to count minimizer as significant."
    )]
    pub max_minimizer_error: f64,

    #[arg(short, long)]
    pub debug: bool,
}
