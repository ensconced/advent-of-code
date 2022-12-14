use std::ops::RangeInclusive;

struct SectionIdRange {
    range: RangeInclusive<u32>,
}

impl SectionIdRange {
    fn fully_contains(&self, other: &SectionIdRange) -> bool {
        self.range.contains(other.range.start()) && self.range.contains(other.range.end())
    }
    fn contains_either_end(&self, other: &SectionIdRange) -> bool {
        self.range.contains(other.range.start()) || self.range.contains(other.range.end())
    }
}

fn parse_range(range_str: &str) -> SectionIdRange {
    let mut numbers = range_str.split('-').map(|range_section| {
        range_section
            .parse()
            .unwrap_or_else(|_| panic!("failed to parse string to u32"))
    });
    let first = numbers
        .next()
        .unwrap_or_else(|| panic!("expected to find first number of range"));
    let second = numbers
        .next()
        .unwrap_or_else(|| panic!("expected to find second number of range"));
    SectionIdRange {
        range: first..=second,
    }
}

fn main() {
    let ranges: Vec<_> = utils::read_input()
        .lines()
        .map(|line| {
            let mut range_sections = line.split(',');
            let first_range_section = range_sections
                .next()
                .unwrap_or_else(|| panic!("expected to find first range section"));
            let second_range_section = range_sections
                .next()
                .unwrap_or_else(|| panic!("expected to find second range section"));
            (
                parse_range(first_range_section),
                parse_range(second_range_section),
            )
        })
        .collect();

    let part_1_answer = ranges
        .iter()
        .filter(|(first_range, second_range)| {
            first_range.fully_contains(second_range) || second_range.fully_contains(first_range)
        })
        .count();

    let part_2_answer = ranges
        .iter()
        .filter(|(first_range, second_range)| {
            first_range.contains_either_end(second_range)
                || second_range.contains_either_end(first_range)
        })
        .count();

    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
}
