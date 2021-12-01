use std::fs;
mod day1;

fn main() {
    run_day_one();
}

fn run_day_one() {
    let file_content = fs::read_to_string("src/puzzle_inputs/day1.input.txt")
        .expect("Something went wrong reading the file");
    let puzzle_input: Vec<u16> = file_content
        .trim()
        .split('\n')
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    println!("Part one: {}", day1::part_one(&puzzle_input));
    println!("Part two: {}", day1::part_two(&puzzle_input));
}
