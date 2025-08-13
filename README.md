⚠️ Depreciated since it is now part of [fastq_rs](https://github.com/OscarAspelin95/fastq_rs).

# sort_fastq_rs
🚧 Work in progress sort fastq file based on the number of low error rate canonical minimizers. Inspired by [isONClust3](https://github.com/aljpetri/isONclust3).

## Requirements
- Linux OS (Ubuntu 24.04.2)
- Rust >= 1.88.0

## Installation
Clone the repository or download the source code. Enter the sort_fastq_rs directory and run:<br>
`cargo build --release`

The generated binary is available in `target/release/sort_fastq_rs`.

## Usage
Run with:<br>
`sort_fastq_rs --fastq <reads.fastq.gz>`

Optional arguments:
<pre>
<b>-o/--outfile [sorted.fastq] - Output file for sorted fastq file.</b>

<b>-k/--kmer-size</b> [15] - Kmer size to use.

<b>-w/--window-size [5] - Number of consecutive kmers to extract minimizer from.</b>

<b>--max-read-error</b> [0.05] - Max mean read error rate to allow.

<b>--max-minimizer-error</b> [0.05] - Max minimizer error to allow.

<b>-d/--debug</b> - Log debug messages.
</pre>
