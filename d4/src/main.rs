use std::ops::RangeInclusive;

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> usize {
    parse_pairs(input).iter().filter(|p| p.fully_overlapping()).count()
}

fn part_2(input: &str) -> usize {
    parse_pairs(input).iter().filter(|p| p.overlapping()).count()
}

fn parse_pairs(input: &str) -> Vec<Pair> {
    input.lines().map(|l| Pair::parse(l)).collect()
}

struct Pair {
    a: Assignment,
    b: Assignment
}

impl Pair {
    fn parse(line: &str) -> Pair {
        let mut it = line.split(',');
        let a = Assignment::parse(it.next().unwrap());
        let b = Assignment::parse(it.next().unwrap());
        Pair{a, b}
    }

    fn fully_overlapping(&self) -> bool {
        self.a.contains(&self.b) || self.b.contains(&self.a)
    }

    fn partially_overlapping(&self) -> bool {
        self.a.partially_overlapping(&self.b)        
    }

    fn overlapping(&self) -> bool {
        self.fully_overlapping() || self.partially_overlapping()
    }
}

struct Assignment {
    min: usize,
    max: usize
}

impl Assignment {
    fn parse(s: &str) -> Assignment {
        let mut it = s.split('-');
        let min = it.next().unwrap().parse::<usize>().unwrap();
        let max = it.next().unwrap().parse::<usize>().unwrap();
        Assignment{min, max}
    }

    fn contains(&self, other: &Assignment) -> bool {
        other.min >= self.min && other.max <= self.max
    }

    fn as_range(&self) -> RangeInclusive<usize> {
        RangeInclusive::new(self.min, self.max)
    }

    fn partially_overlapping(&self, other: &Assignment) -> bool {
        self.as_range().contains(&other.min) || self.as_range().contains(&other.max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 2)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 4)
    }
}