use crate::args::Args;
use crate::utils::{PHRED_TO_ERROR, fastq_reader};
use bio::io::fastq::{Record, Writer};
use log::{debug, info};
use minimizer_iter::MinimizerBuilder;
use rayon::prelude::*;
use std::fs::File;

/// Generates canonical minimizers from a given sequence, kmer_size and window_size.
/// * kmer_size = length of generated minimizer(s).
/// * window_size = number of consecutive kmers for which to find the minimizer.
///
/// TODO - check if the width we provide to MinimizerBuilder is actually the
/// number of consecutive kmers, or just the sequence length.
/// * if width == num_consecutive_kmers, then width can be < kmer_size.
/// * otherwise, we need to check that width >= kmer_size.
#[inline]
fn get_minimizers(seq: &[u8], kmer_size: usize, window_size: usize) -> Vec<(u64, usize)> {
    // Move later on.
    // assert!(window_size <= seq.len());
    // assert!(kmer_size <= window_size);

    let m_iter: Vec<(u64, usize)> = MinimizerBuilder::<u64>::new()
        .canonical()
        .minimizer_size(kmer_size)
        .width(window_size as u16)
        .iter(seq)
        .map(|(mm_seq, mm_pos, _)| return (mm_seq, mm_pos))
        .collect();

    return m_iter;
}

#[inline]
fn is_significant_minimizer(mm_qual: &[u8], max_err: f64) -> bool {
    let mut err = 1.0;

    // Would be nice if there was a function similar to .sum()
    // what would do multiplication. E.g., .mul().
    mm_qual.iter().for_each(|mm_phred| {
        err *= PHRED_TO_ERROR[*mm_phred as usize];
    });

    return err < max_err;
}

#[inline]
fn get_mean_read_error(record_qual: &[u8], record_len: usize) -> f64 {
    // We could also calculate mean error rate for read and skip if too low.
    let error_sum: f64 = record_qual
        .iter()
        .map(|phred| {
            return PHRED_TO_ERROR[*phred as usize];
        })
        .sum();

    let mean_error = error_sum / record_len as f64;

    return mean_error;
}

pub fn sort_fastq(args: &Args) {
    // Check fastq for valid, can read, is gz.
    let reader = fastq_reader(&args.fastq);

    // Window size cannot be even, because Minimizer builder
    // will complain in this case (due to lexicographic tie breaking).
    let window_size = match args.window_size % 2 {
        0 => args.window_size + 1,
        _ => args.window_size,
    };

    info!("Extracting minimizers...");
    let mut records_with_metrics: Vec<(usize, Record)> = reader
        .records()
        .par_bridge()
        .filter_map(|record| {
            // Skip faulty records.
            let record = match record {
                Ok(record) => record,
                // Should log.
                Err(e) => {
                    debug!("Skipping faulty record {:?}", e);
                    return None;
                }
            };
            let record_seq = record.seq();
            let record_qual = record.qual();

            let record_len = record_seq.len();

            // Not sure what this is about.
            if record_len < 2 * args.kmer_size {
                debug!("Skipping record, too short. {}", record.id());
                return None;
            }

            let mean_read_error = get_mean_read_error(&record_qual, record_len);

            if mean_read_error > args.max_read_error {
                debug!("Skipping record, too low quality. {}", record.id());
                return None;
            }

            let mms = get_minimizers(&record_seq, args.kmer_size, window_size);

            let mut num_significant: usize = 0;
            // Each minimizer in the read.
            for (_, mm_pos) in mms {
                // Extract quality slice from minimizer position.
                let mm_qual = &record_qual[mm_pos..mm_pos + args.kmer_size];

                if is_significant_minimizer(&mm_qual, args.max_minimizer_error) {
                    num_significant += 1;
                }
            }

            return Some((num_significant, record));
        })
        .collect();

    // Sort records based on number of significant minimizers.
    info!("Sorting reads...");
    records_with_metrics.par_sort_by(|a, b| a.0.cmp(&b.0));

    // Write records. For now, only writes to plain fastq.gz.
    info!("Writing sorted reads...");
    let mut writer = Writer::new(File::create(&args.outfile).expect(""));
    for (_, record) in records_with_metrics {
        writer.write_record(&record).expect("");
    }
}
