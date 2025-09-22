use library::grid::Direction;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

type Vst = HashMap<(Direction, usize), usize>;

pub struct VisitStates {
    states: Vst,
    stopped: usize,
}
impl VisitStates {
    pub fn set_weight(&mut self, run: usize, weight: usize, direction: Direction) -> bool {
        if direction == Direction::None {
            return if weight > self.stopped {
                println!("False: {weight} > {}, ", self.stopped);
                false
            } else {
                // println!("{weight}");
                self.stopped = weight;
                true
            };
        }

        let visit_find = self.states.get(&(direction, run));

        if let Some(visit) = visit_find {
            // if new weight is lower than current weight
            if weight > *visit {
                return false;
            }
        }
        self.states.insert((direction, run), weight);

        true
    }

    pub fn get_stopped(&self) -> usize {
        self.stopped
    }
    pub fn get_weight(&self, direction: Direction, run: usize) -> usize {
        if direction == Direction::None {
            return self.stopped;
        }

        let visited = self.states.get(&(direction, run));

        match visited {
            Some(visited) => *visited,
            None => usize::MAX,
        }
    }
    pub fn _iter(&self) -> Iter<'_, (Direction, usize), usize> {
        self.states.iter()
    }

    pub fn new() -> Self {
        Self {
            states: Vst::new(),
            stopped: usize::MAX,
        }
    }
}
