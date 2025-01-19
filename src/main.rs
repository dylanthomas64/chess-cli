use chess::{Board, Player};
use clap::Parser;
use std::io;
use std::io::Write;

mod chess;
//mod board;

fn main() {
    //println!("♔ 	♕ 	♖ 	♗ 	♘ 	♙ 	♚ 	♛ 	♜ 	♝ 	♞ 	♟ ");
    let mut board = Board::new();
    let white = Player::White;
    let black = Player::Black;

    /* loop {
                loop {get user white input -> if good play move -> if err continue, else break}
                loop {black}


    }

    */

    board.print();

    'main: loop {
        // main loop
        loop {
            // white turn
            match get_usr_input(&white) {
                Ok(usr_input) => {
                    if usr_input == "q" {
                        break 'main;
                    };
                    if let Err(err) = board.play_move(&white, &usr_input) {
                        eprintln!("{:?}", err);
                        continue;
                    } else {
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    continue;
                }
            }
        }
        loop {
            // black turn
            match get_usr_input(&black) {
                Ok(usr_input) => {
                    if usr_input == "q" {
                        break 'main;
                    };
                    if let Err(err) = board.play_move(&black, &usr_input) {
                        eprintln!("{:?}", err);
                        continue;
                    } else {
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("{:?}", err);
                    continue;
                }
            }
        }
    }

    println!("Exiting program...");
}

fn get_usr_input(player: &Player) -> Result<String, std::io::Error> {
    let mut usr_input = String::new();
    match player {
        Player::White => {
            println!("White to move: ");
        }
        Player::Black => {
            println!("Black to move: ");
        }
    }

    usr_input.clear();
    io::stdout().flush().expect("Cannot flush stdout");
    io::stdin().read_line(&mut usr_input)?;

    usr_input = usr_input
        .strip_suffix("\r\n")
        .or(usr_input.strip_suffix("\n"))
        .unwrap_or(&usr_input)
        .to_string();
    Ok(usr_input)
}

#[derive(Parser, Debug)]
struct Args {}
