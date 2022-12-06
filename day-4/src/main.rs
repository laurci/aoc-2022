use std::{fs::File, io::{BufRead, BufReader}};

fn parse_pair(pair: &str) -> (i32, i32) {
    let mut iter = pair.split('-');
    let x = iter.next().unwrap().parse::<i32>().unwrap();
    let y = iter.next().unwrap().parse::<i32>().unwrap();
    (x, y)
}

fn parse_input(file_path: &str) -> Vec<((i32, i32), (i32, i32))> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().map(|line| {
        let line = line.unwrap();
        let mut split = line.split(',');
        let first_pair = split.next().unwrap();
        let second_pair = split.next().unwrap();

        let first_pair = parse_pair(first_pair);
        let second_pair = parse_pair(second_pair);

        (first_pair, second_pair)
    }).collect()
}

fn is_contained(left: (i32, i32), right: (i32, i32)) -> bool {
    left.0 >= right.0 && left.1 <= right.1
}

fn is_overlapping(left: (i32, i32), right: (i32, i32)) -> bool {
    (left.1 >= right.0 && left.0 <= right.0) || (right.1 >= left.0 && right.0 <= left.0)
}

fn main() {
    let pairs = parse_input("input.txt");
    
    let mut count = 0;

    for pair in pairs {
        if is_overlapping(pair.0, pair.1) {
            count += 1;
        }
    }

    println!("{}", count);
}
