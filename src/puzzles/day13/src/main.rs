use std::{cmp::Ordering, iter::Peekable};

use itertools::{EitherOrBoth, Itertools};
use utils::read_input;

#[derive(Debug)]
enum Item {
    List(Vec<Item>),
    Num(u32),
}

fn items_are_correctly_ordered(left_item: Item, right_item: Item) -> Option<bool> {
    match left_item {
        Item::Num(left_num) => match right_item {
            Item::Num(right_num) => match left_num.cmp(&right_num) {
                Ordering::Less => Some(true),
                Ordering::Greater => Some(false),
                Ordering::Equal => None,
            },
            Item::List(right_vec) => pair_of_lists_is_correctly_ordered(vec![left_item], right_vec),
        },
        Item::List(left_vec) => match right_item {
            Item::Num(right_num) => {
                pair_of_lists_is_correctly_ordered(left_vec, vec![Item::Num(right_num)])
            }
            Item::List(right_vec) => pair_of_lists_is_correctly_ordered(left_vec, right_vec),
        },
    }
}

fn pair_of_lists_is_correctly_ordered(
    left_packet: Vec<Item>,
    right_packet: Vec<Item>,
) -> Option<bool> {
    for either_or_both in left_packet.into_iter().zip_longest(right_packet) {
        match either_or_both {
            EitherOrBoth::Right(_) => return Some(true),
            EitherOrBoth::Left(_) => return Some(false),
            EitherOrBoth::Both(left_item, right_item) => {
                if let Some(result) = items_are_correctly_ordered(left_item, right_item) {
                    return Some(result);
                }
            }
        }
    }
    None
}

fn maybe_take_digit(packet: &mut Peekable<impl Iterator<Item = char>>) -> Option<char> {
    packet.next_if(|ch| ch.is_ascii_digit())
}

fn take_num(packet: &mut Peekable<impl Iterator<Item = char>>) -> u32 {
    let mut result = String::new();
    while let Some(digit) = maybe_take_digit(packet) {
        result.push(digit);
    }
    str::parse(&result).unwrap()
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

fn parse_packet(packet: &str) -> Vec<Item> {
    let mut peekable_chars = packet.chars().peekable();
    take_vec(&mut peekable_chars)
}

fn main() {
    let input = read_input();
    let chunks = input.lines().chunks(3);
    let packets: Vec<_> = chunks
        .into_iter()
        .map(|chunk| {
            let lines: Vec<_> = chunk.into_iter().collect();
            (parse_packet(lines[0]), parse_packet(lines[1]))
        })
        .collect();

    let part_1_answer: usize = packets
        .into_iter()
        .map(|(a, b)| pair_of_lists_is_correctly_ordered(a, b))
        .enumerate()
        .filter_map(|(pair_idx, is_correctly_ordered)| {
            let result = is_correctly_ordered
                .unwrap_or_else(|| panic!("no result for pair_idx {}", pair_idx + 1));
            result.then_some(pair_idx + 1)
        })
        .sum();

    println!("part 1: {}", part_1_answer)
}
