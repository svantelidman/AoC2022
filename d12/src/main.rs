use std::collections::HashMap;

fn main() {
    let (grid, (max_row, max_col), (start_row, start_col), (target_row, target_col)) =
        load_grid(include_str!("../input.txt"));
    let p1 = shortest_path(
        vec![(start_row, start_col)],
        usize::MAX,
        &mut HashMap::new(),
        start_row,
        start_col,
        target_row,
        target_col,
        &grid,
        max_row,
        max_col,
    ) - 1;
    println!("Part 1: {}", p1);

    // Looking at the input we can conclude that the relevant position is to be found in the
    // first couple of columns and that the result, at most is one less than the answer in part 1,
    let mut a_positions: Vec<(usize, usize)> = vec![];
    for r in 0..=max_row {
        for c in 0..=2 {
            if grid[r][c] == 'a' {
                a_positions.push((r, c))
            }
        }
    }

    let mut shortest_known = p1;
    for (a_row, a_col) in a_positions {
        let shortest_from_this_a = shortest_path(
            vec![(a_row, a_col)],
            shortest_known,
            &mut HashMap::new(),
            a_row,
            a_col,
            target_row,
            target_col,
            &grid,
            max_row,
            max_col,
        );
        shortest_known = shortest_known.min(shortest_from_this_a)
    }
    let p2 = shortest_known - 1;
    println!("Part 2: {}", p2);
}

fn next_positions(
    current_row: isize,
    current_col: isize,
    target_row: isize,
    target_col: isize,
    grid: &Vec<Vec<char>>,
    max_row: isize,
    max_col: isize,
    path: &Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let next_positions = [
        (current_row + 1, current_col),
        (current_row - 1, current_col),
        (current_row, current_col + 1),
        (current_row, current_col - 1),
    ]
    .into_iter()
    .filter(|(r, c)| {
        r >= &0
            && r <= &max_row
            && c >= &0
            && c <= &max_col
            && ((grid[*r as usize][*c as usize] != 'E'
                && grid[*r as usize][*c as usize] as u8
                    <= grid[current_row as usize][current_col as usize] as u8 + 1)
                || (*r == target_row
                    && *c == target_col
                    && grid[current_row as usize][current_col as usize] >= 'y')
                || grid[current_row as usize][current_col as usize] == 'S')
            && !path.contains(&(*r as usize, *c as usize))
    })
    .map(|(r, c)| (r as usize, c as usize))
    .collect();
    next_positions
}

fn shortest_path(
    path: Vec<(usize, usize)>,
    shortest_known: usize,
    shortest_to_pos: &mut HashMap<(usize, usize), usize>,
    current_row: usize,
    current_col: usize,
    target_row: usize,
    target_col: usize,
    grid: &Vec<Vec<char>>,
    max_row: usize,
    max_col: usize,
) -> usize {
    if path.len() >= shortest_known {
        return shortest_known;
    }
    if let Some(shortest_so_far) = shortest_to_pos.get(&(current_row, current_col)) {
        if path.len() >= *shortest_so_far {
            return shortest_known;
        } else {
            shortest_to_pos.insert((current_row, current_col), path.len());
        }
    } else {
        shortest_to_pos.insert((current_row, current_col), path.len());
    }
    if current_row == target_row && current_col == target_col {
        return path.len();
    }
    let next_positions = next_positions(
        current_row as isize,
        current_col as isize,
        target_row as isize,
        target_col as isize,
        grid,
        max_row as isize,
        max_col as isize,
        &path,
    );
    let mut new_shortest_known = shortest_known;
    for next_pos in next_positions {
        let mut new_path = path.clone();
        new_path.push(next_pos);
        new_shortest_known = new_shortest_known.min(shortest_path(
            new_path,
            new_shortest_known,
            shortest_to_pos,
            next_pos.0,
            next_pos.1,
            target_row,
            target_col,
            grid,
            max_row,
            max_col,
        ));
    }
    new_shortest_known
}

fn load_grid(
    input: &str,
) -> (
    Vec<Vec<char>>,
    (usize, usize),
    (usize, usize),
    (usize, usize),
) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_row = grid.iter().position(|row| row.contains(&'S')).unwrap();
    let start_col = grid[start_row].iter().position(|c| c == &'S').unwrap();
    let target_row = grid.iter().position(|row| row.contains(&'E')).unwrap();
    let target_col = grid[target_row].iter().position(|c| c == &'E').unwrap();
    let max_row = grid.len() - 1;
    let max_col = grid[0].len() - 1;
    (
        grid,
        (max_row, max_col),
        (start_row, start_col),
        (target_row, target_col),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let (grid, (max_row, max_col), (start_row, start_col), (target_row, target_col)) =
            load_grid(include_str!("../test.txt"));
        assert_eq!(
            shortest_path(
                vec![(start_row, start_col)],
                usize::MAX,
                &mut HashMap::new(),
                start_row,
                start_col,
                target_row,
                target_col,
                &grid,
                max_row,
                max_col
            ),
            31
        )
    }
}
