use utils::read_input;

#[derive(Clone)]
struct Matrix<T>(Vec<Vec<T>>);

impl<T: Copy> Matrix<T> {
    fn transpose(&self) -> Matrix<T> {
        Matrix(
            self.0
                .iter()
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
                    .map(|row| row.into_iter().rev().collect())
                    .collect(),
            )
        })
    }

    fn transform_lines<S: Copy, F: Fn(&Vec<T>) -> Vec<S>>(
        &self,
        line_direction: u32,
        transform: F,
    ) -> Matrix<S> {
        let turns = line_direction % 4;
        Matrix(self.rotate(turns).0.iter().map(transform).collect()).rotate(4 - turns)
    }

    fn rows(self) -> impl Iterator<Item = Vec<T>> {
        self.0.into_iter()
    }
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

fn visibility_from_side(line_direction: u32, grid: &Matrix<i32>) -> Matrix<bool> {
    grid.transform_lines(line_direction, |row| {
        let blockers = max_blocking_tree_heights(row);
        row.iter()
            .enumerate()
            .map(|(col_idx, tree)| *tree > blockers[col_idx])
            .collect()
    })
}

fn main() {
    let input = read_input();
    let grid = Matrix(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|ch| char::to_digit(ch, 10).unwrap() as i32)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    let visibility_grid = (0..4)
        .map(|line_direction| visibility_from_side(line_direction, &grid))
        .reduce(|overall_vis_grid, side_vis_grid| {
            Matrix(
                overall_vis_grid
                    .rows()
                    .enumerate()
                    .map(|(row_idx, row)| {
                        row.into_iter()
                            .enumerate()
                            .map(|(col_idx, val)| val || side_vis_grid.0[row_idx][col_idx])
                            .collect()
                    })
                    .collect(),
            )
        })
        .unwrap();

    let part_1_answer = visibility_grid.rows().flatten().filter(|b| *b).count();
    println!("part 1: {}", part_1_answer);

    // TODO - use running array of length 10 to lookup latest idx of trees of given height (array of options, to account for "none" case, I think)
}
