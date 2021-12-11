use super::utils;
use std::io::{self, BufRead};

const CROSSED_BINGO_NUMBER: i32 = -1;
struct BingoBoard {
    board: Vec<Vec<i32>>,
    width: u32,
    height: u32,
    has_won: bool
}

struct NumbersToCall {
    index_to_call: usize,
    numbers: Vec<i32>,
    length: usize
}

struct FileParser {
    next_index: usize,
    file_lines: Vec<String>,
    length: usize
}

pub fn play_bingo_last_win() -> i32 {
    let input = utils::read_input("day4");
    let lines: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
    let mut moves = parse_numbers(lines[0].clone());

    let mut input_lines = FileParser{
        next_index: 2,
        length: lines.len(),
        file_lines: lines
    };
    let mut boards = parse_boards(input_lines);

    let mut win_index: i32 = -1;
    let mut just_called: i32 = -1;
    while !have_all_boards_won(&boards) && moves.index_to_call < moves.length{
        just_called = play_round(&mut moves, &mut boards);
        moves.index_to_call += 1;
        for i in 0..boards.len() {
            if check_for_win(&mut boards[i]) && !boards[i].has_won {
                boards[i].has_won = true;
                win_index = i as i32;
            }
        }
    }
    calculate_winning_score(&mut boards[win_index as usize], just_called)
}

fn have_all_boards_won(boards: &Vec<BingoBoard>) -> bool {
    for i in 0..boards.len() {
        if !boards[i].has_won {
            return false;
        }
    }
    true
}

pub fn play_bingo() -> i32 {
    let input = utils::read_input("day4");
    let lines: Vec<String> = input.split("\n").map(|x| x.to_string()).collect();
    let mut moves = parse_numbers(lines[0].clone());

    let mut input_lines = FileParser{
        next_index: 2,
        length: lines.len(),
        file_lines: lines
    };
    let mut boards = parse_boards(input_lines);

    let mut win_index: i32 = -1;
    let mut just_called: i32 = -1;
    while win_index == -1 {
        just_called = play_round(&mut moves, &mut boards);
        moves.index_to_call += 1;
        for i in 0..boards.len() {
            if check_for_win(&mut boards[i]) {
                win_index = i as i32;
                break;
            }
        }
    }
    calculate_winning_score(&mut boards[win_index as usize], just_called)
}

fn play_round(moves: &mut NumbersToCall, boards: &mut Vec<BingoBoard>) -> i32 {
    let called = moves.numbers[moves.index_to_call];
    for board in boards {
        for r in 0..board.board.len() {
            for c in 0..board.board[r].len(){
                if board.board[r][c] == called {
                    board.board[r][c] = CROSSED_BINGO_NUMBER;
                }
            }
        }
    }
    called
}

fn parse_numbers<'a>(unparsed_numbers: String) -> NumbersToCall {
    let mut parsed_numbers = Vec::new();
    for number in unparsed_numbers.split(",") {
        parsed_numbers.push(number.parse::<i32>().unwrap());
    }
    NumbersToCall{
        index_to_call: 0, 
        length: parsed_numbers.len(),
        numbers: parsed_numbers,
    }
}

fn parse_boards(mut input_lines: FileParser) -> Vec<BingoBoard> {
    let mut boards = Vec::new();
    
    while input_lines.next_index < input_lines.length - 1 {
        // if line == "" {
        //     input_lines.next_index = input_lines.next_index + 1;
        //     break;
        // }

        let new_board = parse_board(&mut input_lines);
        boards.push(new_board);
        input_lines.next_index += 1;
    }
    boards
}

fn parse_board(input_lines: &mut FileParser) -> BingoBoard {
    let mut board  = Vec::new();
    while input_lines.next_index < input_lines.length {
        let line = &input_lines.file_lines[input_lines.next_index];
        if line == "" {
            break;
        }
        let mut row = Vec::new();
        for value in line.split(" ") {
            if value != "" {
                row.push(value.parse::<i32>().unwrap());
            }
        }
        board.push(row);
        input_lines.next_index = input_lines.next_index + 1;
    }
    BingoBoard {
        width: board[0].len() as u32, 
        height: board.len() as u32,
        board: board,
        has_won: false
    }
}

fn calculate_winning_score(winning_board: &mut BingoBoard, just_called: i32) -> i32 {
    let mut sum_unmarked = 0;
    for r in 0..winning_board.height {
        for c in 0..winning_board.width {
            if winning_board.board[r as usize][c as usize] != CROSSED_BINGO_NUMBER {
                sum_unmarked += winning_board.board[r as usize][c as usize];
            }
        }
    }
    sum_unmarked * just_called
}

fn check_for_win(board: &mut BingoBoard) -> bool {
    let mut row_won = true;
    for row in board.board.iter() {
        for &value in row {
            if value != CROSSED_BINGO_NUMBER {
                row_won = false;
                break;
            }
        }
        if row_won == true {
            return true;
        }
        row_won = true;
    }

    let mut col_won = true;
    for c in 0..board.width {
        for r in 0..board.height {
            if board.board[r as usize][c as usize] != CROSSED_BINGO_NUMBER {
                col_won = false;
                break;
            }
        }
        if col_won == true {
            return true;
        }
        col_won = true;
    }
    false
}

// [#cfg(test)]
// mod tests {
//     #[test]
//     fn test_example() {

//     }
// }