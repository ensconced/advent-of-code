use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{shortest_paths::ShortestPaths, Valve, MINUTES};

#[derive(Clone, Debug)]
pub enum ThreadAction<'a> {
    Move(&'a str),
    OpenValve(&'a str),
}

#[derive(Clone, Debug)]
pub struct ValveThread<'a> {
    pub actions: Vec<ThreadAction<'a>>,
    pub current_valve: &'a Valve<'a>,
    pub prev_steps: Vec<&'a str>,
    pub steps_since_opening_valve: usize,
    pub done: bool,
    pub opened_valves: HashSet<&'a str>,
}

impl<'a> ValveThread<'a> {
    pub fn new(start_valve: &'a Valve) -> Self {
        Self {
            actions: vec![],
            steps_since_opening_valve: 0,
            prev_steps: vec![],
            current_valve: start_valve,
            opened_valves: HashSet::new(),
            done: false,
        }
    }

    pub fn move_to_valve(&self, valve: &'a str, valve_lookup: &'a HashMap<&str, Valve>) -> Self {
        let mut prev_steps = self.prev_steps.clone();
        prev_steps.push(self.current_valve.name);
        let mut actions = self.actions.clone();
        actions.push(ThreadAction::Move(valve));
        Self {
            actions,
            steps_since_opening_valve: self.steps_since_opening_valve + 1,
            prev_steps,
            current_valve: valve_lookup.get(valve).unwrap(),
            opened_valves: self.opened_valves.clone(),
            done: false,
        }
    }

    pub fn open_valve(mut self) -> Self {
        let mut opened_valves = self.opened_valves;
        opened_valves.insert(self.current_valve.name);
        self.actions
            .push(ThreadAction::OpenValve(self.current_valve.name));
        Self {
            actions: self.actions,
            steps_since_opening_valve: 0,
            prev_steps: self.prev_steps,
            current_valve: self.current_valve,
            opened_valves,
            done: false,
        }
    }

    fn do_nothing(mut self) -> Self {
        self.done = true;
        self
    }

    pub fn remaining_reachable_values(
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

    pub fn ends_with_pointless_cycle(&self) -> bool {
        self.prev_steps
            .iter()
            .rev()
            .take(self.steps_since_opening_valve)
            .any(|el| el == &self.current_valve.name)
    }

    pub fn all_possible_extensions(
        self,
        valve_lookup: &'a HashMap<&str, Valve>,
        open_valves: &HashSet<&str>,
    ) -> Vec<ValveThread<'a>> {
        let mut result = Vec::new();

        let neighbours = &self.current_valve.neighbours;

        for path in neighbours
            .iter()
            .map(|neighbour| self.move_to_valve(neighbour, valve_lookup))
            .filter(|thread| !thread.ends_with_pointless_cycle())
        {
            result.push(path);
        }

        if open_valves.contains(self.current_valve.name) {
            result.push(self.do_nothing());
        } else if self.current_valve.flow_rate > 0 {
            result.push(self.open_valve());
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct ThreadCombinationSet<'a> {
    pub candidates: Vec<Vec<ValveThread<'a>>>,
}

impl<'a> ThreadCombinationSet<'a> {
    pub fn add_thread_extensions(&self, thread_extensions: Vec<ValveThread<'a>>) -> Self {
        ThreadCombinationSet {
            candidates: self
                .candidates
                .iter()
                .cartesian_product(thread_extensions)
                .map(|(thread_set, new_thread)| {
                    let mut threads = thread_set.clone();
                    threads.push(new_thread);
                    threads
                })
                .collect(),
        }
    }

    pub fn new() -> Self {
        Self {
            candidates: vec![vec![]],
        }
    }
}
