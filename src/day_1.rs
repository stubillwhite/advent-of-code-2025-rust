use std::fs;

#[derive(Debug)]
struct ProblemInput {
    direction: String,
    distance: i64,
}

fn parse_line(line: &str) -> ProblemInput {
    let direction = line[0..1].to_string();
    let distance = line[1..].parse().unwrap();

    ProblemInput {
        direction,
        distance,
    }
}

fn parse_input(input: String) -> Vec<ProblemInput> {
    input.lines().map(parse_line).collect()
}

#[derive(Debug)]
struct Safe {
    position: i64,
    zeroes: i64,
}

pub fn problem_input() -> String {
    fs::read_to_string("resources/day-1-input.txt").expect("Unable to read file")
}

fn count_zeroes_hit(safe: Safe, input: &ProblemInput) -> Safe {
    let new_position = if input.direction == "L" {
        (safe.position - input.distance).rem_euclid(100)
    } else {
        (safe.position + input.distance).rem_euclid(100)
    };

    let new_zeroes = safe.zeroes + (if new_position == 0 { 1 } else { 0 });

    Safe {
        position: new_position,
        zeroes: new_zeroes,
    }
}

pub fn solution_part_one(input: String) -> i64 {
    let final_state = parse_input(input).iter().fold(
        Safe {
            position: 50,
            zeroes: 0,
        },
        count_zeroes_hit,
    );

    final_state.zeroes
}

// Part two

fn count_zeroes_passed(safe: Safe, input: &ProblemInput) -> Safe {
    match input.direction.as_str() {
        "R" => {
            let new_position = (safe.position + input.distance).rem_euclid(100);
            let new_zeroes = safe.zeroes + ((input.distance + safe.position) / 100);

            Safe {
                position: new_position,
                zeroes: new_zeroes,
            }
        }
        "L" => {
            let new_position = (safe.position - input.distance).rem_euclid(100);
            let passed_zero = safe.position > 0 && (safe.position - input.distance) <= 0;
            let new_zeroes = safe.zeroes
                + ((input.distance - safe.position) / 100).abs()
                + if passed_zero { 1 } else { 0 };

            Safe {
                position: new_position,
                zeroes: new_zeroes,
            }
        }
        _ => panic!("Invalid direction {}", input.direction.as_str()),
    }
}

pub fn solution_part_two(input: String) -> i64 {
    let final_state = parse_input(input).iter().fold(
        Safe {
            position: 50,
            zeroes: 0,
        },
        count_zeroes_passed,
    );

    final_state.zeroes
}
