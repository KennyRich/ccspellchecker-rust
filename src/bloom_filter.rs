use bitvec::prelude::*;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{self, BufRead, BufReader, BufWriter, Read, Write};
use std::path::Path;
use twox_hash::XxHash64;

pub struct BloomFilter {
    bit_array: BitVec,
    size: usize,
    hash_functions: usize,
}

impl BloomFilter {
    pub fn new(size: usize, hash_functions: usize) -> Self {
        BloomFilter {
            bit_array: bitvec![0; size],
            size,
            hash_functions,
        }
    }

    fn hash(&self, item: &str) -> Vec<u64> {
        (0..self.hash_functions)
            .map(|i| {
                let mut hasher = XxHash64::with_seed(i as u64);
                item.hash(&mut hasher);
                hasher.finish()
            })
            .collect()
    }

    pub fn insert(&mut self, item: &str) {
        let hashes = self.hash(item);
        for hash in hashes {
            let index = hash as usize % self.size;
            self.bit_array.set(index, true)
        }
    }

    pub fn query(&self, item: &str) -> bool {
        let hashes = self.hash(item);
        for hash in hashes {
            let index = hash as usize % self.size;
            if !self.bit_array[index] {
                return false;
            }
        }
        true
    }

    pub fn save_to_file(&self, file_path: &Path) -> io::Result<()> {
        let file = File::create(file_path)?;
        let mut writer = BufWriter::new(file);

        writer.write_all(b"CCBF")?;
        writer.write_all(&1u16.to_be_bytes())?;
        writer.write_all(&(self.hash_functions as u16).to_be_bytes())?;
        writer.write_all(&(self.size as u32).to_be_bytes())?;

        for chunk in self.bit_array.as_raw_slice() {
            writer.write_all(&chunk.to_le_bytes())?;
        }

        Ok(())
    }

    pub fn load_from_file(file_path: Option<&Path>) -> io::Result<Self> {
        let file_path = file_path.unwrap_or(Path::new("default_words.bf"));

        let file = File::open(file_path)?;
        let mut reader = BufReader::new(file);

        // First four bytes will be an identifier CCBF
        // Next two bytes will be version number to describe the version of the file
        // Next two bytes will be the number of hash functions used
        // The next four bytes will be the number of bits used for the filter
        let mut header = [0u8; 12];
        reader.read_exact(&mut header)?;

        if &header[0..4] != b"CCBF" {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid Header"));
        }

        let version = u16::from_be_bytes([header[4], header[5]]);
        if version != 1 {
            return Err(io::Error::new(
                io::ErrorKind::Unsupported,
                "Unsupported Version",
            ));
        }

        let hash_functions = u16::from_be_bytes([header[6], header[7]]) as usize;
        let size = u32::from_be_bytes([header[8], header[9], header[10], header[11]]) as usize;

        let mut bit_array = bitvec![0; size];

        // Calculate the number of bytes needed to represent the bit array
        let byte_size = (size + 7)/8;  // Round up to account for partial bytes
        let mut bytes = vec![0u8; byte_size];
        reader.read_exact(&mut bytes)?;

        // Convert the bytes read into bits and set them in the bit_array
        for (index, byte) in bytes.iter().enumerate() {
            for bit_index in 0..8 {
                if index * 8 + bit_index < size {
                    let bit = byte & (1 << bit_index) != 0;
                    bit_array.set(index * 8 + bit_index, bit);
                }
            }
        }


        Ok(BloomFilter {
            bit_array,
            size,
            hash_functions,
        })
    }

    pub fn build_from_file(&mut self, path: &Path, filename: Option<&Path>) -> io::Result<()> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(word) = line {
                self.insert(&word)
            }
        }

        let filename = filename.unwrap_or(Path::new("default_words.bf"));
        self.save_to_file(filename)?;
        println!("Bloom filter created and saved as '{:?}'", filename);
        Ok(())
    }

    pub fn check_words(&self, words: &[&str]) {
        for word in words {
            if self.query(word) {
                println!("'{}' is probably spelled correctly.", word);
            } else {
                println!("'{}' might be misspelled.", word);
            }
        }
    }
}
