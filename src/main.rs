mod board;

use rand;
use rand::Rng;

fn generate_boards(n: u8, size: usize) -> Vec<board::Board>{
    let mut board_list: Vec<board::Board> = Vec::new();
    for _i in 0..size{
        let board = create_board(n);
        board_list.push(board::Board::new(n, board));
    }
    board_list
}

fn create_board(n: u8) -> Vec<u8>{
    let mut b = Vec::new();
    let mut r = rand::thread_rng();
    for _i in 0..n{
        let val: u8 = r.gen_range(0u8, n);
        b.push(val);
    }
    
    b
}

fn hill_climb(b: &board::Board) -> board::Board{
    let init = b.clone();
    let peak_plateau = false;
    while peak_plateau != false && init.score() != 0{

    }
    init
}

fn main() {
    let n = 8;
    let mut size = 1;
    let tables: Vec<board::Board> = generate_boards(n, size);
    
    for i in tables{
        println!("current board\n{}successors:\n", i.to_string());
        for successor in i.get_neighbors(){
            println!("The score: {}\n{}", successor.score(), successor.to_string());
        }
    }
}
