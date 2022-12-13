use std::cmp::Ordering;
use std::iter::Peekable;
use std::str::Chars;

fn main() {
    let packet_pairs = load_packet_pairs(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&packet_pairs));
    println!("Part 2: {}", part_2(packet_pairs));
}

fn part_1(packet_pairs: &Vec<(PacketValue, PacketValue)>) -> usize {
    packet_pairs
        .iter()
        .enumerate()
        .map(|(ind, (p1, p2))| if p1 <= p2 { ind + 1 } else { 0 })
        .sum()
}

fn part_2(packet_pairs: Vec<(PacketValue, PacketValue)>) -> usize {
    let mut packets: Vec<_> = packet_pairs
        .into_iter()
        .map(|(p1, p2)| vec![p1, p2])
        .flatten()
        .collect();
    let divider_1 = PacketValue::parse(&mut "[[2]]".chars().peekable());
    let divider_2 = PacketValue::parse(&mut "[[6]]".chars().peekable());
    packets.push(divider_1.clone());
    packets.push(divider_2.clone());
    packets.sort();
    let div_1_index = packets.iter().position(|p| p == &divider_1).unwrap();
    let div_2_index = packets.iter().position(|p| p == &divider_2).unwrap();
    (div_1_index + 1) * (div_2_index + 1)
}

#[derive(Eq, Debug, Clone)]
enum PacketValue {
    Integer { value: usize },
    List { value: Vec<PacketValue> },
}

impl Ord for PacketValue {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            PacketValue::Integer { value: my_value } => match other {
                PacketValue::Integer { value: other_value } => my_value.cmp(other_value),
                PacketValue::List { value: _other_list } => PacketValue::List {
                    value: vec![PacketValue::Integer { value: *my_value }],
                }
                .cmp(other),
            },
            PacketValue::List { value: my_list } => match other {
                PacketValue::Integer { value: other_value } => self.cmp(&PacketValue::List {
                    value: vec![PacketValue::Integer {
                        value: *other_value,
                    }],
                }),
                PacketValue::List { value: other_list } => {
                    let min_length = my_list.len().min(other_list.len());
                    for ind in 0..min_length {
                        if my_list[ind] != other_list[ind] {
                            return my_list[ind].cmp(&other_list[ind]);
                        } else {
                            continue;
                        }
                    }
                    my_list.len().cmp(&other_list.len())
                }
            }
        }
    }
}

impl PartialOrd for PacketValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PacketValue {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl PacketValue {
    fn parse(s: &mut Peekable<Chars>) -> Self {
        if *s.peek().unwrap() == '[' {
            Self::parse_list(s)
        } else {
            Self::parse_integer(s)
        }
    }

    fn parse_integer(s: &mut Peekable<Chars>) -> PacketValue {
        let mut digits: Vec<char> = vec![];
        while s.peek().unwrap().is_numeric() {
            digits.push(s.next().unwrap())
        }
        let value = digits
            .into_iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap();
        PacketValue::Integer { value }
    }

    fn parse_list(s: &mut Peekable<Chars>) -> PacketValue {
        let mut value: Vec<PacketValue> = vec![];
        s.next();
        loop {
            match s.peek().unwrap() {
                ']' => {
                    s.next();
                    break;
                }
                ',' => {
                    s.next();
                    continue;
                }
                _ => value.push(PacketValue::parse(s)),
            }
        }
        PacketValue::List { value }
    }
}

fn load_packet_pairs(input: &str) -> Vec<(PacketValue, PacketValue)> {
    input
        .split("\n\n")
        .map(|line_pair| {
            let mut lines = line_pair.lines();
            (
                PacketValue::parse(&mut lines.next().unwrap().chars().peekable()),
                PacketValue::parse(&mut lines.next().unwrap().chars().peekable()),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let packet_pairs = load_packet_pairs(include_str!("../test.txt"));
        assert_eq!(part_1(&packet_pairs), 13)
    }

    #[test]
    fn test_part_2() {
        let packet_pairs = load_packet_pairs(include_str!("../test.txt"));
        assert_eq!(part_2(packet_pairs), 140)
    }
}
