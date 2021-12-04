use super::utils;

pub fn day_2a() -> i32 {
    let input = utils::read_input("day2");

    let mut horizontal_pos = 0;
    let mut depth = 0;

    for instruction in input.split("\n") {
        if instruction == "" {
            break;
        }

        let parsed_instruction: Vec<&str> = instruction.split(" ").collect();
        let direction = parsed_instruction[0];
        let distance = parsed_instruction[1].parse::<i32>().unwrap();

        match direction {
            "forward" => horizontal_pos += distance,
            "down" => depth += distance,
            "up" => depth -= distance,
            &_ => println!("Fell through")
        }
    }

    horizontal_pos * depth
}

pub fn day_2b() -> i32 {
    let input = utils::read_input("day2");

    let mut horizontal_pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for instruction in input.split("\n") {
        if instruction == "" {
            break;
        }

        let parsed_instruction: Vec<&str> = instruction.split(" ").collect();
        let direction = parsed_instruction[0];
        let distance = parsed_instruction[1].parse::<i32>().unwrap();

        if direction == "forward" {
            horizontal_pos = horizontal_pos + distance;
            depth = depth + distance * aim;
        } else if direction == "down" {
            aim += distance
        } else if direction == "up" {
            aim -= distance
        }
    }

    horizontal_pos * depth
}