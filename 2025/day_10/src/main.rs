use colored::Colorize;
use itertools::Itertools;
use library::input::{Day, InputType};
use nalgebra::{DMatrix, Dyn, OMatrix};
use num_traits::{One, Zero};
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
impl Index<usize> for Machines {
    type Output = Machine;

    fn index(&self, index: usize) -> &Self::Output {
        &self.machines[index]
    }
}
impl IntoIterator for Machines {
    type Item = Machine;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.machines.into_iter()
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
    fn test_button_presses(&self, button_presses: &[u32]) -> ButtonPressResult {
        let mut state = vec![0; self.joltage_requirement.requirements.len()];

        for (i, press) in button_presses.iter().enumerate() {
            let button = self.wiring_diagrams.get(i);
            // eprintln!("{button}");
            for pos in &button.positions {
                state[*pos] += press
            }
        }

        let joltage_req = &self.joltage_requirement.requirements;
        for (state, joltage) in state.iter().zip(joltage_req.iter()) {
            if *state < *joltage as u32 {
                return ButtonPressResult::Under;
            } else if *state > *joltage as u32 {
                return ButtonPressResult::Over;
            }
        }
        ButtonPressResult::Equal(button_presses.to_vec())
    }
    fn max_presses(&self, button: usize) -> u32 {
        let button = self.wiring_diagrams.get(button);

        let joltage_req = &self.joltage_requirement.requirements;

        button
            .positions
            .iter()
            .map(|pos| joltage_req[*pos] as u32)
            .min()
            .unwrap()
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
#[derive(Debug)]
enum ButtonPressResult {
    Equal(Vec<u32>),
    Over,
    Under,
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

#[derive(Debug, Clone)]
struct ButtonCount {
    button_count: Vec<(u32, bool)>,
}
impl ButtonCount {
    fn new(size: usize) -> Self {
        Self {
            button_count: vec![(0, false); size],
        }
    }
    fn get_count(&self) -> Vec<u32> {
        self.button_count.iter().map(|(count, _)| *count).collect()
    }
    fn len(&self) -> usize {
        self.button_count.len()
    }
    fn set(&mut self, new_values: Vec<u32>) {
        for (set, new_value) in self.button_count.iter_mut().zip(new_values.iter()) {
            *set = (*new_value, true)
        }
    }
}
impl Index<usize> for ButtonCount {
    type Output = u32;

    fn index(&self, index: usize) -> &u32 {
        let (count, _) = &self.button_count[index];
        count
    }
}
impl IndexMut<usize> for ButtonCount {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let (count, set) = &mut self.button_count[index];
        *set = true;
        count
    }
}
impl fmt::Display for ButtonCount {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (count, set) in &self.button_count {
            if *set {
                write!(f, "{count:?},\t")?;
            } else {
                write!(f, "None,\t")?;
            }
        }
        Ok(())
    }
}

#[derive(Clone)]
struct AOCMatrix<T> {
    positions: Vec<usize>,
    matrix: Vec<Vec<T>>,
}
impl<
    T: std::ops::SubAssign
        + Copy
        + std::ops::AddAssign
        + std::cmp::PartialOrd<T>
        + From<i16>
        + std::ops::Div<Output = T>
        + num_traits::Signed
        + Ord
        + std::fmt::Display
        + std::fmt::Debug
        + std::ops::MulAssign,
> AOCMatrix<T>
{
    fn col(&self, index: usize) -> Vec<T> {
        (0..self.nrows()).map(|i| self.matrix[i][index]).collect()
    }
    fn row(&self, index: usize) -> Vec<T> {
        self.matrix[index].clone()
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
    fn sub_cols(&mut self, lhs: usize, rhs: usize) {
        for i in 0..self.nrows() {
            let val = self[(rhs, i)];
            self[(lhs, i)] -= val;
        }
    }
    fn add_cols(&mut self, lhs: usize, rhs: usize) {
        for i in 0..self.nrows() {
            let val = self[(rhs, i)];
            self[(lhs, i)] += val;
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
    fn scale_col(&mut self, index: usize, scalar: T)
    where
        T: MulAssign<T>,
    {
        for i in 0..self.nrows() {
            self[(index, i)] *= scalar;
        }
    }
    fn div_row(&mut self, row: usize, scalar: T) {
        for v in self.matrix[row].iter_mut() {
            *v = *v / scalar;
        }
    }
    fn rotate_row(&mut self) {
        self.matrix.rotate_left(1);
    }
    fn min_row(&self, index: usize) -> Option<&T> {
        self.matrix[index]
            .iter()
            .filter(|a| **a != 0.into())
            .min_by(|a, b| a.abs().cmp(&b.abs()))
    }
    fn validate_solution(&self) -> bool {
        let last_column = self.ncols() - 1;
        self.matrix[0..self.solution_area()]
            .iter()
            .all(|s| s[last_column] >= 0.into())
    }

    fn remove_empty_rows(&mut self) {
        // filter out empty
        let empty = self
            .matrix
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|v| *v == 0.into()))
            .map(|(i, _)| i)
            .collect::<Vec<_>>();
        for i in empty.into_iter().rev() {
            self.remove_row(i);
        }
    }
    fn l(&mut self) -> bool {
        let matrix = self;
        let solve_area = matrix.solution_area();

        for i in 0..solve_area {
            if matrix[(i, i)].is_zero() {
                for j in i..matrix.nrows() {
                    if matrix[(i, j)].abs().is_one() {
                        matrix.swap_rows(i, j);
                        break;
                    }
                }
            }
            if !matrix[(i, i)].abs().is_one() {
                for j in i..matrix.ncols() - 1 {
                    if matrix[(j, i)].abs().is_one() {
                        matrix.swap_cols(i, j);
                        break;
                    }
                }
            }
            if !matrix[(i, i)].abs().is_one() {
                for j in i..matrix.nrows() {
                    if !matrix[(i, j)].abs().is_zero() {
                        matrix.swap_rows(i, j);
                        break;
                    }
                }
            }
            if !matrix[(i, i)].abs().is_one() {
                for j in i..matrix.ncols() - 1 {
                    if !matrix[(j, i)].abs().is_zero() {
                        matrix.swap_cols(i, j);
                        break;
                    }
                }
            }

            if matrix[(i, i)].is_zero() {
                continue;
            }
            if !matrix[(i, i)].is_one() {
                let Some(min_row) = matrix.min_row(i) else {
                    continue;
                };
                if matrix.row(i).iter().all(|v| *v % *min_row == 0.into()) {
                    matrix.div_row(i, *min_row);
                }
            }

            if !matrix[(i, i)].is_positive() {
                matrix.scale_row(i, (-1).into());
            }
            for j in i..matrix.nrows() {
                if i == j {
                    continue;
                }

                while !matrix[(i, j)].is_zero()
                    && !(matrix[(i, j)].is_positive() && matrix[(i, i)].is_positive())
                {
                    matrix.add_rows(j, i);
                }

                while !matrix[(i, j)].is_zero()
                    && matrix[(i, j)].is_positive()
                    && matrix[(i, i)].is_positive()
                {
                    matrix.sub_rows(j, i);
                }
            }
        }
        true
    }
    fn u(&mut self) {
        for i in (0..self.solution_area()).rev() {
            if self[(i, i)] == 0.into() {
                continue;
            }

            for j in 0..i {
                while self[(i, i)] * self[(i, j)] > 0.into() {
                    self.sub_rows(j, i);
                }
                while self[(i, i)] * self[(i, j)] < 0.into() {
                    self.add_rows(j, i);
                }
            }
        }
    }
    fn is_identity(&self) -> bool {
        for y in 0..self.solution_area() {
            for x in 0..(self.solution_area() - 1) {
                if x == y && self[(x, y)] == 1.into() {
                    continue;
                } else if self[(x, y)] == 0.into() {
                    continue;
                }
                return false;
            }
        }
        true
    }
    fn find_identity_error_column(&self) -> Option<usize> {
        for y in 0..self.solution_area() {
            for x in 0..(self.ncols() - 1) {
                if x == y && self[(x, y)] == 1.into() {
                    continue;
                } else if self[(x, y)] == 0.into() {
                    continue;
                }
                return Some(x);
            }
        }
        None
    }
    fn solve_row(&self, row_idx: usize) -> Option<(usize, i16, i16)>
    where
        i16: From<T>,
    {
        let row = self.row(row_idx);

        if row.iter().all(|v| *v == 0.into()) {
            return None;
        }

        if row[0..(row.len() - 1)]
            .iter()
            .filter(|v| **v != 0.into())
            .count()
            != 1
        {
            return None;
        }
        if row.len() < 2 {
            return None;
        }
        if row[0..(row.len() - 1)]
            .iter()
            .filter(|v| **v == 0.into())
            .count()
            != row.len() - 2
        {
            return None;
        }
        let button_value: i16 = row[row.len() - 1].into();

        // set first answer
        let button_pos = row.iter().position(|v| *v != 0.into()).unwrap();
        let button_denominator = row[button_pos].into();
        let relative_button_pos = self.positions[button_pos];

        Some((relative_button_pos, button_value, button_denominator))
    }
    fn guess_col(&mut self, button_count: &mut ButtonCount, column: usize, guess: u32) {
        let button = self.positions[column];

        button_count[button] += guess;

        self.scale_col(column, (guess as i16).into());
        self.sub_cols(self.ncols() - 1, column);
        self.remove_col(column);
    }
}
impl<T> AOCMatrix<T> {
    fn new(matrix: Vec<Vec<T>>) -> Self {
        let positions = matrix[0].iter().enumerate().map(|(i, _)| i).collect();
        Self { matrix, positions }
    }
    fn swap_rows(&mut self, row1: usize, row2: usize) {
        self.matrix.swap(row1, row2);
    }
    fn swap_cols(&mut self, col1: usize, col2: usize) {
        self.positions.swap(col1, col2);
        for row in self.matrix.iter_mut() {
            row.swap(col1, col2);
        }
    }
    fn nrows(&self) -> usize {
        self.matrix.len()
    }
    fn ncols(&self) -> usize {
        self.matrix[0].len()
    }
    fn remove_row(&mut self, index: usize) {
        self.matrix.remove(index);
    }
    fn remove_col(&mut self, index: usize) {
        self.positions.remove(index);
        for row in self.matrix.iter_mut() {
            row.remove(index);
        }
    }
    fn solution_area(&self) -> usize {
        self.nrows().min(self.ncols() - 1)
    }
}
impl<T: fmt::Display> fmt::Display for AOCMatrix<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let solve_area = self.solution_area();

        for value in &self.positions {
            let str = format!("{}", value);
            write!(f, "{}\t", str.purple())?;
        }
        writeln!(f)?;

        for (i, row) in self.matrix.iter().enumerate() {
            for (j, value) in row.iter().enumerate() {
                let str = format!("{}", value);
                if j == self.ncols() - 1 {
                    write!(f, "{}\t", str.green())?;
                } else if i == j {
                    write!(f, "{}\t", str.red())?;
                } else if solve_area > j && solve_area > i {
                    write!(f, "{}\t", str.cyan())?;
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

    let button_count = ButtonCount::new(width);

    let mut matrix = matrix.clone();

    let mut min_answer = None;

    for _ in 0..height {
        matrix.rotate_row();
        if let Some(ButtonPressResult::Equal(button_count)) = matrix_solve(
            matrix.clone(),
            button_count.clone(),
            &machine,
            &mut 0,
            DEBUG,
        ) {
            let answer = button_count.iter().sum::<u32>();
            min_answer = Some(answer.min(min_answer.unwrap_or(u32::MAX)));
        }
    }

    min_answer
}
fn matrix_solve(
    mut matrix: AOCMatrix<i16>,
    mut button_count: ButtonCount,
    machine: &Machine,
    min_answer: &mut u32,
    debug: bool,
) -> Option<ButtonPressResult> {
    if debug {
        eprintln!("Solve:\n{button_count}\n{matrix}");
    }

    if !matrix.l() {
        return None;
    }
    if debug {
        eprintln!("L:\n{matrix}");
    }
    // solve area above identity
    matrix.u();
    if debug {
        eprintln!("U:\n{matrix}");
    }

    while matrix.ncols() - 1 > matrix.nrows() {
        let max_presses: Vec<u32> = (0..matrix.ncols() - 1)
            .map(|col| machine.max_presses(matrix.positions[col]))
            .collect();

        let column = max_presses
            .iter()
            .enumerate()
            .filter(|(col, _)| {
                let column = matrix.col(*col);
                !(column.iter().filter(|v| v.is_one()).count() == 1
                    && column.iter().filter(|v| v.is_zero()).count() == matrix.nrows() - 1)
            })
            .min_by_key(|(_, val)| *val)
            .unwrap()
            .0;

        let button = matrix.positions[column];
        let min_presses = matrix.min_presses(column) as u32;

        for i in min_presses..=machine.max_presses(button) {
            let mut matrix = matrix.clone();
            let mut button_count = button_count.clone();

            matrix.guess_col(&mut button_count, column, i);
            if debug {
                eprintln!("Guess B: {button}, G: {i}\n{button_count}\n{matrix}");
            }

            let result = matrix_solve(matrix, button_count.clone(), machine, min_answer, debug);

            if let Some(ButtonPressResult::Equal(_)) = &result {
                return result;
            } else if debug {
                eprintln!("PressResult: {result:?}");
            }
        }
        return None;
    }

    let mut valid: bool = true;
    for row in 0..matrix.nrows() {
        let solve = matrix.solve_row(row);
        if debug {
            eprintln!("\tSolve: {}: {:?}", row, solve);
        }
        if let Some((relative_button_pos, button_value, button_denominator)) = solve {
            if button_value * button_denominator < 0 || button_value % button_denominator != 0 {
                return None;
            }

            button_count[relative_button_pos] = (button_value / button_denominator) as u32;
        }
    }
    if debug {
        eprintln!("Check:\n{button_count}\n{matrix}");
    }

    match machine.test_button_presses(&button_count.get_count()) {
        ButtonPressResult::Under => valid = false,
        _ => (),
    }

    if !valid {
        let unsolved_row = matrix.find_identity_error_column()?;
        let button = matrix.positions[unsolved_row];
        if debug {
            eprintln!("Unsolved row: {unsolved_row}");
        }

        let mut min_press: Option<u32> = None;
        let mut min_button_count = button_count.clone();

        let mut recurse_matrix;
        let mut recurse_button_count;

        let min_presses = matrix.min_presses(unsolved_row) as u32;

        for i in min_presses..machine.max_presses(button) {
            recurse_matrix = matrix.clone();
            recurse_button_count = button_count.clone();

            recurse_matrix.guess_col(&mut recurse_button_count, unsolved_row, i);

            if debug {
                eprintln!("Guess B: {button}, G: {i}\n{button_count}\n{matrix}");
            }

            match machine.test_button_presses(&button_count.get_count()) {
                ButtonPressResult::Over => break,
                _ => (),
            }

            match matrix_solve(
                recurse_matrix,
                recurse_button_count,
                machine,
                min_answer,
                debug,
            ) {
                Some(ButtonPressResult::Equal(button_count)) => {
                    // return Some(ButtonPressResult::Equal(button_count));
                    let answer = button_count.iter().sum::<u32>();
                    if debug {
                        eprintln!("New answer: {answer}");
                    }

                    if answer < min_press.unwrap_or(u32::MAX) {
                        min_press = Some(answer);
                        min_button_count.set(button_count);
                    }
                }
                a => {
                    if debug {
                        eprintln!("PressResult2: {a:?}")
                    }
                }
            }
        }

        if min_press.is_some() {
            for i in 0..button_count.len() {
                button_count[i] = min_button_count[i];
            }
        } else {
            return None;
        }
    }
    if debug {
        eprintln!("result\n{}", matrix);
        eprintln!("{:?}", matrix.col(matrix.ncols() - 1));
        eprintln!("button Count: {:?}", button_count);
    }

    Some(machine.test_button_presses(&button_count.get_count()))
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

        for (i, machine) in machines.machines.iter().enumerate() {
            let machine_result = solve_by_algebra(machine);

            eprintln!("{}/{}: {machine_result:?}", i, machines.machines.len() - 1);

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
    DAY.run(InputType::UserInput)
}
