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
                .map(|str_num| str_num.parse::<i64>().unwrap())
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
            let parts: Vec<_> = line
                .trim()
                .strip_prefix("Operation: new = ")
                .unwrap()
                .split(' ')
                .map(|part| part.to_owned())
                .collect();

            Box::new(move |old_worry_level: i64| {
                let lhs = match parts[0].as_str() {
                    "old" => old_worry_level,
                    other_str => other_str.parse::<i64>().unwrap(),
                };
                let rhs = match parts[2].as_str() {
                    "old" => old_worry_level,
                    other_str => other_str.parse::<i64>().unwrap(),
                };
                match parts[1].as_str() {
                    "*" => lhs * rhs,
                    "+" => lhs + rhs,
                    _ => panic!(),
                }
            })
        })
        .unwrap()
}

fn take_test_divisor<'a>(lines: &mut impl Iterator<Item = &'a str>) -> i64 {
    lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("Test: divisible by ")
        .unwrap()
        .parse::<i64>()
        .unwrap()
}

fn take_next_monkey_if_true<'a>(lines: &mut impl Iterator<Item = &'a str>) -> usize {
    lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("If true: throw to monkey ")
        .unwrap()
        .parse()
        .unwrap()
}

fn take_next_monkey_if_false<'a>(lines: &mut impl Iterator<Item = &'a str>) -> usize {
    lines
        .next()
        .unwrap()
        .trim()
        .strip_prefix("If false: throw to monkey ")
        .unwrap()
        .parse()
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

fn monkey_business_level<F: Fn(i64) -> i64>(
    mut monkeys: Vec<Monkey>,
    rounds: u32,
    worry_level_adjustment: F,
) -> u64 {
    (0..rounds).for_each(|_| {
        (0..monkeys.len()).for_each(|monkey_idx| {
            while !monkeys[monkey_idx].items.is_empty() {
                monkeys[monkey_idx].activity += 1;
                let item = monkeys[monkey_idx].items.remove(0);
                let new_worry_level =
                    worry_level_adjustment((monkeys[monkey_idx].compute_new_worry_level)(item));
                let next_monkey = if new_worry_level % monkeys[monkey_idx].test_divisor == 0 {
                    monkeys[monkey_idx].next_monkey_if_true
                } else {
                    monkeys[monkey_idx].next_monkey_if_false
                };
                monkeys[next_monkey].items.push(new_worry_level);
            }
        });
    });

    monkeys.sort_by_key(|monkey| monkey.activity);
    monkeys.reverse();
    monkeys
        .into_iter()
        .take(2)
        .map(|monkey| monkey.activity)
        .product()
}

fn main() {
    let monkeys = parse_monkeys(read_input());
    println!(
        "part 1: {}",
        monkey_business_level(monkeys, 20, |worry_level| worry_level / 3)
    );

    let monkeys = parse_monkeys(read_input());
    let divisor_product: i64 = monkeys.iter().map(|monkey| monkey.test_divisor).product();
    println!(
        "part 2: {}",
        monkey_business_level(monkeys, 10000, |worry_level| worry_level % divisor_product)
    );
}
