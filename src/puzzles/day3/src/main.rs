use std::collections::HashSet;

fn item_priority(item: char) -> u32 {
    match item {
        'a'..='z' => (item as u32) - 96,
        'A'..='Z' => (item as u32) - 38,
        _ => panic!("unexpected input"),
    }
}

fn compartment_members(compartment: &str) -> HashSet<char> {
    HashSet::from_iter(compartment.chars())
}

fn main() {
    let input = utils::read_input();
    let part_1_answer = input
        .lines()
        .map(|line| {
            let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
            let first_compartment_members = compartment_members(first_compartment);
            let second_compartment_members = compartment_members(second_compartment);
            let misplaced_items: Vec<_> = first_compartment_members
                .intersection(&second_compartment_members)
                .collect();
            if misplaced_items.len() > 1 {
                panic!("found multiple misplaced items");
            }
            let misplaced_item = misplaced_items
                .first()
                .unwrap_or_else(|| panic!("found no misplaced item"));
            item_priority(**misplaced_item)
        })
        .sum::<u32>();
    println!("part 1: {}", part_1_answer);
}
