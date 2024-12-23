use std::time::Instant;

fn has_special_char(compare_top: &str, start_cmp_val: usize, end_value: usize) -> bool {
    let compare_top_section = compare_top.get(start_cmp_val..end_value);

    match compare_top_section {
        Some(val) => {
            let res = val.chars().any(|c| !c.is_numeric() && c != '.');
            res
        }
        None => panic!("No value found"),
    }
}

fn get_number_from_lines(
    compare: &&str,
    compare_top: Option<&&str>,
    compare_bottom: Option<&&str>,
) -> i32 {
    let mut count: i32 = 0;

    let mut start_cmp: Option<usize> = None;
    let mut len: usize = 0;

    for (pos, char) in compare.chars().enumerate() {
        if char.is_numeric() {
            if start_cmp.is_none() {
                start_cmp = Some(pos);
            }
            len = match start_cmp {
                None => 0,
                Some(start) => pos - start + 1,
            }
        }

        if (!char.is_numeric() || pos == compare.len() - 1) && start_cmp.is_some() {
            // remove option
            let mut start_cmp_val = match start_cmp {
                None => 0,
                Some(val) => val,
            };
            // end
            let mut end_val = start_cmp_val + len;

            let part_number: i32 = match compare.get(start_cmp_val..end_val) {
                Some(val) => {
                    let num = val.parse().unwrap();
                    num
                }
                None => panic!("No value found"),
            };

            // add 1 to end and remove one from start to check adjacent values
            if start_cmp_val > 0 {
                start_cmp_val = start_cmp_val - 1;
            }
            if end_val < compare.len() {
                end_val += 1;
            }

            let top_check: bool = match compare_top {
                Some(top) => has_special_char(top, start_cmp_val, end_val),
                None => false,
            };

            let compare_check = has_special_char(&compare, start_cmp_val, end_val);

            let bottom_check: bool = match compare_bottom {
                Some(top) => has_special_char(top, start_cmp_val, end_val),
                None => false,
            };

            if compare_check || top_check || bottom_check {
                count += part_number;
            }

            start_cmp = None;
            len = 0;
        }
    }
    count
}

fn first_try_day_1(lines: Vec<&str>) -> i32 {
    let mut count: i32 = 0;

    for pos in 0..lines.len() {
        let line = lines.get(pos).unwrap();
        let next_line = lines.get(pos + 1);
        let previous_line = if pos == 0 { None } else { lines.get(pos - 1) };

        count += get_number_from_lines(line, previous_line, next_line);
    }
    count
}

// y then x is so vectors get sorted properly
#[derive(PartialEq, Debug, Clone, Copy, Eq, PartialOrd, Ord)]
struct Coord {
    y: usize,
    x: usize,
}

fn check_adjacent_spaces(
    lines: &Vec<&str>,
    coord: Coord,
    offset_to_check: &Vec<[i16; 2]>,
    mut checked_coords: &mut Vec<Coord>,
) -> Vec<Coord> {
    let mut askii_coords: Vec<Coord> = Vec::new();
    let height = lines.len();
    let width = lines[0].len();

    // push current coord
    if !checked_coords.contains(&coord) {
        checked_coords.push(coord);
    }

    for [i, j] in offset_to_check {
        // get coordinate to check
        let offset_x = coord.x as i16 + i;
        let offset_y = coord.y as i16 + j;

        // checks if x or y offset is negative or out of bounds
        if offset_x < 0 || offset_x >= width as i16 || offset_y < 0 || offset_y >= height as i16 {
            continue;
        }
        let new_coord = Coord {
            x: offset_x as usize,
            y: offset_y as usize,
        };

        if checked_coords.contains(&new_coord) {
            continue;
        }
        checked_coords.push(new_coord);

        let line = lines[offset_y as usize];
        let letter = line.as_bytes()[offset_x as usize];
        if !letter.is_ascii_digit() {
            continue;
        }
        askii_coords.push(new_coord);
        // check coordinates to the left and right of the askii
        let secondary_check_coords = vec![[-1, 0], [1, 0]];

        // println!("Letter: {}", letter as char);
        let mut res: Vec<Coord> = check_adjacent_spaces(
            &lines,
            new_coord,
            &secondary_check_coords,
            &mut checked_coords,
        );
        askii_coords.append(&mut res);
    }

    askii_coords
}

fn debug_print(height: usize, width: usize, checked_coords: Vec<Coord>, askii_coords: Vec<Coord>) {
    let mut char_2d: Vec<Vec<char>> = Vec::new();
    for _y in 0..height {
        let mut char_1d: Vec<char> = Vec::new();
        for _x in 0..width {
            char_1d.push('.');
        }
        char_2d.push(char_1d)
    }

    for coord in checked_coords {
        char_2d[coord.y][coord.x] = 'c';
    }

    for coord in askii_coords {
        char_2d[coord.y][coord.x] = 'a';
    }

    for y in 0..height {
        for x in 0..width {
            print!("{}", char_2d[y][x]);
        }
        println!("");
    }
}

fn combine_letters_to_numbers(lines: Vec<&str>, mut number_coords: Vec<Coord>) -> Vec<i32> {
    let mut ret: Vec<i32> = Vec::new();
    let mut string: String = Default::default();
    let mut last_coord = Coord { x: 0, y: 0 };

    number_coords.sort();

    for number_coord in number_coords {
        if last_coord.y != number_coord.y || last_coord.x + 1 != number_coord.x {
            if !string.is_empty() {
                let value = string.parse::<i32>().unwrap();
                ret.push(value);

                string.clear();
            }
        }

        let line = lines[number_coord.y];
        let letter = line.as_bytes()[number_coord.x];
        string.push(letter as char);

        last_coord = number_coord;
    }

    if !string.is_empty() {
        let value = string.parse::<i32>().unwrap();
        ret.push(value);

        string.clear();
    }

    ret
}

fn part_1(input: &str) -> i32 {
    first_try_day_1(input.lines().collect())
}

fn part_2(input: &str) -> i32 {
    let str_in = input.to_string();

    let lines: Vec<&str> = str_in.lines().collect();
    let width = lines[0].len();
    let height = lines.len();

    // offsets from -1..1, -1..1 excluding 0,0
    let offset_to_check: Vec<[i16; 2]> = vec![
        [-1, 1],
        [0, 1],
        [1, 1],
        [-1, 0],
        [1, 0],
        [-1, -1],
        [0, -1],
        [1, -1],
    ];

    let mut askii_coords: Vec<Coord> = Vec::new();
    let mut checked_coords: Vec<Coord> = Vec::new();
    let mut day_2_result: i32 = 0;

    for y in 0..height {
        for x in 0..width {
            if lines[y].as_bytes()[x] == b'*' {
                // println!("Checking {} {}", x, y);
                // check all adjacent spaces
                let res = check_adjacent_spaces(
                    &lines,
                    Coord { x: x, y: y },
                    &offset_to_check,
                    &mut checked_coords,
                );

                askii_coords.append(&mut res.clone());

                let gear_ratoios = combine_letters_to_numbers(lines.clone(), res.clone());
                if gear_ratoios.len() == 2 {
                    day_2_result += gear_ratoios[0] * gear_ratoios[1];
                }
            }
        }
    }
    day_2_result
}

fn main() {
    let input = include_str!("../input.txt");

    let start: Instant = Instant::now();
    let part_1_answer = part_1(input);
    let duration = start.elapsed();
    println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

    let start: Instant = Instant::now();
    let part_2_answer = part_2(input);
    let duration = start.elapsed();
    println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);
}

#[cfg(test)]
mod tests {
    use crate::get_number_from_lines;
    #[test]
    fn test1() {
        let top_line =    ".............*..........948..808..158..........%...............*................&......*537.......................=............-....529.....";
        let line =        "164*....18..753............/..*............................96.......................316........382........946...685.........455.............";
        let bottom_line = "....672...........*205.........695...........................$...279....#625....138..................115....*.........566..........410......";

        let result = get_number_from_lines(&line, Some(&top_line), Some(&bottom_line));
        assert_eq!(3415, result);
    }

    #[test]
    fn test2() {
        let top_line =    ".....*.*...........223..........&19..........*.....*...246.....*........................526*939..........*....33..51....403..........706....";
        let line =        "...832..383...287.........................216....103...........710..................958...................288...............................";

        let result = get_number_from_lines(&line, Some(&top_line), None);
        assert_eq!(2532, result);
    }
}
