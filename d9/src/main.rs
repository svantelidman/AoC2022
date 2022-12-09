use std::collections::HashSet;
fn main() {
    let moves = parse_moves(include_str!("../input.txt"));
    println!("Part 1: {}", count_tail_positions(&moves, 2));
    println!("Part 2: {}", count_tail_positions(&moves, 10));
}

struct Move {
    n_steps: usize,
    delta_x: isize,
    delta_y: isize
}

impl Move {
    fn parse(s: &str) -> Self {
        let n_steps = s[2..].parse::<usize>().unwrap();
        match s.chars().next().unwrap() {
            'L' => Move {n_steps: n_steps, delta_x: -1, delta_y: 0},
            'R' => Move {n_steps: n_steps, delta_x: 1, delta_y: 0},
            'D' => Move {n_steps: n_steps, delta_x: 0, delta_y: -1},
            'U' => Move {n_steps: n_steps, delta_x: 0, delta_y: 1},
            _ => panic!("Unexpected direction for Move")
        }
    }
}

fn parse_moves(input: &str) -> Vec<Move> {
    input.lines().map(|line|{
        Move::parse(line)
    }).collect()
}

fn count_tail_positions(moves: &Vec<Move>, n_knots: usize) -> usize {
    let mut tail_visits: HashSet<(isize, isize)> = HashSet::new();
    let mut knots: Vec<(isize, isize)> = vec![];
    knots.resize(n_knots, (0, 0));
    tail_visits.insert(knots[n_knots - 1]);
    for mv in moves {
        for _ in 0..(mv.n_steps) {
            knots[0].0 += mv.delta_x;
            knots[0].1 += mv.delta_y;
            for ind in 1..n_knots {
                knots[ind] = update_tail_pos(knots[ind], &knots[ind-1])
            }
            tail_visits.insert(knots[n_knots - 1]);
        }
    }
    tail_visits.len()
}

fn update_tail_pos(mut tail_pos: (isize, isize), head_pos: &(isize, isize)) -> (isize, isize) {
    let x_dist = (tail_pos.0 - head_pos.0).abs();
    let y_dist = (tail_pos.1 - head_pos.1).abs();
    if ((x_dist + y_dist) > 1) && (x_dist != 1 || y_dist != 1) {
        tail_pos.1 += (head_pos.1 - tail_pos.1).signum();
        tail_pos.0 += (head_pos.0 - tail_pos.0).signum();
    }
    tail_pos
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let moves = parse_moves(include_str!("../test1.txt"));
        assert_eq!(count_tail_positions(&moves, 2), 13)
    }

    #[test]
    fn test_part_2() {
        let moves = parse_moves(include_str!("../test2.txt"));
        assert_eq!(count_tail_positions(&moves, 10), 36)
    }
}