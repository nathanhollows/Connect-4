use crate::player;
use crate::game::Game;

// Starts the game of Connect 4
pub fn start_game(){
    let mut game = Game::new();

    while game.winner == 0 {
        let col: usize = player::player_turn(&game);
        if game.valid(col) {
            game.drop_token(col);
        }
    }

    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("\nGame over!");
    match game.winner {
        -1 => println!("Congratulations player {}!", 2),
        _ => println!("Congratulations player {}!", game.winner),
    }
    println!();
    game.print();
}