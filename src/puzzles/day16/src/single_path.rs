use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{shortest_paths::ShortestPaths, Valve, MINUTES};

#[derive(Clone, Debug)]
struct ValveThread<'a> {
    steps_since_opening_valve: usize,
    prev_steps: Vec<&'a str>,
    current_valve: &'a Valve<'a>,
}

impl<'a> ValveThread<'a> {
    fn new(start_valve: &'a Valve) -> Self {
        Self {
            steps_since_opening_valve: 0,
            prev_steps: vec![],
            current_valve: start_valve,
        }
    }

    fn move_to_valve(&self, valve: &str, valve_lookup: &'a HashMap<&str, Valve>) -> Self {
        let mut prev_steps = self.prev_steps.clone();
        prev_steps.push(self.current_valve.name);
        Self {
            steps_since_opening_valve: self.steps_since_opening_valve + 1,
            prev_steps,
            current_valve: valve_lookup.get(valve).unwrap(),
        }
    }

    fn open_valve(self) -> Self {
        Self {
            steps_since_opening_valve: 0,
            prev_steps: self.prev_steps,
            current_valve: self.current_valve,
        }
    }

    fn upper_bound_of_remaining_reachable_value(
        &self,
        shortest_paths: &'a ShortestPaths,
        open_valves: &HashSet<&str>,
        minute: u32,
        valve_lookup: &'a HashMap<&str, Valve>,
    ) -> HashMap<&'a str, u32> {
        shortest_paths
            .all_shortest_paths_from(self.current_valve.name)
            .unwrap()
            .iter()
            .filter(|(valve_name, _)| !open_valves.contains(*valve_name))
            .map(|(&valve_name, path_length)| {
                let min_minute_to_open_valve = minute + path_length + 1;
                let value = if min_minute_to_open_valve >= MINUTES {
                    0
                } else {
                    let max_minutes_of_flow = MINUTES - min_minute_to_open_valve;
                    let flow_rate = valve_lookup.get(valve_name).unwrap().flow_rate;
                    flow_rate * max_minutes_of_flow
                };
                (valve_name, value)
            })
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct ValvePath<'a> {
    pub done: bool,
    pub score: u32,
    pub score_upper_bound: u32,
    open_valves: HashSet<&'a str>,
    thread: ValveThread<'a>,
}

impl<'a> ValvePath<'a> {
    pub fn new(
        start_valve: &'a Valve,
        shortest_paths: &ShortestPaths,
        valve_lookup: &'a HashMap<&'a str, Valve>,
    ) -> Self {
        let minute = 0;
        let current_score = 0;
        let open_valves = HashSet::new();

        let mut result = Self {
            thread: ValveThread::new(start_valve),
            open_valves,
            done: false,
            score: 0,
            score_upper_bound: 0,
        };

        result.score_upper_bound = result.final_score_upper_bound(
            &result.open_valves,
            valve_lookup,
            shortest_paths,
            current_score,
            minute,
        );

        result
    }

    fn do_nothing(mut self) -> ValvePath<'a> {
        self.done = true;
        self
    }

    fn move_to_valve(
        &self,
        valve: &str,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        shortest_paths: &ShortestPaths,
        minute: u32,
    ) -> ValvePath<'a> {
        let open_valves = self.open_valves.clone();

        let thread = self.thread.move_to_valve(valve, valve_lookup);

        let score_upper_bound = self.final_score_upper_bound(
            &open_valves,
            valve_lookup,
            shortest_paths,
            self.score,
            minute,
        );

        ValvePath {
            thread,
            open_valves,
            done: false,
            score: self.score,
            score_upper_bound,
        }
    }

    fn open_valve(
        mut self,
        minute: u32,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        shortest_paths: &ShortestPaths,
    ) -> ValvePath<'a> {
        let score = self.score + self.thread.current_valve.flow_rate * (MINUTES - minute);
        self.open_valves.insert(self.thread.current_valve.name);

        let score_upper_bound = self.final_score_upper_bound(
            &self.open_valves,
            valve_lookup,
            shortest_paths,
            score,
            minute,
        );
        ValvePath {
            thread: self.thread.open_valve(),
            open_valves: self.open_valves,
            done: false,
            score,
            score_upper_bound,
        }
    }

    fn ends_with_pointless_cycle(&self) -> bool {
        self.thread
            .prev_steps
            .iter()
            .rev()
            .take(self.thread.steps_since_opening_valve)
            .any(|el| el == &self.thread.current_valve.name)
    }

    pub fn all_possible_extensions(
        self,
        minute: u32,
        valve_lookup: &'a HashMap<&str, Valve>,
        shortest_paths: &ShortestPaths,
    ) -> BinaryHeap<ValvePath<'a>> {
        let mut result = BinaryHeap::new();

        for path in self
            .thread
            .current_valve
            .neighbours
            .iter()
            .map(|neighbour| self.move_to_valve(neighbour, valve_lookup, shortest_paths, minute))
            .filter(|path| !path.ends_with_pointless_cycle())
        {
            result.push(path);
        }

        if self.open_valves.contains(self.thread.current_valve.name) {
            result.push(self.do_nothing());
        } else if self.thread.current_valve.flow_rate > 0 {
            result.push(self.open_valve(minute, valve_lookup, shortest_paths));
        }
        result
    }

    fn final_score_upper_bound(
        &self,
        open_valves: &HashSet<&str>,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        shortest_paths: &ShortestPaths,
        current_score: u32,
        minute: u32,
    ) -> u32 {
        let reachable_valve_values = self.thread.upper_bound_of_remaining_reachable_value(
            shortest_paths,
            open_valves,
            minute,
            valve_lookup,
        );
        current_score + reachable_valve_values.values().sum::<u32>()
    }
}

impl<'a> PartialEq for ValvePath<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.score_upper_bound == other.score_upper_bound
    }
}

impl<'a> Eq for ValvePath<'a> {}

impl<'a> PartialOrd for ValvePath<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.score_upper_bound.partial_cmp(&other.score_upper_bound)
    }
}

impl<'a> Ord for ValvePath<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score_upper_bound.cmp(&other.score_upper_bound)
    }
}
