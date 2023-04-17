fn main() {
    println!("Part 1: {}", part_1_and_2(include_str!("../input.txt"), false));
    println!("Part 2: {}", part_1_and_2(include_str!("../input.txt"), true));
}

fn part_1_and_2(input: &str, is_part_2: bool) -> usize {
    let mut valley = load_input(input);
    let mut cursors: Vec<(usize, usize)> = vec![];
    let mut entrance = valley.entrance.clone();
    let mut exit = valley.exit.clone();
    cursors.push(entrance.clone());
    let mut n_found = 0;
    for round in 1.. {
        cursors = cursors
            .into_iter()
            .flat_map(|(row, col)| {
                if (row, col) == valley.entrance {
                    vec![(row, col), (row + 1, col)]
                } else {
                    vec![
                        (row, col),
                        (row + 1, col),
                        (row - 1, col),
                        (row, col + 1),
                        (row, col - 1),
                    ]
                }
            })
            .collect();
        if cursors.iter().any(|c| *c == exit) {
            if !is_part_2 {
                return round;
            } else {
                n_found += 1;
                if n_found == 3 {
                    return round;
                }
                cursors = vec![exit.clone()];    
                let tmp = entrance;
                entrance = exit;
                exit = tmp;
            }
        }
        valley.advance_blizzards();
        cursors = cursors
            .into_iter()
            .filter(|(row, col)| {
                (*row > 0
                    && *col > 0
                    && *row < valley.n_rows - 1
                    && *col < valley.n_cols - 1
                    && !valley.blizzards.iter().any(|blz| blz.row == *row && blz.col == *col))
                || (*row, *col) == valley.entrance || (*row, *col) == valley.exit
            })
            .collect();
        cursors.sort();
        cursors.dedup();
    }
    panic!("Should not get here!")
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Blizzard {
    row: usize,
    col: usize,
    direction: Direction,
}
struct Valley {
    blizzards: Vec<Blizzard>,
    n_rows: usize, // Including walls, upper left is (row= 0, col = 0)
    n_cols: usize,
    entrance: (usize, usize),
    exit: (usize, usize),
}

impl Valley {
    fn advance_blizzards(&mut self) {
        self.blizzards = self
            .blizzards
            .iter()
            .map(
                |Blizzard {
                     row,
                     col,
                     direction,
                 }| match direction {
                    Direction::Up => {
                        let new_row = if *row == 1 { self.n_rows - 2 } else { row - 1 };
                        Blizzard {
                            row: new_row,
                            col: *col,
                            direction: *direction,
                        }
                    }
                    Direction::Down => {
                        let new_row = if *row == self.n_rows - 2 { 1 } else { row + 1 };
                        Blizzard {
                            row: new_row,
                            col: *col,
                            direction: *direction,
                        }
                    }
                    Direction::Left => {
                        let new_col = if *col == 1 { self.n_cols - 2 } else { col - 1 };
                        Blizzard {
                            row: *row,
                            col: new_col,
                            direction: *direction,
                        }
                    }
                    Direction::Right => {
                        let new_col = if *col == self.n_cols - 2 { 1 } else { col + 1 };
                        Blizzard {
                            row: *row,
                            col: new_col,
                            direction: *direction,
                        }
                    }
                },
            )
            .collect();
    }
}

fn load_input(input: &str) -> Valley {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let entrance = (0, 1);
    let n_rows = grid.len();
    let n_cols = grid[0].len();
    let exit = (n_rows - 1, n_cols - 2);
    let mut blizzards: Vec<Blizzard> = vec![];
    for row in 0..n_rows {
        for col in 0..n_cols {
            match grid[row][col] {
                '^' => blizzards.push(Blizzard {
                    row,
                    col,
                    direction: Direction::Up,
                }),
                'v' => blizzards.push(Blizzard {
                    row,
                    col,
                    direction: Direction::Down,
                }),
                '<' => blizzards.push(Blizzard {
                    row,
                    col,
                    direction: Direction::Left,
                }),
                '>' => blizzards.push(Blizzard {
                    row,
                    col,
                    direction: Direction::Right,
                }),
                _ => (),
            }
        }
    }
    Valley {
        blizzards,
        n_rows,
        n_cols,
        entrance,
        exit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1_and_2(include_str!("../test.txt"), false), 18)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_1_and_2(include_str!("../test.txt"), true), 54)
    }

}
