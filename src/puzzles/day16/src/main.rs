use std::collections::{HashMap, HashSet};

use utils::read_input;

#[derive(Debug)]
struct Valve<'a> {
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

#[derive(Debug)]
struct ValvePath<'a> {
    steps_since_opening_valve: usize,
    prev_steps: Vec<&'a str>,
    current_valve: &'a Valve<'a>,
    valves_opened_when: HashMap<&'a str, u32>,
    done: bool,
    score: u32,
}

impl<'a> ValvePath<'a> {
    fn new(start_valve: &'a Valve) -> Self {
        Self {
            steps_since_opening_valve: 0,
            prev_steps: vec![],
            current_valve: start_valve,
            valves_opened_when: HashMap::new(),
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
        let next_score = self.next_score(valve_lookup);
        if self.done {
            self.score = next_score;
            vec![self]
        } else {
            let mut extended_paths: Vec<_> = self
                .current_valve
                .neighbours
                .iter()
                .map(|neighb| {
                    let mut prev_steps = self.prev_steps.clone();
                    prev_steps.push(self.current_valve.name);
                    ValvePath {
                        steps_since_opening_valve: self.steps_since_opening_valve + 1,
                        prev_steps,
                        current_valve: valve_lookup.get(neighb).unwrap(),
                        valves_opened_when: self.valves_opened_when.clone(),
                        done: false,
                        score: next_score,
                    }
                })
                .filter(|path| !path.ends_with_pointless_cycle())
                .collect();

            if self
                .valves_opened_when
                .contains_key(self.current_valve.name)
            {
                extended_paths.push(ValvePath {
                    steps_since_opening_valve: self.steps_since_opening_valve,
                    prev_steps: self.prev_steps,
                    current_valve: self.current_valve,
                    valves_opened_when: self.valves_opened_when,
                    done: true,
                    score: next_score,
                });
            } else {
                self.valves_opened_when
                    .insert(self.current_valve.name, minute);
                extended_paths.push(ValvePath {
                    steps_since_opening_valve: 0,
                    prev_steps: self.prev_steps,
                    current_valve: self.current_valve,
                    valves_opened_when: self.valves_opened_when,
                    done: false,
                    score: next_score,
                });
            }
            extended_paths
        }
    }

    fn next_score(&self, valve_lookup: &'a HashMap<&'a str, Valve>) -> u32 {
        let score_for_this_minute = self
            .valves_opened_when
            .keys()
            .map(|open_valve_name| valve_lookup.get(open_valve_name).unwrap().flow_rate)
            .sum::<u32>();

        self.score + score_for_this_minute
    }

    fn final_score_upper_bound(
        &self,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        shortest_paths: &ShortestPaths,
        minute: u32,
    ) -> u32 {
        let scores_we_may_be_able_to_get = shortest_paths
            .all_shortest_paths_from(self.current_valve.name)
            .map(|hashmap| {
                hashmap
                    .iter()
                    .filter(|(valve_name, _)| !self.valves_opened_when.contains_key(**valve_name))
                    .map(|(valve_name, path_length)| {
                        let min_minute_to_open_valve = minute + path_length + 1;
                        let max_minutes_of_flow = 30 - min_minute_to_open_valve;
                        let flow_rate = valve_lookup.get(*valve_name).unwrap().flow_rate;
                        flow_rate * max_minutes_of_flow
                    })
                    .sum::<u32>()
            });
        scores_we_may_be_able_to_get.unwrap_or(0) + self.score
    }
}

#[derive(Debug)]
struct ShortestPaths<'a>(HashMap<&'a &'a str, HashMap<&'a &'a str, u32>>);

impl<'a> ShortestPaths<'a> {
    fn all_shortest_paths_from(&'a self, source: &'a str) -> Option<&'a HashMap<&&str, u32>> {
        self.0.get(&source)
    }

    fn shortest_path(&self, source: &str, target: &str) -> Option<u32> {
        self.0
            .get(&source)
            .and_then(|inner_map| inner_map.get(&target))
            .cloned()
    }

    fn initialise(valve_lookup: &'a HashMap<&'a str, Valve>) -> Self {
        Self(
            valve_lookup
                .iter()
                .map(|(valve_name, valve)| {
                    (
                        valve_name,
                        valve
                            .neighbours
                            .iter()
                            .map(|neighbour_name| (neighbour_name, 1))
                            .collect(),
                    )
                })
                .collect(),
        )
    }

    fn include_valve(
        &self,
        valve: &'a Valve,
        valve_lookup: &'a HashMap<&'a str, Valve>,
    ) -> ShortestPaths<'a> {
        Self(
            valve_lookup
                .keys()
                .map(|source_valve_name| {
                    let inner_hashmap = valve_lookup
                        .keys()
                        .filter_map(|target_valve_name| {
                            let shortest_path_not_using_k =
                                self.shortest_path(source_valve_name, target_valve_name);

                            let shortest_path_from_source_to_k =
                                self.shortest_path(source_valve_name, valve.name);
                            let shortest_path_from_k_to_target =
                                self.shortest_path(valve.name, target_valve_name);
                            let shortest_path_using_k = shortest_path_from_source_to_k
                                .zip(shortest_path_from_k_to_target)
                                .map(|(a, b)| a + b);

                            shortest_path_not_using_k
                                .into_iter()
                                .chain(shortest_path_using_k)
                                .min()
                                .map(|min_score| (target_valve_name, min_score))
                        })
                        .collect();
                    (source_valve_name, inner_hashmap)
                })
                .collect(),
        )
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

fn floyd_warshall_shortest_paths<'a>(
    valve_lookup: &'a HashMap<&'a str, Valve>,
) -> ShortestPaths<'a> {
    valve_lookup
        .values()
        .fold(ShortestPaths::initialise(valve_lookup), |acc, valve| {
            acc.include_valve(valve, valve_lookup)
        })
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
}

fn main() {
    let input = read_input();
    let valve_lookup: HashMap<_, _> = input.lines().map(parse_valve).collect();
    let shortest_paths = floyd_warshall_shortest_paths(&valve_lookup);

    let start_valve = valve_lookup.get("AA").unwrap();
    let starting_possible_paths = PathCollection::new(start_valve);
    let all_paths = (1..=30).fold(starting_possible_paths, |acc, minute| {
        println!("minute: {minute}, found paths: {}", acc.paths.len());
        let new_paths: Vec<_> = acc
            .paths
            .into_iter()
            .flat_map(|path| {
                path.all_possible_extensions(minute, &valve_lookup)
                    .into_iter()
                    .filter(|new_path| {
                        let score_upper_bound = new_path.final_score_upper_bound(
                            &valve_lookup,
                            &shortest_paths,
                            minute,
                        );
                        score_upper_bound > acc.max_score
                    })
            })
            .collect();

        let new_max_score = new_paths.iter().map(|path| path.score).max().unwrap();

        PathCollection {
            paths: new_paths,
            max_score: new_max_score,
        }
    });
    let best_path = all_paths
        .paths
        .iter()
        .max_by_key(|path| path.score)
        .unwrap();

    let part_1_answer = best_path.score;
    println!("part 1: {part_1_answer}");
}
