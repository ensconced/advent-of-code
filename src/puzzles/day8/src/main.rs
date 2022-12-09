use utils::read_input;

#[derive(Clone)]
struct Matrix<T>(Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    fn rows(&self) -> impl Iterator<Item = &Vec<T>> {
        self.0.iter()
    }

    fn transpose(&self) -> Matrix<T> {
        Matrix(
            self.rows()
                .enumerate()
                .map(|(row_idx, row)| {
                    (0..row.len())
                        .map(|col_idx| self.0[col_idx][row_idx])
                        .collect()
                })
                .collect(),
        )
    }

    fn rotate(&self, n: u32) -> Matrix<T> {
        (0..n % 4).fold(self.clone(), |acc, _| {
            Matrix(
                acc.transpose()
                    .rows()
                    .map(|row| row.iter().rev().cloned().collect())
                    .collect(),
            )
        })
    }

    fn transform_lines<U: Copy, F: Fn(&Vec<T>) -> Vec<U>>(
        &self,
        line_direction: u32,
        transform: F,
    ) -> Matrix<U> {
        let turns = line_direction % 4;
        Matrix(self.rotate(turns).rows().map(transform).collect()).rotate(4 - turns)
    }

    fn combine_elementwise<U: Copy, V, F: Fn(T, U) -> V>(
        &self,
        other: Matrix<U>,
        combine_elements: F,
    ) -> Matrix<V> {
        Matrix(
            self.rows()
                .enumerate()
                .map(|(row_idx, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(col_idx, val)| combine_elements(*val, other.0[row_idx][col_idx]))
                        .collect()
                })
                .collect(),
        )
    }
}

fn max_blocking_tree_heights(line: &Vec<u32>) -> Vec<Option<u32>> {
    let mut result = vec![];
    let mut max = None;
    for tree in line {
        result.push(max);
        max = max
            .map(|prev_max| u32::max(*tree, prev_max))
            .or(Some(*tree));
    }
    result
}

fn visibility_from_side(line_direction: u32, grid: &Matrix<u32>) -> Matrix<bool> {
    grid.transform_lines(line_direction, |line| {
        let blockers = max_blocking_tree_heights(line);
        line.iter()
            .enumerate()
            .map(|(col_idx, tree)| blockers[col_idx].map(|b| *tree > b).unwrap_or(true))
            .collect()
    })
}

fn scenic_scores_in_direction(line_direction: u32, grid: &Matrix<u32>) -> Matrix<u32> {
    grid.transform_lines(line_direction, |line| {
        let mut latest_blocker_positions_by_height = [None; 10];
        let mut result = vec![];
        for (tree_idx, tree_height) in line.iter().enumerate() {
            let score = latest_blocker_positions_by_height[*tree_height as usize..]
                .iter()
                .flatten()
                .max()
                .map(|latest_blocker_position| tree_idx - latest_blocker_position)
                .unwrap_or(tree_idx);
            latest_blocker_positions_by_height[*tree_height as usize] = Some(tree_idx);
            result.push(score as u32);
        }
        result
    })
}

fn main() {
    let input = read_input();
    let grid = Matrix(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| char::to_digit(ch, 10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    let visibility_grid = (0..4)
        .map(|line_direction| visibility_from_side(line_direction, &grid))
        .reduce(|overall_vis_grid, direction_vis_grid| {
            overall_vis_grid.combine_elementwise(direction_vis_grid, |a, b| a || b)
        })
        .unwrap();

    let part_1_answer = visibility_grid.rows().flatten().filter(|b| **b).count();

    let scenic_score_grid = (0..4)
        .map(|line_direction| scenic_scores_in_direction(line_direction, &grid))
        .reduce(|overall_scenic_score_grid, direction_scenic_score_grid| {
            overall_scenic_score_grid.combine_elementwise(direction_scenic_score_grid, |a, b| a * b)
        })
        .unwrap();

    let part_2_answer = scenic_score_grid.rows().flatten().max().unwrap();
    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
}
