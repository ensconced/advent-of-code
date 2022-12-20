use itertools::Itertools;

use crate::{shortest_paths::ShortestPaths, Valve, ValveLookup};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum ThreadAction {
    Move {
        valve_name: &'static str,
        distance: u32,
    },
    OpenValve {
        valve_name: &'static str,
        value: u32,
    },
}

impl ThreadAction {
    fn valve(&self) -> &'static str {
        match self {
            ThreadAction::Move { valve_name, .. } | ThreadAction::OpenValve { valve_name, .. } => {
                valve_name
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ValveThread {
    pub minutes_remaining: u32,
    pub actions: Vec<ThreadAction>,
    pub done: bool,
}

impl ValveThread {
    pub fn new(start_valve: &Valve, total_minutes: u32) -> Self {
        Self {
            actions: vec![ThreadAction::Move {
                valve_name: start_valve.name,
                distance: 0,
            }],
            done: false,
            minutes_remaining: total_minutes,
        }
    }

    pub fn move_to_and_open_valve(
        &self,
        valve: &'static str,
        distance: u32,
        valve_lookup: &ValveLookup,
    ) -> Self {
        let minutes_remaining = self.minutes_remaining - distance - 1;
        let mut actions = self.actions.clone();
        actions.push(ThreadAction::Move {
            valve_name: valve,
            distance,
        });
        actions.push(ThreadAction::OpenValve {
            valve_name: self.current_valve_name(),
            value: minutes_remaining
                * valve_lookup
                    .get(self.current_valve_name())
                    .unwrap()
                    .flow_rate,
        });

        Self {
            actions,
            done: false,
            minutes_remaining,
        }
    }

    fn current_valve_name(&self) -> &'static str {
        self.actions.last().unwrap().valve()
    }

    fn do_nothing(mut self) -> Self {
        self.done = true;
        self
    }

    // fn reachable_open_valves(
    //     &self,
    //     shortest_paths: &ShortestPaths,
    //     open_valves: &HashSet<&'static str>,
    // ) -> HashMap<&'static str, u32> {
    //     shortest_paths
    //         .all_shortest_paths_from(self.current_valve_name())
    //         .unwrap()
    //         .iter()
    //         .filter(|(&valve_name, _)| !open_valves.contains(valve_name))
    //         .map(|(&valve_name, &distance)| (valve_name, distance))
    //         .collect()
    // }

    // pub fn remaining_reachable_values(
    //     &self,
    //     shortest_paths: &ShortestPaths,
    //     open_valves: &HashSet<&'static str>,
    //     valve_lookup: &ValveLookup,
    // ) -> HashMap<&'static str, u32> {
    // self.reachable_open_valves(shortest_paths, open_valves)
    //     .into_iter()
    //     .map(|(valve_name, path_length)| {
    //         let time_to_open_valve = path_length + 1;
    //         let value = if time_to_open_valve >= self.minutes_remaining {
    //             0
    //         } else {
    //             let max_minutes_of_flow = self.minutes_remaining - time_to_open_valve;
    //             let flow_rate = valve_lookup.get(valve_name).unwrap().flow_rate;
    //             flow_rate * max_minutes_of_flow
    //         };
    //         (valve_name, value)
    //     })
    //     .collect()
    // }

    // pub fn all_possible_extensions(
    //     self,
    //     open_valves: &HashSet<&'static str>,
    //     valve_lookup: &ValveLookup,
    //     shortest_paths: &ShortestPaths,
    // ) -> Vec<ValveThread> {
    //     let mut result = Vec::new();

    //     for path in self
    //         .reachable_open_valves(shortest_paths, open_valves)
    //         .into_iter()
    //         .filter(|(_, distance)| distance + 1 < self.minutes_remaining)
    //         .map(|(neighbour, distance)| {
    //             self.move_to_and_open_valve(neighbour, distance, valve_lookup)
    //         })
    //     {
    //         result.push(path);
    //     }

    //     if open_valves.contains(self.current_valve_name()) {
    //         result.push(self.do_nothing());
    //     }
    //     result
    // }
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
