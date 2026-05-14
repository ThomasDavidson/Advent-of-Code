use colored::Colorize;
use itertools::Itertools;
use library::input::{Day, InputType};
use nalgebra::{DMatrix, Dyn, OMatrix};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Index, IndexMut, MulAssign, Rem};
use std::slice::Iter;

type Press = u16;

#[derive(PartialOrd, PartialEq, Ord, Eq)]
struct MachineState {
    state: Vec<Press>,
    count: usize,
}
impl MachineState {
    fn init(joltage_size: usize) -> Self {
        Self {
            state: vec![0; joltage_size],
            count: 0,
        }
    }
    fn new(state: Vec<Press>, count: usize) -> Self {
        Self { state, count }
    }
}

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
        let wiring_diagrams: Vec<WiringDiagram> =
            wiring.split(" ").map(WiringDiagram::parse).collect();

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
        // get button positions
        let button_positions = &self.wiring_diagrams.get(button).positions;

        // check number of fulfilled requirements
        let num = self
            .joltage_requirement
            .check_requirement(&state[..])
            .0
            .len();

        // get difference between current joltage and requirement
        let remaining = {
            let mut remaining = state
                .iter()
                .zip(self.joltage_requirement.requirements.iter())
                .map(|(state_press, required)| *required - *state_press as usize)
                .collect::<Vec<_>>();
            remaining.sort();
            remaining.dedup();
            remaining.len()
        };

        let mut count = 0;

        loop {
            count = count + 1;
            for i in button_positions {
                state[*i] += 1
            }
            let new_num = self
                .joltage_requirement
                .check_requirement(&state[..])
                .0
                .len();

            let new_remaining = {
                let mut new_remaining = state
                    .iter()
                    .zip(self.joltage_requirement.requirements.iter())
                    .map(|(state_press, required)| *required - *state_press as usize)
                    .collect::<Vec<_>>();
                new_remaining.sort();

                new_remaining.dedup();
                new_remaining
            };

            if new_remaining.len() < remaining {
                break;
            }

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

        let mut states: Vec<MachineState> = vec![MachineState::init(joltage_size)];

        let mut max_count = 0;
        while let Some(machine_state) = states.pop() {
            max_count = max_count.max(machine_state.count);

            let (satisfied, unsatisfied) = self
                .joltage_requirement
                .check_requirement(&machine_state.state[..]);

            let buttons: Vec<usize> = self
                .wiring_diagrams
                .iter()
                .enumerate()
                .filter_map(|(i, button)| {
                    if button
                        .positions
                        .iter()
                        .any(|pos| unsatisfied.contains(&(*pos as Press)))
                        && !button
                            .positions
                            .iter()
                            .any(|pos| satisfied.contains(&(*pos as Press)))
                    {
                        Some(i)
                    } else {
                        None
                    }
                })
                .collect();
            for button in buttons {
                let (state, pressed) = self.joltage_button(button, machine_state.state.clone());
                let count = machine_state.count + pressed;
                // let mut history = machine_state.history.clone();
                // history.push((button, pressed));
                // eprintln!();

                if state
                    .iter()
                    .zip(self.joltage_requirement.requirements.iter())
                    .all(|(a, b)| *a == *b as Press)
                {
                    eprintln!("Found one solution");
                    minimum = Some(minimum.unwrap_or(u32::MAX).min(count as u32));
                } else if count < minimum.unwrap_or(u32::MAX) as usize {
                    states.push(MachineState::new(state.clone(), count));
                    states.sort();
                    states.dedup();
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

    fn check_requirement(&self, state: &[Press]) -> (Vec<Press>, Vec<Press>) {
        let mut satisfied = Vec::new();
        let mut unsatisfied = Vec::new();
        for (i, (state_press, required)) in state.iter().zip(&self.requirements).enumerate() {
            if *state_press < (*required as Press) && *state_press != (*required as Press) {
                unsatisfied.push(i as Press)
            } else {
                satisfied.push(i as Press);
            }
        }
        (satisfied, unsatisfied)
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
struct AOCMatrix<T> {
    matrix: Vec<Vec<T>>,
}
impl<
    T: std::ops::SubAssign
        + Copy
        + std::ops::AddAssign
        + std::cmp::PartialOrd<T>
        + From<i16>
        + std::ops::Div<Output = T>,
> AOCMatrix<T>
{
    fn new(matrix: Vec<Vec<T>>) -> Self {
        Self { matrix }
    }
    fn col(&self, index: usize) -> Vec<T> {
        (0..self.nrows()).map(|i| self.matrix[i][index]).collect()
    }
    fn nrows(&self) -> usize {
        self.matrix.len()
    }
    fn ncols(&self) -> usize {
        self.matrix[0].len()
    }
    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.matrix.swap(row1, row2);
    }
    fn sub_rows(&mut self, lhs: usize, rhs: usize) {
        if lhs == rhs {
            panic!()
        }
        for i in 0..self.ncols() {
            let val = self[(i, rhs)];
            self[(i, lhs)] -= val;
        }
    }
    fn add_rows(&mut self, lhs: usize, rhs: usize) {
        for i in 0..self.ncols() {
            let val = self[(i, rhs)];
            self[(i, lhs)] += val;
        }
    }

    fn scale_row(&mut self, row: usize, scalar: T)
    where
        T: MulAssign<T>,
    {
        for v in self.matrix[row].iter_mut() {
            *v *= scalar;
        }
    }
    fn div_row(&mut self, row: usize, scalar: T) {
        for v in self.matrix[row].iter_mut() {
            *v = *v / scalar;
        }
    }
}
impl<
    T: fmt::Display
        + std::ops::AddAssign
        + std::ops::Div<Output = T>
        + std::ops::SubAssign
        + std::marker::Copy
        + std::cmp::PartialOrd
        + From<i16>,
> fmt::Display for AOCMatrix<T>
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, row) in self.matrix.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                let str = format!("{}", value);
                if i == j {
                    write!(f, "{}\t", str.red())?;
                } else if j == self.ncols() - 1 {
                    write!(f, "{}\t", str.green())?;
                } else {
                    write!(f, "{}\t", str)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl<T> Index<(usize, usize)> for AOCMatrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.matrix[col][row]
    }
}
impl<T> IndexMut<(usize, usize)> for AOCMatrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.matrix[col][row]
    }
}

fn solve_by_algebra(machine: &Machine) -> Option<u32> {
    const DEBUG: bool = true;
    let width = machine.wiring_diagrams.len();
    let height = machine.joltage_requirement.requirements.len();
    if width > height {
        eprintln!("Cannot Complete Yet");
        return None;
    }

    let mut matrix: Vec<Vec<i16>> = Vec::new();

    let diagonal_end = machine.joltage_requirement.requirements.len().max(height);
    for i in 0..diagonal_end {
        let mut row = Vec::new();
        for button in machine.wiring_diagrams.iter() {
            if button.positions.contains(&i) {
                row.push(1)
            } else {
                row.push(0)
            }
        }
        row.push(machine.joltage_requirement.requirements[i] as i16);
        matrix.push(row);
    }
    let matrix = AOCMatrix::new(matrix);

    matrix_solve(matrix, true)
}
fn matrix_solve(mut matrix: AOCMatrix<i16>, debug: bool) -> Option<u32> {
    let width = matrix.ncols() - 1;
    let height = matrix.nrows();

    // solve area under identity
    for i in 0..matrix.ncols() - 1 {
        if debug {
            eprintln!("i: {i}, {}", matrix[(i, i)]);
        }

        if matrix[(i, i)].abs() != 1 {
            for j in i..matrix.nrows() {
                if matrix[(i, j)].abs() == 1 {
                    matrix.swap_rows(i, j);
                    if debug {
                        eprintln!("swap: {j} > {i}\n{matrix}");
                    }
                    break;
                } else {
                    let min = &matrix.matrix[j]
                        .iter()
                        .filter(|a| **a != 0)
                        .min_by(|a, b| a.abs().cmp(&b.abs()))
                        .unwrap_or(&1)
                        .clone();
                    if matrix.matrix[j].iter().all(|a| *a % *min == 0) && *min != 1 {
                        matrix.div_row(j, *min);
                        matrix.swap_rows(i, j);
                        if debug {
                            eprintln!("divide/swap: {j},{i}, min: {min}\n{matrix}");
                        }
                    }
                }
            }
        }

        if matrix[(i, i)] < 0 {
            matrix.scale_row(i, -1);
            if debug {
                eprintln!("mul: {i}\n{}", matrix);
            }
        }

        if matrix[(i, i)] != 1 {
            return None;
        }

        for j in i..matrix.nrows() {
            if i == j {
                continue;
            }

            while matrix[(i, j)] < 0 {
                matrix.add_rows(j, i);
                if debug {
                    eprintln!("sub: {i}, {j}\n{}", matrix);
                }
            }

            while matrix[(i, j)] > 0 {
                matrix.sub_rows(j, i);
                if debug {
                    eprintln!("sub: {i}, {j}\n{}", matrix);
                }
            }
        }
    }

    // solve area above identity
    for i in (0..matrix.ncols() - 1).rev() {
        for j in 0..i {
            while matrix[(i, j)] > 0 {
                matrix.sub_rows(j, i);
                if debug {
                    eprintln!("sub: {i},{j}\n{}", matrix);
                }
            }
            while matrix[(i, j)] < 0 {
                matrix.add_rows(j, i);
                if debug {
                    eprintln!("add: {i},{j}\n{}", matrix);
                }
            }
        }
    }
    if debug {
        eprintln!("result\n{}", matrix);
        eprintln!("{:?}", matrix.col(matrix.ncols() - 1));
    }

    Some(matrix.col(matrix.ncols() - 1).iter().sum::<i16>() as u32)
}

fn nalgebra_sove(machine: &Machine) -> u32 {
    let width = machine.wiring_diagrams.len();
    let height = machine.joltage_requirement.requirements.len();

    let mut matrix: Vec<f32> = Vec::new();
    for button in machine.wiring_diagrams.iter() {
        for i in 0..height {
            if button.positions.contains(&i) {
                matrix.push(1f32)
            } else {
                matrix.push(0f32)
            }
        }
    }
    let joltage = DMatrix::from_vec(
        height,
        1,
        machine
            .joltage_requirement
            .requirements
            .iter()
            .map(|joltage| *joltage as f32)
            .collect(),
    );

    let matrix = DMatrix::from_vec(height, width, matrix);

    let solution = if width < height {
        solve_over_prevision(matrix, joltage, height, width)
    } else if width > height {
        solve_under_prevision(matrix, joltage, height, width)
    } else {
        solve_matrix(matrix, joltage)
    };

    if let Some(solution) = solution {
        return solution.iter().sum::<f32>() as u32;
    };

    eprintln!("Failed to solve");
    0
}

fn solve_over_prevision(
    matrix: OMatrix<f32, Dyn, Dyn>,
    joltage: OMatrix<f32, Dyn, Dyn>,
    height: usize,
    width: usize,
) -> Option<OMatrix<f32, Dyn, Dyn>> {
    let remove_amount = height - width;

    let remove_list: Vec<Vec<usize>> = (0..height).combinations(remove_amount).collect();
    for mut remove in remove_list {
        let mut matrix = matrix.clone();
        let mut joltage = joltage.clone();

        remove.sort();

        for i in remove.iter().rev() {
            matrix = matrix.remove_row(*i);
            joltage = joltage.remove_row(*i);
        }

        if let Some(solution) = solve_matrix(matrix, joltage) {
            return Some(solution);
        };
    }
    None
}

fn solve_under_prevision(
    matrix: OMatrix<f32, Dyn, Dyn>,
    joltage: OMatrix<f32, Dyn, Dyn>,
    height: usize,
    width: usize,
) -> Option<OMatrix<f32, Dyn, Dyn>> {
    let remove_amount = width - height;

    let remove_list: Vec<Vec<usize>> = (0..height).combinations(remove_amount).collect();

    for mut remove in remove_list {
        let mut matrix = matrix.clone();
        let joltage = joltage.clone();

        remove.sort();

        let mut removed_buttons = Vec::new();
        for i in remove.iter().rev() {
            let removed = DMatrix::from_columns(&[matrix.column(*i)]);
            removed_buttons.push(removed);
            matrix = matrix.remove_column(*i);
        }
        if let Some((mut solution, count)) = guess_button(matrix, joltage, &removed_buttons, 0) {
            solution = solution.insert_row(0, count as f32);
            return Some(solution);
        }
    }
    None
}
fn guess_button(
    matrix: OMatrix<f32, Dyn, Dyn>,
    joltage: OMatrix<f32, Dyn, Dyn>,
    guesses: &[OMatrix<f32, Dyn, Dyn>],
    mut presses: usize,
) -> Option<(OMatrix<f32, Dyn, Dyn>, usize)> {
    if guesses.is_empty() {
        return None;
    }

    for (i, guess) in guesses.iter().enumerate() {
        let mut current_guesses = guesses.to_vec();
        let mut new_joltage = joltage.clone();

        if !new_joltage.iter().any(|s| *s < 0f32) {
            if let Some(solution) = solve_matrix(matrix.clone(), new_joltage.clone()) {
                return Some((solution, presses));
            }
        } else {
            new_joltage += guess;
            current_guesses.remove(i);
        }
        new_joltage -= guess;
        presses += 1;

        if let Some(solution) = guess_button(
            matrix.clone(),
            new_joltage.clone(),
            &current_guesses,
            presses,
        ) {
            return Some(solution);
        };
    }
    None
}

fn solve_matrix(
    matrix: OMatrix<f32, Dyn, Dyn>,
    joltage: OMatrix<f32, Dyn, Dyn>,
) -> Option<OMatrix<f32, Dyn, Dyn>> {
    let solution =
        if let Some(solution) = validate_solution(matrix.clone().full_piv_lu().solve(&joltage)) {
            solution
        } else if let Some(solution) = validate_solution(matrix.clone().lu().solve(&joltage)) {
            solution
        } else if let Some(solution) = validate_solution(matrix.clone().qr().solve(&joltage)) {
            solution
        } else {
            return None;
        };
    Some(solution)
}
fn validate_solution(solution: Option<OMatrix<f32, Dyn, Dyn>>) -> Option<OMatrix<f32, Dyn, Dyn>> {
    let solution = solution?;

    if solution.iter().any(|s| *s < 0f32 || s.rem(1f32) != 0f32) {
        None
    } else {
        Some(solution)
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
        let mut completed = 0;

        for (i, machine) in machines.machines[0..1].iter().enumerate() {
            let machine_result = solve_by_algebra(machine);

            eprintln!(
                "{}/{}: {machine_result:?}\n\n\n",
                i,
                machines.machines.len() - 1
            );

            if let Some(count) = machine_result {
                completed += 1;
                part_2_answer += count as u64;
            }
        }
        eprintln!("Completed: {completed}/{}", machines.machines.len());
        part_2_answer
    }
}
fn main() -> std::io::Result<()> {
    DAY.run(InputType::Example)
}
