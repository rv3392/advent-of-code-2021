mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
    let _day_1a_ans = day1::day_1a();
    let _day_1b_ans = day1::day_1b();
    println!("Day 1 a:{} b:{}", _day_1a_ans.to_string(), _day_1b_ans.to_string());

    let _day_2a_ans = day2::day_2a();
    let _day_2b_ans = day2::day_2b();
    println!("Day 2 a:{} b:{}", _day_2a_ans.to_string(), _day_2b_ans.to_string());

    let _day_3a_ans = day3::day_3a();
    let _day_3b_ans = day3::day_3b();
    println!("Day 3 a:{} b:{}", _day_3a_ans.to_string(), _day_3b_ans.to_string());

    let _day_4a_ans = day4::play_bingo();
    let _day_4b_ans = day4::play_bingo_last_win();
    println!("Day 4 a:{} b:{}", _day_4a_ans.to_string(), _day_4b_ans.to_string());

    let _day_5a_ans = day5::find_overlapping_vents(false);
    let _day_5b_ans = day5::find_overlapping_vents(true);
    println!("Day 5 a:{} b:{}", _day_5a_ans.to_string(), _day_5b_ans.to_string());

    let _day_6a_ans = day6::calculate_lanternfish_population(80);
    let _day_6b_ans = day6::calculate_lanternfish_population(256);
    println!("Day 6 a:{} b:{}", _day_6a_ans.to_string(), _day_6b_ans.to_string());
}
