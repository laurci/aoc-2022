use std::{fs::File, io::{BufReader, BufRead}};

#[derive(Debug, Clone, Copy)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum GameOutcome {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
struct Game {
    opponent: Pick,
    desired_outcome: GameOutcome,
}

fn compute_desired_pick(game: &Game) -> Pick {
    match game.opponent {
        Pick::Rock => match game.desired_outcome {
            GameOutcome::Win => Pick::Paper,
            GameOutcome::Loss => Pick::Scissors,
            GameOutcome::Draw => Pick::Rock,
        },
        Pick::Paper => match game.desired_outcome {
            GameOutcome::Win => Pick::Scissors,
            GameOutcome::Loss => Pick::Rock,
            GameOutcome::Draw => Pick::Paper,
        },
        Pick::Scissors => match game.desired_outcome {
            GameOutcome::Win => Pick::Rock,
            GameOutcome::Loss => Pick::Paper,
            GameOutcome::Draw => Pick::Scissors,
        },
    }
}

fn parse_input(file_path: &str) -> Vec<Game> {
    let mut games: Vec<Game> = vec![];

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" ");

        let opponent = match split.next().unwrap() {
            "A" => Pick::Rock,
            "B" => Pick::Paper,
            "C" => Pick::Scissors,
            _ => panic!("Invalid input"),
        };

        let desired_outcome = match split.next().unwrap() {
            "X" => GameOutcome::Loss,
            "Y" => GameOutcome::Draw,
            "Z" => GameOutcome::Win,
            _ => panic!("Invalid input"),
        };

        games.push(Game { opponent, desired_outcome });
    }

    games
}

fn main() {
    let games = parse_input("input.txt");

    let mut score = 0;

    for game in games {
        let player = compute_desired_pick(&game);
        match game.desired_outcome {
            GameOutcome::Win => score += 6,
            GameOutcome::Draw => score += 3,
            _ => (),
        };

        match player {
            Pick::Rock => score += 1,
            Pick::Paper =>  score += 2,
            Pick::Scissors => score += 3,
        }
    }

    println!("{}", score);
}
