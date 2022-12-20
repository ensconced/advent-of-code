use std::collections::HashMap;

use crate::{Valve, ValveLookup};

#[derive(Debug)]
pub struct ShortestPaths(HashMap<&'static str, HashMap<&'static str, u32>>);

impl ShortestPaths {
    pub fn all_shortest_paths_from(
        &self,
        source: &'static str,
    ) -> Option<&HashMap<&'static str, u32>> {
        self.0.get(&source)
    }

    fn shortest_path(&self, source: &str, target: &str) -> Option<u32> {
        self.0
            .get(&source)
            .and_then(|inner_map| inner_map.get(&target))
            .cloned()
    }

    pub fn initialise(valve_lookup: &HashMap<&'static str, Valve>) -> Self {
        Self(
            valve_lookup
                .iter()
                .map(|(&valve_name, valve)| {
                    (
                        valve_name,
                        valve
                            .neighbours
                            .iter()
                            .flat_map(|&neighbour_name| [(neighbour_name, 1), (valve_name, 0)])
                            .collect(),
                    )
                })
                .collect(),
        )
    }

    fn include_valve(
        &self,
        valve: &Valve,
        valve_lookup: &HashMap<&'static str, Valve>,
    ) -> ShortestPaths {
        Self(
            valve_lookup
                .keys()
                .map(|&source_valve_name| {
                    let inner_hashmap = valve_lookup
                        .keys()
                        .filter_map(|&target_valve_name| {
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

pub fn floyd_warshall_shortest_paths<'a>(valve_lookup: &ValveLookup) -> ShortestPaths {
    valve_lookup
        .values()
        .fold(ShortestPaths::initialise(valve_lookup), |acc, valve| {
            acc.include_valve(valve, valve_lookup)
        })
}
