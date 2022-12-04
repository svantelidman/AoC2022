fn main() {
    let rounds = parse_rounds(include_str!("../input.txt"));
    println!("Part 1: {}", total_score_part_1(&rounds));
    println!("Part 2: {}", total_score_part_2(&rounds))
}

#[derive(PartialEq, Eq, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    fn points(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        }
    }

    fn my_shape_to_get_verdict(&self, verdict: &Verdict) -> Shape {
        match (self, verdict) {
            (Shape::Rock, Verdict::Draw) | (Shape::Paper, Verdict::Lose) | (Shape::Scissors, Verdict::Win) => Shape::Rock,
            (Shape::Scissors, Verdict::Draw) | (Shape::Rock, Verdict::Lose) | (Shape::Paper, Verdict::Win) => Shape::Scissors,
            _ => Shape::Paper        
        }
    }
}
enum Verdict {
    Win,
    Draw,
    Lose
}

impl Verdict {
    fn new(my_shape: &Shape, your_shape: &Shape) -> Self {
        match (my_shape, your_shape) {
            (Shape::Rock, Shape::Scissors) | (Shape::Paper, Shape::Rock) | (Shape::Scissors, Shape::Paper) => Verdict::Win,
            (Shape::Rock, Shape::Rock) | (Shape::Paper, Shape::Paper) | (Shape::Scissors, Shape::Scissors) => Verdict::Draw,
            _ => Verdict::Lose
        }
    }

    fn points(&self) -> usize {
        match self {
            Verdict::Win => 6,
            Verdict::Draw => 3,
            Verdict::Lose => 0
        }
    }
}

struct Round {
    my_shape_part_1: Shape,
    your_shape: Shape,
    verdict_part_2: Verdict
}

impl Round {
    fn new(line: &str) -> Self {
        let mut chars = line.chars().into_iter();
        let your_shape = chars.next().unwrap();
        chars.next();
        let my_shape_or_verdict = chars.next().unwrap();
        let (my_shape_part_1, verdict_part_2) = match my_shape_or_verdict {
            'X' => (Shape::Rock, Verdict::Lose),
            'Y' => (Shape::Paper, Verdict::Draw),
            'Z' => (Shape::Scissors, Verdict::Win),
            _ => panic!("Unexpected my selection {my_shape_or_verdict}")
        };
        let your_shape = match your_shape {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("Unexpected your selection {your_shape}")
        };
        Round {my_shape_part_1, your_shape, verdict_part_2}
    }
    
    fn score_part_1_a(&self) -> usize {
        match (self.my_shape_part_1, self.your_shape) {
            (Shape::Rock, Shape::Rock) => 3 + 1,
            (Shape::Rock, Shape::Paper) => 6 + 2,
            (Shape::Rock, Shape::Scissors) => 0 + 3,
            _ => unimplemented!()
        }
    }

    fn score_part_1_b(&self) -> usize {
        match self {
            Round { my_shape_part_1: Shape::Rock, your_shape: Shape::Rock, verdict_part_2: _ } => 3 + 1,
            Round { my_shape_part_1: Shape::Rock, your_shape: Shape::Paper, verdict_part_2: _ } => 6 + 2,
            Round { my_shape_part_1: Shape::Rock, your_shape: Shape::Scissors, verdict_part_2: _ } => 0 + 3,
            _ => unimplemented!()
        }
    }


    // fn score_part_1(&self) -> usize {
    //     Verdict::new(&self.my_shape_part_1, &self.your_shape).points() + &self.my_shape_part_1.points()
    // }

    fn score_part_2(&self) -> usize {
        let my_shape = &self.your_shape.my_shape_to_get_verdict(&self.verdict_part_2);
        self.verdict_part_2.points() + my_shape.points()
    }
}

fn parse_rounds(input: &str) -> Vec<Round> {
    input.lines().map(|line| Round::new(line)).collect()
}

fn total_score_part_1(rounds: &Vec<Round>) -> usize {
    rounds.iter().map(|round| round.score_part_1()).sum()
}

fn total_score_part_2(rounds: &Vec<Round>) -> usize {
    rounds.iter().map(|round| round.score_part_2()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let rounds = parse_rounds(include_str!("../test.txt"));
        assert_eq!(total_score_part_1(&rounds),  15)
    }

    #[test]
    fn test_part_2() {
        let rounds = parse_rounds(include_str!("../test.txt"));
        assert_eq!(total_score_part_2(&rounds),  12)
    }
}
