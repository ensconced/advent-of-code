use std::collections::HashMap;

use utils::read_input;

// TODO - fix score calculation - check on small input...

// possible optimisations
// - avoid going round in cycles pointlessly... DONE?
// - track which paths are DONE and their final score - if not the max, remove from the pool
// - keep track of which valves might possibly be reachable from current valve, and in how many
//   steps, by pre-computing all shortest paths - to give upper bound on possible score from going
//   to a given valve, which could be compared to current score for paths to rule out some paths as
//   definitely not worth pursuing
// - parallelize it?? but probably better algorithmic solutions...

#[derive(Debug)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
    neighbours: Vec<&'a str>,
}

#[derive(Debug)]
struct ValvePath<'a> {
    steps_since_opening_valve: usize,
    prev_steps: Vec<&'a str>,
    current_valve: &'a Valve<'a>,
    valves_opened_when: HashMap<&'a str, u32>,
    done: bool,
}

impl<'a> ValvePath<'a> {
    fn new(start_valve: &'a Valve) -> Self {
        Self {
            steps_since_opening_valve: 0,
            prev_steps: vec![],
            current_valve: start_valve,
            valves_opened_when: HashMap::new(),
            done: false,
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
                .map(|neighb| {
                    let mut prev_steps = self.prev_steps.clone();
                    prev_steps.push(self.current_valve.name);
                    ValvePath {
                        steps_since_opening_valve: self.steps_since_opening_valve + 1,
                        prev_steps,
                        current_valve: valve_lookup.get(neighb).unwrap(),
                        valves_opened_when: self.valves_opened_when.clone(),
                        done: false,
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
                });
            }
            extended_paths
        }
    }

    fn score(&self, valve_lookup: &HashMap<&str, Valve>) -> u32 {
        self.valves_opened_when
            .iter()
            .map(|(valve_name, minute_opened)| {
                let flow_rate = valve_lookup.get(valve_name).unwrap().flow_rate;
                let valve_open_duration = 30 - minute_opened;
                flow_rate * valve_open_duration
            })
            .sum()
    }
}

fn parse_valve(line: &str) -> (&str, Valve) {
    let parts: Vec<_> = line.split_ascii_whitespace().collect();
    let name = parts[1];
    let flow_rate = parts[4]
        .strip_prefix("rate=")
        .and_then(|rhs| rhs.strip_suffix(';'))
        .unwrap();
    let neighbours = parts[9..]
        .iter()
        .map(|neighb| neighb.trim_end_matches(','))
        .collect();
    (
        name,
        Valve {
            name,
            flow_rate: flow_rate.parse().unwrap(),
            neighbours,
        },
    )
}

fn main() {
    let input = read_input();
    let valve_lookup: HashMap<_, _> = input.lines().map(parse_valve).collect();
    let start_valve = valve_lookup.get("AA").unwrap();
    let starting_possible_paths = vec![ValvePath::new(start_valve)];
    let all_paths = (1..=30).fold(starting_possible_paths, |acc, minute| {
        println!("minute: {minute}, found paths: {}", acc.len());
        acc.into_iter()
            .flat_map(|path| {
                path.all_possible_extensions(minute, &valve_lookup)
                    .into_iter()
            })
            .collect()
    });
    let best_path = all_paths
        .iter()
        .max_by_key(|path| path.score(&valve_lookup))
        .unwrap();

    dbg!(&best_path);
    let part_1_answer = best_path.score(&valve_lookup);
    println!("part 1: {part_1_answer}");
}
