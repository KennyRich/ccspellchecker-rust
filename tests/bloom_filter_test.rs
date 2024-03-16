use ccspellcheck::bloom_filter::BloomFilter;
use std::fs::{remove_file, File};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn test_insert_and_query() {
    let mut bf = BloomFilter::new(100, 3);
    bf.insert("hello");
    bf.insert("world");

    assert!(bf.query("hello"));
    assert!(bf.query("world"));
    assert!(!bf.query("foo"));
}

#[test]
fn test_save_and_load() {
    let mut bf = BloomFilter::new(100, 3);
    bf.insert("hello");
    bf.insert("world");

    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    bf.save_to_file(file_path).unwrap();

    let loaded_bf = BloomFilter::load_from_file(Some(file_path)).unwrap();

    assert!(loaded_bf.query("hello"));
    assert!(loaded_bf.query("world"));
    assert!(!loaded_bf.query("foo"));

    remove_file(file_path).unwrap();
}

#[test]
fn test_build_from_file() {
    let temp_file = NamedTempFile::new().unwrap();
    let file_path = temp_file.path();

    let words = vec!["hello", "world", "foo", "bar"];
    let mut file = File::create(file_path).unwrap();
    for word in &words {
        writeln!(file, "{}", word).unwrap();
    }

    let mut bf = BloomFilter::new(100, 3);
    bf.build_from_file(file_path, None).unwrap();

    for word in &words {
        assert!(bf.query(word));
    }
    assert!(!bf.query("baz"));

    remove_file(file_path).unwrap();
    remove_file("default_words.bf").unwrap();
}

#[test]
fn test_check_words() {
    let mut bf = BloomFilter::new(100, 3);
    bf.insert("hello");
    bf.insert("world");

    let words = vec!["hello", "world", "foo"];
    bf.check_words(&words);
}