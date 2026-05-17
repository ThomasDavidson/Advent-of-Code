use colored::Colorize;
use library::input::{Day, InputType};
use num_traits::{One, Zero, one, zero};
use std::collections::VecDeque;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::ops::{Index, IndexMut, MulAssign};
use std::slice::Iter;

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
        + std::ops::MulAssign
        + Ord
        + std::cmp::PartialOrd<T>
        + std::ops::Div<Output = T>
        + num_traits::Signed
        + std::fmt::Display
        + TryInto<u32>,
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
    fn rotate_col(&mut self) {
        let width = self.ncols() - 1;
        self.positions.rotate_left(1);
        self.positions.swap(width - 1, width);
        for row in self.matrix.iter_mut() {
            row.rotate_left(1);
            row.swap(width - 1, width);
        }
    }
    fn min_row(&self, index: usize) -> Option<&T> {
        self.matrix[index]
            .iter()
            .filter(|a| !a.is_zero())
            .min_by(|a, b| a.abs().cmp(&b.abs()))
    }
    fn validate_solution(&self) -> bool {
        let last_column = self.ncols() - 1;
        self.matrix[0..self.solution_area()]
            .iter()
            .all(|s| !s[last_column].is_negative())
    }

    fn remove_empty_rows(&mut self) {
        // filter out empty
        let empty = self
            .matrix
            .iter()
            .enumerate()
            .filter(|(_, row)| row.iter().all(|v| v.is_zero()))
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
                if matrix.row(i).iter().all(|v| (*v % *min_row).is_zero()) {
                    matrix.div_row(i, *min_row);
                }
            }

            if !matrix[(i, i)].is_positive() {
                matrix.scale_row(i, -one::<T>());
            }
            for j in i..matrix.nrows() {
                if i == j {
                    continue;
                }

                while !(matrix[(i, j)].is_zero()
                    || matrix[(i, j)].is_positive() && matrix[(i, i)].is_positive())
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
            if self[(i, i)].is_zero() {
                continue;
            }

            for j in 0..i {
                while self[(i, i)].is_positive()
                    && self[(i, j)].is_positive()
                    && !self[(i, j)].is_zero()
                {
                    self.sub_rows(j, i);
                }
                while self[(i, i)].is_positive()
                    && self[(i, j)].is_negative()
                    && !self[(i, j)].is_zero()
                {
                    self.add_rows(j, i);
                }
            }
        }
    }
    fn is_identity(&self) -> bool {
        for y in 0..self.solution_area() {
            for x in 0..(self.solution_area() - 1) {
                if x == y && self[(x, y)].is_one() || self[(x, y)].is_zero() {
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
                if x == y && self[(x, y)].is_one() || self[(x, y)].is_zero() {
                    continue;
                }
                return Some(x);
            }
        }
        None
    }
    fn min_presses(&self, column: usize) -> T {
        self.col(column)
            .iter()
            .enumerate()
            .zip(self.col(self.ncols() - 1).iter())
            .filter(|((_, col), last)| !last.is_zero() && !col.is_zero())
            .filter(|((_, col), last)| (**last % **col).is_zero())
            .filter(|((y, _), last)| {
                self.row(*y)
                    .iter()
                    .filter(|col| col.is_positive() && last.is_positive())
                    .count()
                    == 1
            })
            .map(|((_, col), last)| *last / *col)
            .min()
            .unwrap_or(zero())
    }
    fn solve_row(&self, row_idx: usize) -> Option<(usize, i16, i16)>
    where
        i16: From<T>,
    {
        let row = self.row(row_idx);

        if row.iter().all(|v| v.is_zero()) {
            return None;
        }

        if row[0..(row.len() - 1)]
            .iter()
            .filter(|v| !v.is_zero())
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
            .filter(|v| v.is_zero())
            .count()
            != row.len() - 2
        {
            return None;
        }
        let button_value: i16 = row[row.len() - 1].into();

        // set first answer
        let button_pos = row.iter().position(|v| !v.is_zero()).unwrap();
        let button_denominator = row[button_pos].into();
        let relative_button_pos = self.positions[button_pos];

        Some((relative_button_pos, button_value, button_denominator))
    }
    fn guess_col(&mut self, button_count: &mut ButtonCount, column: usize, guess: T)
    where
        <T as TryInto<u32>>::Error: Debug,
    {
        let button = self.positions[column];

        button_count[button] += guess.try_into().unwrap();

        self.scale_col(column, guess);
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
    const DEBUG: bool = false;
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

    for _ in 0..width {
        matrix.rotate_col();
        if let Some(ButtonPressResult::Equal(button_count)) =
            matrix_solve(matrix.clone(), button_count.clone(), machine, DEBUG)
        {
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

    if matrix.ncols() - 1 > matrix.nrows() {
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

        let mut min_press: Option<u32> = None;
        let mut min_button_count = button_count.clone();

        for i in min_presses..=machine.max_presses(button) {
            let mut matrix = matrix.clone();
            let mut button_count = button_count.clone();

            matrix.guess_col(&mut button_count, column, i as i16);
            if debug {
                eprintln!("Guess B: {button}, G: {i}\n{button_count}\n{matrix}");
            }

            let result = matrix_solve(matrix, button_count.clone(), machine, debug);

            if let Some(ButtonPressResult::Equal(button_count)) = result {
                let answer = button_count.iter().sum::<u32>();
                if debug {
                    eprintln!("New answer: {answer}");
                }

                if answer < min_press.unwrap_or(u32::MAX) {
                    min_press = Some(answer);
                    min_button_count.set(button_count);
                }
            } else if debug {
                eprintln!("PressResult: {result:?}");
            }
        }
        return if min_press.is_some() {
            Some(ButtonPressResult::Equal(min_button_count.get_count()))
        } else {
            None
        };
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
        ButtonPressResult::Over => return None,
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

        let min_presses = matrix.min_presses(unsolved_row) as u32;

        for i in min_presses..machine.max_presses(button) {
            let mut matrix = matrix.clone();
            let mut button_count = button_count.clone();

            matrix.guess_col(&mut button_count, unsolved_row, i as i16);

            if debug {
                eprintln!("Guess B: {button}, G: {i}\n{button_count}\n{matrix}");
            }

            if let ButtonPressResult::Over = machine.test_button_presses(&button_count.get_count())
            {
                break;
            }

            match matrix_solve(matrix, button_count, machine, debug) {
                Some(ButtonPressResult::Equal(button_count)) => {
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
        eprintln!("button Count: {:?}", button_count);
    }

    Some(machine.test_button_presses(&button_count.get_count()))
}

struct Day10;
const DAY: Day10 = Day10;
impl Day<u64> for Day10 {
    fn part_1(&self, input: &str) -> u64 {
        let machines = Machines::parse(input);

        let mut part_1_answer = 0;

        for machine in machines {
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

        for machine in machines {
            let machine_result = solve_by_algebra(&machine);

            if let Some(count) = machine_result {
                part_2_answer += count as u64;
            }
        }
        part_2_answer
    }
}
fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
