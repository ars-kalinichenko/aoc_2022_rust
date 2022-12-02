use crate::utils;
use std::collections::HashMap;
use std::io::BufRead;

#[repr(i32)]
#[derive(Hash, Copy, Clone, Debug, Eq, PartialEq)]
enum GameResultValues {
    Win = 6,
    Draw = 3,
    Lost = 0,
}

#[repr(i32)]
#[derive(Hash, Copy, Clone, Eq, PartialEq)]
enum OptionValues {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub fn calculate_score() {
    let mapping_letter_to_option = HashMap::from([
        ("A", OptionValues::Rock),
        ("B", OptionValues::Paper),
        ("C", OptionValues::Scissors),
    ]);
    let mapping_letter_to_result = HashMap::from([
        ("X", GameResultValues::Lost),
        ("Y", GameResultValues::Draw),
        ("Z", GameResultValues::Win),
    ]);

    let file_reader = utils::read_from_file("resources/2.txt");
    let mut score = 0;

    for line in file_reader.lines() {
        let unwrapped_line = line.unwrap();
        let mut split_line = unwrapped_line.split_whitespace();
        let enemy_choice = mapping_letter_to_option.get(split_line.next().unwrap()).unwrap();
        let result = mapping_letter_to_result.get(split_line.next().unwrap()).unwrap();

        let my_option = calculate_option_by_result(enemy_choice, result);
        score += my_option as i32 + *result as i32;
    }
    println!("{:?}", score);
}

fn calculate_option_by_result(enemy_choice: &OptionValues, result: &GameResultValues) -> OptionValues {
    let mapping_result_to_choice = HashMap::from([
        ((OptionValues::Paper, GameResultValues::Win), OptionValues::Scissors),
        ((OptionValues::Paper, GameResultValues::Lost), OptionValues::Rock),
        ((OptionValues::Scissors, GameResultValues::Win), OptionValues::Rock),
        ((OptionValues::Scissors, GameResultValues::Lost), OptionValues::Paper),
        ((OptionValues::Rock, GameResultValues::Win), OptionValues::Paper),
        ((OptionValues::Rock, GameResultValues::Lost), OptionValues::Scissors),
    ]);

    if result == &GameResultValues::Draw {
        *enemy_choice
    } else {
        match mapping_result_to_choice.get(&(*enemy_choice, *result)) {
            Some(&value) => value,
            _ => {
                panic!()
            }
        }
    }
}
