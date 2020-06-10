use crate::player;

// Starts the game of Connect 4
pub fn start_game(){
    let mut board = vec![vec![0; 6]; 7];
    let coin_flip = 1;
    let player1: usize = 1;
    let player2: usize = 2;
    
    println!("Game Start!");
    if coin_flip == 1{
        player::player_turn(&mut board, player1);
    } else{
        player::player_turn(&mut board, player2);
    }
    
}

// Takes in a move made by a player and updates the board
pub fn update_board(update: &mut Vec<Vec<usize>>, posX: usize, posY: usize, player_num: usize) -> &mut Vec<Vec<usize>>{
    update[posX][posY] = player_num;
    update
}

// Checks the players turn, If the game is won, and continues the game loop
pub fn game_loop(board: &mut Vec<Vec<usize>>, mut player_num: usize){
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
fn board_check(board: &Vec<Vec<usize>>) -> bool{
    let game_won: bool = false;
    
    game_won
}

// Prints out the current board of Connect 4
pub fn print_board(board_print: &Vec<Vec<usize>>){
    println!("Current Board:");
    for y_print in 0..6{
        for x_print in 0..7{
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