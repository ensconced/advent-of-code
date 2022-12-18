use std::collections::{HashMap, HashSet};

use crate::{shortest_paths::ShortestPaths, Valve, MINUTES};

#[derive(Clone, Debug)]
pub struct ValveThread<'a> {
    pub current_valve: &'a Valve<'a>,
    pub prev_steps: Vec<&'a str>,
    pub steps_since_opening_valve: usize,
}

impl<'a> ValveThread<'a> {
    pub fn new(start_valve: &'a Valve) -> Self {
        Self {
            steps_since_opening_valve: 0,
            prev_steps: vec![],
            current_valve: start_valve,
        }
    }

    pub fn move_to_valve(&self, valve: &str, valve_lookup: &'a HashMap<&str, Valve>) -> Self {
        let mut prev_steps = self.prev_steps.clone();
        prev_steps.push(self.current_valve.name);
        Self {
            steps_since_opening_valve: self.steps_since_opening_valve + 1,
            prev_steps,
            current_valve: valve_lookup.get(valve).unwrap(),
        }
    }

    pub fn open_valve(self) -> Self {
        Self {
            steps_since_opening_valve: 0,
            prev_steps: self.prev_steps,
            current_valve: self.current_valve,
        }
    }

    pub fn upper_bound_of_remaining_reachable_value(
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
