fn main() {
    println!("Part 1: {}", rearrange(include_str!("../input.txt"), false));
    println!("Part 2: {}", rearrange(include_str!("../input.txt"), true));
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<(usize, usize, usize)>) {
    let mut parts = input.split("\n\n");
    let stacks_input = parts.next().unwrap();
    let moves_input = parts.next().unwrap();
    (parse_stacks(stacks_input), parse_moves(moves_input))
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec!();
    let mut lines: Vec<_> = input.lines().rev().collect();
    let index_line = lines.remove(0);
    let n_stacks = (index_line.len() + 1) / 4;
    stacks.resize(n_stacks, vec!());
    for line in lines {
        for stack_ind in 0..n_stacks {
            let cr = line.chars().nth( 1 + stack_ind*4).unwrap();
            if cr != ' ' {
                stacks[stack_ind].push(cr)
            }
        }
    }
    stacks
}

fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    let mut moves: Vec<(usize, usize, usize)> = vec!();
    for line in input.lines() {
        let parts: Vec<_> = line.split(' ').collect();
        let n = parts[1].parse::<usize>().unwrap();
        let from = parts[3].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();
        moves.push((n, from, to))
    }
    moves
}

fn rearrange(input: &str, fancy_crane: bool) -> String {
    let (mut stacks, moves) = parse_input(input);
    for mv in moves {
        stacks = make_move(stacks, mv, fancy_crane);
    }
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn make_move(mut stacks: Vec<Vec<char>>, mv: (usize, usize, usize), fancy_crane: bool) -> Vec<Vec<char>> {
    if fancy_crane {
        stacks = move_all_items(stacks, mv)
    } else {
        for _ in 0..mv.0 {
            stacks = move_one_item(stacks, mv)
        }    
    }
    stacks
}

fn move_one_item(mut stacks: Vec<Vec<char>>, mv: (usize, usize, usize)) -> Vec<Vec<char>> {
    let cr = stacks[mv.1 - 1].pop().unwrap();
    stacks[mv.2 - 1].push(cr);
    stacks
}

fn move_all_items(mut stacks: Vec<Vec<char>>, mv: (usize, usize, usize)) -> Vec<Vec<char>> {
    let split_ind = stacks[mv.1 - 1].len() - mv.0;
    let (r, m) = stacks[mv.1 - 1].split_at(split_ind);
    let mut m: Vec<char> = m.iter().map(|c| *c).collect();
    stacks[mv.1 - 1] = r.iter().map(|c| *c).collect();
    stacks[mv.2 - 1].append(&mut m);
    stacks
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_1() {
    //     assert_eq!(rearrange(include_str!("../test.txt"), false), String::from("CMZ"))
    // }
    #[test]

    fn test_part_2() {
        assert_eq!(rearrange(include_str!("../test.txt"), true), String::from("MCD"))
    }
}