use std::collections::{HashSet, HashMap};
fn main() {
    println!("Part 1: {}", part_1_and_2(load_input(include_str!("../input.txt")), false));
    println!("Part 2: {}", part_1_and_2(load_input(include_str!("../input.txt")), true));
}

fn print_elves(elves: &HashSet<(isize, isize)>) {
    let min_row = elves.iter().map(|(row, _)| row).min().unwrap();
    let max_row = elves.iter().map(|(row, _)| row).max().unwrap();
    let min_col = elves.iter().map(|(_, col)| col).min().unwrap();
    let max_col = elves.iter().map(|(_, col)| col).max().unwrap();
    let n_rows = (max_row - min_row + 1) as usize;
    let n_cols = (max_col - min_col + 1) as usize ;
    let mut grid: Vec<Vec<char>> = vec![];
    let mut row: Vec<char> = vec![];
    row.resize(n_cols, '.');
    grid.resize(n_rows, row);


    for (row, col) in elves {
        grid[(*row - *min_row) as usize][(*col - *min_col) as usize] = '#'
    }

    println!();
    for row in grid {
        println!("{}", row.iter().collect::<String>())
    }
}

fn part_1_and_2(input: HashSet<(isize, isize)>, is_part_2: bool) -> usize {
    let mut elves = input.clone();
    // print_elves(&elves);
    let mut proposer = Proposer::new();
    let n_rounds = if is_part_2 {
        usize::MAX
    } else {
        10
    };
    for round in 1..n_rounds {
        // println!("\nRound: {} First direction: {:?}", round, proposer.first_direction);

        let mut proposals: HashMap<(isize, isize), (isize, isize)> = HashMap::new(); // (proposed_pos, current_pos)
        let mut duplicate_proposals: HashSet<(isize, isize)> = HashSet::new(); 
        for (current_row, current_col) in &elves {
            if let Some((proposed_row, proposed_col)) = proposer.propose(*current_row, *current_col, &elves) {
                if let Some(_) = proposals.insert((proposed_row, proposed_col), (*current_row, *current_col)) {
                    duplicate_proposals.insert((proposed_row, proposed_col));
                }
            }
        }
        let valid_proposals: HashMap<_, _> = proposals.into_iter()
            .filter(|((proposed_row, proposed_col), _)| !duplicate_proposals.contains(&(*proposed_row, *proposed_col)))
            .map(|((proposed_row, proposed_col), (current_row, current_col))| ((current_row, current_col), (proposed_row, proposed_col))).collect();
        if is_part_2 && valid_proposals.is_empty() {
            return round
        }

        elves = elves.into_iter()
            .map(|current_pos| 
                if let Some((new_row, new_col)) = valid_proposals.get(&current_pos) {
                    (*new_row, *new_col)
                } else {
                    current_pos
                }
            )
            .collect();

        proposer.increment_first_direction();
        // print_elves(&elves);
    }
    let min_row = elves.iter().map(|(row, _)| row).min().unwrap();
    let max_row = elves.iter().map(|(row, _)| row).max().unwrap();
    let min_col = elves.iter().map(|(_, col)| col).min().unwrap();
    let max_col = elves.iter().map(|(_, col)| col).max().unwrap();
    ((max_row - min_row + 1) * (max_col - min_col + 1)) as usize - elves.len()

}


#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    West,
    East
}

impl Direction {
    fn next(&self) -> Direction {
        match self {
            Direction:: North => Direction::South,
            Direction:: South => Direction::West,
            Direction:: West => Direction::East,
            Direction:: East => Direction::North,
        }
    }
}

fn can_move(direction: Direction, row: isize, col: isize, grid: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
    let all_positions = [(row - 1, col - 1), (row - 1, col), (row - 1, col + 1), (row, col + 1), (row + 1, col + 1), (row + 1, col), (row + 1, col - 1), (row, col - 1)];
    if all_positions.iter().any(|pos| grid.contains(pos)) {
        let check_positions = match direction {
            Direction::North => [(row - 1, col - 1), (row - 1, col), (row - 1, col + 1)],
            Direction::South => [(row + 1, col - 1), (row + 1, col), (row + 1, col + 1)],
            Direction::West => [(row - 1, col - 1), (row, col - 1), (row + 1, col - 1)],
            Direction::East => [(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)]
        };
        if check_positions.iter().any(|pos| grid.contains(pos)) {
            None
        } else {
            match direction {
                Direction::North => Some((row - 1, col)),
                Direction::South => Some((row + 1, col)),
                Direction::West => Some((row, col - 1)),
                Direction::East => Some((row, col + 1)),
            }
        }
    } else {
        None
    }
}

struct Proposer {
    first_direction: Direction
}

impl Proposer {
    fn new() -> Self {
        Self {first_direction: Direction::North}
    }

    fn increment_first_direction(&mut self) {
        self.first_direction = self.first_direction.next()
    }
 
    fn propose(&self, row: isize, col: isize, grid: &HashSet<(isize, isize)>) -> Option<(isize, isize)> {
        let mut direction = self.first_direction.clone();
        for _ in 0..4 {
            if let Some((row, col)) = can_move(direction, row, col, grid) {
                return Some((row, col))
            }
            direction = direction.next()
        }
        None
    }
}


fn load_input(input: &str) -> HashSet<(isize, isize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(move |(col_index, c)| (row_index, col_index, c))
        })
        .filter(|(_row_index, _col_index, c)| *c == '#')
        .map(|(row_index, col_index, _)| (row_index as isize, col_index as isize))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_and_2(load_input(include_str!("../test.txt")), false), 110)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_1_and_2(load_input(include_str!("../test.txt")), true), 20)
    }}
