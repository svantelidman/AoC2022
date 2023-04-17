use regex::Regex;
use std::collections::BTreeMap;

fn main() {
    let blueprints = parse_blueprints(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&blueprints));
    println!("Part 2: {}", 0);
}

fn part_1(blueprints: &Vec<Blueprint>) -> u16 {
    blueprints.iter().enumerate().map(|(ind, bp)| max_geodes_for_blueprint(bp) * (ind as u16 + 1)).sum::<u16>()
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
enum Resource {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

#[derive(Debug, Clone)]
struct RobotType {
    produces: Resource,
    cost: BTreeMap<Resource, u16>,
}

impl RobotType {
    fn can_build(&self, resources: &BTreeMap<Resource, u16>) -> bool {
        self.cost.iter().all(|(resource, required)| 
        if let Some(available) = resources.get(resource) {
            available >= required
        } else {
            false
        })
    }
}

#[derive(Debug)]
struct Blueprint {
    id: u16,
    robot_types: Vec<RobotType>,
}

fn determine_buildable_robot_types(blueprint: &Blueprint, resources: &BTreeMap<Resource, u16>) -> Vec<RobotType> {
    blueprint.robot_types.iter().filter(|rt| rt.can_build(resources)).map(|rt| rt.clone()).collect()
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct SearchState {
    current_production: BTreeMap<Resource, u16>,
    resources: BTreeMap<Resource, u16>
}

impl SearchState {

    fn robot_key(&self) -> (u16, u16, u16, u16) {
        (
            *self.current_production.get(&Resource::Ore).unwrap(),
            *self.current_production.get(&Resource::Clay).unwrap(),
            *self.current_production.get(&Resource::Obsidian).unwrap(),
            *self.current_production.get(&Resource::Geode).unwrap()
        )
    }

    fn has_better_robots_than(&self, other: &SearchState) -> bool {
        if self.current_production.get(&Resource::Geode).unwrap() > other.current_production.get(&Resource::Geode).unwrap() {
            return true
        } else if self.current_production.get(&Resource::Geode).unwrap() == other.current_production.get(&Resource::Geode).unwrap() &&
                 self.current_production.get(&Resource::Obsidian).unwrap() > other.current_production.get(&Resource::Obsidian).unwrap() {
            return true
        } 

        let better = self.current_production.get(&Resource::Ore).unwrap() > other.current_production.get(&Resource::Ore).unwrap() &&
        self.current_production.get(&Resource::Clay).unwrap() > other.current_production.get(&Resource::Clay).unwrap() &&
        self.current_production.get(&Resource::Obsidian).unwrap() > other.current_production.get(&Resource::Obsidian).unwrap() &&
        self.current_production.get(&Resource::Geode).unwrap() > other.current_production.get(&Resource::Geode).unwrap();
        better
    }


    // fn has_better_robots_than(&self, other: &SearchState) -> bool {
    //     let better = self.current_production.get(&Resource::Ore).unwrap() > other.current_production.get(&Resource::Ore).unwrap() &&
    //     self.current_production.get(&Resource::Clay).unwrap() > other.current_production.get(&Resource::Clay).unwrap() &&
    //     self.current_production.get(&Resource::Obsidian).unwrap() > other.current_production.get(&Resource::Obsidian).unwrap() &&
    //     self.current_production.get(&Resource::Geode).unwrap() > other.current_production.get(&Resource::Geode).unwrap();
    //     better
    // }

    fn has_any_better_robot_than(&self, other: &SearchState) -> bool {
        let better = self.current_production.get(&Resource::Ore).unwrap() > other.current_production.get(&Resource::Ore).unwrap() ||
        self.current_production.get(&Resource::Clay).unwrap() > other.current_production.get(&Resource::Clay).unwrap() ||
        self.current_production.get(&Resource::Obsidian).unwrap() > other.current_production.get(&Resource::Obsidian).unwrap() ||
        self.current_production.get(&Resource::Geode).unwrap() > other.current_production.get(&Resource::Geode).unwrap();
        better
    }


    fn has_better_robots_than_all(&self, other_states: &Vec<SearchState>) -> bool {
        other_states.iter().all(|other_state| self.has_better_robots_than(other_state))
    }

    fn has_worse_robots_than_all(&self, other_states: &Vec<SearchState>) -> bool {
        other_states.iter().all(|other_state| other_state.has_better_robots_than(self))
    }

    fn resource_key(&self) -> (u16, u16, u16, u16) {
        (
            *self.resources.get(&Resource::Ore).unwrap(),
            *self.resources.get(&Resource::Clay).unwrap(),
            *self.resources.get(&Resource::Obsidian).unwrap(),
            *self.resources.get(&Resource::Geode).unwrap()
        )
    }

    fn has_better_resources_than(&self, other: &SearchState) -> bool {
        let better = self.resources.get(&Resource::Ore).unwrap() >= other.resources.get(&Resource::Ore).unwrap() &&
        self.resources.get(&Resource::Clay).unwrap() >= other.resources.get(&Resource::Clay).unwrap() &&
        self.resources.get(&Resource::Obsidian).unwrap() >= other.resources.get(&Resource::Obsidian).unwrap() &&
        self.resources.get(&Resource::Geode).unwrap() >= other.resources.get(&Resource::Geode).unwrap();
        better
    }

    fn has_any_better_resource_than(&self, other: &SearchState) -> bool {
        let better = self.resources.get(&Resource::Ore).unwrap() >= other.resources.get(&Resource::Ore).unwrap() ||
        self.resources.get(&Resource::Clay).unwrap() >= other.resources.get(&Resource::Clay).unwrap() ||
        self.resources.get(&Resource::Obsidian).unwrap() >= other.resources.get(&Resource::Obsidian).unwrap() ||
        self.resources.get(&Resource::Geode).unwrap() >= other.resources.get(&Resource::Geode).unwrap();
        better
    }


    fn has_better_resources_than_all(&self, other_states: &Vec<SearchState>) -> bool {
        other_states.iter().all(|other_state| self.has_better_resources_than(other_state))
    }

    fn has_worse_resources_than_all(&self, other_states: &Vec<SearchState>) -> bool {
        other_states.iter().all(|other_state| other_state.has_better_resources_than(self))
    }

    fn new() -> Self {
        Self {
            current_production: [(Resource::Ore, 1), (Resource::Clay, 0), (Resource::Obsidian, 0), (Resource::Geode, 0), ].into_iter().collect(),
            resources: [(Resource::Ore, 0), (Resource::Clay, 0), (Resource::Obsidian, 0), (Resource::Geode, 0)].into_iter().collect()
        }
    }

    fn produce(mut self) -> Self {
        self.resources = self.resources.into_iter()
            .map(|(resource, current_units)| 
                if let Some(new_units) = self.current_production.get(&resource) {
                    (resource, current_units + new_units)
                } else {
                    (resource, current_units)
                }
            ).collect();
        self
    }

    fn build_robot(mut self, robot: &RobotType) -> Self {
        self.resources = self.resources.into_iter()
            .map(|(resource, current_units)|  
                if let Some(cost) = robot.cost.get(&resource) {
                    (resource, current_units - cost)
                } else {
                    (resource, current_units)
                }
            ).collect();
        self.current_production = self.current_production.into_iter()
            .map(|(resource, n_units)|
                if resource == robot.produces {
                    (resource, n_units + 1)
                } else {
                    (resource, n_units)
                }
            ).collect();
        self
    }
}

fn drop_worse_robots(search_states: Vec<SearchState>) -> Vec<SearchState> {
    let mut kept_states: BTreeMap<(u16, u16, u16, u16), Vec<SearchState>> = BTreeMap::new();
    for state in search_states {
        let key = state.resource_key();
        if let Some(other_states) = kept_states.get_mut(&key) {
            if state.has_better_robots_than_all(other_states) {
                // println!("\nDropping other states: {:?}", other_states);
                // println!("In favour of state: {:?}", state);
                kept_states.insert(key, vec![state]);
            } else if state.has_worse_robots_than_all(other_states) {
                // println!("\nDropping state: {:?}", state);
                // println!("In favour of other states: {:?}", other_states);
            } else {
                // println!("\nKeeping state: {:?}", state);
                // println!("As well as other states: {:?}", other_states);
                other_states.push(state);
            }
        } else {
            kept_states.insert(key, vec![state]);
        }
    }
    kept_states.into_iter().flat_map(|(_, states)| states.into_iter()).collect()
}

fn drop_worse_resources(search_states: Vec<SearchState>) -> Vec<SearchState> {
    let mut kept_states: BTreeMap<(u16, u16, u16, u16), Vec<SearchState>> = BTreeMap::new();
    for state in search_states {
        let key = state.robot_key();
        if let Some(other_states) = kept_states.get_mut(&key) {
            if state.has_better_resources_than_all(other_states) {
                // println!("\nDropping other states: {:?}", other_states);
                // println!("In favour of state: {:?}", state);
                kept_states.insert(key, vec![state]);
            } else if state.has_worse_resources_than_all(other_states) {
                // println!("\nDropping state: {:?}", state);
                // println!("In favour of other states: {:?}", other_states);
            } else {
                // println!("\nKeeping state: {:?}", state);
                // println!("As well as other states: {:?}", other_states);
                other_states.push(state);
            }
        } else {
            kept_states.insert(key, vec![state]);
        }
    }
    kept_states.into_iter().flat_map(|(_, states)| states.into_iter()).collect()
}

fn drop_worse(search_states: Vec<SearchState>) -> Vec<SearchState> {
    let mut kept_states: Vec<SearchState> = vec![];
    for state in search_states {
        if kept_states.is_empty() {
            kept_states.push(state)
        } else {
            // Först filtrerar vi genom att bara behålla sådana som antingen har bättre resurser eller bättre robotar än den nya.
            kept_states = kept_states.into_iter().filter(|kept| kept.has_any_better_resource_than(&state) || kept.has_any_better_robot_than(&state)).collect();
            // Sedan lägger vi till den nya endast om det har bättre state eller robotar än någon befintlig
            if kept_states.iter().any(|kept| state.has_any_better_resource_than(kept) || state.has_any_better_robot_than(kept)) {
                kept_states.push(state)
            } else {
                println!("Dropping {:?}", state);
                println!("In favour of {:?}", kept_states);
            }
        }
    }
    kept_states
}

/*
    Eller ska vi helt enkelt uppdatera has_better_robots 
    till att prioritera Geode robots, dvs fler Geode robotar är alltid bättre.


    Eller ska vi filtrera på vad som totalt har producerats?
    Vid lika robotar och lika resurser så är det samma värde.

    Eller ska vi räkna Ore värdet som vad vi har i Ore + hur mycket Ore vi pröjsat för robotar
    - Samma för Clay och Obsidian


*/


fn keep_best(mut search_states: Vec<SearchState>, round: u16) -> Vec<SearchState> {
    search_states.sort();
    let before = search_states.len();
    search_states.dedup();


    search_states = drop_worse_robots(search_states);
    search_states = drop_worse_resources(search_states);
    // search_states = drop_worse(search_states);
    let after = search_states.len();
    println!("Pruned {} Remaining {}", before - after, after);
    search_states
}

fn max_geodes_for_blueprint(blueprint: &Blueprint) -> u16 {
    println!("New blueprint");
    let mut search_states = vec![SearchState::new()];
    for round in 0..32 {
        println!("round={round}");
        let mut new_search_states = vec![];
        for search_state in search_states {
            let buildable_robots = determine_buildable_robot_types(blueprint, &search_state.resources);
            let produced_state = search_state.clone().produce();
            for robot in buildable_robots {
                let new_state = produced_state.clone().build_robot(&robot);
                new_search_states.push(new_state);
            }
            new_search_states.push(produced_state);
    }
        search_states = keep_best(new_search_states, round);
    }
    let bp = *search_states.iter().map(|state| state.resources.get(&Resource::Geode).unwrap()).max().unwrap();
    println!("BP: {bp}");
    bp
}


fn parse_blueprints(input: &str) -> Vec<Blueprint> {
    let re = Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            Blueprint {
                id: caps[1].parse().unwrap(),
                robot_types: vec![
                    RobotType {
                        produces: Resource::Ore,
                        cost: BTreeMap::from([(Resource::Ore, caps[2].parse().unwrap())]),
                    },
                    RobotType {
                        produces: Resource::Clay,
                        cost: BTreeMap::from([(Resource::Ore, caps[3].parse().unwrap())]),
                    },
                    RobotType {
                        produces: Resource::Obsidian,
                        cost: BTreeMap::from([
                            (Resource::Ore, caps[4].parse().unwrap()),
                            (Resource::Clay, caps[5].parse().unwrap()),
                        ]),
                    },
                    RobotType {
                        produces: Resource::Geode,
                        cost: BTreeMap::from([
                            (Resource::Ore, caps[6].parse().unwrap()),
                            (Resource::Obsidian, caps[7].parse().unwrap()),
                        ]),
                    },
                ],
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let blueprints = parse_blueprints(include_str!("../test.txt"));
        assert_eq!(part_1(&blueprints), 33)
    }
}
