use std::io;
use tic_tac_toe::Board;

fn main() {
    let user_player = Board::PLAYERS[0];
    let comp_player = Board::PLAYERS[1];
    let start_player = user_player;
    let mut board = Board::new(start_player);

    while !board.is_completed() {
        println!("{}", board);
        println!("{}'s move", board.get_current_turn());

        if board.get_current_turn() == user_player {
            let mut position_string = String::new();
            let pos: usize;
            println!("Enter number from 1-9 to make a move: ");
            io::stdin()
                .read_line(&mut position_string)
                .expect("Could not read it");

            if let Ok(p) = position_string.trim().parse::<usize>() {
                pos = p;
            } else {
                println!("Invalid input");
                continue;
            }

            if let Err(e) = board.player_move(user_player, pos - 1) {
                println!("Error: {}", e);
                continue;
            }
        } else {
            board.comp_move(comp_player);
        }
    }

    println!("{}", board);
    if let Some(winner) = board.get_winner() {
        println!("Player {} has won the game", winner);
    } else {
        println!("The game has ended as draw");
    }
}
