use rand::Rng;
use std::{self, vec};

//Rust doesn't have a sqrt function for integers / usize
const NR_OF_COL_AS_FLOAT: f32 = 9.0;
const NR_OF_COL: usize = NR_OF_COL_AS_FLOAT as usize;

fn main() {
    generate_filled_soduko();
}

fn generate_filled_soduko(){
    let mut nr_of_tries: u128 = 1;
    let mut succes = true;
    let mut matrix = vec![0; NR_OF_COL.pow(2)];

    loop {
        if !succes {
            nr_of_tries += 1;
            wipe_all(&mut matrix);
            succes = true;
        } else {
            if fill_all(&mut matrix).is_err() {
                succes = false
            } else {
                break;
            }
        }
    }
    println!("    Generated in {} tries", nr_of_tries);
    print_matrix(&matrix);
}

fn fill_mini_matrix(
    start_row: usize,
    start_col: usize,
    matrix: &mut Vec<u8>,
) -> Result<&mut Vec<u8>, bool> {
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

            let mut valid: bool = false;
            while !valid {
                let number = generate_number();

                if used_numbers_matrix.len() != NR_OF_COL {
                    if no_available_numbers(
                        &used_numbers_row,
                        &used_numbers_col,
                        &used_numbers_matrix,
                    ) {
                        return Err(false);
                    }

                    if check_valid(&used_numbers_matrix, number)
                        && check_valid(&used_numbers_row, number)
                        && check_valid(&used_numbers_col, number)
                    {
                        matrix[(current_row * NR_OF_COL) + current_col] = number;
                        used_numbers_matrix.push(number);
                        valid = true;
                    }
                }
            }
        }
    }
    Ok(matrix)
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
    matrix
}

fn fill_all(matrix: &mut Vec<u8>) -> Result<&mut Vec<u8>, bool> {
    for matrices_in_row in 0..NR_OF_COL_AS_FLOAT.sqrt() as usize {
        for matrices_in_col in 0..NR_OF_COL_AS_FLOAT.sqrt() as usize{
            fill_mini_matrix(
                matrices_in_row * NR_OF_COL_AS_FLOAT.sqrt() as usize,
                matrices_in_col * NR_OF_COL_AS_FLOAT.sqrt() as usize,
                matrix,
            )?;
        }
    }

    Ok(matrix)
}

fn wipe_all(matrix: &mut Vec<u8>) -> &mut Vec<u8> {
    for matrices_in_row in 0..NR_OF_COL_AS_FLOAT.sqrt() as usize {
        for matrices_in_col in 0..NR_OF_COL_AS_FLOAT.sqrt() as usize {
            wipe_mini_matrix(
                matrices_in_row * NR_OF_COL_AS_FLOAT.sqrt() as usize,
                matrices_in_col * NR_OF_COL_AS_FLOAT.sqrt() as usize,
                matrix,
            );
        }
    }
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
    println!("\n ============ end of matrix =============== \n")
}
