use itertools::iproduct;
fn main() {
    let trees = load_trees(include_str!("../input.txt"));
    println!("Part 1: {}", count_visible(&trees));
    println!("Part 2: {}", highest_scenic_score(&trees));
}

fn load_trees(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn count_visible(trees: &Vec<Vec<usize>>) -> usize {
    let tree_cols = as_cols(trees);
    let n_cols = trees[0].len();
    let n_rows = trees.len();
    (0..n_rows)
        .into_iter()
        .map(|ir| {
            (0..n_cols)
                .into_iter()
                .filter(|ic| is_visible(trees, &tree_cols, ir, *ic))
                .count()
        })
        .sum()
}

fn highest_scenic_score(trees: &Vec<Vec<usize>>) -> usize {
    let tree_cols = as_cols(trees);
    let n_rows = trees.len();
    let n_cols = trees[0].len();
    iproduct!(0..n_rows, 0..n_cols)
        .into_iter()
        .map(|(ic, ir)| scenic_score(trees, &tree_cols, ir, ic))
        .max()
        .unwrap()
}

fn in_outer_line_of_sight(trees: &Vec<usize>, ind: usize) -> bool {
    if ind == 0 || ind == trees.len() - 1 {
        true
    } else {
        *trees[0..ind].iter().max().unwrap() < trees[ind]
            || *trees[ind + 1..].iter().max().unwrap() < trees[ind]
    }
}

fn as_cols(rows: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n_cols = rows[0].len();
    let n_rows = rows.len();
    (0..n_cols)
        .into_iter()
        .map(|ic| {
            (0..n_rows)
                .into_iter()
                .map(|ir| rows[ir][ic])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_visible(rows: &Vec<Vec<usize>>, cols: &Vec<Vec<usize>>, row: usize, col: usize) -> bool {
    in_outer_line_of_sight(&rows[row], col) || in_outer_line_of_sight(&cols[col], row)
}

fn scenic_score(rows: &Vec<Vec<usize>>, cols: &Vec<Vec<usize>>, row: usize, col: usize) -> usize {
    line_of_sight_score(&rows[row], col) * line_of_sight_score(&cols[col], row)
}

fn line_of_sight_score(trees: &Vec<usize>, ind: usize) -> usize {
    let upper_length = trees.len() - ind - 1;
    let lower_length = ind;
    let hi_score = if let Some(pos) = trees[ind + 1..].iter().position(|t| t >= &trees[ind]) {
        pos + 1
    } else {
        upper_length
    };
    let lo_score = if let Some(pos) = trees[0..ind].iter().rev().position(|t| t >= &trees[ind]) {
        pos + 1
    } else {
        lower_length
    };
    hi_score * lo_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let trees = load_trees(include_str!("../test.txt"));
        assert_eq!(count_visible(&trees), 21)
    }

    #[test]
    fn test_part_2() {
        let trees = load_trees(include_str!("../test.txt"));
        assert_eq!(highest_scenic_score(&trees), 8)
    }
}
