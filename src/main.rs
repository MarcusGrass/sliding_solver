pub mod solver;
pub mod tools;
use crate::solver::solve_puzzle;
use crate::tools::{print_moves, puzzle_from_string};
use std::fs;

fn main() {
    const FILE_NAME: &str = "test_input/maps_moves.txt";
    let input = fs::read_to_string(FILE_NAME).expect("File not found.");
    let mut i = 0;
    for line in input.lines() {
        i += 1;
        let (board, state) = puzzle_from_string(line);
        let (_, _, moves) = solve_puzzle(&board, state).unwrap();
        if i % 20 == 0 {
            println!("Puzzle {}, sol found: {}", i, moves.len());
            print_moves(&moves);
        }
    }
}
