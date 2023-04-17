
fn main() {
    println!("Part 1: {}", part_1(include_str!("../input.txt")));
    println!("Part 2: {}", 0);
}

fn part_1(input: &str) -> String {
    decimal_to_snafu(input.lines().map(|s| snafu_to_decimal(s)).sum())
}

fn snafu_to_decimal(snafu: &str) -> isize {
    snafu.chars().rev().enumerate()
        .fold(0, |acc, (ind, c)|
        {
            let delta = match c {
                '=' =>   - 5isize.pow(ind as u32) * 2,
                '-' =>  - 5isize.pow(ind as u32),
                '0' => 0,
                '1' => 5isize.pow(ind as u32),
                '2' => 5isize.pow(ind as u32) * 2,
                _ => panic!("Not a SNAFU digit.")
            };
            acc + delta
        }
    )
}

fn get_snafu_max_pow(value: isize) -> u32 {
    let abs_value = value.abs();
    let mut exp = 0;
    let mut acc_pow = 0;
    loop {
        let pow = 5isize.pow(exp);
        if abs_value <= 2*pow + acc_pow  {
            return exp
        }
        acc_pow += 2*pow;
        exp += 1;
    }
}

fn deduced_pow(original_pow: u32) -> isize {
    if original_pow == 0 {
        0
    } else {
        (0..=(original_pow - 1)).rev().map(|pow| 5isize.pow(pow) * 2).sum()
    }
}

fn decimal_to_snafu(decimal: isize) -> String {
    let mut snafu_digits: Vec<char> = vec![];
    let max_pow = get_snafu_max_pow(decimal as isize);
    let mut remaining = decimal;
    for pow in (0..=max_pow).rev() {
        let abs_remaining = remaining.abs();
        let sign_remaining = remaining.signum();
        let position_value = 5isize.pow(pow);
        snafu_digits.push(
            if abs_remaining < (position_value - deduced_pow(pow)) { // Måste vara så
                '0' 
            } else if abs_remaining < (position_value * 2 - deduced_pow(pow))  {
                if sign_remaining < 0 {
                    remaining += position_value;
                    '-'
                } else {
                    remaining -= position_value;
                    '1'
                }
            } else {
                if sign_remaining < 0 {
                    remaining += 2 * position_value;
                    '='
                } else {
                    remaining -= 2 * position_value;
                    '2'
                }
            }
        );
    }
    snafu_digits.into_iter().collect::<String>()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snafu_to_decimal() {
        assert_eq!(snafu_to_decimal("20"), 10);
        assert_eq!(snafu_to_decimal("1=-0-2"), 1747);
        assert_eq!(snafu_to_decimal("12111"), 906);
        assert_eq!(snafu_to_decimal("2=0="), 198);
    }

    #[test]
    fn test_decimal_to_snafu() {
        assert_eq!(decimal_to_snafu(-2), String::from("="));
        assert_eq!(decimal_to_snafu(-1), String::from("-"));
        assert_eq!(decimal_to_snafu(0), String::from("0"));
        assert_eq!(decimal_to_snafu(1), String::from("1"));
        assert_eq!(decimal_to_snafu(2), String::from("2"));
        assert_eq!(decimal_to_snafu(snafu_to_decimal("1=")), String::from("1="));
        assert_eq!(decimal_to_snafu(snafu_to_decimal("1-")), String::from("1-"));
        assert_eq!(decimal_to_snafu(snafu_to_decimal("10")), String::from("10"));
        assert_eq!(decimal_to_snafu(snafu_to_decimal("20")), String::from("20"));
        assert_eq!(decimal_to_snafu(6), String::from("11"));
        assert_eq!(decimal_to_snafu(snafu_to_decimal("12")), String::from("12"));
        assert_eq!(decimal_to_snafu(11), String::from("21"));
        assert_eq!(decimal_to_snafu(snafu_to_decimal("22")), String::from("22"));
        assert_eq!(decimal_to_snafu(10), String::from("20"));
        assert_eq!(decimal_to_snafu(8), String::from("2="));
        assert_eq!(decimal_to_snafu(906), String::from("12111"));
        assert_eq!(decimal_to_snafu(198), String::from("2=0="));
        assert_eq!(decimal_to_snafu(1747), String::from("1=-0-2"));
    }


    #[test]
    fn test_part_1() {
        assert_eq!(part_1(include_str!("../test.txt")), String::from("2=-1=0"))
    }
}