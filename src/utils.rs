use std::fs;

pub fn read_input(day: &str) -> String {
    let input_filename = format!("./input/{}.txt", day);
    let data = fs::read_to_string(input_filename).expect("Unable to read file");
    data
}