use std::time::{Instant};

// The board dimensions.
const NUM_ROWS: usize = 8;
const NUM_COLS: usize = NUM_ROWS;
const INUM_ROWS: i32 = NUM_ROWS as i32;
const INUM_COLS: i32 = NUM_COLS as i32;
const NUM_SQUARES: i32 = INUM_ROWS * INUM_COLS;
const START_COL: i32 = 0;
const START_ROW: i32 = 0;

// Whether we want an open or closed tour.
const REQUIRE_CLOSED_TOUR: bool = false;

// Value to represent a square that we have not visited.
const UNVISITED: i32 = -1;

fn main() {
    // Initialize the vector of move offsets.
    let mut offsets = [
        [-2, -1],
        [-1, -2],
        [ 2, -1],
        [ 1, -2],
        [-2,  1],
        [-1,  2],
        [ 2,  1],
        [ 1,  2],
    ];

    // Create a NUM_ROWS x NUM_COLS vector with all entries Initialized to UNVISITED.
    let mut board = [[UNVISITED; NUM_COLS]; NUM_ROWS];

    // Start at board[0][0].
    board[0][0] = 0;

    // Try to find a tour.
    let start = Instant::now();
    let success = find_tour(&mut board, &mut offsets, 0, 0, 1);
    let duration = start.elapsed();
    println!("Time: {:?}", duration);

    if success {
        println!("Success!");
    } else {
        println!("Could not find a tour.");
    }

    dump_board(&board);
}

// Try to extend a knight's tour starting at (start_row, start_col).
// Return true or false to indicate whether we have found a solution.
fn find_tour(board: &mut [[i32; NUM_COLS]; NUM_ROWS],
    offsets: &mut [[i32; 2]; 8],    // 8 possible moves, 2 coordinates each.
    cur_row: i32, cur_col: i32,
    num_visited: i32) -> bool {

    for [row_offset, col_offset] in *offsets {
        let pot_row= cur_row + row_offset;
        let pot_col = cur_col + col_offset;

        let row_in_bounds = pot_row >= 0 && pot_row < INUM_ROWS;
        let col_in_bounds = pot_col >= 0 && pot_col < INUM_COLS;
        if col_in_bounds && row_in_bounds {
            if board[pot_row as usize][pot_col as usize] == UNVISITED {
                board[pot_row as usize][pot_col as usize] = num_visited;
                if num_visited == NUM_SQUARES - 1 {
                    if REQUIRE_CLOSED_TOUR {
                        if can_close_tour(pot_row, pot_col) {
                           return true;
                        }
                        board[pot_row as usize][pot_col as usize] = UNVISITED;
                    } else {
                        return true;
                    }
                } else {
                    if find_tour(board, offsets, pot_row, pot_col, num_visited + 1) {
                        return true;
                    }
                    board[pot_row as usize][pot_col as usize] = UNVISITED;    
                }
            }
        }
    }
    false
}


// If the current position is one move away from the starting position,
// we can close the tour.
fn can_close_tour(cur_row: i32, cur_col: i32) -> bool {
   let row_offs = (cur_row - START_ROW).abs();
   let col_offs = (cur_col - START_COL).abs();
   (row_offs == 1 && col_offs == 2) || (row_offs == 2 && col_offs == 1)
}

fn dump_board(board: &[[i32; NUM_COLS]; NUM_ROWS]) {
    for &row in board.iter() {
        for &square in row.iter() {
            print!("{:2} ", square);
        }
        println!();
    }
}