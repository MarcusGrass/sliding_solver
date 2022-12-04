use std::collections::{BTreeSet, HashSet};

// Represents a direction that a piece can move in.
#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Represents a position on the board.
#[derive(PartialOrd, Ord, Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    // Returns a new Position with the given coordinates.
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// Represents a piece on the board.
#[derive(PartialOrd, Ord, Clone, Debug, Hash, Eq, PartialEq)]
struct Piece {
    // The position of the piece.
    position: Position,
    // Whether this is the Main Piece.
    is_main: bool,
}

impl Piece {
    // Returns a new Piece at the given position.
    fn new(position: Position, is_main: bool) -> Self {
        Self { position, is_main }
    }
}

// Represents the state of the puzzle.
#[derive(PartialOrd, Ord, Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    // The pieces on the board.
    pieces: Vec<Piece>,
    // The target position.
    target: Position,
    // The blockers on the board.
    blockers: BTreeSet<Position>,
}

impl State {
    // Returns a new State with the given pieces, target, and blockers.
    fn new(pieces: Vec<Piece>, target: Position, blockers: BTreeSet<Position>) -> Self {
        Self {
            pieces,
            target,
            blockers,
        }
    }

    // Returns the Main Piece in this State, or None if there is no Main Piece.
    fn main_piece(&self) -> Option<&Piece> {
        self.pieces
            .iter()
            .find(|piece| piece.is_main)
            .map(|piece| piece)
    }

    // Returns the next position that the Main Piece would move to if it were to move in the
    // given direction. If the Main Piece would hit a blocker or another piece, returns None.
    fn next_position(&self, direction: Direction) -> Option<Position> {
        let main_piece = self.main_piece()?;
        let mut next_position = main_piece.position;

        match direction {
            Direction::Up => next_position.y -= 1,
            Direction::Down => next_position.y += 1,
            Direction::Left => next_position.x -= 1,
            Direction::Right => next_position.x += 1,
        }

        if self.blockers.contains(&next_position)
            || self
                .pieces
                .iter()
                .any(|piece| piece.position == next_position)
        {
            None
        } else {
            Some(next_position)
        }
    }
    // Returns a new State with the Main Piece moved in the given direction. If the Main Piece
    // cannot be moved in the given direction, returns None.
    fn move_main_piece(&self, direction: Direction) -> Option<Self> {
        let main_piece = self.main_piece()?;
        let next_position = self.next_position(direction)?;

        let mut new_pieces = self.pieces.clone();

        for piece in &mut new_pieces {
            if piece.is_main {
                piece.position = next_position;
                break;
            }
        }

        Some(State::new(new_pieces, self.target, self.blockers.clone()))
    }
}

// Solves the puzzle and returns the sequence of moves required to solve it, or None if the puzzle
// cannot be solved.
fn solve_puzzle(mut state: State) -> Option<Vec<Direction>> {
    let mut moves = vec![];
    let mut visited_states = BTreeSet::new();
    visited_states.insert(state.clone());

    // Move the Main Piece to the target.
    while state.main_piece()?.position != state.target {
        let mut found_move = false;

        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(new_state) = state.move_main_piece(*direction) {
                if !visited_states.contains(&new_state) {
                    state = new_state;
                    moves.push(*direction);
                    visited_states.insert(state.clone());
                    found_move = true;
                    break;
                }
            }
        }

        if !found_move {
            return None;
        }
    }

    // Move the Main Piece back to its original position.
    // while state.main_piece()?.position != state.main_piece()?.position {
    //     let mut found_move = false;

    //     for direction in &[
    //         Direction::Up,
    //         Direction::Down,
    //         Direction::Left,
    //         Direction::Right,
    //     ] {
    //         if let Some(new_state) = state.move_main_piece(*direction) {
    //             if !visited_states.contains(&new_state) {
    //                 state = new_state;
    //                 moves.push(*direction);
    //                 visited_states.insert(state);
    //                 found_move = true;
    //                 break;
    //             }
    //         }
    //     }

    //     if !found_move {
    //         return None;
    //     }
    // }

    Some(moves)
}

// Solves the puzzle and returns the sequence of moves required to solve it, or None if the puzzle
// cannot be solved.
fn solve_puzzle_old(mut state: State) -> Option<Vec<Direction>> {
    let mut moves = vec![];

    // Move the Main Piece to the target.
    while state.main_piece()?.position != state.target {
        print_board(&state);
        let mut found_move = false;

        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(new_state) = state.move_main_piece(*direction) {
                state = new_state;
                moves.push(*direction);
                found_move = true;
                break;
            }
        }

        if !found_move {
            return None;
        }
    }

    // Move the Main Piece back to its original position.
    while state.main_piece()?.position != state.main_piece()?.position {
        let mut found_move = false;

        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(new_state) = state.move_main_piece(*direction) {
                state = new_state;
                moves.push(*direction);
                found_move = true;
                break;
            }
        }

        if !found_move {
            return None;
        }
    }

    Some(moves)
}

fn print_board(state: &State) {
    // Create an 8x8 grid of spaces.
    let mut grid = vec![vec![' '; 8]; 8];

    // Place the blockers on the grid.
    for position in &state.blockers {
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

fn main() {
    // Set up the initial state of the puzzle.
    let pieces = vec![
        Piece::new(Position::new(0, 0), false),
        Piece::new(Position::new(7, 0), false),
        Piece::new(Position::new(0, 7), true),
    ];
    let target = Position::new(7, 7);
    let blockers = [
        Position::new(3, 1),
        Position::new(4, 1),
        Position::new(5, 1),
        Position::new(1, 3),
        Position::new(1, 4),
        Position::new(1, 5),
        Position::new(3, 7),
        Position::new(4, 7),
        Position::new(5, 7),
    ]
    .iter()
    .cloned()
    .collect();
    let state = State::new(pieces, target, blockers);

    // Solve the puzzle and print the sequence of moves.
    if let Some(moves) = solve_puzzle(state) {
        for direction in moves {
            match direction {
                Direction::Up => println!("Move the Main Piece up"),
                Direction::Down => println!("Move the Main Piece down"),
                Direction::Left => println!("Move the Main Piece left"),
                Direction::Right => println!("Move the Main Piece right"),
            }
        }
    } else {
        println!("The puzzle cannot be solved");
    }
}
