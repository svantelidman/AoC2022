use std::collections::HashSet;
fn main() {
    println!("Part 1: {}", start_position(include_str!("../input.txt"), 4));
    println!("Part 2: {}", start_position(include_str!("../input.txt"), 14))
}

fn start_position(s: &str, n: usize) -> usize {
    let mut n_received = n;
    loop {
        let test_str = &s[(n_received-n)..n_received];
        let test_set: HashSet<_> = test_str.chars().collect();
        if test_set.len() == n {
            break
        }
        n_received += 1;
    }
    n_received
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_part_1() {
        assert_eq!(start_position(include_str!("../test1.txt"), 4), 7)
    }
    #[test]
    fn test_2_part_1() {
        assert_eq!(start_position(include_str!("../test2.txt"), 4), 5)
    }
    #[test]
    fn test_3_part_1() {
        assert_eq!(start_position(include_str!("../test3.txt"), 4), 6)
    }
    #[test]
    fn test_4_part_1() {
        assert_eq!(start_position(include_str!("../test4.txt"), 4), 10)
    }
    #[test]
    fn test_5_part_1() {
        assert_eq!(start_position(include_str!("../test5.txt"), 4), 11)
    }

    #[test]
    fn test_21_part_2() {
        assert_eq!(start_position(include_str!("../test21.txt"), 14), 19)
    }
    #[test]
    fn test_22_part_2() {
        assert_eq!(start_position(include_str!("../test22.txt"), 14), 23)
    }
    #[test]
    fn test_23_part_2() {
        assert_eq!(start_position(include_str!("../test23.txt"), 14), 23)
    }
    #[test]
    fn test_24_part_2() {
        assert_eq!(start_position(include_str!("../test24.txt"), 14), 29)
    }
    #[test]
    fn test_25_part_2() {
        assert_eq!(start_position(include_str!("../test25.txt"), 14), 26)
    }
}