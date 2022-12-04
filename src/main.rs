use std::collections::{HashSet, VecDeque};

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

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Piece {
    ptype: PieceType,
    pos: Position,
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

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct State {
    pieces: [Piece; 3],
    goal_visited: bool,
}

type Board = [[BoardPiece; 8]; 8];
type Position = (i8, i8);

fn step(pos: Position, dir: Direction) -> Position {
    let (dx, dy) = match dir {
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
    };

    let (x, y) = pos;

    (x + dx, y + dy)
}

fn next_position(
    board: Board,
    state: State,
    piece: Piece,
    direction: Direction,
) -> Option<Position> {
    let mut new_pos = step(piece.pos, direction);
    if try_collision(new_pos, board, state) {
        return None;
    };

    let mut prev_pos = new_pos;

    while !try_collision(new_pos, board, state) {
        prev_pos = new_pos;
        new_pos = step(new_pos, direction);
    }

    Some(prev_pos)
}

fn try_collision(pos: Position, board: Board, state: State) -> bool {
    let (sx, sy) = pos;

    if sx < 0 || sx > 7 || sy < 0 || sy > 7 {
        return true;
    };

    if board[sx as usize][sy as usize] == BoardPiece::Blocker
        || board[sx as usize][sy as usize] == BoardPiece::Helper
        || board[sx as usize][sy as usize] == BoardPiece::Main
    {
        return true;
    };

    // We can not collide with ourselves since we check after first step
    for other in state.pieces {
        if other.pos == (sx, sy) {
            return true;
        }
    }
    return false;
}

// Returns the next position that the given piece would reach if it
// Returns a new State with the given piece moved in the given direction. If the piece
// cannot be moved in the given direction, returns None.
fn move_piece(board: Board, state: State, piece: Piece, direction: Direction) -> Option<State> {
    let next_position = next_position(board, state, piece, direction)?;

    let new_piece = Piece {
        ptype: piece.ptype,
        pos: next_position,
    };
    Some(match piece.ptype {
        PieceType::Main => State {
            pieces: [new_piece, state.pieces[1], state.pieces[2]],
            goal_visited: board[next_position.0 as usize][next_position.1 as usize]
                == BoardPiece::Goal
                || state.goal_visited,
        },
        PieceType::Helper1 => State {
            pieces: [state.pieces[0], new_piece, state.pieces[2]],
            goal_visited: false,
        },
        PieceType::Helper2 => State {
            pieces: [state.pieces[0], state.pieces[1], new_piece],
            goal_visited: false,
        },
    })
}

// Returns a Vec of all possible next states that can be reached from this state.
fn next_states(board: Board, state: State) -> Vec<State> {
    let mut states = Vec::new();

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    {
        for piece in state.pieces.iter() {
            if let Some(state) = move_piece(board, state, *piece, *direction) {
                states.push(state);
            }
        }
    }

    states
}

// Solves the puzzle and returns a sequence of moves that solves the puzzle, or None if the puzzle
// cannot be solved.
// Returns a sequence of moves that can be made to reach the target and then return to the starting
// position, or None if no such sequence of moves exists.
fn solve_puzzle(board: Board, state: State) -> Option<Vec<Direction>> {
    let mut queue = VecDeque::new();
    let mut visited_states: HashSet<State> = HashSet::new();
    queue.push_back((state, Vec::new()));

    while let Some((state, moves)) = queue.pop_front() {
        // print_board(board, state);
        // If the Main Piece has reached the target and then returned to its original position,
        // return the sequence of moves that were made.
        let (x, y) = state.pieces[0].pos;
        if state.goal_visited && board[x as usize][y as usize] == BoardPiece::Start {
            return Some(moves);
        }

        for new_state in next_states(board, state) {
            if visited_states.contains(&new_state) {
                // println!("Avoided old state.");
                continue;
            }
            queue.push_back((new_state, vec![Direction::Up]));
        }

        visited_states.insert(state);
        // println!("Queue size : {}", queue.len());
        // println!("Nodes visited: {}", visited_states.len());
    }

    // If we reach this point, we have exhausted all possible states without finding a solution.
    None
}

fn print_moves(moves: Vec<Direction>) {
    for dir in moves {
        match dir {
            Direction::Up => println!("Up"),
            Direction::Down => println!("Down"),
            Direction::Left => println!("Left"),
            Direction::Right => println!("Right"),
        }
    }
}

fn print_board(mut board: Board, state: State) {
    let (mx, my) = state.pieces[0].pos;
    let (h1x, h1y) = state.pieces[1].pos;
    let (h2x, h2y) = state.pieces[2].pos;
    board[mx as usize][my as usize] = BoardPiece::Main;
    board[h1x as usize][h1y as usize] = BoardPiece::Helper;
    board[h2x as usize][h2y as usize] = BoardPiece::Helper;

    println!("========");
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
    println!("========");
}

fn main() {
    let mut board = [[BoardPiece::Empty; 8]; 8];

    board[0][1] = BoardPiece::Blocker;
    board[1][1] = BoardPiece::Blocker;
    board[1][2] = BoardPiece::Blocker;
    board[3][7] = BoardPiece::Blocker;
    board[5][0] = BoardPiece::Blocker;
    board[6][3] = BoardPiece::Blocker;
    board[6][4] = BoardPiece::Blocker;
    board[6][5] = BoardPiece::Blocker;

    board[0][4] = BoardPiece::Start;
    board[5][4] = BoardPiece::Goal;

    let main = Piece {
        ptype: PieceType::Main,
        pos: (0, 4),
    };
    let helper1 = Piece {
        ptype: PieceType::Helper1,
        pos: (1, 4),
    };
    let helper2 = Piece {
        ptype: PieceType::Helper2,
        pos: (3, 2),
    };
    let state = State {
        pieces: [main, helper1, helper2],
        goal_visited: false,
    };
    let moves = solve_puzzle(board, state);

    match moves {
        None => println!("Could not solve puzzle."),
        Some(move_list) => print_moves(move_list),
    }
}
