fn main() {
    let prog = parse_prog(include_str!("../input.txt"));
    let (p1, p2) = part_1_and_2(&prog);
    println!("Part 1: {}", p1);
    println!("Part 2:\n{}", p2.into_iter().collect::<String>());
}

#[derive(Debug)]
enum OpCode {
    Addx {val: isize},
    Noop
}
#[derive(Debug)]
struct Operation {
    op: OpCode,
    cycles: isize
}


impl Operation {
    fn parse(s: &str) -> Self {
        match &s[0..4] {
            "addx" => {
                let val = s[5..].parse::<isize>().unwrap();
                Operation{op: OpCode::Addx{val}, cycles: 2}

            },
            "noop" => {
                Operation{op: OpCode::Noop, cycles: 1}
            }
            _ => panic!("Unknown OpCode: {}", &s[0..4])
        }
    }
}

fn parse_prog(input: &str) -> Vec<Operation> {
    input.lines().map(|line |Operation::parse(line)).collect()
}

fn part_1_and_2(prog: &Vec<Operation>) -> (isize, Vec<char>)  {
    let check_points: [isize; 6]= [20, 60,  100, 140, 180, 220];
    let mut screen: Vec<char> = vec![];
    let mut result: isize = 0;
    let mut x: isize = 1;
    let mut ins_ptr = 0;
    let mut remaining_op_cycles = prog[ins_ptr].cycles;
    for clock in 1..=240 {
        
        // Part 1 stuff
        if check_points.contains(&clock) {
            result += x * clock;
        }


        // Part 2 stuff
        let sprite_positions = [x -1, x, x +1];
        let scan_position = (clock - 1) % 40;
        screen.push(if sprite_positions.contains(&scan_position) {
            '#'
        } else {
            ' '
        });
        if scan_position == 39 {
            screen.push('\n')
        }

        // Common stuff
        remaining_op_cycles -= 1;
        if remaining_op_cycles == 0 {
            if let Operation{op: OpCode::Addx { val }, cycles: _} = prog[ins_ptr] {
                x += val
            }
            ins_ptr += 1;
            if ins_ptr < prog.len() {
                remaining_op_cycles = prog[ins_ptr].cycles;
            }
        }
    }
    (result, screen)
}
