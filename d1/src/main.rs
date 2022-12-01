fn main() {
    println!("Most calories: {}", part_1(include_str!("../input.txt")));
    println!("Total of top three calories: {}", part_2(include_str!("../input.txt")));
}

fn part_1(input: &str) -> usize {
    parse_calories(input).into_iter().max().unwrap()
}

fn part_2(input: &str) -> usize {
    let mut cals = parse_calories(input);
    cals.sort();
    let cals: Vec<_> = cals.into_iter().rev().collect();
    cals.into_iter().take(3).sum::<usize>()
}

fn parse_calories(input: &str) -> Vec<usize> {
    let mut cals: Vec<usize> = vec!();
    let mut cal_count = 0;
    for line in input.lines() {
        if let Ok(item_cal) = line.parse::<usize>() {
            cal_count += item_cal;
        } else {
            cals.push(cal_count);
            cal_count = 0;
        }
    }
    cals.push(cal_count);
    cals
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), 24000)
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2(include_str!("../test.txt")), 45000)
    }
}