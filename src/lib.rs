const WIDTH: u8 = 7;
const HEIGHT: u8 = 6;

const THREE: i32 = 100;
const TWO: i32 = 10;

pub enum State {
    Red,
    Yellow,
    None,
}

pub fn piece(boards: &mut [u64; 2], col: u8) {
    let full = boards[0] | boards[1];

    let player = (full.count_ones() % 2) as usize;

    let column: u64 = (full >> (col * HEIGHT)) & ((1 << HEIGHT) - 1);

    if !(is_legal(column)) {
        println!("Column Full");
        return;
    }

    for i in 0..HEIGHT {
        if column >> i & 1 == 0 {
            boards[player] |= 1 << (col * HEIGHT) + i;
            break;
        }
    }
}

pub fn board(boards: [u64; 2]) {
    for row in (0..HEIGHT).rev() {
        for col in 0..WIDTH {
            let pos = col * HEIGHT + row;

            if (boards[0] | boards[1]) >> pos & 1 == 0 {
                print!(".")
            } else if (boards[0] >> pos) & 1 == 1 {
                print!("R");
            } else {
                print!("Y");
            }

            print!(" ");
        }

        println!();
    }
}

fn has_won(board: u64) -> bool {
    for i in 0..WIDTH {
        let column = (board >> (i * HEIGHT)) & ((1 << HEIGHT) - 1);

        if column == 0 {
            continue;
        }

        for j in 0..HEIGHT {
            let section = board >> (HEIGHT * i + j);
            let masks = [266305, 2113665, 33825];

            if (column >> j) & 15 == 15 {
                return true;
            }

            for m in masks {
                if (section & m) == m {
                    return true;
                }
            }
        }
    }

    false
}

pub fn winner(boards: [u64; 2]) -> State {
    if has_won(boards[0]) {
        return State::Red;
    }

    if has_won(boards[1]) {
        return State::Red;
    }

    State::None
}

fn is_legal(column: u64) -> bool {
    if column.count_ones() == HEIGHT.into() {
        return false;
    }

    true
}

pub fn evaluate(boards: [u64; 2]) -> i32 {
    let mut score = 0;

    if has_won(boards[0]) {
        return i32::MAX;
    }

    if has_won(boards[1]) {
        return i32::MIN;
    }

    score += consecutive(boards);
    score -= consecutive(boards);

    return score;
}

fn consecutive(boards: [u64; 2]) -> i32 {
    let mut score = 0;

    score += count_three_in_a_row(boards[0]) * THREE;
    score += count_two_in_a_row(boards[0]) * TWO;

    return score;
}

fn count_three_in_a_row(board: u64) -> i32 {
    return 0;
}

fn count_two_in_a_row(board: u64) -> i32 {
    return 0;
}
