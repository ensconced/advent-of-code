use std::collections::{HashMap, HashSet, VecDeque};

use utils::read_input;

#[derive(Debug, Default)]
struct NodeInfo {
    in_neighbours: Vec<(usize, usize)>,
    out_neighbours: Vec<(usize, usize)>,
}

struct Edge {
    source: (usize, usize),
    target: (usize, usize),
}

#[derive(Default)]
struct Graph(HashMap<(usize, usize), NodeInfo>);

impl Graph {
    fn add_edge(&mut self, from: (usize, usize), to: (usize, usize)) {
        self.0.entry(from).or_default().out_neighbours.push(to);
        self.0.entry(to).or_default().in_neighbours.push(from);
    }
    fn breadth_first_search<F: FnMut(&(usize, usize), &(usize, usize))>(
        &self,
        start: (usize, usize),
        mut visit: F,
    ) {
        let mut visited = HashSet::new();
        let mut queue: VecDeque<Edge> = VecDeque::new();
        let start_edges: Vec<_> = self
            .node_info(&start)
            .unwrap()
            .out_neighbours
            .iter()
            .map(|&target| Edge {
                source: start,
                target,
            })
            .collect();
        queue.extend(start_edges);

        while let Some(Edge { source, target }) = queue.pop_front() {
            visit(&source, &target);
            if !visited.contains(&target) {
                visited.insert(target);
                let neighbours: Vec<_> = self
                    .node_info(&target)
                    .unwrap()
                    .out_neighbours
                    .iter()
                    .map(|next_target| Edge {
                        source: target,
                        target: *next_target,
                    })
                    .collect();
                queue.extend(neighbours);
            }
        }
    }

    fn dijkstra(&mut self, start: (usize, usize)) -> HashMap<(usize, usize), u32> {
        let mut estimated_path_lengths: HashMap<(usize, usize), u32> =
            self.0.keys().map(|node| (*node, u32::MAX)).collect();

        estimated_path_lengths.insert(start, 0);

        self.breadth_first_search(start, |from_node, to_node| {
            let predecessor_path_length = *estimated_path_lengths.get(from_node).unwrap();
            estimated_path_lengths
                .entry(*to_node)
                .and_modify(|val| *val = u32::min(*val, predecessor_path_length + 1));
        });

        estimated_path_lengths
    }

    fn node_info(&self, node: &(usize, usize)) -> Option<&NodeInfo> {
        self.0.get(node)
    }
}

fn elevation(val: char) -> u32 {
    match val {
        'a'..='z' => val as u32,
        'S' => 'a' as u32,
        'E' => 'z' as u32,
        _ => panic!("unexpected val"),
    }
}

fn build_graph(grid: &[Vec<char>]) -> Graph {
    let mut adjacencies = Graph::default();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, val) in row.iter().enumerate() {
            let mut check_for_edge = |neighbour_row_idx: usize, neighbour_col_idx: usize| {
                if let Some(neighbour_row) = grid.get(neighbour_row_idx) {
                    if let Some(neighbour_val) = neighbour_row.get(neighbour_col_idx) {
                        if elevation(*neighbour_val) <= elevation(*val) + 1 {
                            adjacencies.add_edge(
                                (col_idx, row_idx),
                                (neighbour_col_idx, neighbour_row_idx),
                            );
                        }
                    }
                }
            };

            check_for_edge(row_idx + 1, col_idx);
            check_for_edge(row_idx, col_idx + 1);
            if row_idx > 0 {
                check_for_edge(row_idx - 1, col_idx);
            }
            if col_idx > 0 {
                check_for_edge(row_idx, col_idx - 1);
            }
        }
    }
    adjacencies
}

fn find_char_coords(ch: char, grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(col_idx, val)| (*val == ch).then_some((col_idx, row_idx)))
        })
        .collect()
}

fn main() {
    let grid: Vec<Vec<_>> = read_input()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let start = find_char_coords('S', &grid)[0];
    let end = find_char_coords('E', &grid)[0];
    let mut graph = build_graph(&grid);
    let path_lengths = graph.dijkstra(start);
    let part_1_answer = path_lengths.get(&end).unwrap();
    println!("part 1: {}", part_1_answer);

    // Floyd-Warshall might be faster but this does the job
    let mut all_start_points = find_char_coords('a', &grid);
    all_start_points.push(start);
    let part_2_answer = all_start_points
        .into_iter()
        .map(|start_point| {
            let path_lengths = graph.dijkstra(start_point);
            *path_lengths.get(&end).unwrap()
        })
        .min()
        .unwrap();
    println!("part 2: {}", part_2_answer);
}
