mod shortest_paths;
use std::collections::{HashMap, HashSet};

use shortest_paths::ShortestPaths;
use utils::read_input;

use crate::shortest_paths::floyd_warshall_shortest_paths;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
                // .filter(|path| !path.ends_with_pointless_cycle())
                .collect();

            if self.open_valves.contains(self.current_valve.name) {
                // extended_paths.push(ValvePath {
                //     steps_since_opening_valve: self.steps_since_opening_valve,
                //     prev_steps: self.prev_steps,
                //     current_valve: self.current_valve,
                //     open_valves: self.open_valves,
                //     done: true,
                //     score: self.score,
                // });
            } else {
                let score = self.score + self.current_valve.flow_rate * (30 - minute);
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
        let upper_bound_score_for_reachable_valves = shortest_paths
            .all_shortest_paths_from(self.current_valve.name)
            .unwrap()
            .iter()
            .filter(|(valve_name, _)| !self.open_valves.contains(**valve_name))
            .map(|(valve_name, path_length)| {
                let min_minute_to_open_valve = minute + path_length + 1;
                let max_minutes_of_flow = 30 - min_minute_to_open_valve;
                let flow_rate = valve_lookup.get(*valve_name).unwrap().flow_rate;
                flow_rate * max_minutes_of_flow
            })
            .sum::<u32>();
        upper_bound_score_for_reachable_valves + self.score
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

fn main() {
    let input = read_input();
    let valve_lookup: HashMap<_, _> = input.lines().map(parse_valve).collect();
    let shortest_paths = floyd_warshall_shortest_paths(&valve_lookup);

    let start_valve = valve_lookup.get("AA").unwrap();
    let mut paths = PathCollection::new(start_valve);
    for minute in 1..=30 {
        println!("minute: {minute}");
        paths.extend(&shortest_paths, &valve_lookup, minute);
    }

    let part_1_answer = paths.max_score;
    println!("part 1: {part_1_answer}");
}

#[test]
fn first_minute() {
    let input = read_input();
    let valve_lookup: HashMap<_, _> = input.lines().map(parse_valve).collect();
    let shortest_paths = floyd_warshall_shortest_paths(&valve_lookup);

    let start_valve = valve_lookup.get("AA").unwrap();
    let original_path = ValvePath::new(start_valve);
    assert_eq!(
        original_path,
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![],
            current_valve: &Valve {
                name: "AA",
                flow_rate: 0,
                neighbours: HashSet::from(["DD", "II", "BB"])
            },
            open_valves: HashSet::new(),
            done: false,
            score: 0,
        }
    );

    // move to valve D...

    let expected_second_stage = ValvePath {
        steps_since_opening_valve: 1,
        prev_steps: vec!["AA"],
        current_valve: &Valve {
            name: "DD",
            flow_rate: 20,
            neighbours: HashSet::from(["CC", "AA", "EE"]),
        },
        open_valves: HashSet::new(),
        done: false,
        score: 0,
    };

    assert!(original_path
        .all_possible_extensions(1, &valve_lookup)
        .into_iter()
        .any(|el| el == expected_second_stage));

    // open valve D...

    let expected_third_stage = ValvePath {
        steps_since_opening_valve: 0,
        prev_steps: vec!["AA"],
        current_valve: &Valve {
            name: "DD",
            flow_rate: 20,
            neighbours: HashSet::from(["CC", "AA", "EE"]),
        },
        open_valves: HashSet::from(["DD"]),
        done: false,
        score: 20 * 28,
    };

    let all_possible_third_stages = expected_second_stage.all_possible_extensions(2, &valve_lookup);
    assert!(all_possible_third_stages
        .into_iter()
        .any(|el| el == expected_third_stage));

    // move to valve C...

    let expected_fourth_stage = ValvePath {
        steps_since_opening_valve: 1,
        prev_steps: vec!["AA", "DD"],
        current_valve: &Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        open_valves: HashSet::from(["DD"]),
        done: false,
        score: 20 * 28,
    };

    let all_possible_fourth_stages = expected_third_stage.all_possible_extensions(3, &valve_lookup);
    assert!(all_possible_fourth_stages
        .into_iter()
        .any(|el| el == expected_fourth_stage));

    // move to valve B...

    let expected_fifth_stage = ValvePath {
        steps_since_opening_valve: 2,
        prev_steps: vec!["AA", "DD", "CC"],
        current_valve: &Valve {
            name: "BB",
            flow_rate: 13,
            neighbours: HashSet::from(["CC", "AA"]),
        },
        open_valves: HashSet::from(["DD"]),
        done: false,
        score: 20 * 28,
    };

    let all_possible_fourth_stages =
        expected_fourth_stage.all_possible_extensions(4, &valve_lookup);
    assert!(all_possible_fourth_stages
        .into_iter()
        .any(|el| el == expected_fifth_stage));

    // open valve B...

    let expected_sixth_stage = ValvePath {
        steps_since_opening_valve: 0,
        prev_steps: vec!["AA", "DD", "CC"],
        current_valve: &Valve {
            name: "BB",
            flow_rate: 13,
            neighbours: HashSet::from(["CC", "AA"]),
        },
        open_valves: HashSet::from(["DD", "BB"]),
        done: false,
        score: 20 * 28 + 13 * 25,
    };

    let all_possible_sixth_stages = expected_fifth_stage.all_possible_extensions(5, &valve_lookup);
    assert!(all_possible_sixth_stages
        .into_iter()
        .any(|el| el == expected_sixth_stage));

    // move to valve A

    let expected_seventh_stage = ValvePath {
        steps_since_opening_valve: 1,
        prev_steps: vec!["AA", "DD", "CC", "BB"],
        current_valve: &Valve {
            name: "AA",
            flow_rate: 0,
            neighbours: HashSet::from(["DD", "II", "BB"]),
        },
        open_valves: HashSet::from(["DD", "BB"]),
        done: false,
        score: 20 * 28 + 13 * 25,
    };

    let all_possible_seventh_stages =
        expected_sixth_stage.all_possible_extensions(6, &valve_lookup);
    assert!(all_possible_seventh_stages
        .into_iter()
        .any(|el| el == expected_seventh_stage));

    // move to valve II

    let expected_eighth_stage = ValvePath {
        steps_since_opening_valve: 2,
        prev_steps: vec!["AA", "DD", "CC", "BB", "AA"],
        current_valve: &Valve {
            name: "II",
            flow_rate: 0,
            neighbours: HashSet::from(["AA", "JJ"]),
        },
        open_valves: HashSet::from(["DD", "BB"]),
        done: false,
        score: 20 * 28 + 13 * 25,
    };

    let all_possible_eighth_stages =
        expected_seventh_stage.all_possible_extensions(7, &valve_lookup);
    assert!(all_possible_eighth_stages
        .into_iter()
        .any(|el| el == expected_eighth_stage));

    // move to valve AA

    let expected_ninth_stage = ValvePath {
        steps_since_opening_valve: 3,
        prev_steps: vec!["AA", "DD", "CC", "BB", "AA", "II"],
        current_valve: &Valve {
            name: "AA",
            flow_rate: 0,
            neighbours: HashSet::from(["DD", "II", "BB"]),
        },
        open_valves: HashSet::from(["DD", "BB"]),
        done: false,
        score: 20 * 28 + 13 * 25,
    };

    let all_possible_h_stages = expected_eighth_stage.all_possible_extensions(8, &valve_lookup);
    assert!(all_possible_h_stages
        .into_iter()
        .any(|el| el == expected_ninth_stage));

    // original_path
    //     .all_possible_extensions(1, &valve_lookup)
    //     .into_iter()
    //     .any(|valve_path| valve_path == ValvePath {});
}
