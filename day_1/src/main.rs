use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    
    let lines:Vec<&str> = contents.split("\r\n").collect();

    let mut count:i32 = 0;

    for line in lines {
        let mut isfirst = true;
        let mut first_digit: char = ' ';
        let mut last_digit: char = ' ';

        print!("line: {} ", line);

        // split to remove alphabetical characters
        for line_chars in line.chars().filter(|line_chars| line_chars.is_numeric()) {
            // print!("char: {} ", line_chars);

            if isfirst {
                first_digit = line_chars;
                isfirst = false;
                // print!("\nSet first: {first_digit}\n");
            }
            // allways set since they can be the same
            last_digit = line_chars;

            
        }
        // print result

        let combined_string = first_digit.to_string() + &last_digit.to_string();

        print!(" combined: \"{}\"", combined_string);



        let result: Result<i32, _> = combined_string.parse();
        match result {
            Ok(v) => count += v,
            Err(e) => print!("{:?}\n", e.kind()),
        }

        print!(" count: {}\n", count);
        
    }
    print!("Final count: {}", count);
    Ok(())
}