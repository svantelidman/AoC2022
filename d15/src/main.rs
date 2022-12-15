#![feature(drain_filter)]
use std::ops::RangeInclusive;

use regex::Regex;
fn main() {
    let sensors_and_beacons = parse_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&sensors_and_beacons, 2_000_000));
    println!("Part 2: {}", part_2(&sensors_and_beacons, 4_000_000));
}

fn parse_input(input: &str) -> Vec<((isize, isize), (isize, isize))> {
    let re = Regex::new(r"Sensor at x=(.\d*), y=(.\d*): closest beacon is at x=(.\d*), y=(.\d*)")
        .unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let xs = caps[1].parse::<isize>().unwrap();
            let ys = caps[2].parse::<isize>().unwrap();
            let xb = caps[3].parse::<isize>().unwrap();
            let yb = caps[4].parse::<isize>().unwrap();
            ((xs, ys), (xb, yb))
        })
        .collect::<Vec<_>>()
}

fn compute_merged_ranges_for_row(
    sensors_and_beacons: &Vec<((isize, isize), (isize, isize))>,
    row: isize,
) -> Vec<RangeInclusive<isize>> {
    sensors_and_beacons
        .iter()
        .map(|((sx, sy), (bx, by))| intersecting_range(*sx, *sy, *bx, *by, row))
        .fold(vec![], |acc, range| {
            if let Some(r) = range {
                merge_range_with_ranges(r, acc)
            } else {
                acc
            }
        })
}

fn part_1(sensors_and_beacons: &Vec<((isize, isize), (isize, isize))>, row: isize) -> usize {
    let merged_ranges = compute_merged_ranges_for_row(sensors_and_beacons, row);
    // The -1 is to account for the sensor position which for some reason shouldn't be counted
    merged_ranges
        .iter()
        .map(|r| (r.end() - r.start() + 1) as usize)
        .sum::<usize>()
        - 1
}

fn part_2(sensors_and_beacons: &Vec<((isize, isize), (isize, isize))>, max_coord: isize) -> usize {
    for row in 0..=max_coord {
        let merged_ranges = compute_merged_ranges_for_row(sensors_and_beacons, row);
        let pruned_ranges = prune_ranges(merged_ranges, max_coord);
        if let Some(gap) = find_gap(pruned_ranges, max_coord) {
            return (gap * 4_000_000 + row) as usize;
        }
    }
    panic!("Could not find beacon.")
}

fn find_gap(mut ranges: Vec<RangeInclusive<isize>>, max_coord: isize) -> Option<isize> {
    ranges.sort_by_key(|r| *r.start());
    let last_ind = ranges.len() - 1;
    if *ranges[0].start() > 0 {
        return Some(0);
    } else if *ranges[last_ind].end() < max_coord {
        return Some(max_coord);
    } else if last_ind == 0 {
        return None;
    }
    for ind in 0..last_ind {
        if *ranges[ind].end() + 1 < *ranges[ind + 1].start() {
            return Some(*ranges[ind].end() + 1);
        }
    }
    None
}

fn prune_ranges(
    mut ranges: Vec<RangeInclusive<isize>>,
    max_coord: isize,
) -> Vec<RangeInclusive<isize>> {
    ranges = ranges
        .into_iter()
        .filter(|r| *r.end() >= 0 || *r.start() <= max_coord)
        .map(|r| {
            if *r.start() >= 0 && *r.end() <= max_coord {
                r
            } else {
                0.max(*r.start())..=max_coord.min(*r.end())
            }
        })
        .collect();
    ranges
}

fn ranges_overlapping(r1: &mut RangeInclusive<isize>, r2: &RangeInclusive<isize>) -> bool {
    r1.contains(r2.start())
        || r1.contains(r2.end())
        || r2.contains(r1.start())
        || r2.contains(r1.end())
}

fn merge_range_with_ranges(
    range: RangeInclusive<isize>,
    mut ranges: Vec<RangeInclusive<isize>>,
) -> Vec<RangeInclusive<isize>> {
    let mut overlapping: Vec<_> = ranges
        .drain_filter(|r| ranges_overlapping(r, &range))
        .collect();
    if !overlapping.is_empty() {
        overlapping.push(range);
        let range_min = overlapping.iter().map(|r| r.start()).min().unwrap();
        let range_max = overlapping.iter().map(|r| r.end()).max().unwrap();
        ranges.push(*range_min..=*range_max);
    } else {
        ranges.push(range);
    }
    ranges
}

fn intersecting_range(
    sx: isize,
    sy: isize,
    bx: isize,
    by: isize,
    row: isize,
) -> Option<RangeInclusive<isize>> {
    let reach = (sx - bx).abs() + (sy - by).abs();
    let row_min = sy - reach;
    let row_max = sy + reach;
    if row < row_min || row > row_max {
        None
    } else {
        let intersection_half_length = reach - (sy - row).abs();
        Some((sx - intersection_half_length)..=(sx + intersection_half_length))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let sensors_and_beacons = parse_input(include_str!("../test.txt"));
        assert_eq!(part_1(&sensors_and_beacons, 10), 26)
    }

    #[test]
    fn test_part_2() {
        let sensors_and_beacons = parse_input(include_str!("../test.txt"));
        assert_eq!(part_2(&sensors_and_beacons, 20), 56000011)
    }
}
