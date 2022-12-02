use std::io::BufRead;
use crate::utils;

pub fn count_top_three() {
    let file_reader = utils::read_from_file("resources/1.txt");
    let mut count_stars = 0;
    let mut three_max_stars: (i32, i32, i32) = (0, 0, 0);

    for line in file_reader.lines() {
        let parsed_line = line.unwrap().parse::<i32>();
        match parsed_line {
            Ok(val) => {
                count_stars += val
            }
            Err(..) => {
                compare_with_current_maxes(&mut three_max_stars, count_stars);
                count_stars = 0;
            }
        }
    }
    println!("{:?}", three_max_stars.0 + three_max_stars.1 + three_max_stars.2);
}

fn compare_with_current_maxes(three_max_stars: &mut (i32, i32, i32), x: i32) {
    let temp_value;
    if x > three_max_stars.0 {
        temp_value = three_max_stars.0;
        three_max_stars.0 = x;
        three_max_stars.2 = three_max_stars.1;
        three_max_stars.1 = temp_value;
    } else if x > three_max_stars.1 {
        temp_value = three_max_stars.1;
        three_max_stars.1 = x;
        three_max_stars.2 = temp_value;
    } else if x > three_max_stars.2 {
        three_max_stars.2 = x;
    }
}