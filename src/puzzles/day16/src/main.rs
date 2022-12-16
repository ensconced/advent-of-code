use std::{collections::HashMap, rc::Rc};

use utils::read_input;

// Valve QP has flow rate=0; tunnels lead to valves IS, DG

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: u32,
    neighbours: Vec<&'a str>,
}

fn parse_valve(line: &str) -> (&str, Valve) {
    let parts: Vec<_> = line.split_ascii_whitespace().collect();
    let name = parts[1];
    let flow_rate = parts[4]
        .strip_prefix("rate=")
        .and_then(|rhs| rhs.strip_suffix(';'))
        .unwrap();
    let neighbour = parts[9..].to_vec();
    (
        name,
        Valve {
            flow_rate: flow_rate.parse().unwrap(),
            neighbours: neighbour,
        },
    )
}

fn main() {
    let input = read_input();
    let valves: HashMap<_, _> = input.lines().map(parse_valve).collect();
    dbg!(&valves);
}
