use advent_of_code_2025_rust::day_1::problem_input;
use advent_of_code_2025_rust::day_1::solution_part_one;
use advent_of_code_2025_rust::day_1::solution_part_two;

fn example_input() -> String {
    return [
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ]
    .join("\n")
    .to_string();
}

#[test]
fn solution_part_one_given_example_input_then_example_result() {
    assert_eq!(solution_part_one(example_input()), 3);
}

#[test]
fn solution_part_one_given_problem_input_then_problem_result() {
    assert_eq!(solution_part_one(problem_input()), 1097);
}

#[test]
fn solution_part_two_given_example_input_then_example_result() {
    assert_eq!(solution_part_two(example_input()), 6);
}

#[test]
fn solution_part_two_given_problem_input_then_problem_result() {
    assert_eq!(solution_part_two(problem_input()), 7101);
}
