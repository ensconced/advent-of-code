use std::collections::HashSet;

use itertools::Itertools;
use utils::read_input;

fn add_vertical_section(x: u32, y1: u32, y2: u32, rocks: &mut HashSet<(u32, u32)>) {
    for y in y1..=y2 {
        rocks.insert((x, y));
    }
}

fn add_horizontal_section(y: u32, x1: u32, x2: u32, rocks: &mut HashSet<(u32, u32)>) {
    for x in x1..=x2 {
        rocks.insert((x, y));
    }
}

fn get_rock_positions() -> HashSet<(u32, u32)> {
    let input = read_input();
    let mut rocks = HashSet::new();
    for line in input.lines() {
        line.split(" -> ")
            .map(|section| {
                let coords: Vec<_> = section.split(',').collect();
                let x: u32 = str::parse(coords[0]).unwrap();
                let y: u32 = str::parse(coords[1]).unwrap();
                (x, y)
            })
            .tuple_windows()
            .for_each(|(section_start, section_end)| {
                if section_start.0 == section_end.0 {
                    add_vertical_section(
                        section_start.0,
                        u32::min(section_start.1, section_end.1),
                        u32::max(section_start.1, section_end.1),
                        &mut rocks,
                    );
                } else if section_start.1 == section_end.1 {
                    add_horizontal_section(
                        section_start.1,
                        u32::min(section_start.0, section_end.0),
                        u32::max(section_start.0, section_end.0),
                        &mut rocks,
                    );
                } else {
                    panic!("expected sections to be vertical or horizontal");
                }
            });
    }
    rocks
}

fn count_settled_grains(mut rocks: HashSet<(u32, u32)>) -> usize {
    let rocks_max_y = rocks.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut settled_sand_grains = 0;

    loop {
        let mut sand_position = (500, 0);
        loop {
            if sand_position.1 > rocks_max_y {
                return settled_sand_grains;
            } else if !rocks.contains(&(sand_position.0, sand_position.1 + 1)) {
                sand_position = (sand_position.0, sand_position.1 + 1);
            } else if !rocks.contains(&(sand_position.0 - 1, sand_position.1 + 1)) {
                sand_position = (sand_position.0 - 1, sand_position.1 + 1);
            } else if !rocks.contains(&(sand_position.0 + 1, sand_position.1 + 1)) {
                sand_position = (sand_position.0 + 1, sand_position.1 + 1);
            } else {
                rocks.insert(sand_position);
                settled_sand_grains += 1;
                break;
            }
        }
    }
}

fn main() {
    let rocks = get_rock_positions();
    let part_1_answer = count_settled_grains(rocks);
    println!("part 1: {}", part_1_answer);
}
