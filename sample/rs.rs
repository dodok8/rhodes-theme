// Rust Example

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let path = Path::new("lorem.txt");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    let mut frequency: HashMap<String, i32> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        for word in line.split_whitespace() {
            *frequency.entry(word.to_string()).or_insert(0) += 1;
        }
    }

    let mut count_vec: Vec<(&String, &i32)> = frequency.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));

    for (word, count) in count_vec.into_iter().take(10) {
        println!("{}: {}", word, count);
    }

    Ok(())
}