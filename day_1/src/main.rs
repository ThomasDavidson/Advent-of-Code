use std::num::ParseIntError;

fn get_word_from_line(line: &str) -> Result<i32, ParseIntError> {
    let letters: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut first_digit: char = ' ';
    let mut first_digit_location: i32 = (line.len() + 1) as i32;
    let mut last_digit: char = ' ';
    let mut last_digit_location: i32 = -1;

    print!("line: {} ", line);


    // check for string numbers by iterating through the string letters
    for (i, letter) in letters.iter().enumerate() {
        let find_letter = line.match_indices(letter);

        for (pos, _) in find_letter{
            if first_digit_location > pos as i32 {
                first_digit_location = pos as i32 ;
                first_digit = char::from_digit((i + 1) as u32, 10).unwrap();    
            }
            if last_digit_location < pos as i32 {
                last_digit_location = pos as i32 ;
                last_digit = char::from_digit((i + 1) as u32, 10).unwrap();
            }
        }

    }

    for (i, letter) in line.chars().enumerate() {

        if letter.is_numeric() {
            if first_digit_location > i as i32{
                first_digit_location = i as i32;
                first_digit = letter;    
            }
            if last_digit_location < i as i32{
                last_digit_location = i as i32;
                last_digit = letter;
            }
        }
    }
    // print result

    let combined_string = first_digit.to_string() + &last_digit.to_string();

    print!(" combined: \"{}\"", combined_string);



    let result: Result<i32, _> = combined_string.parse();

    return result;

}

fn main() -> std::io::Result<()> {
    let input = include_str!("../input.txt");

    let lines:Vec<&str> = input.lines().collect();

    let mut count:i32 = 0;

    for line in lines {

        let result: Result<i32, _> = get_word_from_line(line);
        match result {
            Ok(v) => count += v,
            Err(e) => panic!("{:?}\n", e.kind()),
        }

        print!(" count: {}\n", count);
        
    }
    print!("Final count: {}", count);
    Ok(())
}


#[cfg(test)]
mod tests {
    use crate::get_word_from_line;
    #[test]
    fn test1() {
        let result = get_word_from_line("two1nine").unwrap();
        assert_eq!(result, 29);
    }
    #[test]
    fn test2() {
        let result = get_word_from_line("eightwothree").unwrap();
        assert_eq!(result, 83);
    }
    #[test]
    fn test3() {
        let result = get_word_from_line("abcone2threexyz").unwrap();
        assert_eq!(result, 13);
    }
    #[test]
    fn test4() {
        let result = get_word_from_line("xtwone3four").unwrap();
        assert_eq!(result, 24);
    }
    #[test]
    fn test5() {
        let result = get_word_from_line("4nineeightseven2").unwrap();
        assert_eq!(result, 42);
    }
    #[test]
    fn test6() {
        let result = get_word_from_line("zoneight234").unwrap();
        assert_eq!(result, 14);
    }
    #[test]
    fn test7() {
        let result = get_word_from_line("7pqrstsixteen").unwrap();
        assert_eq!(result, 76);
    }
    #[test]
    fn test8() {
        let result = get_word_from_line("nineninesix6nine").unwrap();
        assert_eq!(result, 99);
    }
}