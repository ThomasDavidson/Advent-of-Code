use library::input::{Day, InputType};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

struct Machines {
    machines: Vec<Machine>,
}
impl Machines {
    fn parse(input: &str) -> Self {
        let machines = input.lines().map(Machine::parse).collect();

        Self { machines }
    }
}

#[derive(Debug)]
struct Machine {
    indicator_diagram: IndicatorDiagram,
    wiring_diagrams: Vec<WiringDiagram>,
}
impl Machine {
    fn parse(line: &str) -> Self {
        let (light, rest) = line.split_once("] ").unwrap();
        let indicator_diagram = IndicatorDiagram::parse(light);

        let (wiring, _rest) = rest.split_once(" {").unwrap();
        let wiring_diagrams = wiring.split(" ").map(WiringDiagram::parse).collect();

        Self {
            indicator_diagram,
            wiring_diagrams,
        }
    }

    fn press_button(&self, button: usize, state: u16) -> u16 {
        state ^ self.wiring_diagrams[button].instructions
    }

    fn minimum_button_press(&self) -> u32 {
        let mut states: VecDeque<(u16, usize)> = vec![(0, 1)].into();

        while let Some((state, count)) = states.pop_front() {
            for button in 0..self.wiring_diagrams.len() {
                let state = self.press_button(button, state);

                if state == self.indicator_diagram.indicator {
                    return count as u32;
                } else {
                    states.push_back((state, count + 1))
                }
            }
        }

        panic!()
    }
}
struct IndicatorDiagram {
    indicator: u16,
}
impl IndicatorDiagram {
    fn parse(text: &str) -> Self {
        let indicators: Vec<bool> = text
            .chars()
            .filter_map(|c| match c {
                '.' => Some(false),
                '#' => Some(true),
                _ => None,
            })
            .collect();

        let mut indicator: u16 = 0;
        for (indicator_pos, state) in indicators.iter().enumerate() {
            if !state {
                continue;
            }
            indicator |= 1 << indicator_pos;
        }

        Self { indicator }
    }
}

impl fmt::Debug for IndicatorDiagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#011b}", self.indicator)
    }
}

struct WiringDiagram {
    instructions: u16,
}
impl WiringDiagram {
    fn parse(text: &str) -> Self {
        let filtered_text = text
            .chars()
            .filter(|c| *c != '(' && *c != ')')
            .collect::<String>();

        let instruction_pos: Vec<u16> = filtered_text
            .split(",")
            .filter_map(|str| str.parse::<u16>().ok())
            .collect();

        let mut instructions: u16 = 0;
        for pos in instruction_pos {
            instructions |= 1 << pos;
        }

        Self { instructions }
    }
}
impl fmt::Debug for WiringDiagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#011b}", self.instructions)
    }
}

struct Day10;
const DAY: Day10 = Day10;
impl Day<u64> for Day10 {
    fn part_1(&self, input: &str) -> u64 {
        let machines = Machines::parse(input);

        let mut part_1_answer = 0;

        for machine in &machines.machines {
            let pressed = machine.minimum_button_press();
            if pressed == u32::MAX {
                panic!()
            }
            part_1_answer += pressed as u64;
        }

        part_1_answer
    }
    fn part_2(&self, _input: &str) -> u64 {
        0
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
