use itertools::iproduct;
use std::collections::HashMap;

fn main() {
    let cave = parse_cave(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(cave.clone()));
    println!("Part 2: {}", part_2(cave));
}

fn rock_bottoms(cave: &HashMap<(isize, isize), char>) -> HashMap<isize, isize> {
    let mut bottoms: HashMap<isize, isize> = HashMap::new();
    for (rock_x, rock_y) in cave.keys() {
        if let Some(bottom_y) = bottoms.get(rock_x) {
            if rock_y > bottom_y {
                bottoms.insert(*rock_x, *rock_y);
            }
        } else {
            bottoms.insert(*rock_x, *rock_y);
        }
    }
    bottoms
}

fn can_move_to(x: isize, y: isize, cave: &HashMap<(isize, isize), char>) -> bool {
    match cave.get(&(x, y)) {
        Some(_) => false,
        None => true,
    }
}

fn can_move_down(x: isize, y: isize, cave: &HashMap<(isize, isize), char>) -> bool {
    can_move_to(x, y + 1, cave)
}

fn can_move_left(x: isize, y: isize, cave: &HashMap<(isize, isize), char>) -> bool {
    can_move_to(x - 1, y + 1, cave)
}

fn can_move_right(x: isize, y: isize, cave: &HashMap<(isize, isize), char>) -> bool {
    can_move_to(x + 1, y + 1, cave)
}

fn part_2(mut cave: HashMap<(isize, isize), char>) -> usize {
    let floor = cave.keys().map(|(_x, y)| y.clone()).max().unwrap() + 2;
    for x in 0..1000 {
        cave.insert((x, floor), '#');
    }
    let mut n_sands = 0;
    let mut full = false;
    loop {
        let (mut s_x, mut s_y): (isize, isize) = (500, 0);
        loop {
            if can_move_down(s_x, s_y, &cave) {
                s_y += 1
            } else if can_move_left(s_x, s_y, &cave) {
                s_x -= 1;
                s_y += 1
            } else if can_move_right(s_x, s_y, &cave) {
                s_x += 1;
                s_y += 1
            } else {
                cave.insert((s_x, s_y), 'o');
                if s_x == 500 && s_y == 0 {
                    n_sands += 1;
                    full = true;
                }
                break;
            }
        }
        if !full {
            n_sands += 1;
        } else {
            break;
        }
    }
    n_sands
}

fn part_1(mut cave: HashMap<(isize, isize), char>) -> usize {
    let bottoms = rock_bottoms(&cave);
    let mut n_sands = 0;
    let mut into_the_abyss = false;
    loop {
        let (mut s_x, mut s_y): (isize, isize) = (500, 0);
        loop {
            if can_move_down(s_x, s_y, &cave) {
                s_y += 1
            } else if can_move_left(s_x, s_y, &cave) {
                s_x -= 1;
                s_y += 1;
            } else if can_move_right(s_x, s_y, &cave) {
                s_x += 1;
                s_y += 1;
            } else {
                cave.insert((s_x, s_y), 'o');
                break;
            }
            if let Some(bottom_y) = bottoms.get(&s_x) {
                if s_y > *bottom_y {
                    into_the_abyss = true;
                    break;
                }
            } else {
                into_the_abyss = true;
                break;
            }
        }
        if !into_the_abyss {
            n_sands += 1;
        } else {
            break;
        }
    }
    n_sands
}

fn parse_cave(input: &str) -> HashMap<(isize, isize), char> {
    let mut cave: HashMap<(isize, isize), char> = HashMap::new();
    for line in input.lines() {
        let coord_pairs: Vec<_> = line
            .split(" -> ")
            .map(|s| {
                s.split(',')
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();
        for ind in 0..(coord_pairs.len() - 1) {
            let (cp1, cp2) = (&coord_pairs[ind], &coord_pairs[ind + 1]);
            let xmin = cp1[0].min(cp2[0]);
            let xmax = cp1[0].max(cp2[0]);
            let ymin = cp1[1].min(cp2[1]);
            let ymax = cp1[1].max(cp2[1]);
            for (x, y) in iproduct!(xmin..=xmax, ymin..=ymax) {
                cave.insert((x, y), '#');
            }
        }
    }
    cave
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let cave = parse_cave(include_str!("../test.txt"));
        assert_eq!(part_1(cave), 24)
    }

    #[test]
    fn test_part_2() {
        let cave = parse_cave(include_str!("../test.txt"));
        assert_eq!(part_2(cave), 93)
    }
}
