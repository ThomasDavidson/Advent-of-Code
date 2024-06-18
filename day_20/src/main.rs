use std::collections::{HashMap, VecDeque};
use std::ops::{Not};
use crate::ModuleType::{Broadcast, Conjunction, FlipFlop};
use crate::SignalLevel::{High, Low};

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
    fn handle_pulse(&mut self, label: &String, signal_level: &SignalLevel) -> Option<SignalLevel> {
        match self {
            FlipFlop(state) =>
                match signal_level {
                    High => None,
                    Low => {
                        *state = !*state;
                        Some(*state)
                    }
                }
            ,
            Conjunction(inputs) => {
                // println!("Conjunction: {:?}", inputs);
                let input_pos = inputs.partition_point(|input| input.0.as_str() == label);
                let Some(input) = inputs.get_mut(input_pos)else {
                    panic!("Conjunction called from not connected module");
                };
                input.1 = *signal_level;

                // println!("Conjunction: {:?}", inputs);

                match inputs.iter().all(|input| input.1 == Low) {
                    true => Some(High),
                    false => Some(Low),
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

        let destinations = destinations_str.split(", ").map(|dest| dest.to_string()).collect();

        Self {
            label: label.to_string(),
            destinations,
            module_type,
        }
    }
}

fn main() {
    let input = include_str!("../example.txt");


    let mut modules: HashMap<String, Module> = HashMap::new();

    for line in input.lines() {
        let module = Module::from_string(line);
        let label = module.label.clone();
        modules.insert(label, module);
        // println!("{} -{:?}> {:?}", module.label, module.module_type, module.destinations);
    }

    let mut machine: Vec<_> = input.lines().map(|line| Module::from_string(line)).collect();

    let machine_clone = machine.clone();

    // allow conjunction modules to track each input
    for module in machine.iter_mut() {
        let module_type = match &mut module.module_type {
            Conjunction(t) => t,
            _ => continue,
        };
        for p_module in machine_clone.iter().filter(|f_module| f_module.destinations.contains(&module.label)) {
            module_type.push((p_module.label.clone(), Low));
        }
    }

    for module in &machine {
        println!("{:?}", module);
    }

    println!();

    let mut signals = VecDeque::from([("broadcaster".to_string(), Low)]);

    while let Some((module_label, signal)) = signals.pop_front() {
        let Some(module_index) = machine.iter().position(|module| module.label == module_label) else {
            panic!("Invalid label");
        };
        // println!("{}, index: {}", module_label, module_index);
        let Some(module) = machine.get_mut(module_index) else {
            panic!("Invalid label");
        };

        let Some(new_signal) = module.module_type.handle_pulse(&module_label, &signal) else {
            // println!("Skip {}", module_label);
            continue;
        };

        // println!("{:?}", module);

        for destination in &module.destinations {
            println!("{} -{:?}> {}", module_label, new_signal, destination);
            signals.push_back((destination.to_string(), new_signal));
        }
    }
}
