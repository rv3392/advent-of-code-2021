use super::utils;

pub fn day_1a() -> i32 {
    let input = parse_input(utils::read_input("day1"));
    let mut _prev_depth= -1;
    let mut _increase_count= 0;
    for curr_depth in input {
        if _prev_depth != -1 && curr_depth > _prev_depth {
            _increase_count+=1;
        }
        _prev_depth = curr_depth;
    }
    _increase_count
}

pub fn day_1b() -> i32{
    let input = parse_input(utils::read_input("day1"));
    let mut _curr_window_sum= -1;
    let mut _prev_window_sum= -1;
    let mut _increase_count= 0;
    for i in 0..input.len() - 2{
        _curr_window_sum = input[i] + input[i + 1] + input[i + 2];
        if _prev_window_sum != -1 && _curr_window_sum > _prev_window_sum {
            _increase_count+=1;
        }
        _prev_window_sum = _curr_window_sum;
    }
    _increase_count
}

fn parse_input(input: String) -> Vec<i32> {
    let mut parsed_values = Vec::new();
    for val in input.split("\n") {
        if val == "" {
            break;
        }
        parsed_values.push(val.parse::<i32>().unwrap());
    }
    parsed_values
}

