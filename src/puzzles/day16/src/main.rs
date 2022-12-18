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
        debug: bool,
    ) -> u32 {
        let upper_bound_score_for_reachable_valves = shortest_paths
            .all_shortest_paths_from(self.current_valve.name)
            .unwrap()
            .iter()
            .filter(|(valve_name, _)| !self.open_valves.contains(**valve_name))
            .map(|(valve_name, path_length)| {
                let min_minute_to_open_valve = minute + path_length + 1;
                let max_minutes_of_flow = MINUTES - min_minute_to_open_valve;
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

const MINUTES: u32 = 30;

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
            let old_path_upper_bound =
                old_path.final_score_upper_bound(valve_lookup, shortest_paths, minute, false);
            let extended_paths = old_path.all_possible_extensions(minute, valve_lookup);
            for extended_path in extended_paths {
                let score_upper_bound = extended_path.final_score_upper_bound(
                    valve_lookup,
                    shortest_paths,
                    minute,
                    false,
                );

                assert!(score_upper_bound <= old_path_upper_bound);

                if score_upper_bound > self.max_score {
                    let extended_path_score = extended_path.score;
                    self.paths.push(extended_path);
                    self.max_score = u32::max(self.max_score, extended_path_score);
                }
            }
        }
    }

    fn contains(&self, valve_path: &ValvePath) -> bool {
        self.paths.iter().any(|path| path == valve_path)
    }
}

fn main() {
    let expected_current_valves = vec![
        Valve {
            name: "AA",
            flow_rate: 0,
            neighbours: HashSet::from(["DD", "II", "BB"]),
        },
        Valve {
            name: "DD",
            flow_rate: 20,
            neighbours: HashSet::from(["CC", "AA", "EE"]),
        },
        Valve {
            name: "DD",
            flow_rate: 20,
            neighbours: HashSet::from(["CC", "AA", "EE"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        Valve {
            name: "BB",
            flow_rate: 13,
            neighbours: HashSet::from(["CC", "AA"]),
        },
        Valve {
            name: "BB",
            flow_rate: 13,
            neighbours: HashSet::from(["CC", "AA"]),
        },
        Valve {
            name: "AA",
            flow_rate: 0,
            neighbours: HashSet::from(["DD", "II", "BB"]),
        },
        Valve {
            name: "II",
            flow_rate: 0,
            neighbours: HashSet::from(["AA", "JJ"]),
        },
        Valve {
            name: "JJ",
            flow_rate: 21,
            neighbours: HashSet::from(["II"]),
        },
        Valve {
            name: "JJ",
            flow_rate: 21,
            neighbours: HashSet::from(["II"]),
        },
        Valve {
            name: "II",
            flow_rate: 0,
            neighbours: HashSet::from(["AA", "JJ"]),
        },
        Valve {
            name: "AA",
            flow_rate: 0,
            neighbours: HashSet::from(["DD", "II", "BB"]),
        },
        Valve {
            name: "DD",
            flow_rate: 20,
            neighbours: HashSet::from(["CC", "AA", "EE"]),
        },
        Valve {
            name: "EE",
            flow_rate: 3,
            neighbours: HashSet::from(["FF", "DD"]),
        },
        Valve {
            name: "FF",
            flow_rate: 0,
            neighbours: HashSet::from(["EE", "GG"]),
        },
        Valve {
            name: "GG",
            flow_rate: 0,
            neighbours: HashSet::from(["FF", "HH"]),
        },
        Valve {
            name: "HH",
            flow_rate: 22,
            neighbours: HashSet::from(["GG"]),
        },
        Valve {
            name: "HH",
            flow_rate: 22,
            neighbours: HashSet::from(["GG"]),
        },
        Valve {
            name: "GG",
            flow_rate: 0,
            neighbours: HashSet::from(["FF", "HH"]),
        },
        Valve {
            name: "FF",
            flow_rate: 0,
            neighbours: HashSet::from(["EE", "GG"]),
        },
        Valve {
            name: "EE",
            flow_rate: 3,
            neighbours: HashSet::from(["FF", "DD"]),
        },
        Valve {
            name: "EE",
            flow_rate: 3,
            neighbours: HashSet::from(["FF", "DD"]),
        },
        Valve {
            name: "DD",
            flow_rate: 20,
            neighbours: HashSet::from(["CC", "AA", "EE"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
        Valve {
            name: "CC",
            flow_rate: 2,
            neighbours: HashSet::from(["DD", "BB"]),
        },
    ];

    let mut expected_paths = vec![
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![],
            current_valve: &expected_current_valves[0],
            open_valves: HashSet::new(),
            done: false,
            score: 0,
        },
        ValvePath {
            steps_since_opening_valve: 1,
            prev_steps: vec!["AA"],
            current_valve: &expected_current_valves[1],
            open_valves: HashSet::new(),
            done: false,
            score: 0,
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec!["AA"],
            current_valve: &expected_current_valves[2],
            open_valves: HashSet::from(["DD"]),
            done: false,
            score: 20 * 28,
        },
        ValvePath {
            steps_since_opening_valve: 1,
            prev_steps: vec!["AA", "DD"],
            current_valve: &expected_current_valves[3],
            open_valves: HashSet::from(["DD"]),
            done: false,
            score: 20 * 28,
        },
        ValvePath {
            steps_since_opening_valve: 2,
            prev_steps: vec!["AA", "DD", "CC"],
            current_valve: &expected_current_valves[4],
            open_valves: HashSet::from(["DD"]),
            done: false,
            score: 20 * 28,
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec!["AA", "DD", "CC"],
            current_valve: &expected_current_valves[5],
            open_valves: HashSet::from(["DD", "BB"]),
            done: false,
            score: 20 * 28 + 13 * 25,
        },
        ValvePath {
            steps_since_opening_valve: 1,
            prev_steps: vec!["AA", "DD", "CC", "BB"],
            current_valve: &expected_current_valves[6],
            open_valves: HashSet::from(["DD", "BB"]),
            done: false,
            score: 20 * 28 + 13 * 25,
        },
        ValvePath {
            steps_since_opening_valve: 2,
            prev_steps: vec!["AA", "DD", "CC", "BB", "AA"],
            current_valve: &expected_current_valves[7],
            open_valves: HashSet::from(["DD", "BB"]),
            done: false,
            score: 20 * 28 + 13 * 25,
        },
        ValvePath {
            steps_since_opening_valve: 3,
            prev_steps: vec!["AA", "DD", "CC", "BB", "AA", "II"],
            current_valve: &expected_current_valves[8],
            open_valves: HashSet::from(["DD", "BB"]),
            done: false,
            score: 20 * 28 + 13 * 25,
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec!["AA", "DD", "CC", "BB", "AA", "II"],
            current_valve: &expected_current_valves[9],
            open_valves: HashSet::from(["DD", "BB", "JJ"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21,
        },
        ValvePath {
            steps_since_opening_valve: 1,
            prev_steps: vec!["AA", "DD", "CC", "BB", "AA", "II", "JJ"],
            current_valve: &expected_current_valves[10],
            open_valves: HashSet::from(["DD", "BB", "JJ"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21,
        },
        ValvePath {
            steps_since_opening_valve: 2,
            prev_steps: vec!["AA", "DD", "CC", "BB", "AA", "II", "JJ", "II"],
            current_valve: &expected_current_valves[11],
            open_valves: HashSet::from(["DD", "BB", "JJ"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21,
        },
        ValvePath {
            steps_since_opening_valve: 3,
            prev_steps: vec!["AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA"],
            current_valve: &expected_current_valves[12],
            open_valves: HashSet::from(["DD", "BB", "JJ"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21,
        },
        ValvePath {
            steps_since_opening_valve: 4,
            prev_steps: vec!["AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD"],
            current_valve: &expected_current_valves[13],
            open_valves: HashSet::from(["DD", "BB", "JJ"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21,
        },
        ValvePath {
            steps_since_opening_valve: 5,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE",
            ],
            current_valve: &expected_current_valves[14],
            open_valves: HashSet::from(["DD", "BB", "JJ"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21,
        },
        ValvePath {
            steps_since_opening_valve: 6,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF",
            ],
            current_valve: &expected_current_valves[15],
            open_valves: HashSet::from(["DD", "BB", "JJ"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21,
        },
        ValvePath {
            steps_since_opening_valve: 7,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG",
            ],
            current_valve: &expected_current_valves[16],
            open_valves: HashSet::from(["DD", "BB", "JJ"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21,
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG",
            ],
            current_valve: &expected_current_valves[17],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17),
        },
        ValvePath {
            steps_since_opening_valve: 1,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
            ],
            current_valve: &expected_current_valves[18],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17),
        },
        ValvePath {
            steps_since_opening_valve: 2,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG",
            ],
            current_valve: &expected_current_valves[19],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17),
        },
        ValvePath {
            steps_since_opening_valve: 3,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF",
            ],
            current_valve: &expected_current_valves[20],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17),
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF",
            ],
            current_valve: &expected_current_valves[21],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21),
        },
        ValvePath {
            steps_since_opening_valve: 1,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE",
            ],
            current_valve: &expected_current_valves[22],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21),
        },
        ValvePath {
            steps_since_opening_valve: 2,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE", "DD",
            ],
            current_valve: &expected_current_valves[23],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21),
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE", "DD",
            ],
            current_valve: &expected_current_valves[24],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE", "CC"]),
            done: false,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21) + 2 * (30 - 24),
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE", "DD",
            ],
            current_valve: &expected_current_valves[25],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE", "CC"]),
            done: true,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21) + 2 * (30 - 24),
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE", "DD",
            ],
            current_valve: &expected_current_valves[26],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE", "CC"]),
            done: true,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21) + 2 * (30 - 24),
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE", "DD",
            ],
            current_valve: &expected_current_valves[27],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE", "CC"]),
            done: true,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21) + 2 * (30 - 24),
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE", "DD",
            ],
            current_valve: &expected_current_valves[28],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE", "CC"]),
            done: true,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21) + 2 * (30 - 24),
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE", "DD",
            ],
            current_valve: &expected_current_valves[29],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE", "CC"]),
            done: true,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21) + 2 * (30 - 24),
        },
        ValvePath {
            steps_since_opening_valve: 0,
            prev_steps: vec![
                "AA", "DD", "CC", "BB", "AA", "II", "JJ", "II", "AA", "DD", "EE", "FF", "GG", "HH",
                "GG", "FF", "EE", "DD",
            ],
            current_valve: &expected_current_valves[30],
            open_valves: HashSet::from(["DD", "BB", "JJ", "HH", "EE", "CC"]),
            done: true,
            score: 20 * 28 + 13 * 25 + 21 * 21 + 22 * (30 - 17) + 3 * (30 - 21) + 2 * (30 - 24),
        },
    ];

    let input = read_input();
    let valve_lookup: HashMap<_, _> = input.lines().map(parse_valve).collect();
    let shortest_paths = floyd_warshall_shortest_paths(&valve_lookup);

    let start_valve = valve_lookup.get("AA").unwrap();
    let mut paths = PathCollection::new(start_valve);

    assert!(paths.contains(&expected_paths[0]));

    for minute in 1..=MINUTES {
        println!("minute: {minute}");
        paths.extend(&shortest_paths, &valve_lookup, minute);
    }

    let part_1_answer = paths.max_score;
    println!("part 1: {part_1_answer}");
}
