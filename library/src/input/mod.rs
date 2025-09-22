use num::Integer;
use std::fs;
use std::time::{Duration, Instant};

pub enum InputType {
    Example,
    UserInput,
}
impl InputType {
    fn to_file_name(&self) -> String {
        match self {
            Self::Example => "example.txt".to_string(),
            Self::UserInput => "input.txt".to_string(),
        }
    }
}

pub trait Day<T: Integer + std::fmt::Display> {
    fn part_1(&self, input: &str) -> T;
    fn part_2(&mut self, input: &str) -> T;

    fn run(&mut self, input_type: InputType) -> std::io::Result<()> {
        let input = fs::read_to_string(input_type.to_file_name())?;

        let start: Instant = Instant::now();
        let part_1_answer = self.part_1(&input);
        let duration = start.elapsed();

        println!("Part 1 answer: {}, time: {:?}", part_1_answer, duration);

        let start: Instant = Instant::now();
        let part_2_answer = self.part_2(&input);
        let duration = start.elapsed();
        println!("Part 2 answer: {}, time: {:?}", part_2_answer, duration);

        Ok(())
    }

    fn run_n(&mut self, input_type: InputType, n: usize) -> std::io::Result<()> {
        let input = fs::read_to_string(input_type.to_file_name())?;

        let mut times = Vec::new();

        for _ in 0..n {
            let start: Instant = Instant::now();
            self.part_1(&input);
            let duration = start.elapsed();

            times.push(duration);
        }
        times.sort();

        println!(
            "Part 1 time: {:?}, {:?}, {:?}, n: {n}",
            times[0],
            times.iter().sum::<Duration>() / n.try_into().unwrap(),
            times[n - 1],
        );

        let mut times = Vec::new();

        for _ in 0..n {
            let start: Instant = Instant::now();
            self.part_2(&input);
            let duration = start.elapsed();

            times.push(duration);
        }
        times.sort();

        println!(
            "Part 2 time: {:?}, {:?}, {:?}, n: {n}",
            times[0],
            times.iter().sum::<Duration>() / n.try_into().unwrap(),
            times[n - 1],
        );

        Ok(())
    }
}
