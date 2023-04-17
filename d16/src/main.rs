use regex::{Regex};
use std::{collections::{HashMap, BTreeSet, BTreeMap}};
use itertools::{Itertools};
fn main() {
    let pipe_system = parse_pipe_system(include_str!("../test.txt"));
    let pipe_system = pre_process(pipe_system);
    // println!("Part 1: {}", explore(pipe_system.clone(), 1, 30));
    println!("Part 2: {}", explore(pipe_system, 2, 26));
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

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Explorer {
    round: u16,
    current_valve: String
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct ExplorationState2 {
    explorers: Vec<Explorer>,
    n_rounds: u16,
    open_valves: BTreeMap<String, (u16, u16)>, // Label -> (capacity, round_opened)
}

impl ExplorationState2 {
    
    fn new(n_explorers: u16, n_rounds: u16) -> Self {
        let explorer = Explorer{round: 0, current_valve: String::from("AA")};
        let mut explorers: Vec<Explorer> = vec![];
        explorers.resize(n_explorers as usize, explorer);
        Self{explorers, n_rounds, open_valves: BTreeMap::new()}
    }

    fn released_pressure(&self, round: u16) -> u16 {
        self.open_valves.iter().fold(0, |acc, (_label, (capacity, valve_round))| if *valve_round < round  { acc + capacity * (round - valve_round)} else { acc })
    }

    fn take_valve_path(mut self, valve_path: &ValvePath) -> Self {
        self.explorers.iter_mut()
            .for_each(|explorer|
                if explorer.current_valve == valve_path.start_valve {
                    explorer.current_valve = valve_path.end_valve.clone();
                    explorer.round += valve_path.cost + 1;
                    self.open_valves.insert(valve_path.end_valve.clone(), (valve_path.capacity, explorer.round));
                }
            );
        self
    }
}

fn explore(pipe_system: PipeSystem, n_explorers: u16, n_rounds: u16) -> u16 {
    let mut active_states = vec![ExplorationState2::new(n_explorers, n_rounds)];
    let mut max_released_pressure = u16::MIN;
    // Mjäää. we might want to do adepth first where we are greedy to find a reaonable good "limit"
    // When we have done that we can drop all states that are bound to be worse
    while !active_states.is_empty() {
        println!("\nActive states: {:?}", active_states.len());
        println!("Max released pressure: {:?}", max_released_pressure);
        let max_rounds = active_states.iter().map(|state| state.explorers.iter().map(|x| x.round).max().unwrap()).max().unwrap();
        println!("Max rounds: {:?}", max_rounds);
        let mut next_states: Vec<ExplorationState2> = vec![];
        for state in active_states {
            let closed_valves: BTreeSet<_> = pipe_system.valves_to_open.iter().filter(|label| !state.open_valves.contains_key(*label)).collect();
            if closed_valves.is_empty() {
                max_released_pressure = max_released_pressure.max(state.released_pressure(n_rounds))
            } else {
                let active_explorers: Vec<_> = state.explorers.iter().filter(|explorer| explorer.round < n_rounds - 1).collect();
                //let active_explorers = &state.explorers;    
                if active_explorers.len() == 0 {
                    max_released_pressure = max_released_pressure.max(state.released_pressure(n_rounds))
                // } else if closed_valves.len() == 1 {
                //     for exp in active_explorers {
                //         let path_to_take = pipe_system.valve_paths.get(&exp.current_valve).unwrap().iter().find(|vp| vp.end_valve == **closed_valves.iter().next().unwrap()).unwrap();
                //         next_states.push(state.clone().take_valve_path(path_to_take))
                //     }
                } else {
                    for selected_valve in closed_valves {
                        for exp in &active_explorers {
                            let path_to_take = pipe_system.valve_paths.get(&exp.current_valve).unwrap().iter().find(|vp| vp.end_valve == *selected_valve).unwrap();
                            next_states.push(state.clone().take_valve_path(path_to_take));
                        }
                    }
                    // for selected_valves in closed_valves.iter().permutations(active_explorers.len()) {
                    //     // println!("Selected valves\n{:?}", selected_valves);
                    //     let mut next_state = state.clone();
                    //     for (exp, selected_valve) in active_explorers.iter().zip(selected_valves.iter()) {
                    //         let path_to_take = pipe_system.valve_paths.get(&exp.current_valve).unwrap().iter().find(|vp| vp.end_valve == ***selected_valve).unwrap();
                    //         next_state = next_state.take_valve_path(path_to_take);
                    //     }
                    //     next_states.push(next_state)
                    // }
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

#[derive(Clone,Debug, Ord, PartialOrd, Eq, PartialEq)]
struct ValvePath {
    start_valve: String,
    end_valve: String,
    capacity: u16,
    cost: u16    
}

#[derive(Debug, Clone)]
struct PipeSystem {
    valve_paths: BTreeMap<String, Vec<ValvePath>>,
    valves_to_open: BTreeSet<String>
}

impl PipeSystem {
    fn new(valve_paths: BTreeMap<String, Vec<ValvePath>>, valves_to_open: BTreeSet<String>) -> Self {
        PipeSystem {
            valve_paths,
            valves_to_open: valves_to_open
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
                            let capacity = pipe_system.get(connected_valve).unwrap().capacity;
                            done_valve_paths.push(ValvePath { start_valve: search_path.start_valve.clone(), end_valve: connected_valve.clone(), capacity, cost: search_path.cost + 1 })
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

    let valves_to_open = end_valves.into_iter().collect();
    PipeSystem::new( valve_paths, valves_to_open)

}
