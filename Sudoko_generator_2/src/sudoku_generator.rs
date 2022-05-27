use rand::Rng;
use std::{
    self,
    ops::{Index, IndexMut},
};

const NR_OF_COL: usize = 9;
const NR_OF_COL_MM: usize = NR_OF_COL / 3;
const NR_OF_MM: usize = NR_OF_COL_MM;
const NR_OF_VALUES: usize = NR_OF_COL.pow(2);
const NR_OF_VALUES_MM: usize = NR_OF_COL;

#[derive(Debug, Default)]
pub struct Sudoku {
    sudoku: [MiniMatrix; NR_OF_COL],
}
//Deze kan uiteindelijk op private
#[derive(Clone, Copy, Debug, Default)]
pub struct MiniMatrix {
    minimatrix: [u8; NR_OF_COL],
}
pub trait Matrix {
    // fn generate_blank() -> Self;
    fn generate(start_row : usize, start_col : usize) -> Self;
    fn print(&self);
    fn checkvalid(&self, number: u8) -> bool;
}

impl Sudoku {
    fn check_row(&self, row_index: usize, number_to_be_checked: u8) -> bool {
        self.sudoku[row_index].contains(number_to_be_checked)
    }
    fn check_col(&self, col_index: usize, number_to_be_checked: u8) -> bool {
        //checks if all of the row don't contain a non-valid
        (0..NR_OF_COL).all(|row| self.sudoku[row][col_index] != number_to_be_checked )
    }
}

/// Can also be rows or columns
impl MiniMatrix {
    fn contains(&self, number: u8) -> bool {
        self.minimatrix.contains(&number)
    }
}

//Index is een standaard trait uit Rust, deze kan je zelf implenteren voor eigen types.
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
    fn generate(_start_row : usize, _start_col : usize) -> Self {
        let mut minimatrix = MiniMatrix::default();
        let mut done = false;
        let mut count: usize = 0;
        while !done {
            let random_number = generate_number(1, NR_OF_COL as u8);
            if count == NR_OF_COL {
                done = true;
            }
            if minimatrix.checkvalid(random_number) {
                minimatrix[count] = random_number;
                count += 1;
            }
        }
        
        minimatrix
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
        !self.minimatrix.contains(&number)
    }
}

impl Matrix for Sudoku {
    fn generate(start_row : usize, start_col : usize) -> Self {
        let mut sudoku = Sudoku::default();
        let mut count: usize = 0;
        sudoku.sudoku[count] = MiniMatrix::generate(start_row, start_col);
        for row in start_row..start_row + NR_OF_COL_MM{
            for col in start_col..start_col + NR_OF_COL_MM{
                sudoku.sudoku[row][col] = sudoku.sudoku[start_row][count];
                count +=1;
            }
        }
        sudoku
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

fn generate_number(low: u8, high: u8) -> u8 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(low..=high);
    number
}
