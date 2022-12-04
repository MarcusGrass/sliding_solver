use std::collections::{HashSet, VecDeque};
use std::thread::sleep;
use std::time::Duration;
use std::time::Instant;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
enum PieceType {
    Helper1,
    Helper2,
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

fn next_position(
    board: Board,
    state: State,
    pos: Position,
    direction: Direction,
) -> Option<Position> {
    let mut new_pos = step(pos, direction);
    try_collision(new_pos, board, state)?;

    while let Some(pos) = try_collision(step(new_pos, direction), board, state) {
        new_pos = pos;
    }

    Some(new_pos)
}

fn try_collision(pos: Position, board: Board, state: State) -> Option<Position> {
    let (ix, iy) = pos;
    if ix < 0 || ix > 7 || iy < 0 || iy > 7 {
        return None;
    };

    let (ux, uy) = (ix as usize, iy as usize);

    if board[uy][ux] == BoardPiece::Blocker
        || board[uy][ux] == BoardPiece::Helper
        || board[uy][ux] == BoardPiece::Main
    {
        return None;
    };

    if (state.0, state.1) == (ix, iy)
        || (state.2, state.3) == (ix, iy)
        || (state.4, state.5) == (ix, iy)
    {
        return None;
    }
    Some(pos)
}

fn move_piece(
    board: Board,
    state: State,
    piece_type: PieceType,
    direction: Direction,
) -> Option<State> {
    let start_pos = match piece_type {
        PieceType::Main => (state.0, state.1),
        PieceType::Helper1 => (state.2, state.3),
        PieceType::Helper2 => (state.4, state.5),
    };
    let new_pos = next_position(board, state, start_pos, direction)?;

    Some(match piece_type {
        PieceType::Main => {
            let goal_visited =
                state.6 || board[new_pos.1 as usize][new_pos.0 as usize] == BoardPiece::Goal;
            (
                new_pos.0,
                new_pos.1,
                state.2,
                state.3,
                state.4,
                state.5,
                goal_visited,
            )
        }
        PieceType::Helper1 => (
            state.0, state.1, new_pos.0, new_pos.1, state.4, state.5, state.6,
        ),
        PieceType::Helper2 => (
            state.0, state.1, state.2, state.3, new_pos.0, new_pos.1, state.6,
        ),
    })
}

fn next_states(board: Board, state: State) -> Vec<(PieceType, Direction, State)> {
    use Direction::*;
    use PieceType::*;

    let mut states = Vec::new();

    for direction in [Up, Down, Left, Right] {
        for piece in [Main, Helper1, Helper2] {
            if let Some(state) = move_piece(board, state, piece, direction) {
                states.push((piece, direction, state));
            }
        }
    }

    states
}

type Moves = Vec<(PieceType, Direction)>;
type Result = (Board, State, Moves, usize);

fn solve_puzzle(board: Board, state: State) -> Option<Result> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((state, Vec::new()));

    while let Some((state, moves)) = queue.pop_front() {
        print!("\x1B[2J\x1B[1;1H");
        print_board(board, state);
        sleep(Duration::from_millis(1));
        let sol_found = state.6 && board[state.1 as usize][state.0 as usize] == BoardPiece::Start;
        if sol_found {
            return Some((board, state, moves, visited.len()));
        }

        for (piece, dir, state) in next_states(board, state) {
            if visited.contains(&state) {
                continue;
            }
            let mut new_moves = moves.clone();
            let new_move = (piece, dir);
            new_moves.push(new_move);
            queue.push_back((state, new_moves));
            visited.insert(state);
        }
    }

    None // Exhausted search, no solution found
}

fn puzzle_from_string(input: &str) -> (Board, State) {
    let mut board = [[BoardPiece::Empty; 8]; 8];
    let mut state = (0, 0, 0, 0, 0, 0, false);
    let items = input.split(",");
    for item in items {
        let parts: Vec<&str> = item.split(":").collect();
        match parts[0] {
            "main" => {
                state.0 = parts[1].parse::<i8>().unwrap();
                state.1 = parts[2].parse::<i8>().unwrap();
                board[state.1 as usize][state.0 as usize] = BoardPiece::Start;
            }
            "helper1" => {
                state.2 = parts[1].parse::<i8>().unwrap();
                state.3 = parts[2].parse::<i8>().unwrap();
            }
            "helper2" => {
                state.4 = parts[1].parse::<i8>().unwrap();
                state.5 = parts[2].parse::<i8>().unwrap();
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
fn print_move(m: (PieceType, Direction)) {
    let (piece, dir) = m;
    match piece {
        PieceType::Main => print!("Main "),
        PieceType::Helper1 => print!("Helper1 "),
        PieceType::Helper2 => print!("Helper2 "),
    }
    match dir {
        Direction::Up => print!("Up"),
        Direction::Down => print!("Down"),
        Direction::Left => print!("Left"),
        Direction::Right => print!("Right"),
    }
    println!();
}

fn print_moves(moves: Vec<(PieceType, Direction)>) {
    for m in moves {
        print_move(m);
    }
}

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
    let pstr = "main:0:0,goal:2:4,helper1:7:0,helper2:7:1,blocker:2:0,blocker:5:0,blocker:6:0,blocker:3:3,blocker:2:5,blocker:3:6,blocker:1:7,blocker:4:7";
    let (board, state) = puzzle_from_string(&pstr);
    print_board(board, state);

    let now = Instant::now();
    let res = solve_puzzle(board, state);
    let elapsed = now.elapsed();

    match res {
        None => println!("Could not solve puzzle."),
        Some((board, state, moves, vsize)) => {
            print_board(board, state);
            print_moves(moves);
            println!("Nodes visited: {}", vsize);
            println!("Nodes visited: {}", vsize);
        }
    }

    println!("Elapsed solving: {:.2?}", elapsed);
}
