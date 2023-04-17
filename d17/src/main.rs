use std::collections::{HashSet, HashMap};

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", 0);
}
// Coordinate system is row, cols starting from lower left
// The first row is 1 and the first column is 0 to confguse things.

fn top_signature(rock_pile: &Vec<Rock>) -> (Vec<(isize, isize)>, Option<isize>) {
    let mut top_rocks: HashSet<(isize, isize)> = HashSet::new();
    let mut filled_row: Option<isize> = None;
    rock_pile.iter().rev().any( |rock| {
        let rock_top = *rock.coordinates.iter().map(|(r, _c)| r).max().unwrap();
        let rock_bottom = *rock.coordinates.iter().map(|(r, _c)| r).min().unwrap();
        top_rocks = top_rocks.union(&rock.coordinates).map(|c| *c).collect();
        for row in rock_bottom..=rock_top {
            if (0isize..=6isize).all(|col| top_rocks.contains(&(row, col)) || top_rocks.contains(&(row + 1, col))) {
                filled_row = Some(row);
            }
            if let Some(_) = filled_row {
                break
            }
        }
        if let Some(_) = filled_row {
            true
        } else {
            false
        }
    });
    let mut top_coords: Vec<_> = if let Some(filled_row) = filled_row {
        top_rocks.iter().filter(|(r, _c)| *r > filled_row).map(|(r,c)| (r - filled_row - 1, *c)).collect()
    } else {
        rock_pile.iter().flat_map(|rock| rock.coordinates.iter().map(|c| *c)).collect()
    };
    top_coords.sort();
    (top_coords, filled_row)

}

fn part_1(input: &str) -> usize {
    let n_rocks: usize = 1_000_000_000_000;
    let mut jet_impact = JetImpact::new(input);
    let mut rock_generator = RockGenerator::new();
    let mut highest_bottom = 0;
    let mut rock_pile: Vec<Rock> = vec![];
    let mut last_filled_row: Option<isize> = None;
    let mut top_signatures: HashMap<(Vec<(isize, isize)>, usize, usize), (Option<isize>, usize, isize)> = HashMap::new();
    let mut total_short_circuit_height = 0;
    let mut last_rock = n_rocks;
    let mut loop_detected = false;
    for rock_number in 0..n_rocks {
        let (mut rock, rock_type) = rock_generator.next_rock(highest_bottom + 1);
        let mut fall_next = false;
        loop {
            if fall_next {
                if at_bottom(&rock, &rock_pile) {
                    highest_bottom = rock.coordinates.iter().map(|(row, _)| *row).max().unwrap().max(highest_bottom);
                    rock_pile.push(rock);
                    let (top_signature, filled_row) = top_signature(&rock_pile);
                    if !loop_detected && filled_row != last_filled_row {
                        if let Some((previous_row, previous_number_of_fallen_rocks, previous_highest_bottom)) = top_signatures.get(&(top_signature.clone(), rock_type, jet_impact.jet_position())) {
                            println!("Repeat signature detected for rock type {}", rock_type);
                            println!("Previous filled row {:?} number of fallen rocks {} height {}", previous_row, previous_number_of_fallen_rocks, previous_highest_bottom);
                            println!("Current filled row {:?} number of fallen rocks {} height {} ", filled_row, rock_number + 1, highest_bottom);
                            let delta_rocks =rock_number + 1 - previous_number_of_fallen_rocks;
                            let delta_height = highest_bottom - previous_highest_bottom;
                            let n_short_circuit_loops = (n_rocks - (rock_number + 1)) / delta_rocks;
                            total_short_circuit_height = n_short_circuit_loops * delta_height as usize;
                            last_rock = (n_rocks - (rock_number + 1)) % delta_rocks + rock_number + 1;
                            loop_detected = true;
                            println!("Last rock: {}", last_rock);
                        } else {
                            top_signatures.insert((top_signature, rock_type, jet_impact.jet_position()), (filled_row, rock_number + 1, highest_bottom));
                            last_filled_row = filled_row
                        }
                    }
                    break
                } else {
                    rock.fall();
                    fall_next = false;
                }
            } else {
                rock.adjust_column(jet_impact.next_gust(), &rock_pile);
                fall_next = true;
            }
        }
        if rock_number == last_rock {
            break
        }
    }
    highest_bottom as usize + total_short_circuit_height - 1
}

fn print_rock_pile(rock_pile: &Vec<Rock>, falling_rock: Option<&Rock>, highest_bottom: isize) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut grid_row: Vec<char> = vec![];
    grid_row.resize(7, '.');
    let mut max_row = highest_bottom + 4;
    if let Some(falling_rock) = falling_rock {
        max_row = falling_rock.coordinates.iter().map(|(row, _)| *row).max().unwrap().max(max_row)
    }
    max_row += 1;
    grid.resize(max_row as usize, grid_row);
    for rock in rock_pile {
        for (row, col) in &rock.coordinates {
             grid[*row as usize][*col as usize] = '#'
        }
    }
    if let Some(falling_rock) = falling_rock {
        for (row, col) in &falling_rock.coordinates {
            grid[*row as usize][*col as usize] = '@'
       }
    }

    grid.iter().enumerate().skip(1).rev().for_each(|(row_number, row)| println!("|{}| {}", row.iter().collect::<String>(), row_number));
    println!("+-------+\n");
}

fn at_bottom(rock: &Rock, rock_pile: &Vec<Rock>) -> bool {
    if rock.coordinates.iter().any(|(row, _)| *row == 1) {
        return true
    }
    let mut would_be_rock = rock.clone();
    would_be_rock.fall();
    rock_pile.iter().rev().any(|rock| would_be_rock.intersects(rock))
}


struct JetImpact {
    pattern: Vec<char>,
    next_ind: usize
}

impl JetImpact {
    fn new(input: &str) -> Self {
        Self{
            pattern: input.lines().next().unwrap().chars().collect(),
            next_ind: 0
        }
    }

    fn jet_position(&self) -> usize {
        self.next_ind
    }

    fn next_gust(&mut self) -> isize {
        let gust = match self.pattern[self.next_ind] {
            '>' => 1,
            '<' => -1,
            _ => panic!("Illegal character in gust pattern")
        };
        self.next_ind += 1;
        if self.next_ind == self.pattern.len() {
            self.next_ind = 0;
        }
        gust
    }

}

struct RockGenerator {
    ind: usize
}

impl RockGenerator {
    fn new() -> Self {
        RockGenerator { ind: 0 }
    }

    fn next_rock(&mut self, bottom: isize) -> (Rock, usize) {
        let (rock, rock_type) = (Rock::new(self.ind, bottom), self.ind);
        self.ind += 1;
        if self.ind == 5 {
            self.ind = 0;
        }
        (rock, rock_type)
    } 
}

#[derive(Clone)]
struct Rock {
    coordinates: HashSet<(isize, isize)>
}

impl Rock {
    fn new(ind: usize, bottom: isize) -> Self {
        let shape = match ind {
            0 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
            1 => vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            2 => vec![(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)],
            3 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            4 => vec![(0, 0), (0, 1), (1, 0), (1, 1)],
            _ => panic!("Illegal rock prototype index.")
        };
        let position = (bottom + 3, 2);
        let coordinates = shape.into_iter().map(|(row, col)| (row + position.0, col + position.1)).collect();
        Rock{coordinates}
    }

    fn fall(&mut self) {
        self.coordinates = self.coordinates.iter().map(|(row, col)| (row - 1, *col)).collect();
    }

    fn translate_col(&mut self, delta_col: isize) {
        self.coordinates = self.coordinates.iter().map(|(row, col)| (*row, col + delta_col)).collect();
    }

    fn intersects(&self, other: &Rock) -> bool {
        self.coordinates.intersection(&other.coordinates).count() > 0
    }

    fn adjust_column(&mut self, delta_col: isize, rock_pile: &Vec<Rock>) -> bool {
        if self.coordinates.iter().any(|(_, col)| *col + delta_col < 0 || *col + delta_col > 6) {
            return false;
        }
        let mut would_be_rock = self.clone();
        would_be_rock.translate_col(delta_col);
        if rock_pile.iter().rev().any(|rock| would_be_rock.intersects(rock)) {
            return false;
        }
        self.coordinates = would_be_rock.coordinates;
        return true
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 3068)
    }
}