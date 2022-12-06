use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut current = 0;
    let mut snacks = Vec::new();


    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            snacks.push(current);
            current = 0;
            continue;
        }

        current += line.parse::<u32>().unwrap();
    }

    snacks.sort();
    snacks.reverse();

    println!("({}) + {} + {} = ({})", snacks[0], snacks[1], snacks[2], snacks[0] + snacks[1] + snacks[2]);
}
