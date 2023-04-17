use std::collections::{HashSet, HashMap};
fn main() {
    let instructions = load_instructions(include_str!("../instructions.txt"));
    let map = load_map(include_str!("../map.txt"));
    println!("Part 1: {}", part_1(map, instructions));
    let instructions = load_instructions(include_str!("../instructions.txt"));
    let map = load_map2(include_str!("../map.txt"), false);
    println!("Part 2: {}", part_2(map, instructions));
}

fn part_2(map: Map2, instructions: Vec<Instruction>) -> usize {
    let (start_row, start_col) = map.get_start_position();
    let mut cursor = Cursor2::new(start_row, start_col);
    // map.print();
    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => {
                cursor.turn_left();
            },
            Instruction::TurnRight => {
                cursor.turn_right();
            },
            Instruction::Move {n_steps} => {
                let (new_row, new_col, new_direction, new_face) = map.find_next_position_face_and_direction(cursor.row, cursor.col, cursor.direction,  cursor.face, n_steps);
                cursor.row = new_row;
                cursor.col = new_col;
                cursor.face = new_face;
                cursor.direction = new_direction;
            }
        }
    }
    // println!("Face: {:?}", cursor.face);
    // println!("Direction: {:?}", cursor.direction);
    // println!("Row: {}", cursor.row);
    // println!("Col: {}", cursor.col);

    let (final_row, final_col) = map.original_coordinates(cursor.row, cursor.col, cursor.face);

    1_000 * (final_row + 1) + 4 * (final_col + 1) + cursor.direction.facing()
}


fn part_1(mut map: Map, instructions: Vec<Instruction>) -> usize {
    let (start_row, start_col) = map.get_start_position();
    let mut cursor = Cursor::new(start_row, start_col);
    map.update(cursor.row, cursor.col, cursor.direction);
    // map.print();
    for instruction in instructions {
        match instruction {
            Instruction::TurnLeft => {
                cursor.turn_left();
                map.update(cursor.row, cursor.col, cursor.direction);
            },
            Instruction::TurnRight => {
                cursor.turn_right();
                map.update(cursor.row, cursor.col, cursor.direction);
            },
            Instruction::Move {n_steps} => {
                let trail = map.find_trail(cursor.row, cursor.col, cursor.direction, n_steps);
                let (new_row, new_col) = trail.iter().last().unwrap();
                cursor.row = *new_row;
                cursor.col = *new_col;
                for (r, c) in trail {
                    map.update(r, c, cursor.direction);
                }
            }
        }
    }
    // map.print();
    1_000 * (cursor.row + 1) + 4 * (cursor.col + 1) + cursor.direction.facing()
}

struct Map {
    // (row = 0, col = 0) is in upper left 
    grid: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize
}

impl Map {
    fn update(&mut self, row: usize, col: usize, direction: Direction) {
        self.grid[row][col] = match direction {
            Direction::Right => '>',
            Direction::Left => '<',
            Direction::Up => '^',
            Direction::Down => 'v',
        }
    }


    fn get_start_position(&self) -> (usize, usize) {
        (0, self.grid[0].iter().position(|c| *c == '.').unwrap())
    }

    fn first_col_in_row(&self, row: usize) -> usize {
        for col in 0..self.n_cols {
            if self.grid[row][col] != ' ' {
                return col
            }
        }        
        panic!("Could not find first col in row.")
    }

    fn last_col_in_row(&self, row: usize) -> usize {
        for col in (0..self.n_cols).rev() {
            if self.grid[row][col] != ' ' {
                return col
            }
        }        
        panic!("Could not find last col in row.")
    }


    fn first_row_in_col(&self, col: usize) -> usize {
        for row in 0..self.n_rows {
            if self.grid[row][col] != ' ' {
                return row
            }
        }        
        panic!("Could not find first row in col.")
    }

    fn last_row_in_col(&self, col: usize) -> usize {
        for row in (0..self.n_rows).rev() {
            if self.grid[row][col] != ' ' {
                return row
            }
        }        
        panic!("Could not find last row in col.")
    }

    fn next_position(&self, current_row: usize, current_col: usize, direction: Direction) -> (usize, usize) {
        match direction {
            Direction::Right => {
                if current_col == self.n_cols  - 1 || self.grid[current_row][current_col + 1] == ' ' {
                    (current_row, self.first_col_in_row(current_row))
                } else {
                    (current_row, current_col + 1)
                }
            },
            Direction::Left => {
                if current_col == 0 || self.grid[current_row][current_col - 1] == ' ' {
                    (current_row, self.last_col_in_row(current_row))
                } else {
                    (current_row, current_col - 1)
                }
            },
            Direction::Up => {
                if current_row == 0 || self.grid[current_row - 1][current_col] == ' ' {
                    (self.last_row_in_col(current_col), current_col)
                } else {
                    (current_row - 1, current_col)
                }
            },
            Direction::Down => {
                if current_row == self.n_rows  - 1 || self.grid[current_row + 1][current_col] == ' ' {
                    (self.first_row_in_col(current_col), current_col)
                } else {
                    (current_row + 1, current_col)
                }
            },
        }
    }

    fn find_trail(&self, current_row: usize, current_col: usize, direction: Direction, n_steps: usize) -> Vec<(usize, usize)> {
        let mut trail: Vec<(usize, usize)> = vec![];
        let (mut current_row, mut current_col) = (current_row, current_col);
        trail.push((current_row, current_col));
        for _ in 0..n_steps {
            let (next_row, next_col) = self.next_position(current_row, current_col, direction);
            if self.grid[next_row][next_col] == '#' {
                return trail
            }
            (current_row, current_col) = (next_row, next_col);
            trail.push((current_row, current_col));
        }
        trail
    }

    fn print(&self) {
        println!();
        for row in &self.grid {
            println!("{}", row.iter().collect::<String>())
        }
    }
}

#[derive(Debug)]
enum Instruction {
    TurnLeft,
    TurnRight,
    Move {n_steps: usize}
}


#[derive(Clone, Copy, Debug)]
enum Direction {
    Left, Right, Up, Down
}

impl Direction {
    fn facing(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

struct Cursor {
    row: usize,
    col: usize,
    direction: Direction
}

struct Cursor2 {
    face: Face,
    row: usize,
    col: usize,
    direction: Direction
}

impl Cursor2 {
    fn new(row: usize, col: usize) -> Self {
        Self {row, col, direction: Direction::Right, face: Face::Top}
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left
        }
    }
}

impl Cursor {
    fn new(row: usize, col: usize) -> Self {
        Self {row, col, direction: Direction::Right}
    }

    fn turn_left(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Up,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right
        }
    }

    fn turn_right(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left
        }
    }
}

fn load_instructions(input: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut chars = input.chars().peekable();
    while chars.peek() != None {
        match chars.next() {
            None => break,
            Some('R') => instructions.push(Instruction::TurnRight),
            Some('L') => instructions.push(Instruction::TurnLeft),
            Some(digit) => {
                let mut digits = vec![digit];
                loop {
                    if let Some(c) = chars.peek() {
                        if c.is_digit(10) {
                            digits.push(*c);
                            chars.next();
                        } else {
                            break
                        }
                    } else {
                        break
                    }
                }
                let n_steps = digits.into_iter().collect::<String>().parse::<usize>().unwrap();
                instructions.push(Instruction::Move{n_steps})
            }
        }
    }
    instructions
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Face {
    Top, Bottom, Left, Right, Front, Back
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right
}

struct EdgeTransition {
    edges: HashSet<(Face, Edge)>,
    flip_dimensions: bool,
    invert_coordinates: bool
}

impl EdgeTransition {
    fn new(edge: (Face, Edge), other_edge: (Face, Edge), flip_dimensions: bool, invert_coordinates: bool ) -> Self {
        Self {
            edges: HashSet::from([edge, other_edge]),
            flip_dimensions,
            invert_coordinates
        }
    }
}

struct Map2 {
    faces: HashMap<Face, Vec<Vec<char>>>,
    face_positions: HashMap<Face, (usize, usize)>,
    edge_transitions: Vec<EdgeTransition>,
    n_cols: usize,
    n_rows: usize
}

impl Map2 {

    fn find_next_position_face_and_direction(&self, current_row: usize, current_col: usize, current_direction: Direction, current_face: Face, n_steps: usize) -> (usize, usize, Direction, Face) {

        let (mut current_row, mut current_col, mut current_direction, mut current_face) = (current_row, current_col, current_direction, current_face);
        for _ in 0..n_steps {
            let (next_row, next_col, next_direction, next_face) = self.next_position_face_and_direction(current_row, current_col, current_direction, current_face);
            if self.faces.get(&next_face).unwrap()[next_row][next_col] == '#' {
                return (current_row, current_col, current_direction, current_face)
            }
            (current_row, current_col, current_direction, current_face) = (next_row, next_col, next_direction, next_face)
        }
        (current_row, current_col, current_direction, current_face)
    }

    fn find_transition(&self, face: Face, edge: Edge) -> (Face, Edge, bool, bool) {
        let transition = self.edge_transitions.iter().find(|et| et.edges.contains(&(face, edge))).unwrap();
        let (other_face, other_edge) = transition.edges.iter().find(|e| **e != (face, edge)).unwrap();
        (*other_face, *other_edge, transition.flip_dimensions, transition.invert_coordinates)
    }

    fn determine_next_pos_and_direction(&self, current_row: usize, current_col: usize, exit_edge: Edge, entry_edge: Edge) -> (usize, usize, Direction)  {
        let max_coord = self.n_cols - 1;
        match exit_edge {
            Edge::Right => {
                match entry_edge {
                    Edge::Right => {
                        (max_coord - current_row, max_coord, Direction::Left)
                    },  
                    Edge::Left => {
                        (current_row, 0, Direction::Right)
                    },  
                    Edge::Top => {
                        (0, max_coord - current_row, Direction::Down)
                    },  
                    Edge::Bottom => {
                        (max_coord, current_row, Direction::Up)
                    },  
                }
            },
            Edge::Left => {
                match entry_edge {
                    Edge::Right => {
                        (current_row, max_coord, Direction::Left)
                    },  
                    Edge::Left => {
                        (max_coord - current_row, 0, Direction::Right)
                    },  
                    Edge::Top => {
                        (0, current_row, Direction::Down)
                    },  
                    Edge::Bottom => {
                        (max_coord, max_coord - current_row, Direction::Up)
                    },  
                }

            },
            Edge::Top => {
                match entry_edge {
                    Edge::Right => {
                        (max_coord - current_col, max_coord, Direction::Left)
                    },  
                    Edge::Left => {
                        (current_col, 0, Direction::Right)
                    },  
                    Edge::Top => {
                        (0, max_coord - current_col, Direction::Down)
                    },  
                    Edge::Bottom => {
                        (max_coord, current_col, Direction::Up)
                    },  
                }

            },
            Edge::Bottom => {
                match entry_edge {
                    Edge::Right => {
                        (current_col, max_coord, Direction::Left)
                    },  
                    Edge::Left => {
                        (max_coord - current_col, 0, Direction::Right)
                    },  
                    Edge::Top => {
                        (0, current_col, Direction::Down)
                    },  
                    Edge::Bottom => {
                        (max_coord, max_coord - current_col, Direction::Up)
                    },  
                }

            },
        }
    }

    fn next_position_face_and_direction(&self, current_row: usize, current_col: usize, current_direction: Direction, current_face: Face) -> (usize, usize, Direction, Face) {
        let current_grid = self.faces.get(&current_face).unwrap();
        match current_direction {
            Direction::Right => {
                if current_col == self.n_cols  - 1 || current_grid[current_row][current_col + 1] == ' ' {
                    let (new_face, entry_edge, _flip_dimensions, _invert_coordinates) = self.find_transition(current_face, Edge::Right);
                    let (new_row, new_col, new_direction) = self.determine_next_pos_and_direction(current_row, current_col, Edge::Right, entry_edge);
                    (new_row, new_col, new_direction, new_face)
                } else {
                    (current_row, current_col + 1, current_direction, current_face)
                }
            },
            Direction::Left => {
                if current_col == 0 || current_grid[current_row][current_col - 1] == ' ' {
                    let (new_face, entry_edge, _flip_dimensions, _invert_coordinates) = self.find_transition(current_face, Edge::Left);
                    let (new_row, new_col, new_direction) = self.determine_next_pos_and_direction(current_row, current_col, Edge::Left, entry_edge);
                    (new_row, new_col, new_direction, new_face)
                } else {
                    (current_row, current_col - 1, current_direction, current_face)
                }
            },
            Direction::Up => {
                if current_row == 0 || current_grid[current_row - 1][current_col] == ' ' {
                    let (new_face, entry_edge, _flip_dimensions, _invert_coordinates) = self.find_transition(current_face, Edge::Top);
                    let (new_row, new_col, new_direction) = self.determine_next_pos_and_direction(current_row, current_col, Edge::Top, entry_edge);
                    (new_row, new_col, new_direction, new_face)
                } else {
                    (current_row - 1, current_col, current_direction, current_face)
                }
            },
            Direction::Down => {
                if current_row == self.n_rows  - 1 || current_grid[current_row + 1][current_col] == ' ' {
                    let (new_face, entry_edge, _flip_dimensions, _invert_coordinates) = self.find_transition(current_face, Edge::Bottom);
                    let (new_row, new_col, new_direction) = self.determine_next_pos_and_direction(current_row, current_col, Edge::Bottom, entry_edge);
                    (new_row, new_col, new_direction, new_face)
                } else {
                    (current_row + 1, current_col, current_direction, current_face)
                }
            },
        }

    }

    fn get_start_position(&self) -> (usize, usize) {
        (0, self.faces.get(&Face::Top).unwrap()[0].iter().position(|c| *c == '.').unwrap())
    }    

    fn print(&self) {
        let max_row = *self.face_positions.iter().map(|(_, (m_r, _))| m_r).max().unwrap();
        let max_col = *self.face_positions.iter().map(|(_, (_, m_c))| m_c).max().unwrap();
        let mega_col = (max_col + 1) * self.n_cols;
        let mega_row = (max_row + 1) * self.n_rows;
        let mut grid: Vec<Vec<char>> = vec![];
        let mut grid_row: Vec<char> = vec![];
        grid_row.resize(mega_col, ' ');
        grid.resize(mega_row, grid_row);

        for (face, face_grid) in &self.faces {
            let (mega_row, mega_col) = self.face_positions.get(face).unwrap();
            let delta_row = mega_row * self.n_rows;
            let delta_col = mega_col * self.n_cols;
            for r in 0..self.n_rows {
                for c in 0..self.n_cols {
                    grid[delta_row + r][delta_col + c] = face_grid[r][c]
                }
            }
        }

        println!();
        for row in grid {
            println!("{}", row.iter().collect::<String>())
        }

    }
    
    fn original_coordinates(&self, face_row: usize, face_col: usize, face: Face) -> (usize, usize) {
        let (mega_row, mega_col) = self.face_positions.get(&face).unwrap();
        let delta_row = mega_row * self.n_rows;
        let delta_col = mega_col * self.n_cols;
        (delta_row + face_row, delta_col + face_col)
    }

}

fn load_map2(input: &str, test: bool) -> Map2 {
    let map = load_map(input);
    let face_dim = if test { 4 } else { 50 };
    let face_positions = if test {
        [
            (Face::Top,    (0, 2)),
            (Face::Bottom, (2, 2)),
            (Face::Left,   (1, 1)),
            (Face::Right,  (2, 3)),
            (Face::Front,  (1, 2)),
            (Face::Back,   (1, 0)),        
        ]
    } else {
        [
            (Face::Top,    (0, 1)),
            (Face::Bottom, (2, 1)),
            (Face::Left,   (2, 0)),
            (Face::Right,  (0, 2)),
            (Face::Front,  (1, 1)),
            (Face::Back,   (3, 0)),        
        ]
    };
    let faces = face_positions.iter()
    .map(|(face, (m_row, m_col))| {
        let start_row = m_row * face_dim;
        let start_col = m_col * face_dim;
        let end_row = start_row + face_dim;
        let end_col = start_col + face_dim;
        let mut face_grid: Vec<Vec<char>> = vec![];
        for row in start_row..end_row {
            let mut grid_row = vec![];
            for col in start_col..end_col {
                grid_row.push(map.grid[row][col])
            }
            face_grid.push(grid_row)
        }
        (*face, face_grid)
    }).collect();

    let edge_transitions = if test {
        vec![
            EdgeTransition::new((Face::Top, Edge::Right), (Face::Right, Edge::Right), false , true),
            EdgeTransition::new((Face::Top, Edge::Left), (Face::Left, Edge::Top), true, false),
            EdgeTransition::new((Face::Top, Edge::Top), (Face::Back, Edge::Top), false, true),
            EdgeTransition::new((Face::Top, Edge::Bottom), (Face::Front, Edge::Top), false, false),

            EdgeTransition::new((Face::Bottom, Edge::Right), (Face::Right, Edge::Left), false, false),
            EdgeTransition::new((Face::Bottom, Edge::Left), (Face::Left, Edge::Bottom), true, true),
            EdgeTransition::new((Face::Bottom, Edge::Top), (Face::Front, Edge::Bottom), false, false),
            EdgeTransition::new((Face::Bottom, Edge::Bottom), (Face::Back, Edge::Bottom), false, true),

            EdgeTransition::new((Face::Front, Edge::Right), (Face::Right, Edge::Top), true, true),
            EdgeTransition::new((Face::Front, Edge::Left), (Face::Left, Edge::Right), false, false),
            EdgeTransition::new((Face::Back, Edge::Left), (Face::Right, Edge::Bottom), true, true),
            EdgeTransition::new((Face::Back, Edge::Right), (Face::Left, Edge::Left), false, false),
        ]
    } else {
        vec![                                               // flip_dimensions, invert_coordinates
            EdgeTransition::new((Face::Top, Edge::Right), (Face::Right, Edge::Left), false, false),
            EdgeTransition::new((Face::Top, Edge::Left), (Face::Left, Edge::Left), false, true),
            EdgeTransition::new((Face::Top, Edge::Top), (Face::Back, Edge::Left), true, false),
            EdgeTransition::new((Face::Top, Edge::Bottom), (Face::Front, Edge::Top), false, false),

            EdgeTransition::new((Face::Bottom, Edge::Right), (Face::Right, Edge::Right), false, true),
            EdgeTransition::new((Face::Bottom, Edge::Left), (Face::Left, Edge::Right), false, false),
            EdgeTransition::new((Face::Bottom, Edge::Top), (Face::Front, Edge::Bottom), false, false),
            EdgeTransition::new((Face::Bottom, Edge::Bottom), (Face::Back, Edge::Right), true, false),

            EdgeTransition::new((Face::Front, Edge::Right), (Face::Right, Edge::Bottom), true, false),
            EdgeTransition::new((Face::Front, Edge::Left), (Face::Left, Edge::Top), true, false),
            EdgeTransition::new((Face::Back, Edge::Bottom), (Face::Right, Edge::Top), false, false),
            EdgeTransition::new((Face::Back, Edge::Top), (Face::Left, Edge::Bottom), false, false),
        ]
    };
    Map2{faces, edge_transitions, face_positions: face_positions.into_iter().collect(), n_rows: face_dim, n_cols: face_dim}

}

fn load_map(input: &str) -> Map {
    let mut grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let n_rows = grid.len();
    let n_cols = grid.iter().map(|row| row.len()).max().unwrap();
    grid = grid.into_iter().map(|mut row| {row.resize(n_cols, ' '); row}).collect();
    assert!(grid.iter().all(|row| row.len() == n_cols));
    Map {
        grid, n_rows, n_cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let instructions = load_instructions(include_str!("../test_instructions.txt"));
        let map = load_map(include_str!("../test_map.txt"));
        assert_eq!(part_1(map, instructions), 6032)
    }

    #[test]
    fn test_part_2() {
        let instructions = load_instructions(include_str!("../test_instructions.txt"));
        let map = load_map2(include_str!("../test_map.txt"), true);
        assert_eq!(part_2(map, instructions), 5031)
    }

}
