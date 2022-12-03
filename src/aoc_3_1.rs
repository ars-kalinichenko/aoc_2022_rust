use crate::utils;
use std::collections::HashSet;
use std::io::BufRead;

/// ## Принцип работы
/// 1. Считываем строчку, разбиваем ее пополам (так как два отделения)
/// 2. Разбив пополам, переводим каждую букву в ASCII-код, засовываем в HashSet
/// 3. Находим пересечения множеств, считаем сумму элементов пересечения
/// 4. Считаем сумму сумм
pub fn calculate_sum_of_duplicates() {
    let file_reader = utils::read_from_file("resources/3.txt");
    let mut score = 0;
    for line in file_reader.lines() {
        let mut already_proceed = HashSet::new();
        let unwrapped_line = line.unwrap();
        let (first, second) = unwrapped_line.split_at(unwrapped_line.len() / 2);
        for first_char in first.chars() {
            if second.contains(first_char) && first_char.is_ascii_lowercase() && !already_proceed.contains(&first_char)
            {
                score += first_char as u32 - 96;
            } else if second.contains(first_char)
                && first_char.is_ascii_uppercase()
                && !already_proceed.contains(&first_char)
            {
                score += first_char as u32 - 38;
            }
            already_proceed.insert(first_char);
        }
    }
    println!("{:?}", score);
}
