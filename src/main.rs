use minimizer_iter::MinimizerBuilder;

fn get_minimizers(seq: &[u8], kmer_size: usize, window_size: usize) -> Vec<(u64, usize)> {
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

/// Goals:
/// * Implement minimizer generator.
/// * Add ability to calculate probabilities for minimizers from fastq seq.
/// * Sort fastq file in descending order by high confidence seeds.
fn main() {
    println!("Hello, world!");

    let seq: &'static [u8; 10] = b"AAAAAAAAAA";
    let qual: &'static [u8; 10] = b"!!!??!!!??";

    let kmer_size: usize = 3;
    let window_size: usize = 5;
    // num windows should be "seq.len() - w + 1"

    // Window size cannot be even.
    assert!(window_size % 2 != 0);

    let mms = get_minimizers(seq, kmer_size, window_size);

    for (mm_u64, mm_pos) in mms {
        let mm_qual = &qual[mm_pos..mm_pos + kmer_size];
        println!("{:?}", mm_qual);
    }
}
