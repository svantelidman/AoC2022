use std::collections::HashMap;
use regex::Regex;

fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> usize {
    let monkeys = parse_monkeys(input);
    monkeys.get("root").unwrap().eval(&monkeys)
}

fn part_2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    if let Monkey::AddingMonkey { first, second } = monkeys.remove("root").unwrap() {
        monkeys.insert(String::from("root"), Monkey::EqualityMonkey { first, second});
    } else {
        panic!("Could not get root monkey.")
    }
    for my_shout in 3_243_420_000_000.. {
        let human = monkeys.get_mut("humn").unwrap();
        human.shout_this_number(my_shout);
        if monkeys.get("root").unwrap().eval(&monkeys) == 1 {
            return my_shout
        }
    }
    panic!("Could not find number to shout!")
}

enum Monkey {
    EqualityMonkey {
        first: String,
        second: String
    },
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
            Monkey::EqualityMonkey { first, second } => {
                let first_value  = monkeys.get(first).unwrap().eval(monkeys);
                let second_value = monkeys.get(second).unwrap().eval(monkeys);
                println!("1: {first_value}\n2: {second_value}");
                if  first_value == second_value {
                    1  // C-style true
                } else {
                    0  // C-style false
                }
            },
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
    
    fn shout_this_number(&mut self, number_to_shout: usize) {
        match self {
            Monkey::NumberMonkey { number } => *number = number_to_shout,
            _ => panic!("Only number monkeys can shout numbers.")
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