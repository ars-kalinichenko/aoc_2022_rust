use std::fs::File;
use std::io::BufReader;

pub fn read_from_file(file_path: &str) -> BufReader<File> {
    let file = File::open(file_path).expect("file wasn't found.");
    BufReader::new(file)
}
