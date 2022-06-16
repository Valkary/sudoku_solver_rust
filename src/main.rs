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
    let mut new_board = BOARD;
	println!("Welcome to the sudoku solver!");
	println!("========STARTING BOARD===========");
	display_board(&new_board);

    // let positions_stack: &mut Vec<(usize, usize)> = &mut Vec::new();	
    solve_board(&mut new_board);
}

fn solve_board(board: &mut [[u8; 9]; 9]) {
    let mut positions_that_have_been_set = Vec::new();
    
    'main_while: while let Some(empty_pos) = find_empty_pos(board) {
        // search through numbers to see if any are valid
        for i in 1..10 {
            // Check for num validity in our board
            if verify_num_in_cell(i, empty_pos.0, empty_pos.1, board) {
                // Push latest position to the stack and place it on the board
                positions_that_have_been_set.push(empty_pos);
                board[empty_pos.1][empty_pos.0] = i;

                // Show the board and continue with while loop
                display_board(board);
                continue 'main_while;
            }
        }

        println!("\n=========BACKTRACKING==========");

        // no valid number was found :(
        // back track by incrementing each, removing them if we wrap past 9
        'backtrack: loop {
            let last_position_set = positions_that_have_been_set.last().unwrap();
            // find a new value for the last cell
            for i in (board[last_position_set.1][last_position_set.0]+1)..=9 {
                // Check for num validity in our board
                if verify_num_in_cell(i, last_position_set.0, last_position_set.1, board) {
                    // set to next valid number
                    board[last_position_set.1][last_position_set.0] = i;

                    // Show the board and continue with while loop
                    display_board(board);
                    break 'backtrack;
                }
            }

            // we cant increase the cell so back track farther
            board[last_position_set.1][last_position_set.0] = 0;
            positions_that_have_been_set.pop();
        }
    }

    println!("\nBoard solved! :D");
}

fn display_board(board: &[[u8; 9]; 9]) {
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

fn find_empty_pos(board: &[[u8; 9]; 9]) -> Option<(usize, usize)> {
    /*
     * i represents the row
     * j represents the col
     */
    for (i, row) in board.iter().enumerate() {
        for (j, column) in row.iter().enumerate() {
            if *column == 0 {
                // Empty position found
                return Some((j, i));
            }
        }
    }
    None
}

fn check_num_in_row(num: u8, row: [u8; 9]) -> bool {
	for i in 0..row.len() {
		if row[i] == num {
			return false;
		}
	}

	return true;
}

fn check_num_in_col(num: u8, board: &[[u8; 9]; 9], col: usize) -> bool {
	for i in 0..board.len() {
		if num == board[i][col] {
			return false;
		}
	}

	return true;
}

fn check_num_in_box(num: u8, board: &[[u8; 9]; 9], col: usize, row: usize) -> bool {
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

fn verify_num_in_cell(num: u8, col: usize, row: usize, board: &[[u8; 9]; 9]) -> bool {
    if !check_num_in_row(num, board[row])
		|| !check_num_in_col(num, board, col)
		|| !check_num_in_box(num, board, col, row)
	{
		return false;
	} else {
		return true;
	}
}
