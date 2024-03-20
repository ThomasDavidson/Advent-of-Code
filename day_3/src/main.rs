use std::fs::File;
use std::io::{BufReader, Error, Read};

fn read_file(file_path: &str) -> Result<Vec<String>, Error> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents)?;

    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();

    Ok(lines)
}

fn has_special_char(compare_top: &str, start_cmp_val: usize, end_value: usize) -> bool {
    let compare_top_section = compare_top.get(start_cmp_val..end_value);

    match compare_top_section {
        Some(val) => {
            let res = val.chars().any(|c| !c.is_numeric() && c != '.');
            // println!("String: \"{}\", result: {}", val, res);

            res
        }
        None => panic!("No value found"),
    }
}

fn get_number_from_lines(
    compare: &String,
    compare_top: Option<&String>,
    compare_bottom: Option<&String>,
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
                // println!("part number add: {}", part_number);
                count += part_number;
            }

            start_cmp = None;
            len = 0;
        }
    }
    count
}

fn main() {
    let mut count: i32 = 0;
    let lines = match read_file("input.txt") {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    for (pos, line) in lines.iter().enumerate() {

        let next_line = lines.get(pos + 1);
        let previous_line = if pos == 0 { None } else { lines.get(pos - 1) };

        count += get_number_from_lines(line, previous_line, next_line);
    }
    println!("count: {}", count);
    println!("Number of lines: {}", lines.len());
}

#[cfg(test)]
mod tests {
    use crate::get_number_from_lines;
    #[test]
    fn test1() {
        let top_line =    ".............*..........948..808..158..........%...............*................&......*537.......................=............-....529.....".to_string();
        let line =        "164*....18..753............/..*............................96.......................316........382........946...685.........455.............".to_string();
        let bottom_line = "....672...........*205.........695...........................$...279....#625....138..................115....*.........566..........410......".to_string();

        let result = get_number_from_lines(&line, Some(&top_line), Some(&bottom_line));
        assert_eq!(3415, result);
    }

    #[test]
    fn test2() {
        let top_line =    ".....*.*...........223..........&19..........*.....*...246.....*........................526*939..........*....33..51....403..........706....".to_string();
        let line =        "...832..383...287.........................216....103...........710..................958...................288...............................".to_string();

        let result = get_number_from_lines(&line, Some(&top_line), None);
        assert_eq!(2532, result);
    }
}
