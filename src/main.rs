use rand::Rng;
use std::{self, vec};

const NR_OF_COL_AS_FLOAT: f32 = 9.0;
const NR_OF_COL: usize = NR_OF_COL_AS_FLOAT as usize;

static mut SUCCES: bool = true;
static mut FINISHED: bool = false;
static mut WIPE_COUNT: u128 = 0;

fn main() {
    let mut matrix = vec![0; NR_OF_COL.pow(2)];

    // for n in 0..NR_OF_COL_AS_FLOAT.sqrt() as usize {
    //     println!("n = {}",n);
    //     fill_diagonal_mini_matrix(n * NR_OF_COL_AS_FLOAT.sqrt() as usize, &mut matrix);
    // }

    // print_matrix(&matrix);

    unsafe {
        while !FINISHED {
            fill_all(&mut matrix);
            if !SUCCES {
                wipe_all(&mut matrix);
                SUCCES = true;
            } else {
                FINISHED = true;
            }
        }
    }
unsafe{
    println!("Generated in {} tries", WIPE_COUNT);
}
    print_matrix(&matrix);
}

// fn fill_diagonal_mini_matrix(n: usize, matrix: &mut Vec<u8>) -> &mut Vec<u8> {
//     let mut used_numbers: Vec<u8> = Vec::new();
//     for row in n..n + NR_OF_COL_AS_FLOAT.sqrt() as usize {
//         for col in n..n + NR_OF_COL_AS_FLOAT.sqrt() as usize {
//             let mut valid: bool = false;
//             while !valid {
//                 let number = generate_number();
//                 if check_valid(&used_numbers, number) {
//                     matrix[(row * NR_OF_COL) + col] = number;
//                     used_numbers.push(number);
//                     valid = true;
//                 }
//             }
//         }
//     }
//     matrix
// }

fn fill_mini_matrix(start_row: usize, start_col: usize, matrix: &mut Vec<u8>) -> &mut Vec<u8> {
    let mut used_numbers_matrix: Vec<u8> = Vec::new();
    let mut used_numbers_row: Vec<u8> = Vec::new();
    let mut used_numbers_col: Vec<u8> = Vec::new();

    for current_row in start_row..start_row + NR_OF_COL_AS_FLOAT.sqrt() as usize {
        used_numbers_row.clear();
        for nr_in_row in 0..NR_OF_COL {
            let value = matrix
                [(start_row * NR_OF_COL) + nr_in_row + ((current_row - start_row) * NR_OF_COL)];
            if value != 0 && !used_numbers_row.contains(&value) {
                used_numbers_row.push(value);
            }
        }
        for current_col in start_col..start_col + NR_OF_COL_AS_FLOAT.sqrt() as usize {
            used_numbers_col.clear();
            for nr_in_col in 0..NR_OF_COL {
                let value = matrix[start_col + (current_col - start_col) + (nr_in_col * NR_OF_COL)];
                if value != 0 && !used_numbers_col.contains(&value) {
                    used_numbers_col.push(value);
                }
            }

            // println!(
            //     "invalid numbers row {}: {:?}",
            //     current_row, used_numbers_row
            // );
            // println!("invalid numbers col {}:{:?}", current_col, used_numbers_col);
            // println!("invalid numbers matrix: {:?}", used_numbers_matrix);
            // println!("===================== \n");

            let mut valid: bool = false;
            while !valid {
                let number = generate_number();

                if used_numbers_matrix.len() != NR_OF_COL {
                    if no_available_numbers(
                        &used_numbers_row,
                        &used_numbers_col,
                        &used_numbers_matrix,
                    ) {
                        // println!(
                        //     " NO MORE NUMBERS current_row: {} current_col: {} actual_nr: {}",
                        //     current_row,
                        //     current_col,
                        //     (current_row * NR_OF_COL) + current_col
                        // );

                        println!("NO MORE AVAILABLE NUMBERS");
                        unsafe {
                            SUCCES = false;
                        }
                        // println!(" GEEN NUMMERS MEER BESCHIKBAAR");
                        break;
                        //  wipe_mini_matrix(start_row, start_col, matrix);
                        //  fill_mini_matrix(start_row, start_col, matrix);
                        //return matrix;
                        // break;
                    }

                    if check_valid(&used_numbers_matrix, number)
                        && check_valid(&used_numbers_row, number)
                        && check_valid(&used_numbers_col, number)
                    {
                        println!("value: {}", number);
                        println!(
                            "current_row: {} current_col: {} actual_nr: {}",
                            current_row,
                            current_col,
                            (current_row * NR_OF_COL) + current_col
                        );
                        matrix[(current_row * NR_OF_COL) + current_col] = number;
                        used_numbers_matrix.push(number);
                        println!("invalid numbers row: {:?}", used_numbers_row);
                        println!("invalid numbers col: {:?}", used_numbers_col);
                        println!("invalid numbers matrix: {:?}", used_numbers_matrix);
                        println!("===================== \n");
                        valid = true;
                    }
                    // if used_numbers_matrix.len() != NR_OF_COL {
                    //     if no_available_numbers(
                    //         &used_numbers_row,
                    //         &used_numbers_col,
                    //         &used_numbers_matrix,
                    //     ) {
                    //         // println!(
                    //         //     " NO MORE NUMBERS current_row: {} current_col: {} actual_nr: {}",
                    //         //     current_row,
                    //         //     current_col,
                    //         //     (current_row * NR_OF_COL) + current_col
                    //         // );

                    //         println!("NO MORE AVAILABLE NUMBERS");
                    //         unsafe {
                    //             SUCCES = false;
                    //         }
                    //         // println!(" GEEN NUMMERS MEER BESCHIKBAAR");
                    //         break;
                    //         //  wipe_mini_matrix(start_row, start_col, matrix);
                    //         //  fill_mini_matrix(start_row, start_col, matrix);
                    //         //return matrix;
                    //         // break;
                    //     }
                }
                //check whether number 1-9 are no longer valid
            }
        }
    }
    //   drop(used_numbers);
    print_matrix(&matrix);
    matrix
}

fn generate_number() -> u8 {
    let mut rng = rand::thread_rng();
    let number = rng.gen_range(1, (NR_OF_COL + 1) as u8);
    number
}

fn check_valid(vector: &Vec<u8>, number: u8) -> bool {
    !vector.contains(&number)
}

fn no_available_numbers(
    unavalaible_row: &Vec<u8>,
    unavailable_col: &Vec<u8>,
    unavailable_matrix: &Vec<u8>,
) -> bool {
    let mut used_numbers: Vec<u8> = Vec::new();

    for number in 1..=NR_OF_COL as u8 {
        if !used_numbers.contains(&number)
            && (unavailable_col.contains(&number)
                || unavalaible_row.contains(&number)
                || unavailable_matrix.contains(&number))
        {
            used_numbers.push(number);
        }
    }
    return used_numbers.len() == NR_OF_COL;
}

fn wipe_mini_matrix(start_row: usize, start_col: usize, matrix: &mut Vec<u8>) -> &mut Vec<u8> {
    for current_row in start_row..start_row + NR_OF_COL_AS_FLOAT.sqrt() as usize {
        for current_col in start_col..start_col + NR_OF_COL_AS_FLOAT.sqrt() as usize {
            matrix[(current_row * NR_OF_COL) + current_col] = 0;
        }
    }
    // println!("WIPE");
    // print_matrix(matrix);
    matrix
}

fn fill_all(matrix: &mut Vec<u8>) -> &mut Vec<u8> {
    /*
    4x4
     */
    // fill_mini_matrix(0, 0, matrix);
    // fill_mini_matrix(0, NR_OF_COL_AS_FLOAT.sqrt() as usize, matrix);
    // fill_mini_matrix(NR_OF_COL_AS_FLOAT.sqrt() as usize, 0, matrix);
    // fill_mini_matrix(
    //     NR_OF_COL_AS_FLOAT.sqrt() as usize,
    //     NR_OF_COL_AS_FLOAT.sqrt() as usize,
    //     matrix,
    // );

    // 9x9
    fill_mini_matrix(0, 0, matrix); //diagonal
    fill_mini_matrix(0, NR_OF_COL_AS_FLOAT.sqrt() as usize, matrix);
    fill_mini_matrix(0, NR_OF_COL_AS_FLOAT.sqrt() as usize * 2, matrix);

    fill_mini_matrix(NR_OF_COL_AS_FLOAT.sqrt() as usize, 0, matrix);
    fill_mini_matrix(
        NR_OF_COL_AS_FLOAT.sqrt() as usize,
        NR_OF_COL_AS_FLOAT.sqrt() as usize,
        matrix,
    );
    fill_mini_matrix(
        NR_OF_COL_AS_FLOAT.sqrt() as usize,
        NR_OF_COL_AS_FLOAT.sqrt() as usize * 2,
        matrix,
    ); //diagonal

    fill_mini_matrix(NR_OF_COL_AS_FLOAT.sqrt() as usize * 2, 0, matrix);
    fill_mini_matrix(
        NR_OF_COL_AS_FLOAT.sqrt() as usize * 2,
        NR_OF_COL_AS_FLOAT.sqrt() as usize,
        matrix,
    );
    fill_mini_matrix(
        NR_OF_COL_AS_FLOAT.sqrt() as usize * 2,
        NR_OF_COL_AS_FLOAT.sqrt() as usize * 2,
        matrix,
    );
    matrix
}

fn wipe_all(matrix: &mut Vec<u8>) -> &mut Vec<u8> {
    /*
    4x4
    */
    // wipe_mini_matrix(0, 0, matrix);
    // wipe_mini_matrix(0, NR_OF_COL_AS_FLOAT.sqrt() as usize, matrix);
    // wipe_mini_matrix(NR_OF_COL_AS_FLOAT.sqrt() as usize, 0, matrix);
    // wipe_mini_matrix(
    //     NR_OF_COL_AS_FLOAT.sqrt() as usize,
    //     NR_OF_COL_AS_FLOAT.sqrt() as usize,
    //     matrix,
    // );

    //9x9
    wipe_mini_matrix(0, 0, matrix);
    wipe_mini_matrix(0, NR_OF_COL_AS_FLOAT.sqrt() as usize, matrix);
    wipe_mini_matrix(0, NR_OF_COL_AS_FLOAT.sqrt() as usize * 2, matrix);

    wipe_mini_matrix(NR_OF_COL_AS_FLOAT.sqrt() as usize, 0, matrix);
    wipe_mini_matrix(
        NR_OF_COL_AS_FLOAT.sqrt() as usize,
        NR_OF_COL_AS_FLOAT.sqrt() as usize,
        matrix,
    );
    wipe_mini_matrix(
        NR_OF_COL_AS_FLOAT.sqrt() as usize,
        NR_OF_COL_AS_FLOAT.sqrt() as usize * 2,
        matrix,
    );

    wipe_mini_matrix(NR_OF_COL_AS_FLOAT.sqrt() as usize * 2, 0, matrix);
    wipe_mini_matrix(
        NR_OF_COL_AS_FLOAT.sqrt() as usize * 2,
        NR_OF_COL_AS_FLOAT.sqrt() as usize,
        matrix,
    );
    wipe_mini_matrix(
        NR_OF_COL_AS_FLOAT.sqrt() as usize * 2,
        NR_OF_COL_AS_FLOAT.sqrt() as usize * 2,
        matrix,
    );

    println!("==================WIPE===========================");
    unsafe {
        WIPE_COUNT += 1;
        println!("nr of tries: {} ", WIPE_COUNT);
    }
    // print_matrix(matrix);
    matrix
}

fn print_matrix(matrix: &Vec<u8>) {
    for row in 0..NR_OF_COL {
        if row % NR_OF_COL_AS_FLOAT.sqrt() as usize == 0 {
            println!(" ");
        }
        for col in 0..NR_OF_COL {
            if col % NR_OF_COL_AS_FLOAT.sqrt() as usize == 0 {
                print!("    {} ", matrix[(row * NR_OF_COL) + col]);
            } else {
                print!(" {} ", matrix[(row * NR_OF_COL) + col]);
            }
        }
        println!(" ");
    }
    println!("============ end of matrix ===============")
}
