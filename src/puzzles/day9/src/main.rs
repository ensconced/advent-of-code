use std::ops::{Add, Sub};

use itertools::Itertools;
use utils::read_input;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Vector(i32, i32);

impl Add for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Sub for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

fn get_next_position_for_knot(
    current_knot_position: Vector,
    preceding_knot_position: Vector,
) -> Vector {
    let difference = preceding_knot_position - current_knot_position;
    let movement = if difference.0.abs() > 1 || difference.1.abs() > 1 {
        Vector(difference.0.signum(), difference.1.signum())
    } else {
        Vector(0, 0)
    };
    current_knot_position + movement
}

fn run_simulation(knot_count: usize) -> Vec<Vector> {
    let mut knots: Vec<_> = (0..knot_count).map(|_| Vector(0, 0)).collect();
    let mut tail_position_history = vec![knots[knots.len() - 1]];
    for line in read_input().lines() {
        let mut parts = line.split(' ');
        let head_direction = match parts.next().unwrap() {
            "U" => Vector(0, -1),
            "D" => Vector(0, 1),
            "L" => Vector(-1, 0),
            "R" => Vector(1, 0),
            _ => panic!("unexpected direction"),
        };
        let step_count = str::parse::<i32>(parts.next().unwrap()).unwrap();
        (0..step_count).for_each(|_| {
            knots[0] = knots[0] + head_direction;
            for knot_idx in 1..knot_count {
                knots[knot_idx] = get_next_position_for_knot(knots[knot_idx], knots[knot_idx - 1]);
            }
            tail_position_history.push(knots[knots.len() - 1]);
        });
    }
    tail_position_history
}

fn main() {
    let part_1_answer = run_simulation(2).into_iter().unique().count();
    let part_2_answer = run_simulation(10).into_iter().unique().count();
    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
}
