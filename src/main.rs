use std::collections::{BTreeSet, HashSet, VecDeque};
use std::ptr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialOrd, Ord, Clone, Copy, Eq, PartialEq, Hash)]
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

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct State<'a> {
    pieces: Vec<Piece>,
    target: Position,
    blockers: &'a BTreeSet<Position>,
}

impl<'a> State<'a> {
    // Returns a new State with the given pieces, target, and blockers.
    fn new(pieces: Vec<Piece>, target: Position, blockers: &'a BTreeSet<Position>) -> Self {
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
                // .find(|p| p.position == position && p != piece);
                .find(|p| p.position == position && !ptr::eq(p, &piece));
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

        Some(Self::new(pieces, self.target, &self.blockers))
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
}

fn print_board(state: &State) {
    // Create an 8x8 grid of spaces.
    let mut grid = vec![vec![' '; 8]; 8];

    // Place the blockers on the grid.
    for position in state.blockers.iter() {
        grid[position.x as usize][position.y as usize] = 'X';
    }

    // Place the pieces on the grid.
    for piece in &state.pieces {
        grid[piece.position.x as usize][piece.position.y as usize] =
            if piece.is_main { 'M' } else { 'O' };
    }

    // Place the target on the grid.
    grid[state.target.x as usize][state.target.y as usize] = 'T';

    // Print the grid to the terminal.
    for row in grid {
        for cell in row {
            print!("{} ", cell);
        }
        println!();
    }
}

// Solves the puzzle and returns a sequence of moves that solves the puzzle, or None if the puzzle
// cannot be solved.
// Returns a sequence of moves that can be made to reach the target and then return to the starting
// position, or None if no such sequence of moves exists.
fn solve_puzzle(state: State) -> Option<Vec<Direction>> {
    let mut queue = VecDeque::new();
    let mut visited_states = HashSet::new();
    let blockers = state.blockers.clone();
    let target = state.target.clone();

    queue.push_back((state, Vec::new()));

    while let Some((state, moves)) = queue.pop_front() {
        print_board(&state);
        // If the Main Piece has reached the target and then returned to its original position,
        // return the sequence of moves that were made.
        if state.pieces[0].position == state.target && state.pieces[0].is_main {
            return Some(moves);
        }

        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            let piece = &state.pieces[0];
            if let Some(next_position) = state.next_position(piece, *direction) {
                let mut new_pieces = state.pieces.clone();
                new_pieces[0] = Piece::new(next_position, !piece.is_main);

                let new_state = State::new(new_pieces, target, &blockers);
                if visited_states.contains(&new_state) {
                    continue;
                }

                let mut new_moves = moves.clone();
                new_moves.push(*direction);
                queue.push_back((new_state, new_moves));
            }
        }
        visited_states.insert(state);
    }

    // If we reach this point, we have exhausted all possible states without finding a solution.
    None
}

fn print_moves(moves: Vec<Direction>) {
    for dir in moves {
        match dir {
            Direction::Up => print!("Up"),
            Direction::Down => print!("Down"),
            Direction::Left => print!("Left"),
            Direction::Right => print!("Right"),
        }
    }
}

fn main() {
    let blockers = BTreeSet::new();
    let state = State::new(
        vec![
            Piece::new(Position::new(4, 4), true),
            Piece::new(Position::new(0, 0), false),
            Piece::new(Position::new(7, 7), false),
        ],
        Position::new(4, 5),
        &blockers,
    );
    let moves = solve_puzzle(state);

    match moves {
        None => println!("Could not solve puzzle."),
        Some(move_list) => print_moves(move_list),
    }
}
