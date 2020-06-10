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
pub fn game_loop(board: &mut Vec<Vec<usize>>, mut player_num: usize, xPos: usize, yPos: usize){
    if board_check(board, xPos, yPos, player_num){
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
fn board_check(board: &Vec<Vec<usize>>, xPos: usize, yPos: usize, player_num: usize) -> bool{
    let mut game_won: bool = false;
    let mut horizCount: i8 = 0;
    let mut vertiCount: i8 = 0;
    let mut diagCount1: i8 = 0;
    let mut diagCount2: i8 = 0;
    
    horizCount = horizCount + token_count(board, xPos, yPos,  1,  0, player_num,  0);
    horizCount = horizCount + token_count(board, xPos, yPos, -1,  0, player_num, -1);
    vertiCount = vertiCount + token_count(board, xPos, yPos,  0,  1, player_num,  0);
    vertiCount = vertiCount + token_count(board, xPos, yPos,  0, -1, player_num, -1);
    diagCount1 = diagCount1 + token_count(board, xPos, yPos, -1,  1, player_num,  0);
    diagCount1 = diagCount1 + token_count(board, xPos, yPos,  1, -1, player_num, -1);
    diagCount2 = diagCount2 + token_count(board, xPos, yPos,  1,  1, player_num,  0);
    diagCount2 = diagCount2 + token_count(board, xPos, yPos, -1, -1, player_num, -1);
    
    // Checks to see if player has beaten the game
    if horizCount >= 4 || vertiCount >= 4 || diagCount1 >= 4 || diagCount2 >= 4{
        print_board(board);
        println!("Player {} has won the game!", player_num);
        game_won = true;
    }
    
    // Checks if the game is a draw
    let mut draw: i8 = 0;
    for full in 0..7{
        if board[full][0] != 0{
            draw += 1;
        }
    }
    if draw == 7{
        print_board(board);
        println!("The game is a draw");
        game_won = true;
    }
    
    game_won
}

// Recursize function to check total amount in a line, from latest token played
fn token_count(board: &Vec<Vec<usize>>, mut xPos: usize, mut yPos: usize, xPosChange: i8, yPosChange: i8, player_num: usize, mut count: i8) -> i8{
    //Player Check
    if board[xPos][yPos] != player_num {
        return count;
    }
    
    //Boundary Checks
    let xBoundCheck = xPos as i8;
    let yBoundCheck = yPos as i8;
    if (xBoundCheck + xPosChange < 0 || yBoundCheck + yPosChange < 0) || (xBoundCheck + xPosChange >= 7 || yBoundCheck + yPosChange >= 6) {
        count += 1;
        return count;
    }
    
    xPos = update_position(xPos, xPosChange);
    yPos = update_position(yPos, yPosChange);
    count += 1;
    token_count(board, xPos, yPos, xPosChange, yPosChange, player_num, count)
}

// This gets given an old usize position and updates it with an i8
fn update_position(mut position: usize, change: i8) -> usize{
    let mut updater: i8 = 0;
    let result = to_i8(position);
    match result {
        Some(x) => updater = x,
        None => what_the_fuck(),
    }
    //println!("Position of X: {}", position);
    updater += change;
    //println!("Position of Updated X: {}", updater);
    position = updater as usize;
    position
}

// Takes a usize and trys to convert to an i8
fn to_i8 (num: usize) -> Option<i8> {
    if num > std::i8::MAX as usize {
        None
    } else {
        Some(num as i8)
    }
}

// Prints out the current board of Connect 4
pub fn print_board(board_print: &[Vec<usize>]){
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

fn what_the_fuck(){
    println!("What did you do to end up here...");
    println!("Seriously, what the fuck!?");
    println!("Good job detective, start over you piece of shit");
    std::process::exit(0);
}