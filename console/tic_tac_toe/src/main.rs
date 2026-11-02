use std::io::{self, Write};

/// A 3 × 3 board; empty cells are stored as spaces.
struct Board {
    cells: [[char; 3]; 3],
}

impl Board {
    /// Creates an empty board.
    fn new() -> Self {
        Self {
            cells: [[' '; 3]; 3],
        }
    }

    /// Displays the board with row and column numbers (0–2).
    fn display(&self) {
        println!("    0   1   2");
        for (row_index, row) in self.cells.iter().enumerate() {
            println!("{row_index}   {} | {} | {}", row[0], row[1], row[2]);
            if row_index < 2 {
                println!("   ---+---+---");
            }
        }
    }

    /// Records a move, rejecting out-of-bounds positions and occupied cells.
    fn make_move(&mut self, row: usize, col: usize, player: char) -> Result<(), &'static str> {
        let Some(cell) = self.cells.get_mut(row).and_then(|line| line.get_mut(col)) else {
            return Err("Row and column must be in the range 0–2.");
        };
        if *cell != ' ' {
            return Err("That cell is already occupied.");
        }
        *cell = player;
        Ok(())
    }

    /// Returns the winner's symbol if a player has placed three marks in a line.
    fn check_winner(&self) -> Option<char> {
        let lines = [
            [self.cells[0][0], self.cells[0][1], self.cells[0][2]],
            [self.cells[1][0], self.cells[1][1], self.cells[1][2]],
            [self.cells[2][0], self.cells[2][1], self.cells[2][2]],
            [self.cells[0][0], self.cells[1][0], self.cells[2][0]],
            [self.cells[0][1], self.cells[1][1], self.cells[2][1]],
            [self.cells[0][2], self.cells[1][2], self.cells[2][2]],
            [self.cells[0][0], self.cells[1][1], self.cells[2][2]],
            [self.cells[0][2], self.cells[1][1], self.cells[2][0]],
        ];
        lines
            .iter()
            .find(|line| line[0] != ' ' && line[0] == line[1] && line[1] == line[2])
            .map(|line| line[0])
    }
}

/// Reads a line; `None` means the input has been closed (EOF).
fn get_input() -> io::Result<Option<String>> {
    let mut input = String::new();
    let bytes_read = io::stdin().read_line(&mut input)?;
    Ok((bytes_read != 0).then(|| input.trim().to_owned()))
}

/// Prompts the current player for a valid move in an empty cell.
fn read_move(board: &mut Board, player: char) -> io::Result<Option<()>> {
    loop {
        print!("Player {player}, enter a row and column (0–2), e.g. 1 2: ");
        io::stdout().flush()?;
        let Some(input) = get_input()? else {
            return Ok(None);
        };
        let mut parts = input.split_whitespace();
        let parsed = parts
            .next()
            .and_then(|row| row.parse::<usize>().ok())
            .zip(parts.next().and_then(|col| col.parse::<usize>().ok()));
        if parts.next().is_some() {
            println!("Enter exactly two numbers.");
            continue;
        }
        let Some((row, col)) = parsed else {
            println!("Enter exactly two numbers in the range 0–2.");
            continue;
        };
        match board.make_move(row, col, player) {
            Ok(()) => return Ok(Some(())),
            Err(message) => println!("{message}"),
        }
    }
}

/// Runs alternating turns until a win, a draw, or EOF.
fn main() -> io::Result<()> {
    let mut board = Board::new();
    board.display();

    for turn in 0..9 {
        let player = if turn % 2 == 0 { 'X' } else { 'O' };
        if read_move(&mut board, player)?.is_none() {
            println!("End of input. Game over.");
            return Ok(());
        }
        board.display();
        if let Some(winner) = board.check_winner() {
            println!("Player {winner} wins!");
            return Ok(());
        }
    }
    println!("It's a draw!");
    Ok(())
}
