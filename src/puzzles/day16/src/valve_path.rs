// use std::{
//     collections::{BinaryHeap, HashMap, HashSet},
//     iter::repeat_with,
// };

// use crate::{
//     shortest_paths::ShortestPaths,
//     valve_thread::{ThreadAction, ThreadCombinationSet, ValveThread},
//     Valve, ValveLookup,
// };

// #[derive(Clone, Debug)]
// pub struct ValvePath {
//     pub done: bool,
//     pub score_upper_bound: u32,
//     pub threads: Vec<ValveThread>,
//     pub score: u32,
//     open_valves: HashSet<&'static str>,
// }

// impl ValvePath {
//     pub fn new(
//         start_valve: &Valve,
//         shortest_paths: &ShortestPaths,
//         valve_lookup: &ValveLookup,
//         thread_count: usize,
//         total_minutes: u32,
//     ) -> Self {
//         let open_valves = HashSet::new();

//         let mut result = Self {
//             score: 0,
//             threads: repeat_with(|| ValveThread::new(start_valve, total_minutes))
//                 .take(thread_count)
//                 .collect(),
//             open_valves,
//             done: false,
//             score_upper_bound: 0,
//         };

//         result.score_upper_bound = result.final_score_upper_bound(shortest_paths, valve_lookup);

//         result
//     }

//     pub fn all_possible_extensions(
//         self,
//         valve_lookup: &ValveLookup,
//         shortest_paths: &ShortestPaths,
//     ) -> BinaryHeap<ValvePath> {
//         let all_thread_combinations = self
//             .threads
//             .into_iter()
//             .map(|thread| {
//                 thread.all_possible_extensions(&self.open_valves, valve_lookup, shortest_paths)
//             })
//             .fold(
//                 ThreadCombinationSet::new(),
//                 |thread_combination_set, thread_extensions| {
//                     thread_combination_set.add_thread_extensions(thread_extensions)
//                 },
//             );

//         all_thread_combinations
//             .candidates
//             .into_iter()
//             .map(|threads| {
//                 let mut valve_openings: HashMap<&str, Vec<(&str, u32)>> = HashMap::new();
//                 for thread in threads.iter() {
//                     if !thread.done {
//                         if let Some(ThreadAction::OpenValve { valve_name, value }) =
//                             thread.actions.last()
//                         {
//                             valve_openings
//                                 .entry(*valve_name)
//                                 .or_default()
//                                 .push((valve_name, *value));
//                         }
//                     }
//                 }
//                 (threads, valve_openings)
//             })
//             .filter(|(_, valve_openings)| !valve_openings.iter().any(|(_, v)| v.len() > 1))
//             .map(|(extended_threads, valve_openings)| {
//                 let score = valve_openings
//                     .into_iter()
//                     .fold(self.score, |acc, (_, actions)| acc + actions[0].1);

//                 let mut path = ValvePath {
//                     score,
//                     done: extended_threads.iter().all(|thread| thread.done),
//                     score_upper_bound: 0,
//                     open_valves: extended_threads
//                         .iter()
//                         .flat_map(|thread| {
//                             thread.actions.iter().filter_map(|action| match action {
//                                 ThreadAction::OpenValve { valve_name, .. } => Some(*valve_name),
//                                 _ => None,
//                             })
//                         })
//                         .collect(),
//                     threads: extended_threads,
//                 };

//                 path.score_upper_bound = path.final_score_upper_bound(shortest_paths, valve_lookup);
//                 path
//             })
//             .collect()
//     }

//     fn final_score_upper_bound(
//         &self,
//         shortest_paths: &ShortestPaths,
//         valve_lookup: &ValveLookup,
//     ) -> u32 {
//         let reachable_values = self.threads.iter().fold(HashMap::new(), |acc, thread| {
//             thread
//                 .remaining_reachable_values(shortest_paths, &self.open_valves, valve_lookup)
//                 .into_iter()
//                 .map(|(reachable_valve, valve_value)| {
//                     let max_val = acc
//                         .get(reachable_valve)
//                         .map(|max_value_obtainable_from_valve_acc| {
//                             u32::max(*max_value_obtainable_from_valve_acc, valve_value)
//                         })
//                         .unwrap_or(valve_value);
//                     (reachable_valve, max_val)
//                 })
//                 .collect()
//         });
//         self.score + reachable_values.values().sum::<u32>()
//     }
// }

// impl PartialEq for ValvePath {
//     fn eq(&self, other: &Self) -> bool {
//         self.score_upper_bound == other.score_upper_bound
//     }
// }

// impl Eq for ValvePath {}

// impl PartialOrd for ValvePath {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.score_upper_bound.partial_cmp(&other.score_upper_bound)
//     }
// }

// impl Ord for ValvePath {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.score_upper_bound.cmp(&other.score_upper_bound)
//     }
// }
