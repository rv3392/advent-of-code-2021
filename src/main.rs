mod utils;
mod day1;
mod day2;

fn main() {
    let _day_1a_ans = day1::day_1a();
    let _day_1b_ans = day1::day_1b();
    println!("Day 1 a:{} b:{}", _day_1a_ans.to_string(), _day_1b_ans.to_string());

    let _day_2a_ans = day2::day_2a();
    let _day_2b_ans = day2::day_2b();
    println!("Day 2 a:{} b:{}", _day_2a_ans.to_string(), _day_2b_ans.to_string());
}
