use itertools::Itertools;
use std::vec;

struct MoveOp {
    count: u32,
    source_stack: usize,
    dest_stack: usize,
}

fn prepare_initial_stacks(lines: &Vec<&str>) -> Vec<Vec<String>> {
    let mut stacks = vec![];
    for line in lines {
        let chunks = line.chars().into_iter().chunks(4);
        let line_crates = chunks
            .into_iter()
            .map(|mut chunk| chunk.join("")[1..2].to_owned());

        for (stack_idx, item) in line_crates.enumerate() {
            if stacks.len() <= stack_idx {
                stacks.push(vec![]);
            }
            if item != " " {
                stacks[stack_idx].push(item);
            }
        }
    }
    for stack in stacks.iter_mut() {
        stack.reverse();
    }
    stacks
}

fn parse_move_operations(move_operations_lines: &[&str]) -> Vec<MoveOp> {
    move_operations_lines
        .iter()
        .map(|line| {
            let parts: Vec<_> = line.split_ascii_whitespace().collect();
            MoveOp {
                count: parts[1]
                    .parse()
                    .unwrap_or_else(|_| panic!("failed to parse count")),
                source_stack: parts[3]
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse source stack"))
                    - 1,
                dest_stack: parts[5]
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("failed to parse dest stack"))
                    - 1,
            }
        })
        .collect()
}

fn execute_move_operations(
    stacks: &mut [Vec<String>],
    move_ops: &Vec<MoveOp>,
    move_multiple_crates_at_once: bool,
) {
    for move_op in move_ops {
        if move_multiple_crates_at_once {
            let mut vals = vec![];
            for _ in 0..move_op.count {
                let val = stacks[move_op.source_stack]
                    .pop()
                    .unwrap_or_else(|| panic!("unexpected empty stack"));
                vals.push(val);
            }
            vals.reverse();
            stacks[move_op.dest_stack].extend(vals);
        } else {
            for _ in 0..move_op.count {
                let val = stacks[move_op.source_stack]
                    .pop()
                    .unwrap_or_else(|| panic!("unexpected empty stack"));
                stacks[move_op.dest_stack].push(val);
            }
        }
    }
}

fn top_stack_items(
    initial_stack_lines: &Vec<&str>,
    move_operations: &Vec<MoveOp>,
    move_multiple_crates_at_once: bool,
) -> String {
    let mut stacks = prepare_initial_stacks(initial_stack_lines);
    execute_move_operations(&mut stacks, move_operations, move_multiple_crates_at_once);
    stacks
        .iter()
        .map(|stack| {
            stack
                .last()
                .unwrap_or_else(|| panic!("unexpected empty stack"))
        })
        .join("")
}

fn main() {
    let input = utils::read_input();
    let mut initial_stack_lines = vec![];
    let mut move_operation_lines = vec![];
    for line in input.lines() {
        if line.trim().starts_with('[') {
            initial_stack_lines.push(line);
        } else if line.starts_with("move") {
            move_operation_lines.push(line);
        }
    }

    let move_operations = parse_move_operations(&move_operation_lines);

    let part_1_answer = top_stack_items(&initial_stack_lines, &move_operations, false);
    let part_2_answer = top_stack_items(&initial_stack_lines, &move_operations, true);

    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
}
