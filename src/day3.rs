use super::utils;

pub fn day_3a() -> u32 {
    let input = utils::read_input("day3");
    let mut zero_occurrences = vec![0; 12];
    let lines = input.split("\n").collect::<Vec<&str>>();
    let num_lines = lines.len();
    for diagnostic in lines {
        if diagnostic == "" {
            break;
        }

        let diagnostic_bytes = diagnostic.as_bytes();
        for bit in 0..12 {
            if diagnostic_bytes[bit] as char == '0' {
                zero_occurrences[bit] += 1;
            }
        }
    }

    println!("{}", zero_occurrences[1]);

    let mut gamma: u32 = 0b00000;
    let mut epsilon: u32 = 0b0;

    for bit in 0..12 {
        gamma = (gamma << 1) + most_common_bit(zero_occurrences[bit], &(num_lines as u32 - 1)) as u32;
        epsilon = (epsilon << 1) + least_common_bit(zero_occurrences[bit], &(num_lines as u32 - 1)) as u32;
    }

    gamma * epsilon
}

pub fn day_3b() -> u32 {
    let oxygen_generator_rating = rating_calculator(utils::read_input("day3"), most_common_bit);
    let co2_scrubber_rating = rating_calculator(utils::read_input("day3"), least_common_bit);
    oxygen_generator_rating * co2_scrubber_rating
}

fn rating_calculator(input: String, bit_criteria: fn(u32, &u32) -> u8) -> u32 {
    let mut rating = 0;
    let mut zero_occurrences = 0;
    let mut lines = input.split("\n").filter(|&x| x != "").collect::<Vec<&str>>();
    let mut num_lines = lines.len();

    for i in 0..12 {
        if num_lines <= 1 {
            break;
        }

        for &diagnostic in &lines {
            if diagnostic == "" {
                break;
            }

            let diagnostic_bytes = diagnostic.as_bytes();
            if diagnostic_bytes[i] as char == '0' {
                zero_occurrences += 1;
            }
        }

        let bit = (bit_criteria(zero_occurrences, &(num_lines as u32)) + '0' as u8) as char;
        lines = lines.iter().cloned().filter(|x| x.as_bytes()[i] as char == bit).collect::<Vec<&str>>();
        num_lines = lines.len();
        zero_occurrences = 0;
        println!("{}", num_lines);
    }

    let binary_rating = lines[0].as_bytes();
    for i in 0..12 {
        rating = (rating << 1) + (binary_rating[i] as char).to_digit(10).unwrap();
    }

    rating
}

fn most_common_bit(num_zeroes: u32, num_lines: &u32) -> u8 {
    if num_lines - num_zeroes >= num_zeroes {
        return 1;
    } else {
        return 0;
    }
}

fn least_common_bit(num_zeroes: u32, num_lines: &u32) -> u8 {
    if num_zeroes > num_lines - num_zeroes {
        return 1;
    } else {
        return 0;
    }
}