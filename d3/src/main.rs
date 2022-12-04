use std::collections::HashSet;
fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> usize {
    get_incorrect_items(input).into_iter().map(|c| c.priority()).sum()
}

fn part_2(input: &str) -> usize {
    get_badges(input).into_iter().map(|c| c.priority()).sum()
}

trait Priority {
    fn priority(&self) -> usize;
}

impl Priority for char {
    fn priority(&self) -> usize {
        if *self <= 'Z' {
            *self as usize - 'A' as usize + 27
        } else {
            *self as usize - 'a' as usize + 1
        }
    }
}

fn get_incorrect_items(input: &str) -> Vec<char> {
    input.lines().map(|line| {
        let n = (line.len() + 1) / 2;
        let types_1: HashSet<char> =  line[0..n].chars().collect();
        let types_2: HashSet<char> =  line[n..].chars().collect();
        let incorrect_item = *types_1.intersection(&types_2).next().unwrap();
        incorrect_item
    }).collect()
} 

fn get_badges(input: &str) -> Vec<char> {
    let lines: Vec<_> = input.lines().map(|l| String::from(l)).collect();
    let groups = lines.chunks(3);
    let badges = groups.into_iter().map(|g| {
        let types_1: HashSet<char> =  g[0].chars().collect();
        let types_2: HashSet<char> =  g[1].chars().collect();
        let types_3: HashSet<char> =  g[2].chars().collect();
        let tmp: HashSet<_> = types_1.intersection(&types_2).map(|c| *c).collect();
        *types_3.intersection(&tmp).next().unwrap()
    }).collect();
    badges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 157)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 70)
    }
}