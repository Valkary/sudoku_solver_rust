use std::vec::Vec;
use colored::Colorize;

/*
const BOARD: [[i32; 9]; 9] = [
	[5, 3, 0, 0, 7, 0, 0, 0, 0],
	[6, 0, 0, 1, 9, 5, 0, 0, 0],
	[0, 9, 8, 0, 0, 0, 0, 6, 0],
	[8, 0, 0, 0, 6, 0, 0, 0, 3],
	[4, 0, 0, 8, 0, 3, 0, 0, 1],
	[7, 0, 0, 0, 2, 0, 0, 0, 6],
	[0, 6, 0, 0, 0, 0, 2, 8, 0],
	[0, 0, 0, 5, 1, 9, 0, 0, 5],
	[0, 0, 0, 0, 8, 0, 0, 7, 9],
];
*/

const BOARD: [[u8; 9]; 9] = [
    [0,9,0,8,6,5,2,0,0],
    [0,0,5,0,1,2,0,6,8],
    [0,0,0,0,0,0,0,4,0],
    [0,0,0,0,0,8,0,5,6],
    [0,0,8,0,0,0,4,0,0],
    [4,5,0,9,0,0,0,0,0],
    [0,8,0,0,0,0,0,0,0],
    [2,4,0,1,7,0,5,0,0],
    [0,0,7,2,8,3,0,9,0]
];

/*
const BOARD: [[i32; 9]; 9] = [
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
    [0,0,0,0,0,0,0,0,0],
];
*/

fn main() {
    let new_board: [[u8; 9]; 9] = BOARD;
	println!("Welcome to the sudoku solver!");
	println!("========STARTING BOARD===========");
	display_board(BOARD);

    let positions_stack: &mut Vec<(usize, usize)> = &mut Vec::new();	
    solve_board(new_board, positions_stack, 1);
}

fn solve_board(mut board: [[u8; 9]; 9], stack: &mut Vec<(usize, usize)>, starting_num: u8) {
	let empty_pos: (usize, usize) = find_empty_pos(board);

    // Checamos que haya una posicion libre
    if empty_pos.0 == 10 && empty_pos.1 == 10 {
        return println!("\nBoard solved! :D");
    } else {
        for i in starting_num..10 as u8 {
            // Check for current num validity in our board
            if verify_num_in_cell(i.try_into().unwrap(), empty_pos.0, empty_pos.1, board) {

                // Push latest position to the stack and place it on the board
                stack.push((empty_pos.1, empty_pos.0));
                board[empty_pos.1][empty_pos.0] = i as u8;

                // Show the board and recurse
                display_board(board);
                solve_board(board, stack, 1);
            }
        }

        /*
         * [BACKTRACKING]
         * No valid number was found
         */
        if board[empty_pos.1][empty_pos.0] == 0 {
            // Get the last and second last elements of the stack
            let last_pos = stack.pop().unwrap();

            // Destructure the rows and cols from the positions
            let (last_row, last_col) = last_pos;

            // Calculate the starting number for the next iteration
            let last_val = board[last_row][last_col];
            let new_val = if (last_val + 1) > 9 { 1 } else { last_val + 1 };
            
            // Clear their values from the board by setting them to 0
            board[last_row][last_col] = 0;

            println!("\n=========BACKTRACKING==========");
            display_board(board);
           
            // Try to solve the board with new values
            solve_board(board, stack, new_val);
        }
    }
}

fn display_board(board: [[u8; 9]; 9]) {
    let mid_part = "-".repeat(9);
    let bar = format!("+{}+{}+{}+", mid_part, mid_part, mid_part);

	for i in 0..board.len() {
		for j in 0..board[i].len() {
            if j == 0 {
                if i != 0 { print!("|") };
                print!("\n");
                if i % 3 == 0 {
                    println!("{}", bar);
                }
			}

			// Imprimir una "|" de separacion
			if j % 3 == 0 || j == 0 {
				print!("|");
			}


            if BOARD[i][j] != 0 {
                print!(" {} ", format!("{}", board[i][j]).yellow())
            } else {
                print!(" {} ", board[i][j]);
            }

            if i == board.len() - 1 && j == board[i].len() - 1 {
                print!("|\n{}", bar);
            }

		}
	}
}

fn find_empty_pos(board: [[u8; 9]; 9]) -> (usize, usize) {
	/*
	 * i represents the row
	 * j represents the col
	 */
	for i in 0..board.len() {
		for j in 0..board[i].len() {
			if board[i][j] == 0 {
				// Empty position found
				return (j, i);
			}
		}
	}
	// Empty position not found
	return (10, 10);
}

fn check_num_in_row(num: u8, row: [u8; 9]) -> bool {
	for i in 0..row.len() {
		if row[i] == num {
			return false;
		}
	}

	return true;
}

fn check_num_in_col(num: u8, board: [[u8; 9]; 9], col: usize) -> bool {
	for i in 0..board.len() {
		if num == board[i][col] {
			return false;
		}
	}

	return true;
}

fn check_num_in_box(num: u8, board: [[u8; 9]; 9], col: usize, row: usize) -> bool {
	let box_x: usize = (col as f32 / 3.0).floor() as usize;
	let box_y: usize = (row as f32 / 3.0).floor() as usize;

	for i in (box_x * 3)..((box_x * 3) + 3) {
		for j in (box_y * 3)..((box_y * 3) + 3) {
			if num == board[j][i] {
				return false;
			}
		}
	}

	return true;
}

fn verify_num_in_cell(num: u8, col: usize, row: usize, board: [[u8; 9]; 9]) -> bool {
    if !check_num_in_row(num, board[row])
		|| !check_num_in_col(num, board, col)
		|| !check_num_in_box(num, board, col, row)
	{
		return false;
	} else {
		return true;
	}
}
