use std::{collections::HashSet, fmt::Debug};

enum WindDirection {
    Left,
    Right,
}

#[derive(Clone)]
struct Rock {
    parts: HashSet<(i64, i64)>,
}

impl Rock {
    fn new(rock_shape: &[(i64, i64)], fallen_rocks: &FallenRocks) -> Self {
        let parts = rock_shape
            .iter()
            .map(|(x, y)| (x + 2, y + fallen_rocks.max_height + 4))
            .collect();

        Self { parts }
    }

    fn blow(&mut self, direction: WindDirection, fallen_rocks: &FallenRocks) {
        let shift = match direction {
            WindDirection::Left => -1,
            WindDirection::Right => 1,
        };

        let shifted_parts: HashSet<_> = self
            .parts
            .iter()
            .cloned()
            .map(|(x, y)| (x + shift, y))
            .collect();

        let can_shift = shifted_parts.iter().all(|(x, _)| *x >= 0 && *x < 7)
            && shifted_parts.intersection(&fallen_rocks.parts).count() == 0;

        if can_shift {
            self.parts = shifted_parts;
        }
    }

    fn drop(&mut self, fallen_rocks: &mut FallenRocks) -> bool {
        let dropped_parts: HashSet<_> = self
            .parts
            .iter()
            .cloned()
            .map(|(x, y)| (x, y - 1))
            .collect();
        let can_drop = dropped_parts.iter().all(|(_, y)| *y > 0)
            && dropped_parts.intersection(&fallen_rocks.parts).count() == 0;

        if can_drop {
            self.parts = dropped_parts;
        }
        can_drop
    }

    fn fall(
        &mut self,
        fallen_rocks: &mut FallenRocks,
        wind: &mut impl Iterator<Item = WindDirection>,
    ) {
        loop {
            self.blow(wind.next().unwrap(), fallen_rocks);
            if !self.drop(fallen_rocks) {
                fallen_rocks.add_rock(self);
                break;
            }
        }
    }
}

struct FallenRocks {
    parts: HashSet<(i64, i64)>,
    max_height: i64,
}

impl FallenRocks {
    fn new() -> Self {
        Self {
            max_height: 0,
            parts: HashSet::new(),
        }
    }

    fn add_rock(&mut self, rock: &Rock) {
        self.parts = self.parts.union(&rock.parts).cloned().collect();
        self.max_height = i64::max(
            self.max_height,
            rock.parts.iter().map(|(_, y)| *y).max().unwrap(),
        );
    }
}

impl Debug for FallenRocks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for row_idx in (0..=self.max_height).rev() {
            for col_idx in 0..=8 {
                if row_idx == 0 {
                    if col_idx == 0 || col_idx == 8 {
                        result.push('+');
                    } else {
                        result.push('-');
                    }
                } else if col_idx == 0 || col_idx == 8 {
                    result.push('|');
                } else if self.parts.contains(&(col_idx - 1, row_idx)) {
                    result.push('#');
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        result.push('\n');
        f.write_str(&result)
    }
}

fn main() {
    let rock_shapes = vec![
        vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        vec![(0, 0), (1, 0), (0, 1), (1, 1)],
    ];
    let repeated_rock_shapes = rock_shapes.iter().cycle();
    let mut wind = include_str!("../input.txt")
        .chars()
        .cycle()
        .map(|ch| match ch {
            '<' => WindDirection::Left,
            '>' => WindDirection::Right,
            _ => panic!("unexpected wind char"),
        });

    let mut fallen_rocks = FallenRocks::new();
    for rock_shape in repeated_rock_shapes.take(2022) {
        let mut rock = Rock::new(rock_shape, &fallen_rocks);
        rock.fall(&mut fallen_rocks, &mut wind);
    }

    let part_1_answer = fallen_rocks.max_height;
    println!("part 1: {part_1_answer}");
}
