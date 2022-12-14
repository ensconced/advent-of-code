use std::collections::HashSet;

use itertools::Itertools;
use utils::read_input;

fn add_vertical_section(x: i32, y1: i32, y2: i32, rocks: &mut HashSet<(i32, i32)>) {
    for y in y1..=y2 {
        rocks.insert((x, y));
    }
}

fn add_horizontal_section(y: i32, x1: i32, x2: i32, rocks: &mut HashSet<(i32, i32)>) {
    for x in x1..=x2 {
        rocks.insert((x, y));
    }
}

fn get_rock_positions() -> HashSet<(i32, i32)> {
    let input = read_input();
    let mut rocks = HashSet::new();
    for line in input.lines() {
        line.split(" -> ")
            .map(|section| {
                let coords: Vec<_> = section.split(',').collect();
                let x: i32 = str::parse(coords[0]).unwrap();
                let y: i32 = str::parse(coords[1]).unwrap();
                (x, y)
            })
            .tuple_windows()
            .for_each(|(section_start, section_end)| {
                if section_start.0 == section_end.0 {
                    add_vertical_section(
                        section_start.0,
                        i32::min(section_start.1, section_end.1),
                        i32::max(section_start.1, section_end.1),
                        &mut rocks,
                    );
                } else if section_start.1 == section_end.1 {
                    add_horizontal_section(
                        section_start.1,
                        i32::min(section_start.0, section_end.0),
                        i32::max(section_start.0, section_end.0),
                        &mut rocks,
                    );
                } else {
                    panic!("expected sections to be vertical or horizontal");
                }
            });
    }
    rocks
}

fn count_settled_grains(mut rocks: HashSet<(i32, i32)>, include_floor: bool) -> usize {
    let rocks_max_y = rocks.iter().max_by_key(|(_, y)| y).unwrap().1;
    let mut settled_sand_grains = 0;

    loop {
        let mut sand_position = (500, -1);
        let mut settled = false;
        loop {
            if !include_floor && sand_position.1 > rocks_max_y {
                return settled_sand_grains;
            } else if include_floor && sand_position.1 == rocks_max_y + 1 {
                settled = true;
            } else if !rocks.contains(&(sand_position.0, sand_position.1 + 1)) {
                sand_position = (sand_position.0, sand_position.1 + 1);
            } else if !rocks.contains(&(sand_position.0 - 1, sand_position.1 + 1)) {
                sand_position = (sand_position.0 - 1, sand_position.1 + 1);
            } else if !rocks.contains(&(sand_position.0 + 1, sand_position.1 + 1)) {
                sand_position = (sand_position.0 + 1, sand_position.1 + 1);
            } else {
                settled = true;
            }
            if settled {
                rocks.insert(sand_position);
                settled_sand_grains += 1;

                if sand_position == (500, 0) {
                    return settled_sand_grains;
                }
                break;
            }
        }
    }
}

fn main() {
    let rocks = get_rock_positions();
    let part_1_answer = count_settled_grains(rocks, false);
    println!("part 1: {}", part_1_answer);

    let rocks = get_rock_positions();
    let part_2_answer = count_settled_grains(rocks, true);
    println!("part 2: {}", part_2_answer);
}
