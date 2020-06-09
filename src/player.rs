use crate::game_master;
use std::io;
use::std::convert::TryInto;

const X: usize = 7;
const Y: usize = 6;

// Takes input from the players turn
pub fn player_turn(board: &mut [[i32; Y]; X], player_num: i8){
    game_master::print_board(& board);
    println!("-------------");
    println!("1 2 3 4 5 6 7");
    println!("Press the number corresponding to the column to place your token");
    
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<i8>() {
        Ok(choice_num) => enter_choice(choice_num, board, player_num),
        Err(..) => error_choice(board, player_num),
    };
}

// Checks if the choice is valid and if so, updates the board
pub fn enter_choice(mut choice_num: i8, mut board: &mut [[i32; Y]; X], player_num: i8){
    choice_num -= 1;
    let choice_check: usize = choice_num.try_into().unwrap();
    if (choice_num > -1 && choice_num < 6) && (board[choice_check][0] == 0){
        for placement in (0..6).rev(){
            let placement_check: usize = placement.try_into().unwrap();
            if board[choice_check][placement_check] == 0{
                //board = game_master::update_board(board, choice_num, placement, player_num);
                break;
            }
        }
    //game_master::game_loop(board, player_num);
    } else{
        //error_choice(board, player_num);
    }
}

// Error code if the user enetered a value that is not valid
fn error_choice(board: &mut [[i32; Y]; X], player_num: i8){
    println!("This is not a valid option");
    println!("Please enter a valid option");
    player_turn(board, player_num);
}

#[cfg(test)]
mod tests{
    use crate::player;
    const X: usize = 7;
    const Y: usize = 6;
    
    #[test]
    fn test_valid(){
        player::enter_choice(7, &mut [[0; Y]; X], 1);
    }
    
    #[test]
    #[should_panic]
    fn test_wrong(){
        player::enter_choice(0, &mut [[0; Y]; X], 1);
    }
}