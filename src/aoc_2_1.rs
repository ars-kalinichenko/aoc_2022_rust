use crate::utils;
use std::collections::HashMap;
use std::io::BufRead;

#[repr(i32)]
#[derive(Hash, Copy, Clone, Debug)]
enum GameResultValues {
    Win = 6,
    Draw = 3,
    Lost = 0,
}

#[repr(i32)]
#[derive(Hash, Clone, Eq, PartialEq)]
enum OptionValues {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

pub fn calculate_score() {
    let mapping_letter_enum_value = HashMap::from([
        ("A", OptionValues::Rock),
        ("B", OptionValues::Paper),
        ("C", OptionValues::Scissors),
        ("X", OptionValues::Rock),
        ("Y", OptionValues::Paper),
        ("Z", OptionValues::Scissors),
    ]);

    let file_reader = utils::read_from_file("resources/2.txt");
    let mut score = 0;

    for line in file_reader.lines() {
        let unwrapped_line = line.unwrap();
        let mut split_line = unwrapped_line.split_whitespace();
        let enemy_choice = mapping_letter_enum_value.get(split_line.next().unwrap()).unwrap();
        let my_choice = mapping_letter_enum_value.get(split_line.next().unwrap()).unwrap();

        let game_result = calculate_round_result(enemy_choice, my_choice);
        score += game_result as i32 + my_choice.clone() as i32;
    }
    println!("{:?}", score);
}

fn calculate_round_result(enemy_choice: &OptionValues, my_choice: &OptionValues) -> GameResultValues {
    let mapping_choices_to_result = HashMap::from([
        ((OptionValues::Paper, OptionValues::Scissors), GameResultValues::Win),
        ((OptionValues::Paper, OptionValues::Rock), GameResultValues::Lost),
        ((OptionValues::Scissors, OptionValues::Paper), GameResultValues::Lost),
        ((OptionValues::Scissors, OptionValues::Rock), GameResultValues::Win),
        ((OptionValues::Rock, OptionValues::Scissors), GameResultValues::Lost),
        ((OptionValues::Rock, OptionValues::Paper), GameResultValues::Win),
    ]);

    if enemy_choice == my_choice {
        GameResultValues::Draw
    } else {
        match mapping_choices_to_result.get(&(enemy_choice.clone(), my_choice.clone())) {
            Some(&value) => value,
            _ => {
                panic!();
            }
        }
    }
}
