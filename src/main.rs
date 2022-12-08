use std::fs;
use std::time::Instant;

use crate::solver::{solve_puzzle, Board, Move, State};
use crate::tools::puzzle_from_string;

pub mod solver;
pub mod tools;

fn main() {
    test1000();
}

fn test1000() {
    const FILE_NAME: &str = "test_input/maps_moves.txt";
    let input = fs::read_to_string(FILE_NAME).expect("File not found.");
    let before = Instant::now();
    let (send, tasks) = crossbeam::channel::unbounded();
    let mut received = 0;
    rayon::scope(|handle| {
        let mut submitted = 0;
        for line in input.lines() {
            let line = line.to_string();
            submitted += 1;
            let s_c = send.clone();
            handle.spawn(move |_| {
                let res = solve_board(line);
                s_c.send(res).unwrap();
            });
        }
        while received < submitted {
            let _next = tasks.recv().unwrap().unwrap();
            received += 1;
        }
    });

    let ms = before.elapsed().as_micros();
    println!(
        "All {} solutions found in: {}.{}ms",
        received,
        ms / 1000,
        ms % 1000
    );
}

fn solve_board(line: String) -> Option<(Board, State, Vec<Move>)> {
    let (board, state) = puzzle_from_string(&line);
    let (_, state, moves) = solve_puzzle(&board, state)?;
    Some((board, state, moves))
}

fn _test_diff_size() {
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
