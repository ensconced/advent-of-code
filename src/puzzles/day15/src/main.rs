use gcollections::ops::{Bounded, Cardinality, Difference, Empty, Union};
use interval::{interval_set::ToIntervalSet, IntervalSet};
use utils::read_input;

#[derive(Debug)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn manhattan_distance(&self, other: &Self) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
    }
}

#[derive(Debug)]
struct Sensor {
    coordinates: Coordinates,
    closest_beacon: Coordinates,
}

fn remove(interval_set: IntervalSet<i64>, element: i64) -> IntervalSet<i64> {
    interval_set.difference(&vec![(element, element)].to_interval_set())
}

impl Sensor {
    fn exclusion_zone_at_y(&self, y: i64) -> IntervalSet<i64> {
        let manhattan_distance = self.coordinates.manhattan_distance(&self.closest_beacon);
        let vertical_distance = i64::abs(self.coordinates.y - y);
        if vertical_distance >= manhattan_distance {
            IntervalSet::empty()
        } else {
            let horizontal_distance = manhattan_distance - vertical_distance;
            let start = self.coordinates.x - horizontal_distance;
            let end = self.coordinates.x + horizontal_distance;
            if y == self.closest_beacon.y {
                remove(vec![(start, end)].to_interval_set(), self.closest_beacon.x)
            } else {
                vec![(start, end)].to_interval_set()
            }
        }
    }
}

fn parse_sensor(line: &str) -> Sensor {
    let parts: Vec<_> = line.split(' ').collect();
    let sensor_x = parts[2]
        .strip_prefix("x=")
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let sensor_y = parts[3]
        .strip_prefix("y=")
        .unwrap()
        .strip_suffix(':')
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let beacon_x = parts[8]
        .strip_prefix("x=")
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let beacon_y = parts[9].strip_prefix("y=").unwrap().parse::<i64>().unwrap();
    Sensor {
        coordinates: Coordinates {
            x: sensor_x,
            y: sensor_y,
        },
        closest_beacon: Coordinates {
            x: beacon_x,
            y: beacon_y,
        },
    }
}

fn exclusion_zone(sensors: &[Sensor], y: i64) -> IntervalSet<i64> {
    sensors.iter().fold(IntervalSet::empty(), |acc, sensor| {
        let mut exclusion_zone = sensor.exclusion_zone_at_y(y);
        if sensor.closest_beacon.y == y {
            exclusion_zone = remove(exclusion_zone, sensor.closest_beacon.y);
        }
        acc.union(&exclusion_zone)
    })
}

fn main() {
    let input = read_input();
    let sensors: Vec<_> = input.lines().map(parse_sensor).collect();

    let part_1_answer = exclusion_zone(&sensors, 2000000).size();
    println!("part 1: {}", part_1_answer);

    let all_col_idxs = vec![(0, 4000000)].to_interval_set();
    // part 2
    (0..4000000).for_each(|y| {
        let possible_locations = all_col_idxs.difference(&exclusion_zone(&sensors, y));
        if let Some(x) = possible_locations.into_iter().next() {
            println!("{}", x.lower() * 4000000 + y);
        }
    });
}
