mod shortest_paths;
use std::collections::{HashMap, HashSet};

use shortest_paths::ShortestPaths;
use utils::read_input;

use crate::shortest_paths::floyd_warshall_shortest_paths;

// TODO - could tighten upper bound calculcation - assume all in a line...

#[derive(Debug)]
pub struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    neighbours: HashSet<&'a str>,
}

impl<'a> Valve<'a> {
    fn new(name: &'a str, flow_rate: &str, neighbours: impl Iterator<Item = &'a str>) -> Self {
        Self {
            name,
            flow_rate: flow_rate.parse().unwrap(),
            neighbours: neighbours.collect(),
        }
    }
}

#[derive(Clone, Debug)]
struct ValvePath<'a> {
    steps_since_opening_valve: usize,
    prev_steps: Vec<&'a str>,
    current_valve: &'a Valve<'a>,
    open_valves: HashSet<&'a str>,
    done: bool,
    score: u32,
}

impl<'a> ValvePath<'a> {
    fn new(start_valve: &'a Valve) -> Self {
        Self {
            steps_since_opening_valve: 0,
            prev_steps: vec![],
            current_valve: start_valve,
            open_valves: HashSet::new(),
            done: false,
            score: 0,
        }
    }

    fn ends_with_pointless_cycle(&self) -> bool {
        self.prev_steps
            .iter()
            .rev()
            .take(self.steps_since_opening_valve)
            .any(|el| el == &self.current_valve.name)
    }

    fn all_possible_extensions(
        mut self,
        minute: u32,
        valve_lookup: &'a HashMap<&str, Valve>,
    ) -> Vec<ValvePath<'a>> {
        if self.done {
            vec![self]
        } else {
            let mut extended_paths: Vec<_> = self
                .current_valve
                .neighbours
                .iter()
                .map(|neighbour| {
                    let mut prev_steps = self.prev_steps.clone();
                    prev_steps.push(self.current_valve.name);
                    ValvePath {
                        steps_since_opening_valve: self.steps_since_opening_valve + 1,
                        prev_steps,
                        current_valve: valve_lookup.get(neighbour).unwrap(),
                        open_valves: self.open_valves.clone(),
                        done: false,
                        score: self.score,
                    }
                })
                .filter(|path| !path.ends_with_pointless_cycle())
                .collect();

            if self.open_valves.contains(self.current_valve.name) {
                extended_paths.push(ValvePath {
                    steps_since_opening_valve: self.steps_since_opening_valve,
                    prev_steps: self.prev_steps,
                    current_valve: self.current_valve,
                    open_valves: self.open_valves,
                    done: true,
                    score: self.score,
                });
            } else {
                let score = self.score + self.current_valve.flow_rate * (MINUTES - minute);
                self.open_valves.insert(self.current_valve.name);
                extended_paths.push(ValvePath {
                    steps_since_opening_valve: 0,
                    prev_steps: self.prev_steps,
                    current_valve: self.current_valve,
                    open_valves: self.open_valves,
                    done: false,
                    score,
                });
            }
            extended_paths
        }
    }

    fn final_score_upper_bound(
        &self,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        shortest_paths: &ShortestPaths,
        minute: u32,
    ) -> u32 {
        let mut reachable_closed_valves: Vec<_> = shortest_paths
            .all_shortest_paths_from(self.current_valve.name)
            .unwrap()
            .iter()
            .filter(|(valve_name, _)| !self.open_valves.contains(**valve_name))
            .collect();

        reachable_closed_valves.sort_by_key(|(_, distance)| *distance);

        let upper_bound = reachable_closed_valves
            .into_iter()
            .enumerate()
            .map(|(idx, (valve_name, path_length))| {
                let min_minute_to_open_valve = minute + path_length + 1 + idx as u32;
                let max_minutes_of_flow = MINUTES - min_minute_to_open_valve;
                let flow_rate = valve_lookup.get(*valve_name).unwrap().flow_rate;
                flow_rate * max_minutes_of_flow
            })
            .sum::<u32>();
        upper_bound + self.score
    }
}

fn parse_valve(line: &str) -> (&str, Valve) {
    let parts: Vec<_> = line.split_ascii_whitespace().collect();
    let name = parts[1];
    let flow_rate = parts[4]
        .strip_prefix("rate=")
        .and_then(|rhs| rhs.strip_suffix(';'))
        .unwrap();
    let neighbours = parts[9..].iter().map(|neighb| neighb.trim_end_matches(','));
    (name, Valve::new(name, flow_rate, neighbours))
}

struct PathCollection<'a> {
    paths: Vec<ValvePath<'a>>,
    max_score: u32,
}

impl<'a> PathCollection<'a> {
    fn new(start_valve: &'a Valve) -> Self {
        let path = ValvePath::new(start_valve);
        let score = path.score;
        Self {
            paths: vec![path],
            max_score: score,
        }
    }

    fn extend(
        &mut self,
        shortest_paths: &ShortestPaths,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        minute: u32,
    ) {
        let old_paths = std::mem::take(&mut self.paths);
        for old_path in old_paths.into_iter() {
            let extended_paths = old_path.all_possible_extensions(minute, valve_lookup);
            for extended_path in extended_paths {
                let score_upper_bound =
                    extended_path.final_score_upper_bound(valve_lookup, shortest_paths, minute);

                if score_upper_bound > self.max_score {
                    let extended_path_score = extended_path.score;
                    self.paths.push(extended_path);
                    self.max_score = u32::max(self.max_score, extended_path_score);
                }
            }
        }
    }
}

const MINUTES: u32 = 30;

fn main() {
    let input = read_input();
    let valve_lookup: HashMap<_, _> = input.lines().map(parse_valve).collect();
    let shortest_paths = floyd_warshall_shortest_paths(&valve_lookup);

    let start_valve = valve_lookup.get("AA").unwrap();
    let mut paths = PathCollection::new(start_valve);

    for minute in 1..=MINUTES {
        println!("minute: {minute}");
        paths.extend(&shortest_paths, &valve_lookup, minute);
    }

    let part_1_answer = paths.max_score;
    println!("part 1: {part_1_answer}");
}
