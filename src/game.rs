pub struct Game {
    pub board: Vec<Vec<isize>>,
    pub current_player: isize,
    pub winner: isize,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: vec![vec![0; 6]; 7],
            current_player: 1,
            winner: 0
        }
    }

    pub fn drop_token(&mut self, col: usize) -> bool {
        if self.valid(col) {
            for i in 0..6 {
                if self.board[col][i] == 0 {
                    self.board[col][i] = self.current_player;
                    self.check_win(col, i);
                    self.current_player *= -1;
                    return true;
                }
            }
        }
        false
    }

    fn valid(&self, col: usize) -> bool {
        col < 7 && self.board[col][5] == 0
    }

    fn check_win (&mut self, col: usize, row: usize) {
        let mut connected = 0;
        for i in 0..7 {
            if i > 6 {continue};
            if self.board[i][row] == self.current_player {
                connected += 1;
            } else {
                connected = 0;
            }
            if connected == 4 {
                self.winner = self.current_player;
                return
            };
        }
        connected = 0;
        for i in 0..6 {
            if self.board[col][i] == self.current_player {
                connected += 1;
            } else {
                connected = 0;
            }
            if connected == 4 {
                self.winner = self.current_player;
                return
            };
        }
    }
    
    pub fn print(&self) {
        for i in (0..6).rev() {
            for j in 0..7 {
                match self.board[j][i] {
                    -1 => print!("X "),
                    1 => print!("O  "),
                    _ => print!("- "),
                }
            }
            println!();
        }
        println!("-------------");
        println!("1 2 3 4 5 6 7");

    }
}

#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn test_create_board() {
        let game = Game::new();
        assert_eq!(game.current_player, 1);
    }

    #[test]
    fn test_update_board() {
        let mut game = Game::new();
        game.drop_token(1);
        assert_eq!(game.board[1][0], 1);
    }

    #[test]
    fn test_full_col_drop () {
        let mut game = Game::new();
        for i in 0..7 {
            game.drop_token(0);
        }
        assert!(!game.drop_token(0));
    }

    #[test]
    fn test_valid_drop() {
        let mut game = Game::new();
        assert!(game.drop_token(0));
    }

    #[test]
    fn test_drop_out_of_bounds() {
        let mut game = Game::new();
        // Should return a false result
        assert!(!game.drop_token(9));
    }

    #[test]
    fn test_win_horizontal() {
        let mut game = Game::new();
        game.drop_token(0);
        game.drop_token(0);
        game.drop_token(1);
        game.drop_token(1);
        game.drop_token(2);
        game.drop_token(2);
        game.drop_token(3);
        assert!(game.winner == 1);
    }

    #[test]
    fn test_win_vertical() {
        let mut game = Game::new();
        game.drop_token(0);
        game.drop_token(1);
        game.drop_token(0);
        game.drop_token(1);
        game.drop_token(0);
        game.drop_token(1);
        game.drop_token(0);
        assert!(game.winner == 1);
    }
    #[test]
    fn test_win_diagonal_up() {
        let mut game = Game::new();
        for i in 1..4 {
            for j in 1..4 {
                game.board[i][j] = 1;
            }
        }
        game.drop_token(0);
        assert_eq!(game.winner, 1);
    }

    #[test]
    fn test_win_diagonal_down() {
        let mut game = Game::new();
        for i in (0..3) {
            for j in (1..4).rev() {
                game.board[i][j] = 1;
            }
        }
        game.drop_token(3);
        assert_eq!(game.winner, 1);
    }
}