use advent_of_code_2025_rust::day_1;
use advent_of_code_2025_rust::day_2;

fn main() {
    println!("Day one");
    println!(
        "  Part one: {}",
        day_1::solution_part_one(day_1::problem_input())
    );
    println!(
        "  Part two: {}",
        day_1::solution_part_two(day_1::problem_input())
    );
    
    println!("Day two");
    println!(
        "  Part one: {}",
        day_2::solution_part_one(day_2::problem_input())
    );
    println!(
        "  Part two: {}",
        day_2::solution_part_two(day_2::problem_input())
    );
}
