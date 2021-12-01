pub fn part_one(puzzle_input: &Vec<u16>) -> u16 {
    let mut result = 0;
    for i in 0..puzzle_input.len() - 1 {
        if puzzle_input[i + 1] > puzzle_input[i] {
            result = result + 1;
        }
    }
    
    result
}

pub fn part_two(puzzle_input: &Vec<u16>) -> u16 {
    let mut result = 0;
    for i in 0..puzzle_input.len() - 3 {
        let first_measure = puzzle_input[i] + puzzle_input[i + 1] + puzzle_input[i + 2];
        let second_measure = puzzle_input[i + 1] + puzzle_input[i + 2] + puzzle_input[i + 3];

        if  second_measure > first_measure {
            result = result + 1;
        }
    }
    
    result
}