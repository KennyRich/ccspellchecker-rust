use ccspellcheck::bloom_filter::BloomFilter;
use clap::{value_parser, Arg, ArgAction, Command};
use std::path::{Path};

mod bloom_filter;

fn main() {
    let matches = Command::new("ccspellcheck")
        .version("1.0")
        .author("Kehinde Richard Ogunyale <kogunyale01@gmail.com>")
        .about("Spell checker using a Bloom Filter")
        .arg(
            Arg::new("build")
                .long("build")
                .value_name("FILE")
                .help("Builds a Bloom Filter from a dictionary file")
                .value_parser(value_parser!(String)),
        )
        .arg(
            Arg::new("size")
                .long("size")
                .value_name("SIZE")
                .value_parser(value_parser!(usize))
                .required_if_eq("build", "some_value")
                .default_value("3670016"),
        )
        .arg(
            Arg::new("num_hashes")
                .long("num_hashes")
                .value_name("NUM_HASHES")
                .value_parser(value_parser!(usize))
                .required_if_eq("build", "some_value")
                .default_value("4"),
        )
        .arg(
            Arg::new("check")
                .long("check")
                .value_name("WORD")
                .help("Checks if a word is in the dictionary")
                .num_args(1..)
                .action(ArgAction::Append),
        )
        .get_matches();

    let size = *matches.get_one::<usize>("size").unwrap();
    let num_hashes = *matches.get_one::<usize>("num_hashes").unwrap();

    if let Some(file) = matches.get_one::<String>("build") {
        let mut bloom_filter = BloomFilter::new(size, num_hashes);
        let dict_file = Path::new(file);

        bloom_filter
            .build_from_file(dict_file, None)
            .expect("Unable to build bloom filter from dict");
    }

    if let Some(words) = matches.get_many::<String>("check") {
        let words: Vec<&str> = words.collect::<Vec<_>>().iter().map(|s| s.as_str()).collect();
        let bloom_filter = match BloomFilter::load_from_file(None){
            Ok(bf) => bf,
            Err(e) => {
                eprintln!("Error loading file {}", e);
                return ;
            }
        };

        bloom_filter.check_words(&words);
    }
}
