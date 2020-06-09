use crate::player;
use::std::convert::TryInto;

const X: usize = 7;
const Y: usize = 6;

// Starts the game of Connect 4
pub fn start_game(){
    let mut board = [[0; Y]; X];
    let coin_flip = 1;
    let player1: i8 = 1;
    let player2: i8 = 2;
    
    println!("Game Start!");
    if coin_flip == 1{
        player::player_turn(&mut board, player1);
    } else{
        player::player_turn(&mut board, player2);
    }
    
}

// Takes in a move made by a player and updates the board
pub fn update_board(update: &mut [[i32; Y]; X], posX: i8, posY: i8, player_num: i8) -> &mut [[i32; Y]; X]{
    //board_print[0][5] = 1;
    let _posX: usize = posX.try_into().unwrap();
    let _posY: usize = posY.try_into().unwrap();
    
    update[_posX][_posY] = player_num.into();
    update
}

// Checks the players turn, If the game is won, and continues the game loop
pub fn game_loop(board: &mut [[i32; Y]; X], mut player_num: i8){
    if board_check(board){
        println!("Player {} Won the game!", player_num);
        std::process::exit(0);
    } else {
        if player_num == 1{
            player_num += 1;
        } else{
            player_num -= 1;
        }
        println!("It is player {}'s turn", player_num);
        player::player_turn(board, player_num);
    }
}

// Checks the board to see if the game has been won
fn board_check(board: &[[i32; Y]; X]) -> bool{
    let game_won: bool = false;
    
    game_won
}

// Prints out the current board of Connect 4
pub fn print_board(board_print: & [[i32; Y]; X]){
    println!("Current Board:");
    for y_print in 0..Y{
        for x_print in 0..X{
            if board_print[x_print][y_print] == 1{
                print!("Y ");
            } else if board_print[x_print][y_print] == 2{
                print!("R ");
            } else{
                print!("0 ");
            }
        }
        println!();
    }
}