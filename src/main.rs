pub mod solver;
pub mod tools;
use crate::solver::solve_puzzle;
use crate::tools::puzzle_from_string;
use std::fs;
use std::time::Instant;

fn main() {
    test1000();
    test_diff_size();
}

fn test1000() {
    const FILE_NAME: &str = "test_input/maps_moves.txt";
    let input = fs::read_to_string(FILE_NAME).expect("File not found.");
    let mut i = 0;

    let before = Instant::now();
    for line in input.lines() {
        i += 1;
        if i % 1 == 20 {
            let now = Instant::now();
            let (board, state) = puzzle_from_string(line);
            let (_, _, _) = solve_puzzle(&board, state).unwrap();
            let ms = now.elapsed().as_micros();
            println!(
                "Puzzle {}, solution found in: {}.{}ms",
                i,
                ms / 1000,
                ms % 1000
            );
        } else {
            let (board, state) = puzzle_from_string(line);
            let (_, _, _) = solve_puzzle(&board, state).unwrap();
        }
    }
    let ms = before.elapsed().as_micros();
    println!(
        "All {} solutions found in: {}.{}ms",
        i,
        ms / 1000,
        ms % 1000
    );
}

fn test_diff_size() {
    const FILE_NAME: &str = "test_input/tests100.json";
    let input = fs::read_to_string(FILE_NAME).expect("File not found.");
    let parsed = json::parse(&input).unwrap();
    let mut i = 0;
    for item in parsed.members() {
        i += 1;
        let (board, state) = puzzle_from_string(item["map"].as_str().unwrap());
        let (_, _, moves) = solve_puzzle(&board, state).unwrap();
        let opt = item["optimal"].as_usize().unwrap();
        println!("Puzzle {}, sol found: {}, sol: {}", i, moves.len(), opt);
        assert_eq!(moves.len(), opt);
    }
}
