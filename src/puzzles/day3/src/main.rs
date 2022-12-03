use std::collections::HashSet;

use itertools::Itertools;

fn item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - 96,
        'A'..='Z' => (item as u32) - 38,
        _ => panic!("unexpected input"),
    }
}

fn single_common_character<'a>(strings: impl Iterator<Item = &'a str>) -> char {
    let sets = strings.map(|s| HashSet::from_iter(s.chars()));
    let common_chars: HashSet<_> = sets
        .reduce(|acc, set| HashSet::from_iter(acc.intersection(&set).cloned()))
        .unwrap_or_else(|| panic!("unexpected empty group of sets"));
    if common_chars.len() > 1 {
        panic!("found multiple common characters");
    }
    common_chars
        .into_iter()
        .next()
        .unwrap_or_else(|| panic!("found no common character"))
}

fn main() {
    let input = utils::read_input();

    let part_1_answer = input
        .lines()
        .map(|line| {
            let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
            let misplaced_item =
                single_common_character(vec![first_compartment, second_compartment].into_iter());
            item_priority(misplaced_item)
        })
        .sum::<u32>();

    let part_2_answer = input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|elf_group| item_priority(single_common_character(elf_group)))
        .sum::<u32>();

    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
}
