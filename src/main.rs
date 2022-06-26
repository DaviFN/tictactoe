use text_io::read;
use std::io::Write;

#[derive(Eq, PartialEq)]
enum PieceType {
    None,
    Cross,
    Circle
}

impl std::fmt::Display for PieceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            PieceType::None => write!(f, "[ ]"),
            PieceType::Cross => write!(f, "[X]"),
            PieceType::Circle => write!(f, "[O]"),
        }
     }
}

fn create_board() -> [PieceType; 9]
{
    [
        PieceType::None, PieceType::None, PieceType::None,
        PieceType::None, PieceType::None, PieceType::None,
        PieceType::None, PieceType::None, PieceType::None
    ]
}

fn display_board(board: &[PieceType; 9])
{
    print!("{} {} {}\n{} {} {}\n{} {} {}\n", board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]);
}

fn display_game_state(board: &[PieceType; 9], player1_name: &String, player2_name: &String) {
    print!("board state:\n");
    display_board(board);
    print!("player 1: {}\n", player1_name);
    print!("player 2: {}\n", player2_name);
}

fn restart_game(board: &mut [PieceType; 9], turn_of_player1: &mut bool)
{
    for piece in board {
        *piece = PieceType::None;
    }
    *turn_of_player1 = true;
}

fn access_piece_from_indices(board: &mut [PieceType; 9], index_x: u32, index_y: u32) -> &mut PieceType {
    let board_position: usize = usize::try_from(index_x).unwrap() + 3 * usize::try_from(index_y).unwrap();
    let piece_specified: &mut PieceType = &mut board[board_position];
    piece_specified
}

fn is_in_win_condition_for_piece_type(board: &[PieceType; 9], piece_type: &PieceType) -> bool {
    for i in 0..3 {
        if board[3 * i] == *piece_type && board[3 * i + 1] == *piece_type && board[3 * i + 2] == *piece_type {
            return true;
        }
        if board[i] == *piece_type && board[i + 3] == *piece_type && board[i + 6] == *piece_type {
            return true;
        }
    }
    if board[0] == *piece_type && board[4] == *piece_type && board[8] == *piece_type {
        return true;
    }
    if board[2] == *piece_type && board[4] == *piece_type && board[6] == *piece_type {
        return true;
    }
    false
}

fn handle_user_input(board: &mut [PieceType; 9], user_wants_to_quit: &mut bool, turn_of_player1: &mut bool, player1_name: &String, player2_name: &String)
{
    print!("enter \"q\" or \"quit\" to quit,\n");
    print!("enter \"r\" or \"restart\" to restart\n");
    print!("\"x-y\" or \"xy\" to specify move (x and y being the coordinates of where you want to place your piece)\n");
    
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

    if string_represents_move(&user_input_string) {
        let input_as_vec: Vec<char> = user_input_string.chars().collect();
        
        let move_x: &char = &input_as_vec[0];
        let mut move_y: &char = &input_as_vec[1];
        if input_as_vec.len() == 3 {
            move_y = &input_as_vec[2];
        }

        let index_x: u32 = move_x.to_digit(10).unwrap() - 1;
        let index_y: u32 = move_y.to_digit(10).unwrap() - 1;

        let specific_piece: &mut PieceType = access_piece_from_indices(board, index_x, index_y);

        let mut type_of_piece_based_on_player: PieceType = PieceType::Cross;
        if !*turn_of_player1 {
            type_of_piece_based_on_player = PieceType::Circle;
        }
        
        if *specific_piece != PieceType::None {
            print!("there is already a piece there; choose another place\n");
            return
        }
        else {
            *specific_piece = type_of_piece_based_on_player;
            
            let mut type_of_piece_based_on_player: PieceType = PieceType::Cross;
            if !*turn_of_player1 {
                type_of_piece_based_on_player = PieceType::Circle;
            }

            if is_in_win_condition_for_piece_type(board, &type_of_piece_based_on_player) {
                print!("{} wins!\n", name_of_player_to_act);
                restart_game(board, turn_of_player1);
            }

            *turn_of_player1 = !*turn_of_player1;
        }

        return
    }

    print!("I don't know what you mean by \"{}\"...", user_input_string);



}

fn string_represents_move(str: &String) -> bool {
    if str.len() == 2 {
        let str_as_vec: Vec<char> = str.chars().collect();
        if(str_as_vec[0] == '1' || str_as_vec[0] == '2' || str_as_vec[0] == '3') {
            if(str_as_vec[1] == '1' || str_as_vec[1] == '2' || str_as_vec[1] == '3') {
                return true;
            }
        }
    }
    else if str.len() == 3 {
        let str_as_vec: Vec<char> = str.chars().collect();
        if(str_as_vec[1] == '-') {
            if(str_as_vec[0] == '1' || str_as_vec[0] == '2' || str_as_vec[0] == '3') {
                if(str_as_vec[2] == '1' || str_as_vec[2] == '2' || str_as_vec[2] == '3') {
                    return true;
                }
            }
        }
    }
    false
}

fn main() {

    let mut board = create_board();

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
