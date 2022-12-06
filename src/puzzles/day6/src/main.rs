use itertools::Itertools;
use utils::read_input;

fn count_chars_to_end_of_unique_window(chars: &[char], window_size: usize) -> usize {
    chars
        .windows(window_size)
        .enumerate()
        .find_map(|(idx, window)| window.iter().all_unique().then_some(idx + window_size))
        .unwrap_or_else(|| panic!("found no unique window"))
}

fn main() {
    let input_chars: Vec<_> = read_input().chars().collect();
    let part_1_answer = count_chars_to_end_of_unique_window(&input_chars, 4);
    let part_2_answer = count_chars_to_end_of_unique_window(&input_chars, 14);
    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
}
