use super::utils;
use std::cmp;

struct Floor {
    width: usize,
    height: usize,
    vents: Vec<Vec<u32>>
}

impl Floor {
    pub fn new(width: usize, height: usize) -> Self {
        let mut vents = Vec::new();
        for r in 0..height {
            let mut row = Vec::new();
            for c in 0..width {
                row.push(0);
            }
            vents.push(row);
        }

        Self {
            width,
            height,
            vents
        }
    }
}

pub fn find_overlapping_vents(allow_diagonal_vents: bool) -> u32 {
    let lines = utils::read_input("day5")
            .split("\n")
            .map(|x| x.to_string())
            .filter(|x| x != "")
            .collect();
    let mut vents = init_floor(&lines);
    map_vents(&mut vents, &lines, allow_diagonal_vents);
    count_dangerous_vents(&vents)
}

fn init_floor(lines: &Vec<String>) -> Floor {
    let (width, height) = get_floor_size(&lines);
    Floor::new(width, height)
}

fn get_floor_size(lines: &Vec<String>) -> (usize, usize) {
    let mut max_height = 0;
    let mut max_width = 0;
    for line in lines {
        let (x1, y1, x2, y2) = get_line_as_coords(line);
        if x1 > max_width || x2 > max_width {
            max_width = if x1 > x2 { x1 } else { x2 };
        }
        if y1 > max_height || y2 > max_height {
            max_height = if y1 > y2 { y1 } else { y2 };
        }
    }
    ((max_width + 1) as usize, (max_height + 1) as usize)
}

fn get_line_as_coords(line: &String) -> (u32, u32, u32, u32) {
    let coords: Vec<String> = line
            .split("->")
            .map(|x| x.to_string())
            .collect();
    let (x1, y1) = parse_coord(&coords[0]);
    let (x2, y2) = parse_coord(&coords[1]);
    (x1, y1, x2, y2)
}

fn parse_coord(coord: &String) -> (u32, u32) {
    let parsed_coord: Vec<u32> = coord
            .split(",")
            .filter(|&x| x != "")
            .map(|x| x.trim())
            .map(|x| x.parse::<u32>().unwrap())
            .collect();
    (parsed_coord[0], parsed_coord[1])
}

fn map_vents(vents: &mut Floor, vent_lines: &Vec<String>, allow_diagonal_vents: bool) {
    for vent_line in vent_lines {
        let (x1, y1, x2, y2) = get_line_as_coords(vent_line);
        if x1 == x2 || y1 == y2 {
            let mut r = cmp::min(y1, y2);
            while r <= cmp::max(y1, y2)  {
                let mut c = cmp::min(x1, x2);
                while c <= cmp::max(x1, x2) {
                    vents.vents[r as usize][c as usize] += 1;
                    c += 1;
                }
                r += 1;
            }
        } else if allow_diagonal_vents {
            if (x1 < x2 && y1 < y2) || (x2 < x1 && y2 < y1) {
                let mut r = cmp::min(y1, y2);
                while r <= cmp::max(y1, y2)  {
                    let mut c = cmp::min(x1, x2);
                    while c <= cmp::max(x1, x2) {
                        vents.vents[r as usize][c as usize] += 1;
                        c += 1;
                        r += 1;
                    }
                }
            } else if (x1 < x2 && y1 > y2) || (x2 < x1 && y2 > y1) {
                let mut r = cmp::min(y1, y2);
                while r <= cmp::max(y1, y2)  {
                    let mut c = cmp::max(x1, x2);
                    while c >= cmp::min(x1, x2) {
                        vents.vents[r as usize][c as usize] += 1;
                        c -= 1;
                        r += 1;
                    }
                }
            }
        }
    }
}

fn count_dangerous_vents(vents: &Floor) -> u32 {
    let mut vent_count = 0;
    for r in 0..vents.height {
        for c in 0..vents.width {
            if vents.vents[r][c] >= 2 {
                vent_count+=1;
            }
        }
    }
    vent_count
}