use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub enum PieceType {
    HelperOne,
    HelperTwo,
    Main,
}

#[derive(PartialEq, Eq, Clone)]
pub enum BoardPiece {
    Start,
    Goal,
    Blocker,
    Empty,
    Helper,
    Main,
}

pub type State = (Position, Position, Position, u8);
pub type Board = Vec<Vec<BoardPiece>>;

type Position = u8;
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

fn pos_to_x(pos: &Position) -> usize {
    (pos >> 4) as usize
}

fn pos_to_y(pos: &Position) -> usize {
    (pos & 0b0000_1111) as usize
}

fn try_move(pos: &Position, dir: &Direction, board: &Board, state: &State) -> Option<Position> {
    let mut x = pos_to_x(pos);
    let mut y = pos_to_y(pos);

    use Direction::*;
    match dir {
        Up => {
            if y == 0 {
                return None;
            } else {
                y = y - 1;
            }
        }
        Down => {
            if y + 1 >= board.len() {
                return None;
            } else {
                y = y + 1;
            }
        }
        Left => {
            if x == 0 {
                return None;
            } else {
                x = x - 1;
            }
        }
        Right => {
            if x + 1 >= board[0].len() {
                return None;
            } else {
                x = x + 1;
            }
        }
    };

    match board[y as usize][x as usize] {
        BoardPiece::Blocker => return None,
        _ => {}
    };

    let pos = ((x << 4) + y) as u8;

    // Check collison with other pieces
    if pos == state.0 || pos == state.1 || pos == state.2 {
        return None;
    }

    Some(pos)
}

fn next_position(
    board: &Board,
    state: &State,
    pos: &Position,
    dir: &Direction,
) -> Option<Position> {
    let mut new_pos = try_move(pos, dir, board, state)?;
    while let Some(pos) = try_move(&new_pos, &dir, board, state) {
        new_pos = pos;
    }
    Some(new_pos)
}

fn move_piece(board: &Board, state: &State, piece: &PieceType, dir: &Direction) -> Option<State> {
    use PieceType::*;
    let start_pos = match piece {
        Main => state.0,
        HelperOne => state.1,
        HelperTwo => state.2,
    };

    let pos = next_position(board, state, &start_pos, dir)?;

    Some(match piece {
        Main => {
            let goal_found =
                if state.3 == 1 || board[pos_to_y(&pos)][pos_to_x(&pos)] == BoardPiece::Goal {
                    1
                } else {
                    0
                };
            (pos, state.1, state.2, goal_found)
        }
        HelperOne => (state.0, pos, state.2, state.3),
        HelperTwo => (state.0, state.1, pos, state.3),
    })
}

fn neighbourhood(board: &Board, state: &State) -> Vec<(Move, State)> {
    use Direction::*;
    use PieceType::*;

    let mut states = Vec::new();

    for piece in [Main, HelperOne, HelperTwo] {
        for direction in [Left, Right, Up, Down] {
            if let Some(state) = move_piece(board, state, &piece, &direction) {
                let move_ = (piece.clone(), direction.clone());
                states.push((move_, state));
            }
        }
    }

    states
}

pub fn solve_puzzle(board: &Board, state: State) -> Option<(&Board, State, Vec<Move>)> {
    let mut visited = [[[[false; 2]; 128]; 128]; 128];
    let mut queue = VecDeque::new();

    queue.push_back(Node::new(None, state, None));

    while let Some(node) = queue.pop_front() {
        let state = node.state;
        let sol_found =
            state.3 == 1 && board[pos_to_y(&state.0)][pos_to_x(&state.0)] == BoardPiece::Start;

        if sol_found {
            return Some((board, state, node.moves())); // Solution found, yay!
        }

        let rc_node = Rc::new(node);
        for (move_, state) in neighbourhood(board, &rc_node.state) {
            if visited[state.0 as usize][state.1 as usize][state.2 as usize][state.3 as usize] {
                continue;
            }

            queue.push_back(Node::new(Some(move_), state, Some(Rc::clone(&rc_node))));
            visited[state.0 as usize][state.1 as usize][state.2 as usize][state.3 as usize] = true;
        }
    }

    None // Exhausted search, no solution found.
}
