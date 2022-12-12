use itertools::Itertools;
use utils::read_input;

struct Monkey {
    items: Vec<i64>,
    compute_new_worry_level: Box<dyn FnMut(i64) -> i64>,
    test_divisor: i64,
    next_monkey_if_true: usize,
    next_monkey_if_false: usize,
    activity: u64,
}

fn take_monkey_index_line<'a>(lines: &mut impl Iterator<Item = &'a str>) {
    lines.next();
}

fn take_starting_items<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<i64> {
    lines
        .next()
        .map(|line| {
            line.trim()
                .strip_prefix("Starting items: ")
                .unwrap()
                .split(", ")
                .map(|str_num| str::parse::<i64>(str_num).unwrap())
                .collect()
        })
        .unwrap()
}

fn take_compute_new_worry_level<'a>(
    lines: &mut impl Iterator<Item = &'a str>,
) -> Box<dyn FnMut(i64) -> i64> {
    lines
        .next()
        .map(|line| {
            let line = line.to_owned();
            Box::new(move |old_worry_level: i64| {
                let mut parts = line
                    .trim()
                    .strip_prefix("Operation: new = ")
                    .unwrap()
                    .split(' ');

                let lhs = match parts.next().unwrap() {
                    "old" => old_worry_level,
                    other_str => str::parse::<i64>(other_str).unwrap(),
                };
                let operation = parts.next().unwrap();
                let rhs = match parts.next().unwrap() {
                    "old" => old_worry_level,
                    other_str => str::parse::<i64>(other_str).unwrap(),
                };
                match operation {
                    "*" => lhs * rhs,
                    "+" => lhs + rhs,
                    _ => panic!(),
                }
            })
        })
        .unwrap()
}

fn take_test_divisor<'a>(lines: &mut impl Iterator<Item = &'a str>) -> i64 {
    str::parse::<i64>(
        lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("Test: divisible by ")
            .unwrap(),
    )
    .unwrap()
}

fn take_next_monkey_if_true<'a>(lines: &mut impl Iterator<Item = &'a str>) -> usize {
    str::parse::<usize>(
        lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If true: throw to monkey ")
            .unwrap(),
    )
    .unwrap()
}

fn take_next_monkey_if_false<'a>(lines: &mut impl Iterator<Item = &'a str>) -> usize {
    str::parse::<usize>(
        lines
            .next()
            .unwrap()
            .trim()
            .strip_prefix("If false: throw to monkey ")
            .unwrap(),
    )
    .unwrap()
}

fn take_monkey<'a>(mut lines: impl Iterator<Item = &'a str>) -> Monkey {
    take_monkey_index_line(&mut lines);
    let starting_items = take_starting_items(&mut lines);
    let compute_new_worry_level = take_compute_new_worry_level(&mut lines);
    let test_divisor = take_test_divisor(&mut lines);
    let next_monkey_if_true = take_next_monkey_if_true(&mut lines);
    let next_monkey_if_false = take_next_monkey_if_false(&mut lines);
    Monkey {
        items: starting_items,
        compute_new_worry_level,
        test_divisor,
        next_monkey_if_true,
        next_monkey_if_false,
        activity: 0,
    }
}

fn parse_monkeys(input: String) -> Vec<Monkey> {
    input
        .lines()
        .chunks(7)
        .into_iter()
        .map(take_monkey)
        .collect()
}

fn monkey_business_level(mut monkeys: Vec<Monkey>) -> u64 {
    monkeys.sort_by_key(|monkey| monkey.activity);
    monkeys.reverse();
    monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.activity)
        .product()
}

fn main() {
    let mut monkeys = parse_monkeys(read_input());
    let divisor_product: i64 = monkeys.iter().map(|monkey| monkey.test_divisor).product();

    (0..10000).for_each(|_| {
        (0..monkeys.len()).for_each(|monkey_idx| {
            while !monkeys[monkey_idx].items.is_empty() {
                monkeys[monkey_idx].activity += 1;
                let item = monkeys[monkey_idx].items.remove(0);
                let new_worry_level =
                    (monkeys[monkey_idx].compute_new_worry_level)(item) % divisor_product;
                let next_monkey = if new_worry_level % monkeys[monkey_idx].test_divisor == 0 {
                    monkeys[monkey_idx].next_monkey_if_true
                } else {
                    monkeys[monkey_idx].next_monkey_if_false
                };
                monkeys[next_monkey].items.push(new_worry_level);
            }
        });
    });
    println!("part 1: {}", monkey_business_level(monkeys));
}
