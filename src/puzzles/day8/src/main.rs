use utils::read_input;

type Matrix<T> = Vec<Vec<T>>;

fn transpose<T: Copy>(grid: Matrix<T>) -> Matrix<T> {
    grid.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            (0..row.len())
                .map(|col_idx| grid[col_idx][row_idx])
                .collect()
        })
        .collect()
}

fn rotate<T: Copy>(grid: &Matrix<T>, n: u32) -> Matrix<T> {
    (0..n).fold(grid.clone(), |acc, _| {
        transpose(acc)
            .into_iter()
            .map(|row| row.into_iter().rev().collect())
            .collect()
    })
}

fn max_blocking_tree_heights(line: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut max = -1;
    for tree in line {
        result.push(max);
        max = i32::max(*tree, max);
    }
    result
}

fn visibility_from_left(grid: &Matrix<i32>) -> Matrix<bool> {
    let blockers: Vec<_> = grid.iter().map(max_blocking_tree_heights).collect();
    grid.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, tree)| *tree > blockers[row_idx][col_idx])
                .collect()
        })
        .collect()
}

fn main() {
    let input = read_input();
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| char::to_digit(ch, 10).unwrap() as i32)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let visibility_grid = (0..4)
        .map(|turns| rotate(&visibility_from_left(&rotate(&grid, turns)), 4 - turns))
        .reduce(|overall_vis_grid, side_vis_grid| {
            overall_vis_grid
                .into_iter()
                .enumerate()
                .map(|(row_idx, row)| {
                    row.into_iter()
                        .enumerate()
                        .map(|(col_idx, val)| val || side_vis_grid[row_idx][col_idx])
                        .collect()
                })
                .collect()
        })
        .unwrap();

    let part_1_answer = visibility_grid.into_iter().flatten().filter(|b| *b).count();
    println!("part 1: {}", part_1_answer);
}
