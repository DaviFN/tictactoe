use text_io::read;
use std::io::Write;

mod board;

use board::Board;
use board::PieceLocation;
use board::piece::Piece;

fn display_game_state(board: &Board, player1_name: &String, player2_name: &String) {
    print!("board state:\n");
    print!("{}\n", board);
    print!("player 1: {}\n", player1_name);
    print!("player 2: {}\n", player2_name);
}

fn restart_game(board: &mut Board, turn_of_player1: &mut bool)
{
    board.restart();
    *turn_of_player1 = true;
}

fn handle_user_input(board: &mut Board, user_wants_to_quit: &mut bool, turn_of_player1: &mut bool, player1_name: &String, player2_name: &String)
{
    print!("enter \"q\" or \"quit\" to quit\n");
    print!("enter \"r\" or \"restart\" to restart\n");
    print!("\"x-y\" or \"xy\" to specify move (x and y being the coordinates of where you want to place your piece)\n");
    print!("for example, a2 will specify row a and column 2\n");
    
    let mut name_of_player_to_act: &String = player1_name;
    if !*turn_of_player1 {
        name_of_player_to_act = player2_name;
    }

    print!("{}'s turn: ", name_of_player_to_act);
    std::io::stdout().flush();

    let mut user_input_string: String = read!();

    user_input_string.to_lowercase();

    if user_input_string == "q" || user_input_string == "quit" {
        print!("quitting game...\n");
        *user_wants_to_quit = true;
        return
    }

    if user_input_string == "r" || user_input_string == "restart" {
        print!("restarting game...\n");
        restart_game(board, turn_of_player1);
        return
    }
    
    let piece_location = PieceLocation::new(&user_input_string);

    if(piece_location.is_valid()) {
        let mut type_of_piece_based_on_player: Piece = Piece::Cross;
        if !*turn_of_player1 {
            type_of_piece_based_on_player = Piece::Circle;
        }

        let piece = board.piece_in(&piece_location);
        
        match piece {
            Some(_) => {
                print!("there is already a piece there; choose another place\n");
            return
            },
            None => {
                board.place_piece(&piece_location, &type_of_piece_based_on_player);
            }
        }
        if board.is_in_win_condition_for_piece_type(&type_of_piece_based_on_player) {
            print!("{} wins!\n", name_of_player_to_act);
            restart_game(board, turn_of_player1);
        }
        *turn_of_player1 = !*turn_of_player1;
    }
    else {
        print!("I don't know what you mean by \"{}\"...", user_input_string);
    }
}
fn main() {

    let mut board = Board::new();

    print!("enter player 1's name: ");
    std::io::stdout().flush();

    let player1_name: String =  read!();

    print!("enter player 2's name: ");
    std::io::stdout().flush();

    let player2_name: String = read!();

    let mut turn_of_player1: bool = true;
    
    let mut user_wants_to_quit: bool = false;
    while !user_wants_to_quit {
        display_game_state(&board, &player1_name, &player2_name);
        handle_user_input(&mut board, &mut user_wants_to_quit, &mut turn_of_player1, &player1_name, &player2_name);
    }





}
