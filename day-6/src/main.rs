use std::{fs::File, io::{BufReader, BufRead, Read}};


fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    
    for line in reader.lines() {
        let line = line.unwrap();
        let bytes = line.as_bytes();

        let mut i = 0;
        let bytes_count = bytes.len();
        let window = 14;
        'outer: while i < bytes_count - window {
            for c1 in 0..window - 1 {
                for c2 in c1 + 1..window {
                    if bytes[i + c1] == bytes[i + c2] {
                        i += 1;
                        continue 'outer
                    }
                }
            }
            break
        }
        println!("result {}", i + window);
        break;
    }


}
