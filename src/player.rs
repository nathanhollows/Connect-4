use crate::game_master;
use std::io;

// Takes input from the players turn
pub fn player_turn(board: &mut Vec<Vec<usize>>, player_num: usize){
    game_master::print_board(& board);
    println!("-------------");
    println!("1 2 3 4 5 6 7");
    println!("Press the number corresponding to the column to place your token");
    
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(choice_num) => enter_choice(choice_num, board, player_num),
        Err(..) => error_choice(board, player_num),
    };
}

// Checks if the choice is valid and if so, updates the board
pub fn enter_choice(mut choice_num: usize, mut board: &mut Vec<Vec<usize>>, player_num: usize){
    if choice_num == 0{
        error_choice(board, player_num);
    } else{
        choice_num -= 1;
    }
    if choice_num < 7 && board[choice_num][0] == 0{
        for placement in (0..6).rev(){
            if board[choice_num][placement] == 0{
                board = game_master::update_board(board, choice_num, placement, player_num);
                break;
            }
        }
        game_master::game_loop(board, player_num);
    } else{
        
        error_choice(board, player_num);
    }
}

// Error code if the user enetered a value that is not valid
fn error_choice(board: &mut Vec<Vec<usize>>, player_num: usize){
    println!("This is not a valid option");
    println!("Please enter a valid option");
    player_turn(board, player_num);
}

#[cfg(test)]
mod tests{
    use crate::player;
    
    #[test]
    fn test_valid(){
        player::enter_choice(7, &mut vec![vec![0; 6]; 7], 1);
    }
    
    #[test]
    fn test_wrong(){
        player::enter_choice(8, &mut vec![vec![0; 6]; 7], 1);
    }
}