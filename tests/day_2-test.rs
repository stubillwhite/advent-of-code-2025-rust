use advent_of_code_2025_rust::day_2::problem_input;
use advent_of_code_2025_rust::day_2::solution_part_one;
use advent_of_code_2025_rust::day_2::solution_part_two;

fn example_input() -> String {
    "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124".to_string()
}


#[test]
fn solution_part_one_given_example_input_then_example_result() {
    assert_eq!(solution_part_one(example_input()), 1227775554);
}

#[test]
fn solution_part_one_given_problem_input_then_problem_result() {
    assert_eq!(solution_part_one(problem_input()), 9188031749);
}

#[test]
fn solution_part_two_given_example_input_then_example_result() {
    assert_eq!(solution_part_two(example_input()), 4174379265);
}

#[test]
fn solution_part_two_given_problem_input_then_problem_result() {
    assert_eq!(solution_part_two(problem_input()), 11323661261);
}
