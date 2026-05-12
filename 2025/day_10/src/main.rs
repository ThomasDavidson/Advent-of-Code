use itertools::Itertools;
use library::input::{Day, InputType};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;
use std::slice::Iter;

type Press = u16;

struct Machines {
    machines: Vec<Machine>,
}
impl Machines {
    fn parse(input: &str) -> Self {
        let machines = input.lines().map(Machine::parse).collect();
        Self { machines }
    }
}

struct Machine {
    indicator_diagram: IndicatorDiagram,
    wiring_diagrams: WiringDiagrams,
    joltage_requirement: JoltageRequirement,
}
impl Machine {
    fn parse(line: &str) -> Self {
        let (light, rest) = line.split_once("] ").unwrap();
        let indicator_diagram = IndicatorDiagram::parse(light);

        let (wiring, rest) = rest.split_once(" {").unwrap();
        let mut wiring_diagrams: Vec<WiringDiagram> =
            wiring.split(" ").map(WiringDiagram::parse).collect();
        wiring_diagrams.sort_by(|b, a| a.positions.len().cmp(&b.positions.len()));

        let joltage_requirement = JoltageRequirement::parse(rest);

        Self {
            indicator_diagram,
            wiring_diagrams: WiringDiagrams::new(wiring_diagrams),
            joltage_requirement,
        }
    }

    // part 1 toggle state on or off
    fn press_button(&self, button: usize, state: u16) -> u16 {
        state ^ self.wiring_diagrams.get(button).instructions
    }
    fn get_instructions(&self, button: usize) -> u16 {
        self.wiring_diagrams.get(button).instructions
    }
    // press button until one of the requirements are met
    fn joltage_button(&self, button: usize, mut state: Vec<Press>) -> (Vec<Press>, usize) {
        let num = self.joltage_requirement.check_requirement(&state[..]).len();
        let mut count = 0;

        loop {
            count = count + 1;
            for i in &self.wiring_diagrams.get(button).positions {
                state[*i] += 1
            }
            let new_num = self.joltage_requirement.check_requirement(&state[..]).len();
            if new_num != num {
                break;
            }
        }
        (state, count)
    }

    // get fewest button presses for part 1
    fn config_wiring(&self) -> u32 {
        let mut states: VecDeque<(u16, usize)> = vec![(0, 1)].into();
        let goal = self.indicator_diagram.indicator;

        while let Some((state, count)) = states.pop_front() {
            for button in 0..self.wiring_diagrams.len() {
                if self.get_instructions(button) & (goal ^ state) == 0 {
                    continue;
                }

                let new_state = self.press_button(button, state);

                if new_state == goal {
                    return count as u32;
                } else {
                    states.push_back((new_state, count + 1))
                }
            }
        }

        panic!()
    }

    fn minimum_config_joltage(&self) -> Option<u32> {
        let joltage_size = self.joltage_requirement.requirements.len();

        let mut minimum: Option<u32> = None;

        let mut states: VecDeque<(Vec<Press>, usize)> = vec![(vec![0; joltage_size], 0)].into();

        let mut i: u64 = 0;
        let mut max_count = 0;
        while let Some((state, count)) = states.pop_front() {
            max_count = max_count.max(count);
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
                let (state, pressed) = self.joltage_button(button, state.clone());
                let count = count + pressed;

                if state
                    .iter()
                    .zip(self.joltage_requirement.requirements.iter())
                    .all(|(a, b)| *a >= *b as Press)
                {
                    minimum = Some(minimum.unwrap_or(u32::MAX).min(count as u32));
                } else if count < minimum.unwrap_or(u32::MAX) as usize {
                    states.push_back((state.clone(), count))
                }
            }
        }

        minimum
    }
}
impl fmt::Debug for Machine {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.indicator_diagram)?;
        write!(f, "{}", self.wiring_diagrams)?;
        write!(f, " {}", self.joltage_requirement)?;

        Ok(())
    }
}

struct IndicatorDiagram {
    indicator: u16,
    len: usize,
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
        let len = indicators.len();

        let mut indicator: u16 = 0;
        for (indicator_pos, state) in indicators.iter().enumerate() {
            if !state {
                continue;
            }
            indicator |= 1 << indicator_pos;
        }

        Self { indicator, len }
    }
}

impl fmt::Debug for IndicatorDiagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:#011b}", self.indicator)
    }
}
impl fmt::Display for IndicatorDiagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for i in 0..self.len {
            if self.indicator >> i & 1 == 1 {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}

#[derive(Debug)]
struct WiringDiagrams {
    wiring_diagrams: Vec<WiringDiagram>,
}
impl WiringDiagrams {
    fn new(wiring_diagrams: Vec<WiringDiagram>) -> Self {
        Self { wiring_diagrams }
    }
    fn get(&self, index: usize) -> &WiringDiagram {
        &self.wiring_diagrams[index]
    }
    fn len(&self) -> usize {
        self.wiring_diagrams.len()
    }
    fn iter(&self) -> Iter<'_, WiringDiagram> {
        self.wiring_diagrams.iter()
    }
    fn remove_redundant_buttons(&mut self) {
        let mut remove: Vec<usize> = Vec::new();
        for wiring_diagram in self
            .wiring_diagrams
            .clone()
            .iter()
            .enumerate()
            .combinations(2)
        {
            let (i, first) = wiring_diagram[0];
            let (j, second) = wiring_diagram[1];
            if first
                .positions
                .iter()
                .all(|&pos| second.positions.contains(&pos))
            {
                remove.push(i);
            }

            if second
                .positions
                .iter()
                .all(|&pos| first.positions.contains(&pos))
            {
                remove.push(j);
            }
        }
        remove.sort();
        remove.dedup();
        for i in remove.iter().rev() {
            self.wiring_diagrams.remove(*i);
        }
    }
}
impl fmt::Display for WiringDiagrams {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for wiring_diagram in self.wiring_diagrams.iter() {
            write!(f, " {}", wiring_diagram)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
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
impl fmt::Display for WiringDiagram {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (i, position) in self.positions.iter().enumerate() {
            write!(f, "{position}")?;
            if i != self.positions.len() - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, ")")?;
        Ok(())
    }
}

#[derive(Debug)]
struct JoltageRequirement {
    requirements: Vec<usize>,
}
impl JoltageRequirement {
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
impl fmt::Display for JoltageRequirement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for (i, requirements) in self.requirements.iter().enumerate() {
            write!(f, "{requirements}")?;
            if i != self.requirements.len() - 1 {
                write!(f, ",")?;
            }
        }
        write!(f, "}}")?;
        Ok(())
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
        let mut machines = Machines::parse(input);

        for machine in machines.machines.iter_mut() {
            machine.wiring_diagrams.remove_redundant_buttons()
        }

        let mut part_2_answer = 0;

        for (i, machine) in machines.machines.iter().enumerate() {
            let Some(pressed) = machine.minimum_config_joltage() else {
                panic!()
            };

            eprintln!("{}/{}\t{pressed}\t", i, machines.machines.len());
            part_2_answer += pressed as u64;
        }

        part_2_answer
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
