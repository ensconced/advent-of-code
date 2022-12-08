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

fn rotate<T: Copy>(grid: Matrix<T>, n: u32) -> Matrix<T> {
    (0..n).fold(grid, |acc, _| {
        transpose(acc)
            .into_iter()
            .map(|row| row.into_iter().rev().collect())
            .collect()
    })
}

fn running_max(line: &Vec<i32>) -> Vec<i32> {
    let mut result = vec![];
    let mut max = -1;
    for tree in line {
        result.push(max);
        max = i32::max(*tree, max);
    }
    result
}

fn leftwards_blockers_height(grid: &Matrix<i32>) -> Matrix<i32> {
    grid.iter().map(running_max).collect()
}

fn visibility_from_left(grid: &Matrix<i32>) -> Matrix<bool> {
    let blockers = leftwards_blockers_height(grid);
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

    let visible_from_left = visibility_from_left(&grid);

    let rotated90 = rotate(grid, 1);
    let visible_from_bottom = rotate(visibility_from_left(&rotated90), 3);

    let rotated180 = rotate(rotated90, 1);
    let visible_from_right = rotate(visibility_from_left(&rotated180), 2);

    let rotated270 = rotate(rotated180, 1);
    let visible_from_top = rotate(visibility_from_left(&rotated270), 1);

    let part_1_answer = visible_from_left
        .into_iter()
        .enumerate()
        .fold(0, |acc, (row_idx, row)| {
            acc + row
                .into_iter()
                .enumerate()
                .filter(|(col_idx, visible_left)| {
                    *visible_left
                        || visible_from_bottom[row_idx][*col_idx]
                        || visible_from_right[row_idx][*col_idx]
                        || visible_from_top[row_idx][*col_idx]
                })
                .count()
        });

    println!("part 1: {}", part_1_answer);
}
