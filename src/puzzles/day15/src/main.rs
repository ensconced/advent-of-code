use gcollections::ops::{Cardinality, Empty, Union};
use interval::{interval_set::ToIntervalSet, Interval, IntervalSet};
use utils::read_input;

#[derive(Debug)]
struct Coordinates {
    x: i32,
    y: i32,
}

impl Coordinates {
    fn manhattan_distance(&self, other: &Self) -> i32 {
        i32::abs(self.x - other.x) + i32::abs(self.y - other.y)
    }
}

#[derive(Debug)]
struct Sensor {
    coordinates: Coordinates,
    closest_beacon: Coordinates,
}

impl Sensor {
    fn exclusion_zone_at_y(&self, y: i32) -> IntervalSet<i32> {
        let manhattan_distance = self.coordinates.manhattan_distance(&self.closest_beacon);
        let vertical_distance = i32::abs(self.coordinates.y - y);
        if vertical_distance > manhattan_distance {
            IntervalSet::empty()
        } else {
            let horizontal_distance = manhattan_distance - vertical_distance;
            let start = self.coordinates.x - horizontal_distance;
            let end = self.coordinates.x + horizontal_distance;
            vec![(start, end)].to_interval_set()
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
        .parse::<i32>()
        .unwrap();
    let sensor_y = parts[3]
        .strip_prefix("y=")
        .unwrap()
        .strip_suffix(':')
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let beacon_x = parts[8]
        .strip_prefix("x=")
        .unwrap()
        .strip_suffix(',')
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let beacon_y = parts[9].strip_prefix("y=").unwrap().parse::<i32>().unwrap();
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

fn main() {
    let input = read_input();
    let row_of_interest = 2000000;
    let part_1_answer = input
        .lines()
        .map(parse_sensor)
        .fold(IntervalSet::empty(), |acc, sensor| {
            let mut exclusion_zone = sensor.exclusion_zone_at_y(row_of_interest);
            if sensor.coordinates.y == row_of_interest {
                exclusion_zone = exclusion_zone - sensor.coordinates.y;
            }
            if sensor.closest_beacon.y == row_of_interest {
                exclusion_zone = exclusion_zone - sensor.closest_beacon.y;
            }
            acc.union(&exclusion_zone)
        })
        .size();

    println!("part 1: {}", part_1_answer);
}
