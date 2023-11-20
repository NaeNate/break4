use break4::{
    board, piece, winner,
    State::{None, Red, Yellow},
};

fn main() {
    let mut boards: [u64; 2] = [0, 0];

    loop {
        board(boards);

        let mut col = String::new();

        std::io::stdin().read_line(&mut col).unwrap();

        let col = col.trim().parse::<u8>().unwrap() - 1;

        print!("\x1B[2J");

        piece(&mut boards, col);

        match winner(boards) {
            Red => {
                println!("Red Wins");
                break;
            }
            Yellow => {
                println!("Yellow Wins");
                break;
            }
            None => continue,
        }
    }
}
