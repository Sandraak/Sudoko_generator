use crate::sudoku_generator::{Matrix, MiniMatrix, Sudoku};

use core::result::Result;

mod sudoku_generator;

fn main() {
    // let mut sudoku = Sudoku::default();
    // sudoku_generator::MiniMatrix::generate(sudoku, 0,0);
    // minimatrix.print();
    println!("======================");
    Sudoku::generate(Sudoku::default(), 0, 0);
    // MiniMatrix::generate(Sudoku::default(), 0, 0);
    // sudoko.print();

    // Ok(())
}
