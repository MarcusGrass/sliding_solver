// TODO: Stop copying the move list.
// TODO: Figure out when to use references. Am always copying atm.
// TODO: Find better way to organize state.

use std::time::Instant;
pub mod solver;
pub mod tools;
use crate::solver::solve_puzzle;
use crate::tools::print_board;
use crate::tools::puzzle_from_string;

fn main() {
    let input = "map:8:8:main_robot:0:0:goal:2:4:helper_robot:7:0:helper_robot:7:1:blocker:2:0:blocker:5:0:blocker:6:0:blocker:3:3:blocker:2:5:blocker:3:6:blocker:1:7:blocker:4:7";
    let (board, state) = puzzle_from_string(&input);
    let now = Instant::now();
    for i in 1..100 {
        solve_puzzle(&board, state);
    }
    let res = solve_puzzle(&board, state);
    let elapsed = now.elapsed();

    match res {
        None => println!("Could not solve puzzle."),
        Some((board, state, vsize)) => {
            print_board(board, state);
            println!("Nodes visited: {}", vsize);
        }
    }

    println!("Elapsed solving: {:.2?}", elapsed);
}
