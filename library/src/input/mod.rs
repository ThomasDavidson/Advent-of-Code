use num::Integer;
use std::fs;
use std::time::Instant;

pub trait Day<T: Integer + std::fmt::Display> {
    fn part_1(&self, input: &str) -> T;
    fn part_2(&self, input: &str) -> T;

    fn run(&self) -> std::io::Result<()> {
        let input = fs::read_to_string("./input.txt")?;

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
}
