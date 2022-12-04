use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("src/input").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut points = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");

        if line.starts_with("A") {
            if line.contains("X") {
                points += 3;
                continue;
            } else if line.contains("Y") {
                points += 4;
                continue;
            } else {
                points += 8;
                continue;
            }
        } else if line.starts_with("B") {
            if line.contains("X") {
                points += 1;
                continue;
            } else if line.contains("Y") {
                points += 5;
                continue;
            } else {
                points += 9;
                continue;
            }
        } else if line.starts_with("C") {
            if line.contains("X") {
                points += 2;
                continue;
            } else if line.contains("Y") {
                points += 6;
                continue;
            } else {
                points += 7;
                continue;
            }
        }
    }
    println!("Points: {}", points);
}
