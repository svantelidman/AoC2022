use std::{collections::HashSet, hash::Hash};

fn main() {
    let cubes = parse_cubes(include_str!("../input.txt"));
    let p1 = part_1(&cubes);
    println!("Part 1: {}", p1);
    println!("Part 2: {}", p1 - part_2(cubes));
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Cube {
    x: isize,
    y: isize,
    z: isize
}

impl Cube {
    fn from_vec(vec: Vec<isize>) -> Self {
        Cube{x: vec[0], y: vec[1], z: vec[2]}
    }

    fn surface_codes(&self) -> HashSet<(isize, isize, isize)> {
        let x2 = self.x * 2;
        let y2 = self.y * 2;
        let z2 = self.z * 2;
        vec!(
            (x2 + 1, y2, z2),
            (x2 - 1, y2, z2),
            (x2, y2 + 1, z2),
            (x2, y2 - 1, z2),
            (x2, y2, z2 + 1),
            (x2, y2, z2 - 1),
        ).into_iter().collect()
    }

    fn is_adjacent(&self, other: &Cube) -> bool {
        self.y == other.y && self.z == other.z && (self.x - other.x).abs() == 1 ||
        self.x == other.x && self.z == other.z && (self.y - other.y).abs() == 1 ||
        self.x == other.x && self.y == other.y && (self.z - other.z).abs() == 1
    }
} 

fn parse_cubes(input: &str) -> Vec<Cube> {
    input.lines().map(|s| {
        let vec: Vec<isize> = s.split(',').map(|s| s.parse::<isize>().unwrap()).collect();
        Cube::from_vec(vec)
    }).collect()
}

fn calculate_surface_size(cubes: &Vec<Cube>) -> usize {
    let mut outer_surface: HashSet<(isize, isize, isize)> = HashSet::new();
    for cube in cubes {
        let cube_surfaces = cube.surface_codes();
        let common_surfaces: HashSet<_> = outer_surface.intersection(&cube_surfaces).map(|s| *s).collect();
        outer_surface = outer_surface.union(&cube_surfaces).map(|s| *s).collect();
        outer_surface = outer_surface.difference(&common_surfaces).map(|s| *s).collect();
    }
    outer_surface.len()

}

fn part_1(cubes: &Vec<Cube>) -> usize {
    calculate_surface_size(cubes)
}


fn part_2(cubes: Vec<Cube>) -> usize {
    let (xmin, xmax, ymin, ymax, zmin, zmax) = cubes.iter().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN, isize::MAX, isize::MIN), |(xmin, xmax, ymin, ymax, zmin, zmax), cube|
            (xmin.min(cube.x), xmax.max(cube.x), ymin.min(cube.y), ymax.max(cube.y), zmin.min(cube.z), zmax.max(cube.z)));
    let mut extent_cubes: HashSet<Cube> = HashSet::new();
    for x in (xmin-1)..=(xmax+1) {
        for y in (ymin-1)..=(ymax+1) {
            for z in (zmin-1)..=(zmax+1) {
                extent_cubes.insert(Cube{x, y, z});
            }
        }
    }
    let cube_set: HashSet<Cube> = cubes.into_iter().collect();
    let empty_space_cubes = extent_cubes.difference(&cube_set).map(|c| *c).collect();
    let distinct_empty_spaces = find_distinct_bodies(&empty_space_cubes);

    // Filter out the surrounding space
    let hollows: Vec<_> = distinct_empty_spaces.into_iter().filter(|space| !space.contains(&Cube{x: xmin - 1, y: ymin - 1, z: zmin - 1})).collect();
    hollows.iter().map(|hollow| calculate_surface_size(hollow)).sum()
}

fn find_distinct_bodies(cubes: &Vec<Cube>) -> Vec<Vec<Cube>> {
    let mut distinct_bodies: Vec<Vec<Cube>> = vec![];
    for cube in cubes {
        let (mergable, non_mergable): (Vec<_>, Vec<_>) = distinct_bodies.into_iter().partition(|body| body.iter().any(|body_cube| body_cube.is_adjacent(&cube)));
        let mut merged: Vec<_> = mergable.into_iter().flatten().collect();
        distinct_bodies = non_mergable;
        merged.push(cube.clone());
        distinct_bodies.push(merged)
    }
    distinct_bodies
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_surface_corners() {
    //     let cube_1 = Cube{x: 1, y: 1, z: 1};
    //     let surface_codes_1 = cube_1.surface_codes();
    //     let cube_corners_1: HashSet<_> = surface_codes_1.into_iter().map(|sc| surface_corners(&sc).into_iter()).flatten().collect();
    //     let cube_2 = Cube{x: 2, y: 2, z: 2};
    //     let surface_codes_2 = cube_2.surface_codes();
    //     let cube_corners_2: HashSet<_> = surface_codes_2.into_iter().map(|sc| surface_corners(&sc).into_iter()).flatten().collect();
    //     assert_eq!(cube_corners_1.intersection(&cube_corners_2).count(), 1)
    // }

    // #[test]
    // fn test_part_2_simple() {
    //     let xmin = 1;
    //     let xmax = 1;
    //     let ymin = 1;
    //     let ymax = 1;
    //     let zmin = 1;
    //     let zmax = 1;
        
    //     let mut outer_cubes: HashSet<Cube> = HashSet::new();

    //     for x in (xmin-1)..=(xmax+1) {
    //         for y in (ymin-1)..=(ymax+1) {
    //             for z in (zmin-1)..=(zmax+1) {
    //                 outer_cubes.insert(Cube{x, y, z});
    //             }
    //         }
    //     }
    //     let cube_set: HashSet<Cube> = HashSet::new();//cubes.into_iter().collect();
    //     let outer_cubes = outer_cubes.difference(&cube_set).map(|c| *c).collect();
    //     let distinct_bodies = find_distinct_bodies(&outer_cubes);
    //     println!("{:?}", distinct_bodies.iter().map(|body| body.len()).collect::<Vec<_>>());
    //     assert_eq!(distinct_bodies.len(), 1)

    // }

    // #[test]
    // fn test_part_1() {
    //     let cubes = parse_cubes(include_str!("../test.txt"));
    //     assert_eq!(part_1(&cubes), 64)
    // }

    #[test]
    fn test_part_2() {
        let cubes = parse_cubes(include_str!("../input.txt"));
        assert_eq!(part_2(cubes), 7)
    }
}