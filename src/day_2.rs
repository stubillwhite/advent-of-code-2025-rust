use std::fs;

#[derive(Debug)]
struct Range {
    start: i64,
    end: i64,
}

pub fn problem_input() -> String {
    fs::read_to_string("resources/day-2-input.txt").expect("Unable to read file")
}

fn parse_input(input: String) -> Vec<Range> {
    input.split(",")
        .map(|line| {
            let parts: Vec<_> = line.split('-').collect();
            let start = parts[0].parse().unwrap();
            let end = parts[1].parse().unwrap();
            Range { start, end }
        })
        .collect()
}

fn invalid_ids_in_range(is_invalid: fn(i64) -> bool, range: &Range) -> Vec<i64> {
    (range.start..=range.end)
        .filter(|id| is_invalid(*id))
        .collect()
}

fn string_chunks(s: &str, n: usize) -> Vec<String> {
    let chars = s.chars().collect::<Vec<char>>();
    
    chars.chunks(n)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
}

fn repeated_chunks(x: i64) -> Vec<Vec<String>> {
    let s = x.to_string();
    let len = s.len();

    let chunk_sizes = (1..=len / 2)
        .filter(|x| len.is_multiple_of(*x))
        .collect::<Vec<usize>>();

    chunk_sizes
        .iter()
        .map(|n| string_chunks(&s, *n))
        .filter(|chunk| all_equal(chunk))
        .collect::<Vec<Vec<String>>>()
}

fn all_equal<T: PartialEq>(xs: &[T]) -> bool {
    match xs {
        [] => true,
        [_] => true,
        [a, b, ..] if a != b => false,
        [_, ..] => all_equal(&xs[1..])
    }
}

fn has_two_repeated_chunks(x: i64) -> bool {
    repeated_chunks(x).iter().any(|chunks| chunks.len() == 2)
}

pub fn solution_part_one(input: String) -> i64 {
    let ranges = parse_input(input);

    ranges.iter()
        .flat_map(|range| invalid_ids_in_range(has_two_repeated_chunks, range))
        .sum()
}

// Part two

fn has_at_least_two_repeated_chunks(x: i64) -> bool {
    repeated_chunks(x).iter().any(|chunks| chunks.len() >= 2)
}

pub fn solution_part_two(input: String) -> i64 {
    let ranges = parse_input(input);

    ranges.iter()
        .flat_map(|range| invalid_ids_in_range(has_at_least_two_repeated_chunks, range))
        .sum()
}
