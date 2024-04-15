use murmur_hash_rust::HashDict;
use std::io::{self, BufReader, Read};

use regex::Regex;

/// Taken straight from chatgpt
fn clean_text(input: &str) -> Vec<String> {
    let re = Regex::new(r"[^\w\s]").expect("Failed to compile regex");
    let mut results = Vec::new();

    // Iterate over lines or chunks of text
    for line in input.lines() {
        // Remove punctuation
        let clean_line = re.replace_all(line, "");
        // Split the line into words and remove empty entries
        results.extend(
            clean_line
                .split_whitespace()
                .map(|word| word.to_lowercase()), // Convert to lowercase to standardize
        );
    }

    results
}

const ARR_LEN: usize = 1000000;
const SEED: u32 = 50;
fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);
    let mut buffer = vec![0; 1024]; // Adjust buffer size as needed

    let mut word_vec: Vec<String> = Vec::new();
    while let Ok(bytes_read) = reader.read(&mut buffer) {
        if bytes_read == 0 {
            break;
        } // End of file/stream
        let chunk = String::from_utf8_lossy(&buffer[..bytes_read]);
        let words = clean_text(&chunk);
        word_vec.extend(words);
    }

    let mut dict: HashDict<String, usize> = HashDict::new(ARR_LEN, SEED);
    for word in word_vec {
        // dict.insert(word, 0);
        if dict.get(&word).is_some() {
            dict.get_mut(&word, |v| *v += 1);
        } else {
            dict.insert(word, 1);
        }
    }
    for (k, v) in dict.iter() {
        println!("{k} => {v}");
    }
}
