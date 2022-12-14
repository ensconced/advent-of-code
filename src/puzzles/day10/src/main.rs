use utils::read_input;

fn main() {
    let mut signal_strengths_for_part_1 = vec![];
    let mut register_x = 1;
    let mut cycle = 0;
    let mut pixels = vec![];

    let mut run_cycle = |register_x: i32| {
        let pixel = if ((cycle % 40) - register_x).abs() <= 1 {
            '#'
        } else {
            '.'
        };
        pixels.push(pixel);
        cycle += 1;
        if (cycle % 40) == 20 {
            signal_strengths_for_part_1.push(register_x * cycle);
        }
    };

    read_input().lines().for_each(|line| {
        if line == "noop" {
            run_cycle(register_x);
        } else {
            let parts: Vec<_> = line.split(' ').collect();
            run_cycle(register_x);
            run_cycle(register_x);
            register_x += parts[1].parse::<i32>().unwrap();
        }
    });

    let part_1_answer = signal_strengths_for_part_1.into_iter().sum::<i32>();
    println!("part 1: {}", part_1_answer);

    println!("part 2:");
    pixels.chunks(40).for_each(|chunk| {
        for ch in chunk {
            print!("{}", ch);
        }
        println!();
    })
}
