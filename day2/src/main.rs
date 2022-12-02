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
    let mut score_fudged: i32 = 0;
    for line in contents.lines() {
        let strat: Vec<&str> = line.split_whitespace().collect();
        match strat[0] {
            "A" => { //Rock
                if strat[1] == "X" { //Rock / Lose
                    score_win = score_win + draw + rock;
                    score_fudged = score_fudged + scissors;
                } else if strat[1] == "Y" { //Paper / Draw
                    score_win = score_win + win + paper;
                    score_fudged = score_fudged + rock + draw
                } else { //Scissors / Win
                    score_win = score_win + scissors;
                    score_fudged = score_fudged + paper + win;
                }
            }
            "B" => { //Paper
                if strat[1] == "X" {
                    score_win = score_win + rock;
                    score_fudged = score_fudged + rock;
                } else if strat[1] == "Y" {
                    score_win = score_win + draw + paper;
                    score_fudged = score_fudged + draw + paper;
                } else {
                    score_win = score_win + win + scissors;
                    score_fudged = score_fudged + win + scissors;
                }
            }
            "C" => { //Scissors
                if strat[1] == "X" {
                    score_win = score_win + win + rock;
                    score_fudged = score_fudged + paper;
                } else if strat[1] == "Y" {
                    score_win = score_win + paper;
                    score_fudged = score_fudged + scissors + draw;
                } else {
                    score_win = score_win + draw + scissors;
                    score_fudged = score_fudged + win + rock;
                }
            }
            _ => {
                print!("wat")
            }
        }

    }
    println!("Winning score: {}", score_win);
    println!("Fudged score: {}", score_fudged);
}
