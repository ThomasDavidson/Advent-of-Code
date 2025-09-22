use crate::ModuleType::{Broadcast, Conjunction, FlipFlop};
use crate::SignalLevel::{High, Low};
use library::input::{Day, InputType};
use library::math::lcm;
use std::collections::{HashMap, VecDeque};
use std::ops::Not;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
enum SignalLevel {
    Low,
    High,
}

impl Not for SignalLevel {
    type Output = Self;
    fn not(self) -> Self::Output {
        match self {
            Low => High,
            High => Low,
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    FlipFlop(SignalLevel),
    Conjunction(Vec<(String, SignalLevel)>),
    Broadcast,
}

impl ModuleType {
    fn handle_pulse(&mut self, label: &str, signal_level: &SignalLevel) -> Option<SignalLevel> {
        match self {
            FlipFlop(state) => match signal_level {
                High => None,
                Low => {
                    *state = !*state;
                    Some(*state)
                }
            },
            Conjunction(inputs) => {
                let Some(input_pos) = inputs.iter().position(|input| input.0.as_str() == label)
                else {
                    panic!("Not found: {}", label);
                };

                let Some(input) = inputs.get_mut(input_pos) else {
                    panic!("Conjunction called from not connected module");
                };
                input.1 = *signal_level;

                match inputs.iter().all(|input| input.1 == High) {
                    true => Some(Low),
                    false => Some(High),
                }
            }
            Broadcast => Some(*signal_level),
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    label: String,
    destinations: Vec<String>,
    module_type: ModuleType,
}

impl Module {
    fn from_string(text: &str) -> Self {
        let Some((first, destinations_str)) = text.split_once(" -> ") else {
            panic!("Cannot split text");
        };

        let (module_type_str, label) = if first == "broadcaster" {
            ("broadcaster", "broadcaster")
        } else {
            first.split_at(1)
        };

        let module_type = match module_type_str {
            "broadcaster" => Broadcast,
            "%" => FlipFlop(Low),
            "&" => Conjunction(Vec::new()),
            t => panic!("Unknown Type '{}'", t),
        };

        let destinations = destinations_str
            .split(", ")
            .map(|dest| dest.to_string())
            .collect();

        Self {
            label: label.to_string(),
            destinations,
            module_type,
        }
    }
}

enum EndCondition {
    ButtonPresses(u64),
    ModuleReceiveSignal(String, SignalLevel),
}

#[derive(Clone)]
struct Machine {
    modules: HashMap<String, Module>,
}

impl Machine {
    fn from_string(input: &str) -> Self {
        let mut machine: Vec<_> = input.lines().map(Module::from_string).collect();

        let machine_clone = machine.clone();

        // allow conjunction modules to track each input
        for module in machine.iter_mut() {
            let module_type = match &mut module.module_type {
                Conjunction(t) => t,
                _ => continue,
            };
            for p_module in machine_clone
                .iter()
                .filter(|f_module| f_module.destinations.contains(&module.label))
            {
                module_type.push((p_module.label.clone(), Low));
            }
        }

        let mut modules: HashMap<String, Module> = HashMap::new();

        for module in machine.into_iter() {
            modules.insert(module.label.clone(), module);
        }

        Machine { modules }
    }
    fn press_button(&mut self, condition: EndCondition) -> (u64, u64) {
        let mut signals: VecDeque<(String, String, SignalLevel)> = VecDeque::new();

        let mut low_pulses: u64 = 0;
        let mut high_pulses: u64 = 0;

        let mut button_presses: u64 = 0;
        loop {
            button_presses += 1;
            if let EndCondition::ButtonPresses(max_presses) = condition {
                if button_presses > max_presses {
                    break;
                }
            }

            signals.push_front(("button".to_string(), "broadcaster".to_string(), Low));

            while let Some((source, module_label, signal)) = signals.pop_front() {
                match signal {
                    Low => low_pulses += 1,
                    High => high_pulses += 1,
                }

                let Some(module) = self.modules.get_mut(&module_label) else {
                    continue;
                };

                let Some(new_signal) = module.module_type.handle_pulse(&source, &signal) else {
                    continue;
                };

                if let EndCondition::ModuleReceiveSignal(ref label, cond_signal) = condition {
                    if module_label == label.as_str() && new_signal == cond_signal {
                        return (button_presses, 0);
                    }
                }

                for destination in &module.destinations {
                    signals.push_back((
                        module_label.to_string(),
                        destination.to_string(),
                        new_signal,
                    ));
                }
            }
        }

        (low_pulses, high_pulses)
    }
}

fn find_conjecture(machine: &Machine, module_label: &str) -> Option<String> {
    let Some(module) = machine.modules.get(module_label) else {
        panic!("Cannot find module");
    };

    if let Conjunction(_) = module.module_type {
        return Some(module_label.to_string());
    }

    for destination in module.destinations.iter() {
        match find_conjecture(machine, destination) {
            None => (),
            Some(label) => return Some(label),
        }
    }

    None
}

struct Day20;
const DAY: Day20 = Day20;
impl Day<u64> for Day20 {
    fn part_1(&self, input: &str) -> u64 {
        let mut machine = Machine::from_string(input);

        let (low_pulses, high_pulses) = machine.press_button(EndCondition::ButtonPresses(1000));

        high_pulses * low_pulses
    }
    fn part_2(&self, input: &str) -> u64 {
        let machine = Machine::from_string(input);

        let Some(broadcaster) = machine.modules.get("broadcaster") else {
            panic!("Can't find broadcaster")
        };

        let mut partial_answers = broadcaster
            .destinations
            .iter()
            .filter_map(|destination| find_conjecture(&machine, destination))
            .map(|conj_label| {
                machine
                    .clone()
                    .press_button(EndCondition::ModuleReceiveSignal(conj_label, Low))
                    .0
            });

        let initial = partial_answers.next().unwrap();

        partial_answers.fold(initial, |part_2_answer, partial_answer| {
            lcm(part_2_answer, partial_answer)
        })
    }
}

fn main() -> std::io::Result<()> {
    DAY.run(InputType::UserInput)
}
