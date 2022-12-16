use std::collections::{HashMap, HashSet};

use utils::read_input;

// possible optimisations
// - avoid going round in cycles pointlessly
// - keep track of which valves might possibly be reachable from current valve - to give upper bound
//   on possible score from going to a given valve, which could be compared to current score for
//   paths to rule out some paths as definitely not worth pursuing

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    neighbours: Vec<&'a str>,
}

struct ValvePath<'a> {
    current_valve: &'a Valve<'a>,
    opened_valves: HashSet<&'a str>,
}

impl<'a> ValvePath<'a> {
    fn new(start_valve: &'a Valve) -> Self {
        Self {
            current_valve: start_valve,
            opened_valves: HashSet::new(),
        }
    }

    /// Get all the possible ways of extending the path within one minute.
    fn extend(mut self, valve_lookup: &'a HashMap<&str, Valve>) -> Vec<ValvePath<'a>> {
        // for each neighbour, include paths:
        // - where we move to the neighbour
        // - if current valve flow_rate > 0, and current valve is not already on, where we turn on current valve
        let mut extended_paths: Vec<_> = self
            .current_valve
            .neighbours
            .iter()
            .map(|neighb| ValvePath {
                current_valve: valve_lookup.get(neighb).unwrap(),
                opened_valves: self.opened_valves.clone(),
            })
            .collect();

        if !self.opened_valves.contains(self.current_valve.name) {
            self.opened_valves.insert(self.current_valve.name);
            extended_paths.push(ValvePath {
                current_valve: self.current_valve,
                opened_valves: self.opened_valves,
            });
        }

        extended_paths
    }

    fn score(&self, valve_lookup: &HashMap<&str, Valve>) -> u32 {
        self.opened_valves
            .iter()
            .map(|valve_name| valve_lookup.get(valve_name).unwrap().flow_rate)
            .sum()
    }
}

fn parse_valve(line: &str) -> (&str, Valve) {
    let parts: Vec<_> = line.split_ascii_whitespace().collect();
    let name = parts[1];
    let flow_rate = parts[4]
        .strip_prefix("rate=")
        .and_then(|rhs| rhs.strip_suffix(';'))
        .unwrap();
    let neighbours = parts[9..]
        .iter()
        .map(|neighb| neighb.trim_end_matches(','))
        .collect();
    (
        name,
        Valve {
            name,
            flow_rate: flow_rate.parse().unwrap(),
            neighbours,
        },
    )
}

fn main() {
    let input = read_input();
    let valve_lookup: HashMap<_, _> = input.lines().map(parse_valve).collect();
    let start_valve = valve_lookup.get("AA").unwrap();
    let starting_possible_paths = vec![ValvePath::new(start_valve)];
    let part_1_answer = (0..30)
        .fold(starting_possible_paths, |acc, minute| {
            println!("minute: {minute}, found paths: {}", acc.len());

            acc.into_iter()
                .flat_map(|path| path.extend(&valve_lookup).into_iter())
                .collect()
        })
        .iter()
        .map(|path| path.score(&valve_lookup))
        .max()
        .unwrap();

    println!("part 1: {part_1_answer}");
}
