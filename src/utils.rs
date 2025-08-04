use bio::io::fastq::Reader;
use flate2::read::MultiGzDecoder;
use lazy_static::lazy_static;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

lazy_static! {
    pub static ref PHRED_TO_ERROR: [f64; 126] = {
        let mut error_lookup: [f64; 126] = [1.0; 126];

        for i in 0..126 {
            if i >= 33 {
                error_lookup[i] = 10_f64.powf(-1.0 * ((i - 33) as f64) / 10.0);
            };
        }

        return error_lookup;
    };
}

pub fn fastq_reader(fastq: &PathBuf) -> Reader<BufReader<MultiGzDecoder<File>>> {
    assert!(fastq.extension().unwrap() == "gz");

    let f = File::open(fastq).expect("");
    let reader = Reader::from_bufread(BufReader::new(MultiGzDecoder::new(f)));

    return reader;
}
