use std::iter::Peekable;

use itertools::Itertools;
use utils::read_input;

enum Item {
    List(Vec<Item>),
    Num(u32),
}

fn items_are_correctly_ordered(left_item: Item, right_item: Item) -> bool {
    todo!()
}

fn pair_of_lists_is_correctly_ordered(left_packet: Vec<Item>, right_packet: Vec<Item>) -> bool {
    todo!()
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

fn take_vec(packet: &mut Peekable<impl Iterator<Item = char>>) -> Vec<Item> {
    packet.next(); // '['
    let mut result = vec![];
    while let Some(item) = maybe_take_item(packet) {
        result.push(item);
    }
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
    let part_1_answer: usize = chunks
        .into_iter()
        .map(|chunk| {
            let lines: Vec<_> = chunk.into_iter().collect();
            pair_of_lists_is_correctly_ordered(parse_packet(lines[0]), parse_packet(lines[1]))
        })
        .enumerate()
        .filter_map(|(pair_idx, is_correctly_ordered)| is_correctly_ordered.then_some(pair_idx))
        .sum();

    println!("part 1: {}", part_1_answer)
}
