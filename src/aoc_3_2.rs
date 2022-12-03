use crate::utils;
use std::collections::HashSet;
use std::io::BufRead;

fn get_intersection(a: &HashSet<char>, b: &HashSet<char>) -> HashSet<char> {
    let mut intersection = HashSet::new();
    for value in a.iter() {
        if b.contains(value) {
            intersection.insert(*value);
        }
    }
    intersection
}

pub fn calculate_sum_of_badges() {
    let file_reader = utils::read_from_file("resources/3.txt");
    let mut score = 0;
    let mut line_count = 0;
    let mut tmp_set = HashSet::new();
    for line in file_reader.lines() {
        let unwrapped_line = line.unwrap();
        let mut hash_set_from_line = HashSet::new();
        for char_from_line in unwrapped_line.chars() {
            hash_set_from_line.insert(char_from_line);
        }
        if line_count == 0 {
            for char_from_line in unwrapped_line.chars() {
                tmp_set.insert(char_from_line);
            }
        }

        if line_count > 1 {
            tmp_set = get_intersection(&tmp_set, &hash_set_from_line);
            let common_char = *tmp_set.iter().next().unwrap();
            if common_char.is_ascii_lowercase() {
                score += common_char as u32 - 96;
            } else if common_char.is_ascii_uppercase() {
                score += common_char as u32 - 38;
            }
            line_count = 0;
            tmp_set.clear();
        } else {
            line_count += 1;
            tmp_set = get_intersection(&tmp_set, &hash_set_from_line);
        }
    }
    println!("{:?}", score);
}
