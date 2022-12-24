use regex::Regex;
use std::{collections::{HashMap}};

fn main() {
    let pipe_system = parse_pipe_system(include_str!("../test.txt"));
    println!("Part 1: {}", part_1(pipe_system));
    println!("Part 2: {}", 0);
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Valve {
    label: String,
    capacity: usize,
    connected_valves: Vec<String>,
}

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq, Hash)]
enum Action {
    Move { from_valve_label:String, to_valve_label: String },
    Open { valve_label: String, capacity: usize },
}

impl Action {
    fn start_valve(&self) -> &String {
        match self {
            Action::Move { from_valve_label, to_valve_label: _ } => from_valve_label,
            Action::Open { valve_label, capacity: _ } => valve_label
        }
    }

    fn is_move(&self) -> bool {
        if let Action::Move{from_valve_label: _, to_valve_label: _} = self {
            true
        } else {
            false
        }
    }
}

fn parse_pipe_system(input: &str) -> HashMap<String, Valve> {
    let re =
        Regex::new(r"^Valve ([A-Z]+) has flow rate=(\d*); tunnel[s]? lead[s]? to valve[s]? (.+)$")
            .unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let label = String::from(&caps[1]);
            let capacity = caps[2].parse::<usize>().unwrap();
            let connected_valves = caps[3].split(", ").map(|s| String::from(s)).collect();
            (
                label.clone(),
                Valve {
                    label,
                    capacity,
                    connected_valves,
                },
            )
        })
        .collect()
}

#[derive(Clone)]
struct ActionSequence {
    actions: Vec<Action>,
    open_valves: Vec<String>,
    released_pressure: usize,
    total_capacity: usize,
    current_valve: String
}

impl ActionSequence {
    fn add_action(&mut self, action: Action) {
        self.released_pressure += self.total_capacity;
        match &action {
            Action::Open{valve_label, capacity} => {
                self.current_valve = valve_label.clone();
                self.total_capacity += capacity;
                self.open_valves.push(valve_label.clone());
                // Av någon jävla anledningen så blir resulatet instabilt om man sorterar här
                // self.open_valves.sort();
            },
            Action::Move { from_valve_label: _, to_valve_label } => {
                self.current_valve = to_valve_label.clone()
            }
        }
        self.actions.push(action)
    }

    fn is_current_valve_open(&self) -> bool {
        self.open_valves.contains(&self.current_valve)
    }
}

fn keep_best(action_sequences: Vec<ActionSequence>) -> Vec<ActionSequence> {
    let mut best_by_pipe_system_state: HashMap<(String, Vec<String>), ActionSequence> = HashMap::new();
    for sequence in action_sequences {
        match best_by_pipe_system_state.get(&(sequence.current_valve.clone(), sequence.open_valves.clone())) {
            Some(best_sequence) => {
                if sequence.total_capacity > best_sequence.total_capacity && sequence.released_pressure > best_sequence.released_pressure {
                    best_by_pipe_system_state.insert((sequence.current_valve.clone(), sequence.open_valves.clone()), sequence);
                }    
            },
            None => {
                best_by_pipe_system_state.insert((sequence.current_valve.clone(), sequence.open_valves.clone()), sequence);
            }
        }
    }

    // TODO: På något sätt behöver vi plocka bort dom som har valve-openings som är ett subset av en annan med högre released_pressure och kapacitet

    best_by_pipe_system_state.into_iter().map(|(_state, sequence)| sequence).collect()
}

// fn print_preferred_action_sequence(n_steps: usize, action_sequences: &Vec<Vec<Action>>) {
//     let mut preferred_path = vec![
//         Action::Move{valve: String::from("DD")},
//         Action::Open{valve: String::from("DD"), capacity: 20},
//         Action::Move{valve: String::from("CC")},
//         Action::Move{valve: String::from("BB")},
//         Action::Open{valve: String::from("BB"), capacity: 13},
//         Action::Move{valve: String::from("AA")},
//         Action::Move{valve: String::from("II")},
//         Action::Move{valve: String::from("JJ")},
//         Action::Open{valve: String::from("JJ"), capacity: 21},
//         Action::Move{valve: String::from("II")},
//         Action::Move{valve: String::from("AA")},
//         Action::Move{valve: String::from("DD")},
//         Action::Move{valve: String::from("EE")},
//         Action::Move{valve: String::from("FF")},
//         Action::Move{valve: String::from("GG")},
//         Action::Move{valve: String::from("HH")},
//         Action::Open{valve: String::from("HH"), capacity: 22},
//         Action::Move{valve: String::from("GG")},
//         Action::Move{valve: String::from("FF")},
//         Action::Move{valve: String::from("EE")},
//         Action::Open{valve: String::from("EE"), capacity: 3},
//         Action::Move{valve: String::from("DD")},
//         Action::Move{valve: String::from("CC")},
//         Action::Open{valve: String::from("CC"), capacity: 2},       
//         Action::Move{valve: String::from("DD")},
//         Action::Move{valve: String::from("CC")},
//         Action::Move{valve: String::from("DD")},
//         Action::Move{valve: String::from("CC")},
//         Action::Move{valve: String::from("DD")},
//         Action::Move{valve: String::from("CC")},
//         Action::Move{valve: String::from("DD")},
//         Action::Move{valve: String::from("CC")},
//     ];
//     preferred_path = preferred_path.into_iter().take(n_steps).collect();
//     let preferred_valve_openings_and_rounds = valve_openings_and_rounds(&preferred_path);
//     let preferred_valve = preferred_path[preferred_path.len()-1].at_valve();
//     let sequence = action_sequences.iter().find(|seq| {
//         let valve_openings_and_rounds = valve_openings_and_rounds(seq);
//         let valve = seq[seq.len() -1].at_valve();
//         valve == preferred_valve && valve_openings_and_rounds == preferred_valve_openings_and_rounds
//     });
//     let sequence = sequence.unwrap();
//     println!("Total capacity and released pressure: {:?}", total_capacity_and_released_pressure(sequence));

// }

fn part_1(pipe_system: HashMap<String, Valve>) -> usize {
    let mut action_sequences= vec![];
    for ind in 1..=30 {
        println!("\nTime: {ind}");
        action_sequences = step_once(action_sequences, &pipe_system);
        let n_seq = action_sequences.len();
        action_sequences = keep_best(action_sequences);
        println!("Total action sequences {}, Pruned action sequences: {}", action_sequences.len(), n_seq - action_sequences.len());
        // print_preferred_action_sequence(ind, &action_sequences);
    }
    action_sequences.into_iter().map(|sequence| sequence.released_pressure).max().unwrap()
}

fn determine_next_actions(sequence: &ActionSequence, pipe_system: &HashMap<String, Valve>) -> Vec<Action> {
    let previous_action = sequence.actions.iter().last().unwrap();
    let current_valve = pipe_system.get(&sequence.current_valve).unwrap();
    let mut actions: Vec<_> = current_valve.connected_valves.iter() 
    // Av någon jävla anledning så blir det instabilt om man tar bort tillbaka-kakan
    .filter(|connected_valve| /* *connected_valve != previous_action.start_valve() || !previous_action.is_move() */ true)
    .map(|v| Action::Move{from_valve_label: sequence.current_valve.clone(), to_valve_label: String::from(v)}).collect();
    if let Action::Move{from_valve_label: _, to_valve_label: current_valve_label} = previous_action {
        if !sequence.is_current_valve_open() && current_valve.capacity > 0 {
            actions.push(Action::Open{valve_label: String::from(current_valve_label), capacity: current_valve.capacity})
        }   
    }
    actions   
}

fn step_once(
    action_sequences: Vec<ActionSequence>,
    pipe_system: &HashMap<String, Valve>) -> Vec<ActionSequence> {
    if action_sequences.len() == 0 {
        let start_valve = pipe_system.get("AA").unwrap();
        start_valve
            .connected_valves
            .iter()
            .map(|v| {
                ActionSequence {actions: vec![Action::Move { to_valve_label: String::from(v), from_valve_label: String::from("AA") }], released_pressure: 0, total_capacity: 0, current_valve: String::from(v), open_valves: vec![]}
            })
            .collect()
    } else {
        action_sequences
            .into_iter()
            .flat_map(|sequence| {
                determine_next_actions(&sequence, &pipe_system)
                    .into_iter()
                    .map(|action| {
                        let mut new_sequence = sequence.clone();
                        new_sequence.add_action(action);
                        new_sequence
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let pipe_system = parse_pipe_system(include_str!("../test.txt"));
        assert_eq!(part_1(pipe_system), 1651)
    }
}
