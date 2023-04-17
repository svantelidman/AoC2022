use regex::Regex;
use std::collections::BTreeMap;
use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*;

fn main() {
    let blueprints = parse_blueprints(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&blueprints));
    // println!("Part 2: {}", part_2(&blueprints));
}

fn part_1(blueprints: &Vec<Blueprint>) -> usize {
    blueprints.iter().enumerate().map(|(ind, bp)| {let mx = max_geodes_for_blueprint(bp, 24); println!("{ind} {mx}"); mx * (ind + 1)}).sum::<usize>()
}

fn part_2(blueprints: &Vec<Blueprint>) -> usize {
    blueprints.iter().take(3).enumerate().map(|(_ind, bp)| max_geodes_for_blueprint(bp, 32)).product::<usize>()
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
    cost: BTreeMap<Resource, usize>,
}

impl RobotType {
    fn can_build(&self, resources: &BTreeMap<Resource, usize>) -> bool {
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
    id: usize,
    robot_types: Vec<RobotType>,
}

fn determine_buildable_robot_types(blueprint: &Blueprint, resources: &BTreeMap<Resource, usize>) -> Vec<RobotType> {
    let geode_robot = blueprint.robot_types.iter().find(|rt| rt.produces == Resource::Geode).unwrap();
    let obsidian_robot = blueprint.robot_types.iter().find(|rt| rt.produces == Resource::Obsidian).unwrap();
    if geode_robot.can_build(resources) {
        vec![geode_robot.clone()]
    } else if obsidian_robot.can_build(resources) {
        vec![obsidian_robot.clone()]
    } else {
        blueprint.robot_types.iter().filter(|rt| rt.can_build(resources)).map(|rt| rt.clone()).collect()
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct SearchState {
    current_production: BTreeMap<Resource, usize>,
    resources: BTreeMap<Resource, usize>
}

impl SearchState {

    fn max_possible_geode(&self, rounds: usize) -> usize {
        // Assume one robot can be produced in each round
        let n_geode_robots = *self.current_production.get(&Resource::Geode).unwrap();
        let (geode_produced, _n_geode_robots) = (0..rounds).fold((0, n_geode_robots), |(acc_geodes, acc_robots), _ind| (acc_geodes + acc_robots, acc_robots + 1));
        geode_produced + rounds
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
                        produces: Resource::Geode,
                        cost: BTreeMap::from([
                            (Resource::Ore, caps[6].parse().unwrap()),
                            (Resource::Obsidian, caps[7].parse().unwrap()),
                        ]),
                    },
                    RobotType {
                        produces: Resource::Obsidian,
                        cost: BTreeMap::from([
                            (Resource::Ore, caps[4].parse().unwrap()),
                            (Resource::Clay, caps[5].parse().unwrap()),
                        ]),
                    },
                    RobotType {
                        produces: Resource::Clay,
                        cost: BTreeMap::from([(Resource::Ore, caps[3].parse().unwrap())]),
                    },
                    RobotType {
                        produces: Resource::Ore,
                        cost: BTreeMap::from([(Resource::Ore, caps[2].parse().unwrap())]),
                    },
                ],
            }
        })
        .collect()
}


fn search(blueprint: &Blueprint, n_rounds: usize, next_round: usize, state: SearchState, best_so_far: usize) -> usize {
    // println!("Search, n_rounds:{n_rounds} next_round:{next_round} best_so_far:{best_so_far}");
    let mut buildable_robots = determine_buildable_robot_types(blueprint, &state.resources);
    buildable_robots.shuffle(&mut thread_rng());
    let produced_state = state.clone().produce();
    if next_round == n_rounds {
        return *produced_state.resources.get(&Resource::Geode).unwrap()
    }
    let remaining_rounds = n_rounds - next_round;
    let max_geodes_from_here = produced_state.max_possible_geode(remaining_rounds);
    if produced_state.resources.get(&Resource::Geode).unwrap() + max_geodes_from_here <= best_so_far {
        return best_so_far
    }
    let mut best_so_far = best_so_far;
    // let mut new_states = buildable_robots.into_iter()
    //     .map(|rt| produced_state.clone().build_robot(&rt)).collect::<Vec<_>>();
    // new_states.push(produced_state);
    // let result = new_states.par_iter()
    //     .fold(|| best_so_far, |acc_best, state| acc_best.max(search(blueprint, n_rounds, next_round + 1, state.clone(), acc_best))).max().unwrap();
    // if result > best_so_far {
    //     best_so_far = result;
    //     // println!("New best: {best_so_far}")
    // }
    
    for robot in buildable_robots {
        let new_state = produced_state.clone().build_robot(&robot);
        let result = search(blueprint, n_rounds, next_round + 1, new_state, best_so_far);
        if result > best_so_far {
            best_so_far = result;
            println!("New best: {best_so_far} Round {next_round}")
        }
    }
    let result = search(blueprint, n_rounds, next_round + 1, produced_state, best_so_far);
    if result > best_so_far {
        best_so_far = result;
        println!("New best: {best_so_far} Round {next_round}")
    }
    return best_so_far
} 

fn max_geodes_for_blueprint(blueprint: &Blueprint, n_rounds: usize) -> usize {
    println!("New blueprint");
    search(blueprint, n_rounds, 1, SearchState::new(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let blueprints = parse_blueprints(include_str!("../test.txt"));
        assert_eq!(part_1(&blueprints), 33)
    }

    // #[test]
    // fn test_part_2() {
    //     let blueprints = parse_blueprints(include_str!("../test.txt"));
    //     assert_eq!(part_2(&blueprints), 62 * 56)
    // }

}
