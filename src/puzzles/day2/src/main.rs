use utils::read_input;

#[derive(Clone, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn from_char(s: char) -> Self {
        match s {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("unexpected input"),
        }
    }

    fn winning_shape_against(other: &Self) -> Self {
        match other {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn losing_shape_against(other: &Self) -> Self {
        match other {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn shape_score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn outcome_score_against(&self, other: &Self) -> u32 {
        if self == other {
            3
        } else if self == &Self::winning_shape_against(other) {
            6
        } else {
            0
        }
    }
}

fn part_1_game_score(game: &str) -> u32 {
    let chars: Vec<_> = game.chars().collect();
    let their_shape = Shape::from_char(chars[0]);
    let my_shape = Shape::from_char(chars[2]);
    my_shape.shape_score() + my_shape.outcome_score_against(&their_shape)
}

fn part_2_game_score(game: &str) -> u32 {
    let chars: Vec<_> = game.chars().collect();
    let their_shape = Shape::from_char(chars[0]);
    let my_shape = match chars[2] {
        'X' => Shape::losing_shape_against(&their_shape),
        'Y' => their_shape.clone(),
        'Z' => Shape::winning_shape_against(&their_shape),
        _ => panic!("unexpected input"),
    };
    my_shape.shape_score() + my_shape.outcome_score_against(&their_shape)
}

fn main() {
    let input = read_input();
    let part_1_answer = input.lines().map(part_1_game_score).sum::<u32>();
    let part_2_answer = input.lines().map(part_2_game_score).sum::<u32>();
    println!("part 1: {}", part_1_answer);
    println!("part 2: {}", part_2_answer);
    assert_eq!(part_1_answer, 11906);
    assert_eq!(part_2_answer, 11186);
}
