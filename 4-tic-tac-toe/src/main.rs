// A simple 2 player Tic Tac Toe Game
// This is a excerpt of the excellent tutorial by sunjay at https://github.com/sunjay/tic-tac-toe
// Please checkout his tutorial for a more in depth explanation of code.

// Declare the module this is a part of
mod game;

// Standard Library Import Statements
use std::io::{self, Write}; // Import the "Write" trait
use std::process; // Gives access to the exit function

// Package Import Statements
use game::{Game, Piece, Winner, Tiles, MoveError};

// Simplify error handling later on
#[derive(Debug, Clone)]
pub struct InvalidMove(pub String);

fn main() {
    // Create the empty Tic Tac Toe Board
    let mut game = Game::new();

    // Main loop for the game
    while !game.is_finished() {
        print_tiles(game.tiles());

        // Prints the current piece (Who's turn is it?)
        println!("Current piece: {}", match game.current_piece() {
            Piece::X => "x",
            Piece::O => "o",
        });

        // Collect the player's intended move
        let (row, col) = prompt_move();

        // Detect if the move is valid
        // unreachable!() exits the program with an error message
        match game.make_move(row, col) {
            Ok(()) => {},
            Err(MoveError::GameAlreadyOver) => unreachable!("Game was already over when it should not have been"),
            Err(MoveError::InvalidPosition {row, col}) => {
                unreachable!("Should not be able to enter an invalid move, but still got ({}, {})", row, col)
            },
            Err(MoveError::TileNotEmpty {other_piece, row, col}) => eprintln!(
                "The tile at position {}{} already has piece {} in it!",
                row + 1,
                (b'A' + col as u8) as char,
                match other_piece {
                    Piece::X => "x",
                    Piece::O => "o",
                },
            ),
        }
    }

    // Refresh the game board
    print_tiles(game.tiles());

    // Detect Game Over
    match game.winner().expect("finished game should have winner") {
        Winner::X => println!("X wins!"),
        Winner::O => println!("O wins!"),
        Winner::Tie => println!("Tie!"),
    }
}

// Prompts and attempt to collect the player's intended move
fn prompt_move() -> (usize, usize) {
    // Loops until the player enters a valid move
    loop {
        // Print a prompt. Flush is necessary due to line-buffering not printing
        // unless it detects a new line character or a flush.
        print!("Enter move (e.g. 1A): ");
        io::stdout().flush().expect("Failed to flush stdout");

        // Collect keyboard input
        let line = read_line();

        // Matching the output of parse_move() to see if we need to prompt again
        match parse_move(&line) {
            Ok((row, col)) => break (row, col),
            Err(InvalidMove(invalid_str)) => eprintln!(
                "Invalid move: '{}'. Please try again.",
                invalid_str,
            ),
        }
    }
}

// Parses the input for correct formatting and valid moves
fn parse_move(input: &str) -> Result<(usize, usize), InvalidMove> {
    // Input should consist of exactly one row and one col
    if input.len() != 2 {
        return Err(InvalidMove(input.to_string()));
    }

    // Valid rows are 1-3
    let row = match &input[0..1] {
        "1" => 0,
        "2" => 1,
        "3" => 2,
        _   => return Err(InvalidMove(input.to_string())), 
    };

    // Valid rows are A-C (ignoring case)
    let col = match &input[1..2] {
        "A" | "a" => 0,
        "B" | "b" => 1,
        "C" | "c" => 2,

        invalid => return Err(InvalidMove(invalid.to_string())),
    };

    // If all checks pass, return a (row, col) tuple
    Ok((row, col))
}

// Wrapper function for io::stdin().read_line()
fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    // Exit program if the input is empty
    if input.is_empty() {
        println!();
        process::exit(0);
    }

    // Trim input
    let len_without_newline = input.trim_end().len();
    input.truncate(len_without_newline);
    input
}

// Print the game board
fn print_tiles(tiles: &Tiles) {
    print!("  ");

    // Print col headers
    for j in 0..tiles[0].len() as u8 {
        print!(" {}", (b'A' + j) as char);
    }
    println!(); // Spacing

    // Printing gameboard
    for (i, row) in tiles.iter().enumerate() {
        // Print row headers
        print!(" {}", i + 1);

        // Read actual enum values from the row
        for tile in row {
            print!(" {}", match *tile {
                Some(Piece::X) => "x",
                Some(Piece::O) => "o",
                None => "\u{25A2}",
            })
        }
        println!(); // Spacing
    }

    println!(); // Spacing
}
