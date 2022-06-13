use core::num;
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

#[derive(Clone, Copy, Debug, Default)]
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
    fn generate(sudoku: Sudoku, start_row: usize, start_col: usize) -> Result<Self, bool>
    where
        Self: Sized;
    fn print(&self);
    fn checkvalid(&self, number: u8) -> bool;
}

impl Sudoku {
    fn check_row(&self, row_index: usize, number_to_be_checked: u8) -> bool {
        //    if !self.sudoku[row_index].contains(number_to_be_checked){
        if (0..NR_OF_COL).all(|col| self.sudoku[row_index][col] != number_to_be_checked) {
            println!(
                "Row: {}, already contains {}",
                row_index, number_to_be_checked
            );
        }
        (0..NR_OF_COL).all(|col: usize| self.sudoku[row_index][col] != number_to_be_checked)
    }

    fn check_col(&self, col_index: usize, number_to_be_checked: u8) -> bool {
        //checks whether the full col don't contain a non-valid number
        if !(0..NR_OF_COL).all(|row| self.sudoku[row][col_index] != number_to_be_checked) {
            println!(
                "Col: {}, already contains {}",
                col_index, number_to_be_checked
            );
        }
        (0..NR_OF_COL).all(|row| self.sudoku[row][col_index] != number_to_be_checked)
    }
    fn available_numbers(&self, row_index: usize, col_index: usize) -> bool {
        true
    }
}

/// Can also be rows or columns
impl MiniMatrix {
    fn contains(&self, number: u8) -> bool {
        self.minimatrix.contains(&number)
    }

    fn place_mini_matrix(sudoku: Sudoku) {}
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
    fn generate(sudoku: Sudoku, start_row: usize, start_col: usize) -> Result<Self, bool> {
        println!(
            "generate mini matrix with start_row: {}, start_col: {}",
            start_row, start_col
        );

        let mut minimatrix = MiniMatrix::default();
        let mut count: usize = 0;
        let mut numbers_tried: u8 = 0;

        while count != 9 && numbers_tried < 20 {
            let random_number = generate_number(1, NR_OF_COL as u8);
                if !sudoku.available_numbers(start_row, start_col) {
                    println!("error!");
                    //start over
                    return Err(false);
                }
                if minimatrix.checkvalid(random_number)
                    // && sudoku.check_col(start_col, random_number)
                    && sudoku.check_row(start_row, random_number)
                {
                    minimatrix[count] = random_number; //TODO FIX, hier klopt geen pepernoot van mbt rows en cols
                    count += 1;
                }
                numbers_tried += 1  
        }
        //      minimatrix.print();
        Ok(minimatrix)
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
    fn generate(mut sudoku: Sudoku, _start_row: usize, _start_col: usize) -> Result<Self, bool> {
        // Wss kloppen de start_row en start_col hier, ik denk dat deze meer overeen komen met welke mini matrix en de index binnen die minimatrix
        // TODO sloop minimatrix want dit werkt niet zo lekker als ik zou willen
        let mut _nr_of_minimatrices: usize = 0;
        for matrices_in_row in 0..NR_OF_COL_MM {
            // println!("loop through matrices in row {}", matrices_in_row);
            for matrices_in_col in 0..NR_OF_COL_MM {
                // println!("loop through matrices in col {}", matrices_in_col);
                let mut count: usize = 0;
                // if MiniMatrix::generate(sudoku, matrices_in_row*NR_OF_COL_MM, matrices_in_col*NR_OF_COL_MM).is_err() {
                //     // wipe alles en opnieuw
                //     println!("foutje");
                // } else {
                let minimatrix = MiniMatrix::generate(
                    sudoku,
                    matrices_in_row * NR_OF_COL_MM,
                    matrices_in_col * NR_OF_COL_MM,
                )?;
                for row in 0..NR_OF_COL_MM {
                    for col in 0..NR_OF_COL_MM {
                        sudoku.sudoku[row + (matrices_in_row * NR_OF_COL_MM)]
                            [col + (matrices_in_col * NR_OF_COL_MM)] = minimatrix[count];
                        count += 1;
                    }
                }
                println!("print sudoku");
                sudoku.print();
                // }
            }
        }
        println!("==========================================");
        sudoku.print();
        Ok(sudoku)
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
