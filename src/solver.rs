use std::collections::{HashSet, VecDeque};
use std::rc::Rc;

#[derive(Hash, Debug, Clone, Copy, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub enum PieceType {
    HelperOne,
    HelperTwo,
    Main,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BoardPiece {
    Start,
    Goal,
    Blocker,
    Empty,
    Helper,
    Main,
}

pub type State = (i8, i8, i8, i8, i8, i8, bool);

pub type Board = Vec<Vec<BoardPiece>>;

type Position = (i8, i8);

type VisitedCount = usize;

type Move = (PieceType, Direction);

struct Node {
    m: Option<Move>,
    state: State,
    prev: Option<Rc<Node>>,
}

impl Node {
    fn len(&self) -> usize {
        let mut len = 0;
        let mut current = self;
        while let Some(prev) = &current.prev {
            len += 1;
            current = prev.as_ref();
        }
        len
    }
}

fn step(pos: Position, dir: Direction) -> Position {
    match dir {
        Direction::Up => (pos.0, pos.1 - 1),
        Direction::Down => (pos.0, pos.1 + 1),
        Direction::Left => (pos.0 - 1, pos.1),
        Direction::Right => (pos.0 + 1, pos.1),
    }
}

fn try_move(pos: Position, board: &Board, state: State) -> Option<Position> {
    let (ix, iy) = pos;

    let out_of_bounds = ix < 0 || ix >= board[0].len() as i8 || iy < 0 || iy >= board.len() as i8;

    if out_of_bounds {
        return None;
    };

    let (ux, uy) = (ix as usize, iy as usize);

    match board[uy][ux] {
        BoardPiece::Blocker => return None,
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
    board: &Board,
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
    board: &Board,
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

fn neighbourhood(board: &Board, state: State) -> Vec<(Move, State)> {
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

pub fn solve_puzzle(board: &Board, state: State) -> Option<(&Board, State, VisitedCount)> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(state);

    while let Some(state) = queue.pop_front() {
        let sol_found = state.6 && board[state.1 as usize][state.0 as usize] == BoardPiece::Start;

        if sol_found {
            return Some((board, state, visited.len())); // Solution found, yay!
        }

        for (_, state) in neighbourhood(board, state) {
            if visited.contains(&state) {
                continue;
            }

            queue.push_back(state);
            visited.insert(state);
        }
    }

    None // Exhausted search, no solution found.
}

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn solver() {
//         const FILE_NAME: &str = "test_input/tests100.json";
//         let input = fs::read_to_string(FILE_NAME).expect("File not found.");
//         let parsed = json::parse(&input).unwrap();
//         let mut i = 0;
//         for item in parsed.members() {
//             i += 1;
//             let (board, state) = puzzle_from_string(item["map"].as_str().unwrap());
//             let (_, _, moves, _) = solve_puzzle(&board, state).unwrap();
//             let opt = item["optimal"].as_usize().unwrap();
//             println!("Puzzle {}, sol found: {}, sol: {}", i, moves.len(), opt);
//             assert_eq!(moves.len(), opt);
//         }
//     }
// }
