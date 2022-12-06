use crate::solver::*;

pub fn puzzle_from_string(input: &str) -> (Board, State) {
    let items: Vec<&str> = input.split(":").collect();
    let mut board =
        vec![vec![BoardPiece::Empty; items[1].parse().unwrap()]; items[2].parse().unwrap()];
    let mut state = (0, 0, 0, 0);
    let mut first_helper_found = false;
    for parts in items.chunks(3) {
        match parts[0] {
            "main_robot" => {
                let x = parts[1].parse::<u8>().unwrap();
                let y = parts[2].parse::<u8>().unwrap();
                board[y as usize][x as usize] = BoardPiece::Start;
                state.0 = (x << 4) + y;
            }
            "helper_robot" => {
                if first_helper_found {
                    let x = parts[1].parse::<u8>().unwrap();
                    let y = parts[2].parse::<u8>().unwrap();
                    state.1 = (x << 4) + y;
                } else {
                    first_helper_found = true;
                    let x = parts[1].parse::<u8>().unwrap();
                    let y = parts[2].parse::<u8>().unwrap();
                    state.2 = (x << 4) + y;
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

pub fn print_moves(moves: &Vec<(PieceType, Direction)>) {
    for m in moves {
        print_move(m);
    }
}

pub fn print_board(board: &Board, state: State) {
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
