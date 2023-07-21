pub mod board;
pub mod tiles;

use std::str::FromStr;

use board::*;
use tiles::*;
fn input<T: FromStr>() -> Result<T, <T as std::str::FromStr>::Err> {
    let mut buf: String = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse()
}
fn main() {
    let mut last_player = Tile::O;
    loop {
        let mut board = Board::new();
        let mut player = last_player.switch();
        last_player = player;

        let mut tiles_placed = 0;
        while tiles_placed < 9 {
            println!("{board}");
            println!(
                "Player {:?}'s turn, type the number present at the box in which you wanna play",
                player
            );

            let pos = match input() {
                Ok(x) => x,
                Err(_) => {
                    eprintln!("Please enter a number");
                    continue;
                }
            };

            match board.put(pos, player) {
                Err(Error::InvalidEntry(x)) | Err(Error::InvalidPosition(x)) => {
                    eprintln!("{}", x);
                    continue;
                }
                Ok(()) => (),
            }

            tiles_placed += 1;
            if board.is_complete() {
                println!("{board}");
                println!("Player {:?} has won!", player);
                break;
            }

            player = player.switch();
        }
        if tiles_placed == 9 && !board.is_complete() {
            println!("{board}");
            println!("Game Draw");
        }

        println!("To exit enter e, to continue press any other key");

        if input().unwrap_or('\\') == 'e' {
            println!("Thanks for playing");
            break;
        }
    }
}
