use rand::Rng;
use std::{self, vec, ops::{Index, IndexMut}};

const NR_OF_COL: usize = 9;
const NR_OF_COL_MM: usize = NR_OF_COL / 3;
const NR_OF_MM : usize = NR_OF_COL_MM;
const NR_OF_VALUES : usize = NR_OF_COL.pow(2);
const NR_OF_VALUES_MM : usize = NR_OF_COL;

///Ik wil graag een trait maken voor het printen op verschillende manieren. Dit is cooler als ik ook verschillende structs heb en niet gewoon 1 sudoko.
/// Vandaar de struct minimatrix, een sudoko is hier uit opgebouwd. Maybe heb ik hierdoor ook een kans/reden om generic types te gebruiken?
pub struct Sudoku {
    sudoku: Vec<MiniMatrix>,
    // sudoku2: Vec<Vec<u8>>
}
//Deze kan uiteindelijk op private
#[derive(Clone, Debug)]
pub struct MiniMatrix {
    minimatrix: Vec<u8>,
    //  minimatrix: [u8; 9],
}
 pub trait Matrix {
    fn generate_blank() -> Self;
    fn generate_with_stub_data(start_value : usize) -> Self;
    fn print(&self);
}

impl Sudoku {}

/// Can also be rows or columns
impl MiniMatrix {
    // fn print_as_row(&self){
    //     for i in 0..NR_OF_VALUES_MM {
    //         if i % NR_OF_COL_MM == 0{
    //             print!("  ");
    //         }
    //         print!(" {} " , self.minimatrix[i]);
    //     }
    //     println!(" ");
    // }

    // fn print_as_col(&self){
    //     for i in 0..NR_OF_VALUES_MM {
    //         if i % NR_OF_COL_MM == 0{
    //             println!(" ");
    //         }
    //         println!(" {} ", self.minimatrix[i]);
    //     }
    // }

    // fn return_as_part_of_row(&self, start_index : usize ) -> Vec<u8>{
    //     let mut part_of_row :Vec<u8> = Vec::new();
    //     for current_index in start_index..start_index + NR_OF_COL_MM {
    //         part_of_row.push(self.minimatrix[current_index]);
    //     }
    //     part_of_row
    // }

    // fn return_as_part_of_col(&self, start_index : usize) -> Vec<u8> {
    //     let mut part_of_col : Vec<u8> = Vec::new();
    //     for current_index in start_index..start_index + NR_OF_COL_MM {
    //         part_of_col.push(self.minimatrix[current_index * NR_OF_COL_MM]);

    //     }
    //     part_of_col
    // }
}

impl Index<usize> for MiniMatrix {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.minimatrix[index]
    }
}

impl IndexMut<usize> for MiniMatrix{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.minimatrix[index]
    }
}

impl Matrix for MiniMatrix {
    fn generate_blank() -> Self {
        let minimatrix = vec![0; NR_OF_VALUES_MM];
        MiniMatrix { minimatrix }
    }

    fn generate_with_stub_data(start_value : usize) -> Self{
        let mut minimatrix :Vec<u8> = Vec::new();
        for i in start_value..start_value + NR_OF_VALUES_MM{
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

    fn generate_with_stub_data(start_value : usize) -> Self{
        // let mut vec = vec![vec!['#'; 80]; 24];
        let row = MiniMatrix::generate_blank();
        let mut sudoku :Vec<MiniMatrix> = vec![row;NR_OF_COL];
        let mut counter = 0;

        for row in start_value..start_value+NR_OF_COL{
            for col in start_value..start_value+NR_OF_COL{
                sudoku[row][col] = counter;
                counter+= 1;
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
}

fn main() {
    let minimatrix = MiniMatrix::generate_with_stub_data(0);
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
