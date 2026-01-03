use library::input::{Day, InputType};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;

type Press = u8;

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
    joltage_requirement: JoltageRequirment,
}
impl Machine {
    fn parse(line: &str) -> Self {
        let (light, rest) = line.split_once("] ").unwrap();
        let indicator_diagram = IndicatorDiagram::parse(light);

        let (wiring, rest) = rest.split_once(" {").unwrap();
        let mut wiring_diagrams: Vec<WiringDiagram> =
            wiring.split(" ").map(WiringDiagram::parse).collect();
        wiring_diagrams.sort_by(|b, a| a.positions.len().cmp(&b.positions.len()));

        let joltage_requirement = JoltageRequirment::parse(rest);

        Self {
            indicator_diagram,
            wiring_diagrams,
            joltage_requirement,
        }
    }

    fn press_button(&self, button: usize, state: u16) -> u16 {
        state ^ self.wiring_diagrams[button].instructions
    }
    fn joltage_button(&self, button: usize, state: &Vec<Press>) -> (Vec<Press>, usize) {
        let num = self.joltage_requirement.check_requirement(&state[..]).len() as u8;
        let mut count = 0;

        let mut state: Vec<Press> = state.clone();
        loop {
            count = count + 1;
            for i in &self.wiring_diagrams[button].positions {
                state[*i] += 1
            }
            let new_num = self.joltage_requirement.check_requirement(&state[..]).len() as u8;
            if new_num != num {
                break;
            }
        }
        (state, count)
    }

    fn config_wiring(&self) -> u32 {
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

    fn minimum_config_joltage(&self) -> u32 {
        let joltage_size = self.joltage_requirement.requirements.len();

        let mut minimum = u32::MAX;

        let mut states: Vec<(Vec<Press>, usize)> = vec![(vec![0; joltage_size], 0)].into();

        let mut i: u64 = 0;
        while let Some((state, count)) = states.pop() {
            let required = self.joltage_requirement.check_requirement(&state[..]);

            let buttons: Vec<usize> = self
                .wiring_diagrams
                .iter()
                .enumerate()
                .filter_map(|(i, button)| {
                    if button
                        .positions
                        .iter()
                        .any(|pos| required.contains(&(*pos as Press)))
                    {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();

            for button in buttons {
                i = i + 1;
                let (state, pressed) = self.joltage_button(button, &state);
                let count = count + pressed;

                if state
                    .iter()
                    .zip(self.joltage_requirement.requirements.iter())
                    .all(|(a, b)| *a >= *b as Press)
                {
                    minimum = minimum.min(count as u32);
                } else {
                    states.push((state.clone(), count))
                }
            }
        }

        minimum
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
    positions: Vec<usize>,
}
impl WiringDiagram {
    fn parse(text: &str) -> Self {
        let filtered_text = text
            .chars()
            .filter(|c| *c != '(' && *c != ')')
            .collect::<String>();

        let instruction_pos: Vec<usize> = filtered_text
            .split(",")
            .filter_map(|str| str.parse::<usize>().ok())
            .collect();

        let mut instructions: u16 = 0;
        for pos in instruction_pos.clone() {
            instructions |= 1 << pos;
        }

        Self {
            instructions,
            positions: instruction_pos,
        }
    }
}
impl fmt::Debug for WiringDiagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#011b}", self.instructions)
    }
}

#[derive(Debug)]
struct JoltageRequirment {
    requirements: Vec<usize>,
}
impl JoltageRequirment {
    fn parse(text: &str) -> Self {
        let filtered_text = text
            .chars()
            .filter(|c| *c != '{' && *c != '}')
            .collect::<String>();

        let requirements = filtered_text
            .split(",")
            .filter_map(|str| str.parse().ok())
            .collect();

        Self { requirements }
    }

    fn check_requirement(&self, state: &[Press]) -> Vec<Press> {
        state
            .iter()
            .zip(&self.requirements)
            .enumerate()
            .filter_map(|(i, (a, b))| {
                if *a < (*b as Press) {
                    Some(i as Press)
                } else {
                    None
                }
            })
            .collect()
    }
}

struct Day10;
const DAY: Day10 = Day10;
impl Day<u64> for Day10 {
    fn part_1(&self, input: &str) -> u64 {
        let machines = Machines::parse(input);

        let mut part_1_answer = 0;

        for machine in &machines.machines {
            let pressed = machine.config_wiring();
            if pressed == u32::MAX {
                panic!()
            }
            part_1_answer += pressed as u64;
        }

        part_1_answer
    }
    fn part_2(&self, input: &str) -> u64 {
        let machines = Machines::parse(input);

        let mut part_2_answer = 0;

        for machine in machines.machines.iter() {
            let pressed = machine.minimum_config_joltage();
            if pressed == u32::MAX {
                panic!()
            }

            part_2_answer += pressed as u64;
        }

        part_2_answer
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::Example)
}
