use std::collections::HashMap;
use regex::Regex;

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", 0);
}

fn part_1(input: &str) -> usize {
    let monkeys = parse_monkeys(input);
    monkeys.get("root").unwrap().eval(&monkeys)
}

enum Monkey {
    AddingMonkey {
        first: String,
        second: String
    },
    SubtractingMonkey {
        first: String,
        second: String
    },
    DividingMonkey {
        first: String,
        second: String,
    },
    MultiplyingMonkey {
        first: String,
        second: String
    },
    NumberMonkey {
        number: usize
    }
}

impl Monkey {
    fn eval(&self, monkeys: &HashMap<String, Monkey>) -> usize {
        match self {
            Monkey::AddingMonkey { first, second } => {
                monkeys.get(first).unwrap().eval(monkeys) + monkeys.get(second).unwrap().eval(monkeys)
            },
            Monkey::SubtractingMonkey { first, second } => {
                monkeys.get(first).unwrap().eval(monkeys) - monkeys.get(second).unwrap().eval(monkeys)
            },
            Monkey::DividingMonkey { first, second } => {
                monkeys.get(first).unwrap().eval(monkeys) / monkeys.get(second).unwrap().eval(monkeys)
            },
            Monkey::MultiplyingMonkey { first, second } => {
                monkeys.get(first).unwrap().eval(monkeys) * monkeys.get(second).unwrap().eval(monkeys)
            },
            Monkey::NumberMonkey { number } => *number
        }
    }
}

fn parse_monkey(line: &str) -> (String, Monkey) {
    let re_arithmetic = Regex::new(r"([a-z]{4}): ([a-z]{4}) (.) ([a-z]{4})").unwrap();
    let re_shouting = Regex::new(r"([a-z]{4}): (\d+)").unwrap();
    if let Some(captures) = re_arithmetic.captures(line) {
        let label = String::from(&captures[1]);
        let first = String::from(&captures[2]);
        let second = String::from(&captures[4]);
        (label, match &captures[3] {
            "+" => Monkey::AddingMonkey { first, second },
            "-" => Monkey::SubtractingMonkey { first, second },
            "/" => Monkey::DividingMonkey { first, second },
            "*" => Monkey::MultiplyingMonkey { first, second },
            _ => panic!("Unknown monkey operation.") 
        })
    } else {
        let captures = re_shouting.captures(line).unwrap();
        (String::from(&captures[1]), Monkey::NumberMonkey { number: captures[2].parse().unwrap() }) 
    }

}

fn parse_monkeys(input: &str) -> HashMap<String, Monkey> {
    input.lines().map(|line| parse_monkey(line)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 152)
    }
}