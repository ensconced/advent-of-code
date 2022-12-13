use std::collections::HashMap;

use utils::read_input;

fn elevation(val: char) -> char {
    match val {
        'a'..='z' => val,
        'S' => 'a',
        'E' => 'z',
        _ => panic!("unexpected val"),
    }
}

fn generate_adjacencies(input: String) -> HashMap<(usize, usize), Vec<(usize, usize)>> {
    let grid: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut adjacencies: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            let mut check_for_edge = |neighbour_row_idx: usize, neighbour_col_idx: usize| {
                if let Some(neighbour_row) = grid.get(neighbour_row_idx) {
                    if let Some(neighbour_val) = neighbour_row.get(neighbour_col_idx) {
                        if elevation(*neighbour_val) >= elevation(*val) {
                            adjacencies
                                .entry((row_idx, col_idx))
                                .or_default()
                                .push((neighbour_row_idx, neighbour_col_idx));
                        }
                    }
                }
            };

            check_for_edge(row_idx + 1, col_idx);
            check_for_edge(row_idx, col_idx + 1);
            check_for_edge(row_idx - 1, col_idx);
            check_for_edge(row_idx, col_idx - 1);
        }
    }
    adjacencies
}

fn main() {
    let adjacencies = generate_adjacencies(read_input());
    // TODO -
    // topologically sort DAG
    // initialise d values - no need to track pi values
    // iterate through DAG and all incoming edges for each node
    // return fully relaxed value for end node
}
