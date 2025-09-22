use library::grid::Vec2;
use library::input::{Day, InputType};
use std::fmt;

fn hash_algorithm(chars: impl Iterator<Item = char>) -> usize {
    chars.fold(0, |mut acc, c| {
        acc += c as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

type Label = Vec<char>;
#[derive(Debug)]
enum Operation {
    Dash { label: Label },
    Equal { label: Label, focal_length: u32 },
}
impl Operation {
    fn parse(string: &str) -> Self {
        let (label, focal_length_str): (&str, Option<u32>) =
            match string.split_at(string.chars().position(|a| a == '=' || a == '-').unwrap()) {
                (label, "-") => (label, None),
                (label, instr_lens_comb) => {
                    let (_, lens) = instr_lens_comb.split_at(1);
                    (label, Some(lens.parse().unwrap()))
                }
            };

        let label = label.chars().collect();

        match focal_length_str {
            None => Self::Dash { label },
            Some(focal_length) => Self::Equal {
                label,
                focal_length,
            },
        }
    }
    fn label(&self) -> &Label {
        match self {
            Self::Dash { label } => label,
            Self::Equal {
                label,
                focal_length: _,
            } => label,
        }
    }
}
impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Self::Dash { label } => {
                write!(f, "{}{}-", label[0], label[1])?;
            }
            Self::Equal {
                label,
                focal_length,
            } => {
                write!(f, "{}{}={focal_length}", label[0], label[1])?;
            }
        }
        Ok(())
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
struct Lens {
    label: Label,
    focal_length: usize,
}

impl fmt::Display for Lens {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "[{}{} {}]",
            self.label[0], self.label[1], self.focal_length
        )
    }
}

struct Day15;
const DAY: Day15 = Day15;
impl Day<usize> for Day15 {
    fn part_1(&self, input: &str) -> usize {
        input
            .split(",")
            .map(|c| c.chars())
            .map(hash_algorithm)
            .sum()
    }
    fn part_2(&self, input: &str) -> usize {
        let mut boxes: [Vec<Lens>; 256] = std::array::from_fn(|_| Vec::new());

        for string in input.split(",") {
            let operation = Operation::parse(string);

            let box_num = hash_algorithm(operation.label().iter().cloned());

            match operation {
                Operation::Dash { label } => {
                    boxes[box_num]
                        .iter()
                        .position(|a| a.label == label)
                        .map(|e| boxes[box_num].remove(e));
                }
                Operation::Equal {
                    label,
                    focal_length,
                } => {
                    match boxes[box_num].iter().position(|a| a.label == label) {
                        None => boxes[box_num].push(Lens {
                            label,
                            focal_length: focal_length as usize,
                        }),
                        Some(a) => boxes[box_num][a].focal_length = focal_length as usize,
                    };
                }
            };
        }

        let boxes: Vec<Vec<Lens>> = boxes.to_vec();
        Vec2::enumerate(&boxes).iter().fold(0, |acc, (c, lens)| {
            acc + (c.x + 1) * (c.y + 1) * lens.focal_length
        })
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
