use rand::Rng;
use std::fmt;

pub struct Board<'a> {
    state: Vec<&'a str>,
    num_of_moves: u32,
    turn: &'a str,
}

impl<'a> Board<'a> {
    const TOTAL_MOVES_ALLOWED: usize = 9;
    pub const PLAYERS: [&'a str; 3] = ["X", "O", "0"];

    pub fn new(start_player: &'static str) -> Self {
        Board {
            state: vec![Board::PLAYERS[2]; Board::TOTAL_MOVES_ALLOWED],
            num_of_moves: 0,
            turn: start_player,
        }
    }

    fn change_turn(&mut self) {
        self.turn = match self.turn {
            "X" => "O",
            "O" => "X",
            _ => self.turn,
        };
    }

    pub fn get_current_turn(&self) -> &str {
        self.turn
    }

    fn update_board(&mut self, player_type: &'a str, board_position: usize) {
        self.state[board_position] = player_type;
        self.num_of_moves += 1;
        self.change_turn();
    }

    pub fn player_move(&mut self, player: &'a str, pos: usize) -> Result<(), &str> {
        if self.is_legal_position(pos) {
            self.update_board(player, pos);
            Ok(())
        } else {
            Err("Illegal Position Supplied")
        }
    }

    pub fn comp_move(&mut self, player: &'a str) {
        let pos = rand::thread_rng().gen_range(0, Board::TOTAL_MOVES_ALLOWED);

        if self.is_legal_position(pos) {
            self.update_board(player, pos);
        }
    }

    fn is_legal_position(&self, pos: usize) -> bool {
        pos < Board::TOTAL_MOVES_ALLOWED && self.state.get(pos).unwrap() == &Board::PLAYERS[2]
    }

    pub fn is_completed(&self) -> bool {
        if let Some(_) = self.get_winner() {
            true
        } else {
            self.num_of_moves == 9
        }
    }

    pub fn get_winner(&self) -> Option<&str> {
        for player in Board::PLAYERS[0..1].iter() {

            for &row in &[0, 3, 6] {
                if self.state.get(row).unwrap() == player && self.state.get(row + 1).unwrap() == player && self.state.get(row + 2).unwrap() == player {
                    return Some(player);
                }
            }

            for &col in &[0, 1, 2] {
                if self.state.get(col).unwrap() == player && self.state.get(col + 3).unwrap() == player && self.state.get(col + 6).unwrap() == player {
                    return Some(player);
                }
            }

            let mut diagonal_element_is_player = true;

            for &diagonal_element in &[0, 4, 8] {
                diagonal_element_is_player = diagonal_element_is_player && self.state.get(diagonal_element).unwrap() == player;
            }

            if diagonal_element_is_player {
                return Some(player);
            }

            let mut opposite_diagonal_element_is_player = true;

            for &diagonal_element in &[0, 4, 8] {
                opposite_diagonal_element_is_player = opposite_diagonal_element_is_player && self.state.get(diagonal_element).unwrap() == player;
            }

            if opposite_diagonal_element_is_player {
                return Some(player);
            }
        }

        None
    }
}

impl fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_display = String::new();

        for (index, elem) in self.state.iter().enumerate() {
            if (index + 1).rem_euclid(3) == 0 {
                board_display = format!("{} {}\n", &board_display, &elem);
            } else {
                board_display = format!("{} {}", &board_display, &elem);
            }
        }

        write!(f, "{}", board_display)
    }
}
