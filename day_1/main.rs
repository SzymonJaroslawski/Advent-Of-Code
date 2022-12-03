use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("src/calories.txt").expect("Unable to open file");
    let f = BufReader::new(f);

    let mut bigest_calorie = 0;
    let mut second_bigest_calorie = 0;
    let mut third_bigest_calorie = 0;
    let mut current_calories = 0;

    for line in f.lines() {
        let line = line.expect("Unable to read line");
        if line.is_empty() {
            if current_calories > bigest_calorie {
                third_bigest_calorie = second_bigest_calorie;
                second_bigest_calorie = bigest_calorie;
                bigest_calorie = current_calories;
            } else if current_calories > second_bigest_calorie {
                third_bigest_calorie = second_bigest_calorie;
                second_bigest_calorie = current_calories;
            } else if current_calories > third_bigest_calorie {
                third_bigest_calorie = current_calories;
            }
            current_calories = 0;
            continue;
        } else {
            current_calories += line.parse::<i32>().unwrap();
            println!("{}", current_calories);
        }
    }

    println!(
        "1nd: {}, 2nd: {}, 3nd: {}, total: {}",
        bigest_calorie,
        second_bigest_calorie,
        third_bigest_calorie,
        bigest_calorie + second_bigest_calorie + third_bigest_calorie
    );
}
