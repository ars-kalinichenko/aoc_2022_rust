use crate::utils;
use std::io::BufRead;

pub fn count_top_one() {
    let file_reader = utils::read_from_file("resources/1.txt");
    let mut max_stars = 0;
    let mut count_stars = 0;

    for line in file_reader.lines() {
        let parsed_line = line.unwrap().parse::<i32>();
        match parsed_line {
            Ok(val) => count_stars += val,
            Err(..) if count_stars > max_stars => {
                max_stars = count_stars;
                count_stars = 0;
            }
            Err(..) => {
                count_stars = 0;
            }
        }
    }
    println!("{:?}", max_stars);
}
