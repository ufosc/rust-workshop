// This is a excerpt of the excellent tutorial by sunjay at https://github.com/sunjay/tic-tac-toe
// Please checkout his tutorial for a more in depth explanation of code.

// Global variable
const BOARD_SIZE: usize = 3;

// Define the pieces
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)] // Derive useful traits automatically
pub enum Piece {
    X,
    O,
}

// Implement Piece and give it the "other" function
impl Piece {
    // Helper function to determine the other piece
    pub fn other(self) -> Piece {
        match self {
            Piece::X => Piece::O,
            Piece::O => Piece::X,
        }
    }
}

// Type aliases
pub type Tile = Option<Piece>;
pub type Tiles = [[Tile; BOARD_SIZE]; BOARD_SIZE];

// Define Game Over states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Winner {
    X,
    O,
    Tie,
}

// Define errors in intended moves
#[derive(Debug, Clone)]
pub enum MoveError {
    GameAlreadyOver, // Should not occur in normal operation
    InvalidPosition { row: usize, col: usize }, // Invalid tile DNE
    TileNotEmpty { other_piece: Piece, row: usize, col: usize }, // Valid tile, but occupied
}

// Define Game State
#[derive(Debug, Clone)]
pub struct Game {
    tiles: Tiles,
    current_piece: Piece,
    winner: Option<Winner>,
}

// Implementation of a Game struct
impl Game {
    // Constructor
    pub fn new() -> Self {
        Self {
            tiles: Default::default(),
            current_piece: Piece::X,
            winner: None,
        }
    }

    // Modifies the gameboard by placing the current piece at a given (row, col)
    pub fn make_move(&mut self, row: usize, col: usize) -> Result<(), MoveError> {
        // Check if Game is over
        if self.is_finished() {
            return Err(MoveError::GameAlreadyOver);
        }
        // Check if the tile is on the gameboard
        else if row >= self.tiles.len() || col >= self.tiles[0].len() {
            return Err(MoveError::InvalidPosition {row, col});
        }
        // Check if there is a piece on the tile already
        else if let Some(other_piece) = self.tiles[row][col] {
            return Err(MoveError::TileNotEmpty {other_piece, row, col});
        }

        // Update state
        self.tiles[row][col] = Some(self.current_piece);
        self.current_piece = self.current_piece.other();
        self.update_winner(row, col);

        // If everything worked, we'll return Ok
        Ok(())
    }

    // Checks for any winning states and updates the winner accordingly
    fn update_winner(&mut self, row: usize, col: usize) {
        // Used for sanity check
        let rows = self.tiles.len();
        let cols = self.tiles[0].len();

        // Gets the tiles in a given row and col
        let tiles_row = self.tiles[row];
        let tiles_col = [self.tiles[0][col], self.tiles[1][col], self.tiles[2][col]];

        // Break if there are too many rows or cols
        assert!(rows == 3 && cols == 3,
            "This code is written with the assumption that there are three rows and columns");

        // Check left diagonal (\)
        let tiles_diagonal_1 = if row == col {
            [self.tiles[0][0], self.tiles[1][1], self.tiles[2][2]]
        }
        else {
            [None, None, None]
        };

        // Check right diagonal (/)
        let tiles_diagonal_2 = if (rows - row - 1) == col {
            [self.tiles[0][2], self.tiles[1][1], self.tiles[2][0]]
        }
        else {
            [None, None, None]
        };

        // Check for game overs and set the winner variable
        fn check_winner(row: &[Tile]) -> Option<Winner> {
            if row[0] == row[1] && row[1] == row[2] {
                match row[0] {
                    Some(Piece::X) => Some(Winner::X),
                    Some(Piece::O) => Some(Winner::O),
                    None => None,
                }
            }

            else {
                None
            }
        }

        // Set winner for any of the win states
        self.winner = self.winner
            .or_else(|| check_winner(&tiles_row))
            .or_else(|| check_winner(&tiles_col))
            .or_else(|| check_winner(&tiles_diagonal_1))
            .or_else(|| check_winner(&tiles_diagonal_2));

        // In the absence of a winner and a valid move, Set winner to Tie
        self.winner = self.winner.or_else(|| {
            if self.tiles.iter().all(|row| row.iter().all(|tile| tile.is_some())) {
                Some(Winner::Tie)
            }
            // If all checks fail, winner is None
            else {
                None
            }
        });
    }

    // ACCESSOR FUNCTIONS //
    pub fn is_finished(&self) -> bool {
        self.winner.is_some()
    }

    pub fn winner(&self) -> Option<Winner> {
        self.winner
    }

    pub fn current_piece(&self) -> Piece {
        self.current_piece
    }

    pub fn tiles(&self) -> &Tiles {
        &self.tiles
    }
}

// Tests Below
