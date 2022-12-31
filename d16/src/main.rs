use regex::{Regex};
use std::{collections::{HashMap}};
use itertools::Itertools;
fn main() {
    let pipe_system = parse_pipe_system(include_str!("../test.txt"));
    let pipe_system = pre_process(pipe_system);
    println!("{:#?}", explore(pipe_system, 30));
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq, Hash)]
struct Valve {
    label: String,
    capacity: u16,
    connected_valves: Vec<String>
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ExplorationState {
    released_pressure: u16,
    total_capacity: u16,
    current_valve: String,
    open_valves: Vec<String>,
    round: u16
}

impl ExplorationState {
    fn move_to_and_open(mut self, valve: &String, cost: u16, capacity: u16) -> Self {
        self.current_valve = valve.clone();
        self.released_pressure += (cost + 1) * self.total_capacity;
        self.open_valves.push(valve.clone());
        self.total_capacity += capacity;
        self.round += cost + 1;
        self
    }

    fn flow_to_round(mut self, target_round: u16) -> Self {
        self.released_pressure += (target_round - self.round) * self.total_capacity;
        self.round = target_round;
        self
    }
}

fn explore(pipe_system: PipeSystem, n_rounds: u16) -> u16 {
    let mut active_states = vec![ExplorationState{ current_valve: String::from("AA"), released_pressure: 0, total_capacity: 0, round: 0, open_valves: vec![]}];
    let mut max_released_pressure = u16::MIN;
    let n_valves_to_open = pipe_system.valve_capacity.iter().filter(|(_, cap)| **cap > 0).count();
    while !active_states.is_empty() {
        let mut next_states: Vec<ExplorationState> = vec![];
        for state in active_states {
            if state.open_valves.len() == n_valves_to_open {
                max_released_pressure = max_released_pressure.max(state.flow_to_round(n_rounds).released_pressure);
            } else {
                for vp in pipe_system.valve_paths.get(&state.current_valve).unwrap() {
                    if !state.open_valves.contains(&vp.end_valve) {
                        if vp.cost + state.round + 1 <= n_rounds {
                            next_states.push(state.clone().move_to_and_open(&vp.end_valve, vp.cost, *pipe_system.valve_capacity.get(&vp.end_valve).unwrap()))
                        } else {
                            max_released_pressure = max_released_pressure.max(state.clone().flow_to_round(n_rounds).released_pressure)
                        }    
                    }
                }
    
            }
        }
        active_states = next_states;
        active_states.sort();
        active_states.dedup();
    }
    max_released_pressure
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
            let capacity = caps[2].parse::<u16>().unwrap();
            let connected_valves: Vec<_> = caps[3].split(", ").map(|s| String::from(s)).collect();
            (
                label.clone(),
                Valve {
                    label,
                    capacity,
                    connected_valves
                },
            )
        })
        .collect()
}

#[derive(Clone, Debug)]
struct SearchPath {
    start_valve: String,
    current_valve: String,
    cost: u16
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct ValvePath {
    start_valve: String,
    end_valve: String,
    cost: u16    
}


#[derive(Debug)]
struct PipeSystem {
    valve_paths: HashMap<String, Vec<ValvePath>>,
    valve_capacity: HashMap<String, u16>
}

impl PipeSystem {
    fn new(valve_capacity: Vec<(String, u16)>, valve_paths: HashMap<String, Vec<ValvePath>>) -> Self {
        PipeSystem {
            valve_capacity: valve_capacity.into_iter().collect(),
            valve_paths
        }
    } 
}

fn pre_process(pipe_system: HashMap<String, Valve>) -> PipeSystem {
    let valves_worth_opening: Vec<_> = pipe_system.iter().filter(|(_, v)| v.capacity > 0).map(|(_, v)| String::from(&v.label)).collect();
    let end_valves = valves_worth_opening;
    let mut start_valves = vec![String::from("AA")];
    start_valves.append(&mut end_valves.clone());
    let mut all_valve_paths: Vec<ValvePath> = vec![];

    for start_valve in &start_valves {
        let mut done_valve_paths: Vec<ValvePath> = vec![];
        let mut search_paths = vec![SearchPath{start_valve: String::from(start_valve), current_valve: String::from(start_valve), cost: 0}];
        let number_of_paths_to_find = if start_valve == "AA" {
            end_valves.len()
        } else {
            end_valves.len() - 1
        };
        while done_valve_paths.len() < number_of_paths_to_find {
            let mut new_search_paths: Vec<SearchPath> = vec![];
            for search_path in search_paths {

                let current_valve = pipe_system.get(&search_path.current_valve).unwrap();
                for connected_valve in &current_valve.connected_valves {
                    if !done_valve_paths.iter().any(|dvp| dvp.end_valve == *connected_valve) {
                        if end_valves.contains(&connected_valve) && start_valve != connected_valve {
                            done_valve_paths.push(ValvePath { start_valve: search_path.start_valve.clone(), end_valve: connected_valve.clone(), cost: search_path.cost + 1 })
                        }
                        let mut new_search_path = search_path.clone();
                        new_search_path.current_valve = connected_valve.clone();
                        new_search_path.cost += 1;
                        new_search_paths.push(new_search_path)
                    }
                }
            }
            search_paths = new_search_paths;
        }
        all_valve_paths.append(&mut done_valve_paths);
    }
    all_valve_paths.sort();
    let valve_paths = all_valve_paths.into_iter().group_by(|vp| vp.start_valve.clone()).into_iter().map(|(key, group)| (key.clone(), group.collect())).collect();

    let valve_capacity = pipe_system.values().map(|ov| (ov.label.clone(), ov.capacity)).collect();
    PipeSystem::new(valve_capacity, valve_paths)

}
