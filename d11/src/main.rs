fn main() {
    let monkeys = parse_monkeys(include_str!("../input.txt"));
    println!("Part 1: {}", part_1_and_2(monkeys.clone(), 1));
    println!("Part 2: {}", part_1_and_2(monkeys, 2));
}

fn part_1_and_2(mut monkeys: Vec<Monkey>, part: usize) -> usize {
    let n_rounds = if part == 1 { 20 } else { 10_000 };
    let part_2_divisor = monkeys.iter().map(|m| m.test_value).product();
    for _ in 0..n_rounds {
        monkeys = monkey_round(monkeys, part, part_2_divisor)
    }
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.n_inspections).collect();
    inspections.sort();
    inspections.into_iter().rev().take(2).product()
}

fn monkey_round(mut monkeys: Vec<Monkey>, part: usize, part_2_divisor: usize) -> Vec<Monkey> {
    for ind in 0..monkeys.len() {
        for (new_item_value, target) in monkeys[ind].new_items_with_targets(part, part_2_divisor) {
            monkeys[target].items.push(new_item_value);
            monkeys[ind].n_inspections += monkeys[ind].items.len();
            monkeys[ind].items = vec!()
        }
    }
    monkeys
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test_value: usize,
    true_target: usize,
    false_target: usize,
    n_inspections: usize
}

impl Monkey {
    fn new_items_with_targets(&self, part: usize, part_2_divisor: usize) -> Vec<(usize, usize)>  { // (new_item_value, target)
        self.items.iter().map(|item| {
            let mut new_item_value =  if part == 1 {
                self.operation.apply(*item) / 3
            } else {
                self.operation.apply(*item)
            };
            let target = if new_item_value % self.test_value == 0 {
                self.true_target
            } else {
                self.false_target
            };
            new_item_value = if part == 1 {
                new_item_value
            } else {
                new_item_value % part_2_divisor
            };
            (new_item_value, target)
        }
        ).collect()
    }

    fn parse(s: &str) -> Self {
        let mut lines = s.lines();
        lines.next();
        let items_line = lines.next().unwrap();
        let items = items_line[18..].split(", ").map(|s| s.parse::<usize>().unwrap()).collect();
        let operation_line = lines.next().unwrap();
        let operation = if let Ok(value) = operation_line[25..].parse::<usize>() {
            match &operation_line[23..24] {
                "+" => Operation::Add{value},
                "*" => Operation::Multiply{value},
                _ => panic!("Unknown operation")
            }
        } else {
            Operation::Square
        };
        let test_line = lines.next().unwrap();
        let test_value = test_line[21..].parse::<usize>().unwrap();
        let true_line = lines.next().unwrap();
        let true_target = true_line[29..].parse::<usize>().unwrap();
        let false_line = lines.next().unwrap();
        let false_target = false_line[30..].parse::<usize>().unwrap();

        let n_inspections = 0;
        Monkey{ items, operation, test_value, true_target, false_target, n_inspections}
    }
}

#[derive(Clone)]
enum Operation {
    Add {value: usize},
    Multiply {value: usize},
    Square
}

impl Operation {
    fn apply(&self, item: usize) -> usize {
        match &self {
            Operation::Add{value} => item + value,
            Operation::Multiply{value} => item * value,
            Operation::Square => item * item      
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(|s| Monkey::parse(s)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let monkeys = parse_monkeys(include_str!("../test.txt"));
        assert_eq!(part_1_and_2(monkeys, 1), 10605)
    }

    #[test]
    fn test_part_2() {
        let monkeys = parse_monkeys(include_str!("../test.txt"));
        assert_eq!(part_1_and_2(monkeys, 2), 2713310158)
    }

}