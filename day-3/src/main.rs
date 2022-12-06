use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    let mut total: u32 = 0;
    let mut group: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();

        if group.len() == 2 {

            let common = line.as_bytes().iter().find(|&&c| group[0].as_bytes().contains(&c) && group[1].as_bytes().contains(&c)).unwrap();
            let common_char = *common as char;

            let priority = if common_char.is_lowercase() {
                common - 96
            } else {
                27 + (common - 65)
            };
    
            total += priority as u32;

            group = Vec::new();

            continue;
        }

        group.push(line);
    }

    println!("{}", total);
}