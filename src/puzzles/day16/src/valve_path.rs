use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::{shortest_paths::ShortestPaths, Valve};

pub trait ValvePath<'a>
where
    Self: Sized,
{
    fn initialise(
        start_valve: &'a Valve,
        shortest_paths: &ShortestPaths,
        valve_lookup: &'a HashMap<&'a str, Valve>,
    ) -> Self;

    fn all_possible_extensions(
        self,
        minute: u32,
        valve_lookup: &'a HashMap<&str, Valve>,
        shortest_paths: &ShortestPaths,
    ) -> BinaryHeap<Self>;

    fn final_score_upper_bound(
        current_valve_name: &str,
        open_valves: &HashSet<&str>,
        valve_lookup: &'a HashMap<&'a str, Valve>,
        shortest_paths: &ShortestPaths,
        current_score: u32,
        minute: u32,
    ) -> u32;

    fn score(&self) -> u32;

    fn score_upper_bound(&self) -> u32;

    fn done(&self) -> bool;
}
