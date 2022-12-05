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
    fn new(m: Option<Move>, state: State, prev: Option<Rc<Self>>) -> Self {
        Self { m, state, prev }
    }
    fn moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        let mut current = self;
        while let Some(prev) = &current.prev {
            moves.push(current.m.unwrap());
            current = prev.as_ref();
        }
        moves.reverse();
        moves
    }
}

fn step((x, y): Position, dir: Direction) -> Position {
    use Direction::*;
    match dir {
        Up => (x, y - 1),
        Down => (x, y + 1),
        Left => (x - 1, y),
        Right => (x + 1, y),
    }
}

fn try_move((x, y): Position, board: &Board, state: State) -> Option<Position> {
    let out_of_bounds = x < 0 || x >= board[0].len() as i8 || y < 0 || y >= board.len() as i8;

    if out_of_bounds {
        return None;
    };

    match board[y as usize][x as usize] {
        BoardPiece::Blocker => return None,
        _ => {}
    };

    // Collison with other pieces
    if (state.0, state.1) == (x, y) || (state.2, state.3) == (x, y) || (state.4, state.5) == (x, y)
    {
        return None;
    }

    Some((x, y))
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
    use PieceType::*;
    let start_pos = match piece_type {
        Main => (state.0, state.1),
        HelperOne => (state.2, state.3),
        HelperTwo => (state.4, state.5),
    };

    let pos = next_position(board, state, start_pos, direction)?;

    Some(match piece_type {
        Main => {
            let goal_found = state.6 || board[pos.1 as usize][pos.0 as usize] == BoardPiece::Goal;
            (pos.0, pos.1, state.2, state.3, state.4, state.5, goal_found)
        }
        HelperOne => (state.0, state.1, pos.0, pos.1, state.4, state.5, state.6),
        HelperTwo => (state.0, state.1, state.2, state.3, pos.0, pos.1, state.6),
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

pub fn solve_puzzle(
    board: &Board,
    state: State,
) -> Option<(&Board, State, Vec<Move>, VisitedCount)> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(Node::new(None, state, None));

    while let Some(node) = queue.pop_front() {
        let state = node.state;
        let sol_found = state.6 && board[state.1 as usize][state.0 as usize] == BoardPiece::Start;

        if sol_found {
            return Some((board, state, node.moves(), visited.len())); // Solution found, yay!
        }

        let rc_node = Rc::new(node);
        for (move_, state) in neighbourhood(board, rc_node.state) {
            if visited.contains(&state) {
                continue;
            }

            queue.push_back(Node::new(Some(move_), state, Some(Rc::clone(&rc_node))));
            visited.insert(state);
        }
    }

    None // Exhausted search, no solution found.
}
