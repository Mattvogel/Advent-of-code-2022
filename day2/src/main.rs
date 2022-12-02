use std::{fs::File, io::{BufReader, Read}};

//Rock A X
//Paper B Y
//Scissors C Z

fn main() {
    let rock = 1;
    let paper = 2;
    let scissors = 3;
    let draw = 3;
    let win = 6;

    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut score_win: i32 = 0;
    for line in contents.lines() {
        let strat: Vec<&str> = line.split_whitespace().collect();
        match strat[0] {
            "A" => { //Rock
                if strat[1] == "X" { //Rock
                    score_win = score_win + draw + rock
                } else if strat[1] == "Y" { //Paper
                    score_win = score_win + win + paper
                } else { //Scissors
                    score_win = score_win  + scissors
                }
            }
            "B" => { //Paper
                if strat[1] == "X" {
                    score_win = score_win + rock
                } else if strat[1] == "Y" {
                    score_win = score_win + draw + paper
                } else {
                    score_win = score_win + win + scissors
                }
            }
            "C" => { //Scissors
                if strat[1] == "X" {
                    score_win = score_win + win + rock
                } else if strat[1] == "Y" {
                    score_win = score_win + paper
                } else {
                    score_win = score_win + draw + scissors
                }
            }
            _ => {
                print!("wat")
            }
        }

    }
    println!("{}", score_win);
}
