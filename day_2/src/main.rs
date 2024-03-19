use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;


#[derive(Debug)]
struct Game {
    red: i32,
    blue: i32,
    green: i32,
}

fn get_regex_from_str(line: &str, regex: &str) -> i32 {
    let num_reg = Regex::new(regex).unwrap();

    let Some(num_capt) = num_reg.captures(line) else {
        return 0
    };

    num_capt[1].parse().unwrap()
}

fn get_color_from_lines(color_line: &str) -> Game {
    Game {
        red: get_regex_from_str(color_line, "([0-9]{1,}) red"),
        blue: get_regex_from_str(color_line, "([0-9]{1,}) blue"),
        green: get_regex_from_str(color_line, "([0-9]{1,}) green"),
    }
}

fn get_max_game_colours_from_line(line: &str) -> Game {
    let mut bag_colors: Vec<Game> = vec![];

    let mut min_required = Game{
        red: 0,
        green: 0,
        blue: 0,
    };

    for color_line in line.split(";") {
        let read_game: Game = get_color_from_lines(color_line);
        bag_colors.push(read_game);
    }

    for bag_color in bag_colors {
        if min_required.red < bag_color.red {
            min_required.red = bag_color.red;
        }
        if min_required.green < bag_color.green {
            min_required.green = bag_color.green;
        }
        if min_required.blue < bag_color.blue {
            min_required.blue = bag_color.blue;
        }
    }
    min_required
}

fn check_game(game: &Game, check: &Game) -> bool {
    check.red <= game.red && check.blue <= game.blue && check.green <= game.green
}

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    
    let lines:Vec<&str> = contents.split("\r\n").collect();

    let game = Game {
        red: 12,
        green: 13,
        blue: 14,
    };

    // print!("Num: {:?}", game_id);

    let mut day_1_count: i32 = 0;
    let mut day_2_count: i32 = 0;

    for line in lines {
        let game_id = get_regex_from_str(line, "Game ([0-9]{1,})");
        let max_game = get_max_game_colours_from_line(line);
        let result = check_game(&game, &max_game);
        
        if result {
            day_1_count += game_id;
        }
        day_2_count += max_game.red * max_game.green * max_game.blue;
        println!("Id: {}, day_1_count: {}, day_2_count {}, day_2_add {}, result: {}, color: {:?}", game_id, day_1_count, day_2_count, max_game.red * max_game.green * max_game.blue, result, max_game);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{get_max_game_colours_from_line, check_game, Game};
    #[test]
    fn test1() {
        let line:String = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, true);
    }
    #[test]
    fn test2() {
        let line:String = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, true);
    }
    #[test]
    fn test3() {
        let line:String = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, false);
    }
    #[test]
    fn test4() {
        let line:String = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, false);
    }
    #[test]
    fn test5() {
        let line:String = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, true);
    }
    #[test]
    fn test_toomanyblue() {
        let line:String = "Game 5: 6 red, 1 blue, 3 green; 20 blue, 1 red, 2 green".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, false);
    }
    #[test]
    fn test_toomanyred() {
        let line:String = "Game 5: 6 red, 1 blue, 3 green; 1 blue, 20 red, 2 green".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, false);
    }
    #[test]
    fn test_toomanygreen() {
        let line:String = "Game 5: 6 red, 1 blue, 3 green; 1 blue, 1 red, 20 green".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, false);
    }
    #[test]
    fn test_id_1() {
        let line:String = "Game 1: 1 green, 2 blue; 15 blue, 12 red, 2 green; 4 red, 6 blue; 10 blue, 8 red; 3 red, 12 blue; 1 green, 12 red, 8 blue".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, false);
    }
    #[test]
    fn test_id_2() {
        let line:String = "Game 2: 5 green, 2 red, 18 blue; 18 blue, 6 red, 9 green; 6 blue, 3 green; 6 green, 1 red, 9 blue; 19 blue, 2 green, 6 red".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, false);
    }
    #[test]
    fn test_id_3() {
        let line:String = "Game 2: 5 green, 2 red, 18 blue; 18 blue, 6 red, 9 green; 6 blue, 3 green; 6 green, 1 red, 9 blue; 19 blue, 2 green, 6 red".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, false);
    }
    #[test]
    fn test_id_53() {
        let line:String = "Game 53: 1 blue, 9 green; 1 red, 2 green; 7 green, 1 red".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, true);
    }
    #[test]
    fn test_exact_green() {
        let line:String = "13 green".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);
        assert_eq!(result, true);
    }
    #[test]
    fn test_exact_blue() {
        let line:String = "Game 5: 14 blue".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);

        assert_eq!(result, true);
    }
    #[test]
    fn test_exact_red() {
        let line:String = "Game 5: 12 red".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);

        assert_eq!(result, true);
    }
    #[test]
    fn test_1more_green() {
        let line:String = "14 green".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);

        assert_eq!(result, false);
    }
    #[test]
    fn test_1more_blue() {
        let line:String = "Game 5: 15 blue".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);

        assert_eq!(result, false);
    }
    #[test]
    fn test_1more_red() {
        let line:String = "Game 5: 13 red".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);

        assert_eq!(result, false);
    }
    #[test]
    fn test_1less_green() {
        let line:String = "12 green".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);

        assert_eq!(result, true);
    }
    #[test]
    fn test_1less_blue() {
        let line:String = "Game 5: 13 blue".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);

        assert_eq!(result, true);
    }
    #[test]
    fn test_1less_red() {
        let line:String = "Game 5: 11 red".to_string();
        let game = Game {
            red: 12,
            green: 13,
            blue: 14,
        };
        let max_game = get_max_game_colours_from_line(&line);
        let result = check_game(&game, &max_game);

        assert_eq!(result, true);
    }

}