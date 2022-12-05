// TODO: Stop copying the move list.
// TODO: Figure out when to use references. Am always copying atm.
// TODO: Find better way to organize state.

use std::collections::{HashSet, VecDeque};
use std::time::Instant;

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum PieceType {
    HelperOne,
    HelperTwo,
    Main,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum BoardPiece {
    Start,
    Goal,
    Blocker,
    Empty,
    Helper,
    Main,
}

type State = (i8, i8, i8, i8, i8, i8, bool);

type Board = [[BoardPiece; 8]; 8];

type Position = (i8, i8);

fn step(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn try_move(pos: Position, board: Board, state: State) -> Option<Position> {
    let (ix, iy) = pos;

    let out_of_bounds = ix < 0 || ix >= board[0].len() as i8 || iy < 0 || iy >= board.len() as i8;

    if out_of_bounds {
        return None;
    };

    let (ux, uy) = (ix as usize, iy as usize);

    match board[uy][ux] {
        BoardPiece::Blocker => return None,
        BoardPiece::Helper => return None,
        BoardPiece::Main => return None,
        _ => {}
    };

    if (state.0, state.1) == (ix, iy) // TODO: Better way to write?
        || (state.2, state.3) == (ix, iy)
        || (state.4, state.5) == (ix, iy)
    {
        return None;
    }

    Some(pos)
}

fn next_position(
    board: Board,
    state: State,
    pos: Position,
    direction: Direction,
) -> Option<Position> {
    let mut new_pos = step(pos, direction);
    try_move(new_pos, board, state)?;

    while let Some(pos) = try_move(step(new_pos, direction), board, state) {
        new_pos = pos;
    }

    Some(new_pos)
}

fn move_piece(
    board: Board,
    state: State,
    piece_type: PieceType,
    direction: Direction,
) -> Option<State> {
    let start_pos = match piece_type {
        PieceType::Main => (state.0, state.1),
        PieceType::HelperOne => (state.2, state.3),
        PieceType::HelperTwo => (state.4, state.5),
    };

    let pos = next_position(board, state, start_pos, direction)?;

    Some(match piece_type {
        PieceType::Main => {
            let goal_found = state.6 || board[pos.1 as usize][pos.0 as usize] == BoardPiece::Goal;
            (pos.0, pos.1, state.2, state.3, state.4, state.5, goal_found)
        }
        PieceType::HelperOne => (state.0, state.1, pos.0, pos.1, state.4, state.5, state.6),
        PieceType::HelperTwo => (state.0, state.1, state.2, state.3, pos.0, pos.1, state.6),
    })
}

fn neighbourhood(board: Board, state: State) -> Vec<(Move, State)> {
    use Direction::*;
    use PieceType::*;

    let mut states = Vec::new();

    for piece in [Main, HelperOne, HelperTwo] {
        for direction in [Left, Right, Up, Down] {
            if let Some(state) = move_piece(board, state, piece, direction) {
                let move_ = (piece, direction);
                states.push((move_, state));
            }
        }
    }

    states
}

// fn get_nop_moves(move_: Move) -> HashSet<Move> {
//     let mut nop_moves = HashSet::new();
//     let (piece_type, dir) = move_;
//     match dir {
//         Direction::Up | Direction::Down => {
//             nop_moves.insert((piece_type, Direction::Down));
//             nop_moves.insert((piece_type, Direction::Up));
//         }
//         Direction::Right | Direction::Left => {
//             nop_moves.insert((piece_type, Direction::Right));
//             nop_moves.insert((piece_type, Direction::Left));
//         }
//     };
//     nop_moves
// }

type VisitedCount = usize;

type Move = (PieceType, Direction);
type Solution = (Board, State, Vec<Move>, VisitedCount);

fn solve_puzzle(board: Board, state: State) -> Option<Solution> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((state, Vec::new()));

    while let Some((state, moves)) = queue.pop_front() {
        let sol_found = state.6 && board[state.1 as usize][state.0 as usize] == BoardPiece::Start;

        if sol_found {
            return Some((board, state, moves, visited.len())); // Solution found, yay!
        }

        // let mut nop_moves = HashSet::new();
        // if !moves.is_empty() {
        //     nop_moves = get_nop_moves(*moves.last().unwrap());
        // }

        for (move_, state) in neighbourhood(board, state) {
            if visited.contains(&state) {
                continue;
            }

            let mut new_moves = moves.clone(); //TODO: Avoid
            new_moves.push(move_);

            queue.push_back((state, new_moves));
            visited.insert(state);
        }
    }

    None // Exhausted search, no solution found.
}

// TODO: Improve parser.
fn puzzle_from_string(input: &str) -> (Board, State) {
    let items: Vec<&str> = input.split(":").collect();
    let mut board = [[BoardPiece::Empty; 8]; 8];
    let mut state = (0, 0, 0, 0, 0, 0, false);
    let mut first_helper_found = false;
    for parts in items.chunks(3) {
        match parts[0] {
            "main_robot" => {
                state.0 = parts[1].parse::<i8>().unwrap();
                state.1 = parts[2].parse::<i8>().unwrap();
                board[state.1 as usize][state.0 as usize] = BoardPiece::Start;
            }
            "helper_robot" => {
                if first_helper_found {
                    state.4 = parts[1].parse::<i8>().unwrap();
                    state.5 = parts[2].parse::<i8>().unwrap();
                } else {
                    first_helper_found = true;
                    state.2 = parts[1].parse::<i8>().unwrap();
                    state.3 = parts[2].parse::<i8>().unwrap();
                }
            }
            "goal" => {
                let (x, y) = (
                    parts[1].parse::<usize>().unwrap(),
                    parts[2].parse::<usize>().unwrap(),
                );
                board[y][x] = BoardPiece::Goal;
            }
            "blocker" => {
                let (x, y) = (
                    parts[1].parse::<usize>().unwrap(),
                    parts[2].parse::<usize>().unwrap(),
                );
                board[y][x] = BoardPiece::Blocker;
            }
            _ => {}
        }
    }
    (board, state)
}
// TODO: Improve print.
fn print_move(m: &(PieceType, Direction)) {
    let (piece, dir) = m;
    match piece {
        PieceType::Main => print!("Main "),
        PieceType::HelperOne => print!("Helper1 "),
        PieceType::HelperTwo => print!("Helper2 "),
    }
    match dir {
        Direction::Up => print!("Up"),
        Direction::Down => print!("Down"),
        Direction::Left => print!("Left"),
        Direction::Right => print!("Right"),
    }
    println!();
}

fn print_moves(moves: &Vec<(PieceType, Direction)>) {
    for m in moves {
        print_move(m);
    }
}

// TODO: Improve print
fn print_board(mut board: Board, state: State) {
    let (mx, my) = (state.0, state.1);
    let (h1x, h1y) = (state.2, state.3);
    let (h2x, h2y) = (state.4, state.5);
    board[my as usize][mx as usize] = BoardPiece::Main;
    board[h1y as usize][h1x as usize] = BoardPiece::Helper;
    board[h2y as usize][h2x as usize] = BoardPiece::Helper;

    println!("==========");
    for line in board {
        let mut output_line = "|".to_string();
        for piece in line {
            let sign = match piece {
                BoardPiece::Main => "M",
                BoardPiece::Helper => "H",
                BoardPiece::Blocker => "#",
                BoardPiece::Goal => "o",
                BoardPiece::Empty => " ",
                BoardPiece::Start => "+",
            };
            output_line = output_line + sign;
        }
        output_line = output_line + "|";
        println!("{output_line}");
    }
    println!("==========");
}

fn main() {
    let input = "map:8:8:helper_robot:1:0:blocker:7:0:blocker:1:1:blocker:3:1:goal:4:1:main_robot:5:1:blocker:6:1:blocker:2:2:helper_robot:1:3:blocker:6:5:blocker:7:6:blocker:1:7";
    let (board, state) = puzzle_from_string(&input);
    print_board(board, state);

    let now = Instant::now();
    let res = solve_puzzle(board, state);
    let elapsed = now.elapsed();

    match res {
        None => println!("Could not solve puzzle."),
        Some((board, state, moves, vsize)) => {
            print_board(board, state);
            print_moves(&moves);
            println!("Nodes visited: {}", vsize);
            println!("Length of solution: {}", moves.len());
        }
    }

    println!("Elapsed solving: {:.2?}", elapsed);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solver() {
        let input = "map:8:8:helper_robot:1:0:blocker:7:0:blocker:1:1:blocker:3:1:goal:4:1:main_robot:5:1:blocker:6:1:blocker:2:2:helper_robot:1:3:blocker:6:5:blocker:7:6:blocker:1:7";
        let (board, state) = puzzle_from_string(input);
        let (_, _, moves, _) = solve_puzzle(board, state).unwrap();
        assert_eq!(moves.len(), 2);
    }
}
