use std::{env, fs::read_to_string, path::Path};

use itertools::Itertools;

fn main() {
    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let input_path = Path::new(&cargo_dir).join("input.txt");
    let input = read_to_string(input_path).unwrap();

    let line_groups = input.lines().group_by(|line| !line.is_empty());
    let elf_items = line_groups
        .into_iter()
        .filter_map(|(not_empty, elf_lines)| not_empty.then_some(elf_lines));
    let elf_calorie_totals =
        elf_items.map(|items| items.fold(0, |acc, item| acc + str::parse::<u32>(item).unwrap()));
    let top_three: Vec<_> = elf_calorie_totals.sorted().rev().take(3).collect();

    let part_1_answer = top_three.first().cloned().unwrap_or(0);
    let part_2_answer = top_three.into_iter().sum::<u32>();
    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
}
