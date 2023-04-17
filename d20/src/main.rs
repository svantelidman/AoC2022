fn main() {
    let input = load_input(include_str!("../input.txt"));
    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));
}

#[derive(Clone)]
struct Value {
    value: isize,
    original_index: usize
}

fn mix(mut file: Vec<Value>) -> Vec<Value> {
    for original_ind in 0..file.len() {
        let current_ind = file.iter().position(|v| v.original_index == original_ind).unwrap();
        file = move_index(file, current_ind)
    }
    file
}

fn part_1(input: &Vec<isize>) -> isize {
    let file: Vec<_> = input.iter().enumerate().map(|(ind, v)| Value {value: *v, original_index: ind }).collect();
    let file = mix(file);
    grove_coordinates(&file).iter().sum()
}

fn part_2(input: &Vec<isize>) -> isize {
    let mut file: Vec<_> = input.iter().enumerate().map(|(ind, v)| Value {value: *v * 811589153, original_index: ind }).collect();
    for _ in 0..10 {
        file = mix(file)
    }
    grove_coordinates(&file).iter().sum()
}

fn move_index(mut file: Vec<Value>, current_ind: usize) -> Vec<Value> {
    let value_to_move = file.remove(current_ind);
    let delta = value_to_move.value;
    let new_ind = if delta < 0 {
        let delta = -delta % file.len() as isize;
        if delta as usize >= current_ind {
            file.len() - (delta as usize - current_ind)
        } else {
            current_ind - delta as usize
        }
    } else if delta > 0 {
        let delta = delta % file.len() as isize;
        if delta as usize + current_ind <= file.len() {
            current_ind + delta as usize
        } else {
            current_ind + delta as usize - file.len()
        }
    } else {
        current_ind
    };
    file.insert(new_ind, value_to_move);
    file
}


fn grove_coordinates(file: &Vec<Value>) -> Vec<isize> {
    let indexes_after_zero: [usize; 3] = [1000, 2000, 3000];
    let zero_index = file.iter().position(|v| v.value == 0).unwrap();
    let file_indexes: Vec<_> = indexes_after_zero.iter()
        .map(|ind| {
            (zero_index + ind) % file.len()
        })
        .collect();
    file_indexes.iter().map(|ind| file[*ind].value).collect()
}

fn load_input(input: &str) -> Vec<isize> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = load_input(include_str!("../test.txt"));
        assert_eq!(part_1(&input), 3)
    }

    #[test]
    fn test_part_2() {
        let input = load_input(include_str!("../test.txt"));
        assert_eq!(part_2(&input), 1623178306)
    }

    #[test]
    fn test_move() {
        let input: [isize; 3] = [1,2,3];

        let mut file: Vec<_> = input.iter().enumerate().map(|(ind, v)| Value {value: *v, original_index: ind }).collect();
        file = move_index(file, 0);
        let after_move = file.into_iter().map(|v|v.value).collect::<Vec<_>>();
        assert_eq!(after_move, vec![2isize,1,3]);

        let mut file: Vec<_> = input.iter().enumerate().map(|(ind, v)| Value {value: *v, original_index: ind }).collect();
        file = move_index(file, 1);
        let after_move = file.into_iter().map(|v|v.value).collect::<Vec<_>>();
        assert_eq!(after_move, vec![1isize,2,3]);

        let mut file: Vec<_> = input.iter().enumerate().map(|(ind, v)| Value {value: *v, original_index: ind }).collect();
        file = move_index(file, 2);
        let after_move = file.into_iter().map(|v|v.value).collect::<Vec<_>>();
        assert_eq!(after_move, vec![1isize,3,2]);

        let input: [isize; 3] = [-1,-2,-3];

        let mut file: Vec<_> = input.iter().enumerate().map(|(ind, v)| Value {value: *v, original_index: ind }).collect();
        file = move_index(file, 0);
        let after_move = file.into_iter().map(|v|v.value).collect::<Vec<_>>();
        assert_eq!(after_move, vec![-2isize,-1,-3]);

        let mut file: Vec<_> = input.iter().enumerate().map(|(ind, v)| Value {value: *v, original_index: ind }).collect();
        file = move_index(file, 1);
        let after_move = file.into_iter().map(|v|v.value).collect::<Vec<_>>();
        assert_eq!(after_move, vec![-1isize,-2,-3]);

        let mut file: Vec<_> = input.iter().enumerate().map(|(ind, v)| Value {value: *v, original_index: ind }).collect();
        file = move_index(file, 2);
        let after_move = file.into_iter().map(|v|v.value).collect::<Vec<_>>();
        assert_eq!(after_move, vec![-1isize,-3,-2]);

    }
}