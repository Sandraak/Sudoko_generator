use rand::Rng;
use std::{
    self,
    ops::{Index, IndexMut},
    vec,
};

const NR_OF_COL: usize = 9;
const NR_OF_COL_MM: usize = NR_OF_COL / 3;
const NR_OF_MM: usize = NR_OF_COL_MM;
const NR_OF_VALUES: usize = NR_OF_COL.pow(2);
const NR_OF_VALUES_MM: usize = NR_OF_COL;
pub struct Sudoku {
    sudoku: Vec<MiniMatrix>,
}
//Deze kan uiteindelijk op private
#[derive(Clone, Debug)]
pub struct MiniMatrix {
    minimatrix: Vec<u8>,
}
pub trait Matrix {
    fn generate_blank() -> Self;
    fn generate_with_stub_data(start_value: usize) -> Self;
    fn print(&self);
    fn checkvalid(&self, number: u8) -> bool;
}

impl Sudoku {
    fn check_row(&self, row_index: usize, number_to_be_checked: u8) -> bool {
            self.sudoku[row_index].contains(number_to_be_checked)
    }
    fn check_col(&self, col_index: usize, number_to_be_checked: u8) -> bool {
        for row in 0..NR_OF_COL {
            if self.sudoku[row][col_index] == number_to_be_checked {
                return false
            }
        }
        true
    }
}

/// Can also be rows or columns
impl MiniMatrix {
    fn contains(&self, number :u8) -> bool{
        self.minimatrix.contains(&number)
    }
}

impl Index<usize> for MiniMatrix {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.minimatrix[index]
    }
}

impl IndexMut<usize> for MiniMatrix {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.minimatrix[index]
    }
}

impl Matrix for MiniMatrix {
    fn generate_blank() -> Self {
        let minimatrix = vec![0; NR_OF_VALUES_MM];
        MiniMatrix { minimatrix }
    }

    fn generate_with_stub_data(start_value: usize) -> Self {
        let mut minimatrix: Vec<u8> = Vec::new();
        for i in start_value..start_value + NR_OF_VALUES_MM {
            minimatrix.push(i.try_into().unwrap());
        }
        MiniMatrix { minimatrix }
    }

    fn print(&self) {
        for row in 0..NR_OF_COL_MM {
            for col in 0..NR_OF_COL_MM {
                print!(" {} ", self.minimatrix[(row * NR_OF_COL_MM) + col]);
            }
            println!(" ");
        }
        println!(" ");
    }
    fn checkvalid(&self, number: u8) -> bool {
        true
    }
}

// *
// Rijen en kollommen maken uit matrices hmm
/// (0,0) (0,1) (0,2)   (0,3) (0,4) ...
/// (1.0) (1,1) (1,2)
/// (2,0) (2,1) (2,2)
///
/// (3,0) (3,1) ...

impl Matrix for Sudoku {
    fn generate_blank() -> Self {
        let sudoku = vec![MiniMatrix::generate_blank(); NR_OF_COL];
        Sudoku { sudoku }
    }

    fn generate_with_stub_data(start_value: usize) -> Self {
        // let mut vec = vec![vec!['#'; 80]; 24];
        let row = MiniMatrix::generate_blank();
        let mut sudoku: Vec<MiniMatrix> = vec![row; NR_OF_COL];
        let mut counter = 0;

        for row in start_value..start_value + NR_OF_COL {
            for col in start_value..start_value + NR_OF_COL {
                sudoku[row][col] = counter;
                counter += 1;
            }
        }
        Sudoku { sudoku }
    }

    fn print(&self) {
        for row in 0..NR_OF_COL {
            if row % NR_OF_COL_MM == 0 {
                println!(" ");
            }
            for col in 0..NR_OF_COL {
                if col % NR_OF_COL_MM == 0 {
                    print!(" ");
                }
                print!(" {} ", self.sudoku[row][col]);
            }
            println!(" ");
        }
    }
    
    fn checkvalid(&self, number: u8) -> bool {
        true
    }
}

// fn main() {
//     let minimatrix = MiniMatrix::generate_with_stub_data(0);
//     minimatrix.print();
//     println!("======================");

//     //  let sudoko = Sudoku::generate_with_stub_data(0);
//     let sudoku = Sudoku::generate_blank();
//     sudoku.print();

//     // minimatrix.print();
//     // minimatrix.print_as_row();
//     // minimatrix.print_as_col();

//     // let sudoku = Sudoku::generate_blank();
//     // sudoku.print();
// }
