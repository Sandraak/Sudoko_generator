use crate::sudoku_generator::{Matrix, Sudoku};

mod sudoku_generator;

fn main() {
    let minimatrix = sudoku_generator::MiniMatrix::generate_with_stub_data(0);
    minimatrix.print();
    println!("======================");

    //  let sudoko = Sudoku::generate_with_stub_data(0);
    let sudoko = Sudoku::generate_blank();
    sudoko.print();

    // minimatrix.print();
    // minimatrix.print_as_row();
    // minimatrix.print_as_col();

    // let sudoku = Sudoku::generate_blank();
    // sudoku.print();
}
