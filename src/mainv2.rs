use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    // Returns a new Position with the given x and y coordinates.
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Clone, Copy)]
struct Piece {
    position: Position,
    is_main: bool,
}

impl Piece {
    // Returns a new Piece with the given position and is_main flag.
    fn new(position: Position, is_main: bool) -> Self {
        Self { position, is_main }
    }
}

#[derive(Clone)]
struct State {
    pieces: Vec<Piece>,
    target: Position,
    blockers: HashSet<Position>,
}

impl State {
    // Returns a new State with the given pieces, target, and blockers.
    fn new(pieces: Vec<Piece>, target: Position, blockers: HashSet<Position>) -> Self {
        Self {
            pieces,
            target,
            blockers,
        }
    }

    // Returns the position of the Main Piece, or None if the Main Piece is not found.
    fn main_piece(&self) -> Option<&Piece> {
        self.pieces.iter().find(|piece| piece.is_main)
    }

    // Returns the next position that the given piece would reach if it were to move in the given
    // direction. If the piece cannot be moved in the given direction, returns None.
    fn next_position(&self, piece: &Piece, direction: Direction) -> Option<Position> {
        let (dx, dy) = match direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        let mut x = piece.position.x + dx;
        let mut y = piece.position.y + dy;

        // Stop the piece if it hits a blocker, another piece, or the edge of the board.
        while x >= 0 && x < 8 && y >= 0 && y < 8 {
            let position = Position::new(x, y);
            if self.blockers.contains(&position) {
                break;
            }

            let other_piece = self
                .pieces
                .iter()
                .find(|p| p.position == position && p != piece);
            if other_piece.is_some() {
                break;
            }

            x += dx;
            y += dy;
        }

        // If the piece moved outside of the board, return None.
        if x < 0 || x >= 8 || y < 0 || y >= 8 {
            return None;
        }

        Some(Position::new(x, y))
    }
}
// Returns a new State with the given piece moved in the given direction. If the piece
// cannot be moved in the given direction, returns None.
fn move_piece(&self, piece: &Piece, direction: Direction) -> Option<Self> {
    let next_position = self.next_position(piece, direction)?;

    let mut pieces = self.pieces.clone();
    for i in 0..pieces.len() {
        if pieces[i].position == piece.position {
            pieces[i].position = next_position;
            break;
        }
    }

    Some(Self::new(pieces, self.target, self.blockers))
}

// Returns a Vec of all possible next states that can be reached from this state.
fn next_states(&self) -> Vec<State> {
    let mut states = Vec::new();

    for direction in [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ]
    .iter()
    {
        for piece in self.pieces.iter() {
            if let Some(state) = self.move_piece(piece, *direction) {
                states.push(state);
            }
        }
    }

    states
}

// Solves the puzzle and returns a sequence of moves that solves the puzzle, or None if the puzzle
// cannot be solved.
fn solve_puzzle(state: State) -> Option<Vec<Direction>> {
    // We will use a breadth-first search algorithm to solve the puzzle.
    let mut queue = VecDeque::new();
    let mut visited_states = HashSet::new();

    // Start by adding the initial state to the queue and marking it as visited.
    queue.push_back((state.clone(), Vec::new()));
    visited_states.insert(state);

    // Keep searching until we either find a solution or exhaust all possible states.
    while let Some((state, moves)) = queue.pop_front() {
        // If the Main Piece is at the target, we have found a solution.
        if state
            .main_piece()
            .map_or(false, |p| p.position == state.target)
        {
            return Some(moves);
        }

        // Add all possible next states to the queue.
        let new_states = state.next_states();
        for new_state in new_states {
            if !visited_states.contains(&new_state) {
                // Add the next state to the queue and mark it as visited.
                let mut new_moves = moves.clone();
                new_moves.push(direction);
                queue.push_back((new_state, new_moves));
                visited_states.insert(new_state);
            }
        }
    }

    // If we reach this point, we have exhausted all possible states without finding a solution.
    None
}

fn main() {
    // Test 1:
    //
    // +---+---+---+---+---+---+---+---+
    // |   |   |   |   |   |   |   |   |
    // +---+---+---+---+---+---+---+---+
    // |   |   |   |   |   |   |   |   |
    // +---+---+---+---+---+---+---+---+
    // |   |   |   |   |   |   |   |   |
    // +---+---+---+---+---+---+---+---+
    // |   |   |   |   |   |   |   |   |
    // +---+---+---+---+---+---+---+---+
    // |   |   |   |   |   |   |   |   |
    // +---+---+---+---+---+---+---+---+
    // |   |   |   |   |   |   |   |   |
    // +---+---+---+---+---+---+---+---+
    // |   |   |   |   |   |   |   |   |
    // +---+---+---+---+---+---+---+---+
    // |   |   |   |   | X |   |   |   |
    // +---+---+---+---+---+---+---+---+
    //
    // Main Piece: (4, 4)
    // Target: (4, 5)
    // Blockers: None
    let state = State::new(
        vec![
            Piece::new(Position::new(4, 4), true),
            Piece::new(Position::new(0, 0), false),
            Piece::new(Position::new(7, 7), false),
        ],
        Position::new(4, 5),
        HashSet::new(),
    );
    let moves = solve_puzzle(state);
    assert_eq!(moves, Some(vec![Direction::Right]));
}
