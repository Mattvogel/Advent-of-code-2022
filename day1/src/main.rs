use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();

    let mut elf = 0;
    let mut elves = Vec::<i32>::new();

    for line in contents.lines() {
        match line.parse::<i32>() {
            Ok(i) => {
                elf = elf + i
            }
            Err(_) => {
                elves.push(elf);
                elf = 0;
                continue;
            }
        }
    }
    elves.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top3 = elves[0] + elves[1] + elves [2];
    println!("Top Elf: {}", elves[0]);
    println!("Top 3 combined: {}", top3);
    
}
