use std::io;
use crate::game::Game;

// Takes input from the players turn
pub fn player_turn(game: &Game) -> usize {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("\n{} Player 1", if game.current_player == 1 {"->"} else {"  "});
    println!("{} Player 2\n", if game.current_player == -1 {"->"} else {"  "});

    game.print();
    
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).expect("Something went wrong, press the any key");

    let trimmed = input_text.trim();
    match trimmed.parse::<usize>() {
        Ok(num) => num - 1,
        Err(..) => {
            eprintln!("Please enter a column between 1-7 to drop your marker");
            10 // This will fail
        }
    }
}