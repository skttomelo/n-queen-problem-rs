#[derive(Clone)]
pub struct Board{
    n: u8,
    board: Vec<u8>
}

impl Board{
    pub fn new(val1: u8, val2: Vec<u8>) -> Board{
        Board{
            n: val1,
            board: val2,
        }
    }
    pub fn get_neighbors(&self) -> Vec<Board>{
        let mut neighbors: Vec<Board> = Vec::new();
        for x in 0..self.n{
            for y in 0..self.n{
                if self.board[x as usize] == y{
                    continue
                }
                let mut neighbor = vec![0;self.n as usize];
                for i in 0..self.n{
                    if i == x{
                        neighbor[i as usize] = y;
                    }else{
                        neighbor[i as usize] = self.board[i as usize];
                    }
                }
                
                neighbors.push(Board{n:self.n, board:neighbor});
            }
        }

        neighbors
    }

    pub fn score(&self) -> u8{
        let mut score = 0u8;
        for queen in 0..self.n{
            for next_queen in queen+1..self.n{
                if self.board[queen as usize] == self.board[next_queen as usize]{
                    score += 1;
                }else if (queen as i8 - next_queen as i8).abs() == (self.board[queen as usize] as i8 - self.board[next_queen as usize] as i8).abs(){
                    score += 1;
                }
            }
        }
        score
    }

    pub fn is_peak_plateau(&self, comparison: &Board) -> bool{
        comparison.score() >= self.score()
    }

    pub fn get_n(&self) -> u8{
        self.n
    }

    pub fn to_string(&self) -> String{
        let mut data = String::from("");
        for y in 0..self.n{
            for x in 0..self.n{
                if self.board[x as usize] == y{
                    data.push_str("[Q]");
                }else{
                    data.push_str("[ ]");
                }
            }
            data.push_str("\n");
        }

        data.push_str("\n\n");

        data
    }
}