use utils::read_input;

type TreeGrid = Vec<Vec<char>>;
type VisibilityGrid = Vec<Vec<bool>>;

// TODO - this isn't rotating...it's transposing...
fn rotate(grid: TreeGrid) -> TreeGrid {
    grid.iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, _)| grid[col_idx][row_idx])
                .collect()
        })
        .collect()
}

fn running_max(line: &Vec<char>) -> Vec<char> {
    let result = vec![];
    let mut max = '0';
    for tree in line {
        max = char::max(*tree, max);
    }
    result
}

fn leftwards_blockers_height(grid: &TreeGrid) -> TreeGrid {
    grid.iter().map(running_max).collect()
}

fn visibility_from_left(grid: &TreeGrid) -> VisibilityGrid {
    let blockers = leftwards_blockers_height(&grid);
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
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let visible_from_left = visibility_from_left(&grid);
    let rotated90 = rotate(grid);
    let visible_from_top = visibility_from_left(&rotated90);
    let rotated180 = rotate(rotated90);
    let visible_from_right = visibility_from_left(&rotated180);
    let rotated270 = rotate(rotated180);
    let visible_from_bottom = visibility_from_left(&rotated270);
    // TODO - OR all the visiblity grid together to get the overall visibility
}
