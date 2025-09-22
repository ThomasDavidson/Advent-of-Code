use library::input::{Day, InputType};

struct Oasis {
    reports: Vec<Prediction>,
}

impl Oasis {
    fn parse(input: &str) -> Self {
        Self {
            reports: input.lines().map(Prediction::parse).collect(),
        }
    }
}

#[derive(Debug)]
struct Prediction {
    prediction: Vec<Vec<i64>>,
}
impl Prediction {
    fn parse(line: &str) -> Self {
        let initial = line
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        Self {
            prediction: vec![initial],
        }
    }

    fn generate_difference(&mut self) {
        let prediction: &mut Vec<i64> = self.prediction.last_mut().unwrap();

        let new_prediction: Vec<i64> = prediction[1..]
            .iter()
            .zip(prediction.iter())
            .map(|(p1, p2)| p1 - p2)
            .collect();

        self.prediction.push(new_prediction);
    }
    fn last_prediction(&self) -> bool {
        self.prediction.last().unwrap().iter().all(|&a| a == 0)
    }

    fn extrapolate_forward(&mut self) {
        let mut add = 0;

        for row in self.prediction.iter_mut().rev() {
            add += row.last().unwrap();

            row.push(add);
        }
    }
    fn extrapolate_backword(&mut self) {
        let mut add = 0;

        for row in self.prediction.iter_mut().rev() {
            add = row.first().unwrap() - add;

            row.insert(0, add);
        }
    }
}

struct Day9;
const DAY: Day9 = Day9;
impl Day<i64> for Day9 {
    fn part_1(&self, input: &str) -> i64 {
        let mut oasis = Oasis::parse(input);

        oasis
            .reports
            .iter_mut()
            .map(|report| {
                while !report.last_prediction() {
                    report.generate_difference();
                }

                report.extrapolate_forward();

                report
                    .prediction
                    .first()
                    .unwrap()
                    .last()
                    .unwrap()
                    .to_owned()
            })
            .sum::<i64>()
    }
    fn part_2(&self, input: &str) -> i64 {
        let oasis = Oasis::parse(input);

        oasis
            .reports
            .into_iter()
            .map(|mut report| {
                while !report.last_prediction() {
                    report.generate_difference();
                }

                report.extrapolate_backword();

                report
                    .prediction
                    .first()
                    .unwrap()
                    .first()
                    .unwrap()
                    .to_owned()
            })
            .sum::<i64>()
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
