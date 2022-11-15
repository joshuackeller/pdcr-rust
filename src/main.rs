

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    time::Instant,
};
use rayon::prelude::*;


// use hex_literal::hex;
use sha2::{Sha256,  Digest};


fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn get_hash(line: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(line.as_bytes());
    let result = format!("{:x}", hasher.finalize());
    return result;
}

fn main() {
    const HASHED_PASSWORD: &str = "12e3cccd824622dc43885e0a227cc1b8fc8ce9e5244f6a79c46dec048e3c52e3";
    
    let now = Instant::now();
    let lines: Vec<String> = (lines_from_file("./rockyou.txt"))
    .par_iter()
    .filter_map(|password| {
        let hash = get_hash((&password).to_string()).to_lowercase();
        if hash == HASHED_PASSWORD {
            Some(password.to_string())
        } else {
            None
        }
    })
    .collect();
    println!("{:?}", lines);


    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}




