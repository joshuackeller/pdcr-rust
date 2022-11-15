

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

fn get_hash(line: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(line.as_bytes());
    let result = format!("{:x}", hasher.finalize());
    return result;
}

fn main() {
    const HASHED_PASSWORD: &str = "12e3cccd824622dc43885e0a227cc1b8fc8ce9e5244f6a79c46dec048e3c52e3";


    let lines: &Vec<String> = &(lines_from_file("./rockyou.txt"));
    let now = Instant::now();
    for line in lines.par_iter() {
        let hash = get_hash(line);
        if hash == HASHED_PASSWORD {
            println!("{:?}", line); 
            break;
            // print!("{0}", line)
        }
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

}





