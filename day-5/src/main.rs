use std::{fs::File, io::{BufReader, BufRead}};

const NUM_STACKS: u8 = 9;

#[derive(Debug)]
struct Map {
    stacks: [Vec<char>; NUM_STACKS as usize],
}

#[derive(Debug)]
struct Move {
    from: i32,
    to: i32,
    quantity: i32,
}

fn parse_input(file_path: &str) -> (Map, Vec<Move>) {
    let mut map = Map {
        stacks: [
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ],
    };

    let mut moves: Vec<Move> = Vec::new();

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut line_idx = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        if(line_idx < 8) {
            for i in 0..NUM_STACKS {
                let pos = 1 + (i * 4) as usize;
                let create_char = line.as_bytes()[pos] as char;

                if(create_char != ' ') {
                    map.stacks[i as usize].push(create_char);
                }
            }
        }

        if(line_idx > 9) {
            let mut splits = line.split(" ");
            splits.next().unwrap();

            let quantity = splits.next().unwrap().parse::<i32>().unwrap();
            splits.next().unwrap();

            let from = splits.next().unwrap().parse::<i32>().unwrap();
            splits.next().unwrap();

            let to = splits.next().unwrap().parse::<i32>().unwrap();

            moves.push(Move {
                from: from - 1,
                to: to - 1,
                quantity,
            });
        }

        line_idx += 1;
    }


    for i in 0..NUM_STACKS {
        map.stacks[i as usize].reverse();
    }

    (map, moves)
}

fn main() {
    let (mut map, mut moves) = parse_input("input.txt");
    
    for m in moves {
        let mut crates: Vec<char> = Vec::new();
        for _ in 0..m.quantity {
            let top = map.stacks[m.from as usize].pop().unwrap();
            crates.push(top);
        }
        
        crates.reverse();

        for c in crates {
            map.stacks[m.to as usize].push(c);
        }
    }

    for i in 0..NUM_STACKS {
        print!("{}", map.stacks[i as usize].pop().unwrap());
    }
}
