use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::{shortest_paths::ShortestPaths, Valve, ValveLookup, MINUTES};

#[derive(Clone, Debug)]
pub enum ThreadAction {
    Move(&'static str),
    OpenValve(&'static str),
}

impl ThreadAction {
    fn valve(&self) -> &'static str {
        match self {
            ThreadAction::Move(valve_name) => valve_name,
            ThreadAction::OpenValve(valve_name) => valve_name,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ValveThread {
    pub actions: Vec<ThreadAction>,
    pub steps_since_opening_valve: usize,
    pub done: bool,
    pub opened_valves: HashSet<&'static str>,
}

impl ValveThread {
    pub fn new(start_valve: &Valve) -> Self {
        Self {
            actions: vec![ThreadAction::Move(&start_valve.name)],
            steps_since_opening_valve: 0,
            opened_valves: HashSet::new(),
            done: false,
        }
    }

    pub fn move_to_valve(&self, valve: &'static str) -> Self {
        let mut actions = self.actions.clone();
        actions.push(ThreadAction::Move(valve));
        Self {
            actions,
            steps_since_opening_valve: self.steps_since_opening_valve + 1,
            opened_valves: self.opened_valves.clone(),
            done: false,
        }
    }

    fn current_valve_name(&self) -> &'static str {
        self.actions.last().unwrap().valve()
    }

    pub fn open_valve(&self) -> Self {
        let mut opened_valves = self.opened_valves.clone();
        opened_valves.insert(self.current_valve_name());
        let mut actions = self.actions.clone();
        actions.push(ThreadAction::OpenValve(self.current_valve_name()));
        Self {
            actions,
            steps_since_opening_valve: 0,
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
        shortest_paths: &ShortestPaths,
        open_valves: &HashSet<&str>,
        minute: u32,
        valve_lookup: &ValveLookup,
    ) -> HashMap<&str, u32> {
        shortest_paths
            .all_shortest_paths_from(self.current_valve_name())
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
        self.actions
            .iter()
            .filter(|action| matches!(action, ThreadAction::Move(_)))
            .rev()
            .take(self.steps_since_opening_valve)
            .any(|el| el.valve() == self.current_valve_name())
    }

    pub fn all_possible_extensions(
        self,
        open_valves: &HashSet<&str>,
        valve_lookup: &ValveLookup,
    ) -> Vec<ValveThread> {
        let mut result = Vec::new();

        let current_valve = valve_lookup.get(self.current_valve_name()).unwrap();
        for path in current_valve
            .neighbours
            .iter()
            .map(|neighbour| self.move_to_valve(neighbour))
            .filter(|thread| !thread.ends_with_pointless_cycle())
        {
            result.push(path);
        }

        if open_valves.contains(self.current_valve_name()) {
            result.push(self.do_nothing());
        } else if current_valve.flow_rate > 0 {
            result.push(self.open_valve());
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct ThreadCombinationSet {
    pub candidates: Vec<Vec<ValveThread>>,
}

impl ThreadCombinationSet {
    pub fn add_thread_extensions(&self, thread_extensions: Vec<ValveThread>) -> Self {
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
