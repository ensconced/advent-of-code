use std::{cmp::Ordering, iter::Peekable};

use itertools::{EitherOrBoth, Itertools};
use utils::read_input;

#[derive(Clone, Debug)]
enum Item {
    List(Vec<Item>),
    Num(u32),
}

#[derive(Clone)]
struct Packet(Vec<Item>);

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        packet_ordering(&self.0, &other.0).is_eq()
    }
}

impl Eq for Packet {}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(packet_ordering(&self.0, &other.0))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn items_are_correctly_ordered(left_item: &Item, right_item: &Item) -> Ordering {
    match left_item {
        Item::Num(left_num) => match right_item {
            Item::Num(right_num) => match left_num.cmp(right_num) {
                Ordering::Less => Ordering::Less,
                Ordering::Greater => Ordering::Greater,
                Ordering::Equal => Ordering::Equal,
            },
            Item::List(right_vec) => packet_ordering(&[left_item.clone()], right_vec),
        },
        Item::List(left_vec) => match right_item {
            Item::Num(right_num) => packet_ordering(left_vec, &[Item::Num(*right_num)]),
            Item::List(right_vec) => packet_ordering(left_vec, right_vec),
        },
    }
}

fn packet_ordering(left_packet: &[Item], right_packet: &[Item]) -> Ordering {
    for either_or_both in left_packet.iter().zip_longest(right_packet) {
        match either_or_both {
            EitherOrBoth::Right(_) => return Ordering::Less,
            EitherOrBoth::Left(_) => return Ordering::Greater,
            EitherOrBoth::Both(left_item, right_item) => {
                let item_ordering = items_are_correctly_ordered(left_item, right_item);
                if item_ordering != Ordering::Equal {
                    return item_ordering;
                }
            }
        }
    }
    Ordering::Equal
}

fn maybe_take_digit(packet: &mut Peekable<impl Iterator<Item = char>>) -> Option<char> {
    packet.next_if(|ch| ch.is_ascii_digit())
}

fn take_num(packet: &mut Peekable<impl Iterator<Item = char>>) -> u32 {
    let mut result = String::new();
    while let Some(digit) = maybe_take_digit(packet) {
        result.push(digit);
    }
    result.parse().unwrap()
}

fn maybe_take_comma(packet: &mut Peekable<impl Iterator<Item = char>>) -> bool {
    if let Some(ch) = packet.peek() {
        if *ch == ',' {
            packet.next();
            return true;
        }
    }
    false
}

fn take_vec(packet: &mut Peekable<impl Iterator<Item = char>>) -> Vec<Item> {
    packet.next(); // '['
    let mut result = vec![];
    if let Some(first_item) = maybe_take_item(packet) {
        result.push(first_item);
    }
    while maybe_take_comma(packet) {
        let item = maybe_take_item(packet).unwrap();
        result.push(item);
    }
    packet.next(); // ']'
    result
}

fn maybe_take_item(packet: &mut Peekable<impl Iterator<Item = char>>) -> Option<Item> {
    let next_char = packet.peek().cloned();
    next_char.and_then(|ch| match ch {
        '0'..='9' => Some(Item::Num(take_num(packet))),
        '[' => Some(Item::List(take_vec(packet))),
        _ => None,
    })
}

fn parse_packet(packet: &str) -> Packet {
    let mut peekable_chars = packet.chars().peekable();
    Packet(take_vec(&mut peekable_chars))
}

fn main() {
    let input = read_input();
    let mut packets: Vec<_> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_packet)
        .collect();

    // part 1
    let packet_pairs = packets.chunks(2);
    let part_1_answer: usize = packet_pairs
        .into_iter()
        .map(|chunk| packet_ordering(&chunk[0].0, &chunk[1].0))
        .enumerate()
        .filter_map(|(pair_idx, is_correctly_ordered)| {
            (is_correctly_ordered == Ordering::Less).then_some(pair_idx + 1)
        })
        .sum();
    println!("part 1: {}", part_1_answer);

    // part 2
    let marker_packet_1 = parse_packet("[[2]]");
    let marker_packet_2 = parse_packet("[[6]]");
    packets.push(marker_packet_1.clone());
    packets.push(marker_packet_2.clone());

    packets.sort();

    let marker_1_position = packets
        .iter()
        .find_position(|packet| packet == &&marker_packet_1);

    let marker_2_position = packets
        .iter()
        .find_position(|packet| packet == &&marker_packet_2);

    let part_2_answer = (marker_1_position.unwrap().0 + 1) * (marker_2_position.unwrap().0 + 1);
    println!("part 2: {}", part_2_answer);
}
