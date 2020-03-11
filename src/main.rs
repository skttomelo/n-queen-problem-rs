mod boards;

use rand;
use rand::Rng;
use std::time::SystemTime;
use boards::chess::Board;
use boards::Problem;


fn generate_boards(n: u8, size: usize) -> Vec<Board>{
    let mut board_list: Vec<Board> = Vec::new();
    for _i in 0..size{
        let board = create_board(n);
        board_list.push(Board::new(n, board));
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

fn hill_climb(b: &Board) -> Board{
    let mut init = b.clone();
    let mut peak_plateau = false;
    while peak_plateau == false && init.score() != 0{
        let mut solution = init.clone();
        for potential_solution in init.get_neighbors(){
            if potential_solution.score() < solution.score(){
                solution = potential_solution;
            }
        }
        if init.is_peak_plateau(&solution){
            peak_plateau = true;
        }else{
            init = solution;
        }
    }

    init
}

fn hill_climb_rand_restart(b: &Board) -> Board{
    let mut init = b.clone();
    while init.score() != 0{
        let mut solution = init.clone();
        for potential_solution in init.get_neighbors(){
            if potential_solution.score() < solution.score(){
                solution = potential_solution;
            }
        }
        if init.is_peak_plateau(&solution){
            init = Board::new(init.get_n(), create_board(init.get_n()));
        }else{
            init = solution;
        }
    }

    init
}

fn hill_climb_rand_restart_max(b: &Board, max_iterations: usize) -> Board{
    let mut init = b.clone();
    let mut iteration = 0;
    while init.score() != 0 && iteration != max_iterations{
        let mut solution = init.clone();
        for potential_solution in init.get_neighbors(){
            if potential_solution.score() < solution.score(){
                solution = potential_solution;
            }
        }
        if init.is_peak_plateau(&solution){
            init = Board::new(init.get_n(), create_board(init.get_n()));
        }else{
            init = solution;
        }
        iteration += 1;
    }

    init
}

fn main() {
    let n = 8;
    let mut size = 1;
    let max_iterations = 25;
    let mut timer;
    let mut runtime;

    // let p = Puzzle::new(vec![8,1,3,4,9,2,7,6,5]);

    // println!("The Score: {}\n{}", p.score(), p.to_string());

    for _i in 0..5{
        size *= 10;
        let tables: Vec<Board> = generate_boards(n, size);
        println!("Sample Size: {}", size);

        let mut avg_score = 0.0;
        let mut success_rate = 0.0;

        for a in &tables{
            avg_score += a.score() as f64;
            if a.score() == 0{
                success_rate += 1.0;
            }
        }
        avg_score = avg_score/size as f64;
        success_rate = (success_rate/size  as f64)*100.0;
        println!("avg: {}; success: {}", avg_score, success_rate);

        let mut hc_avg_score = 0.0;
        let mut hc_success_rate = 0.0;
        let mut hcrr_avg_score = 0.0;
        let mut hcrr_success_rate = 0.0;
        let mut hcrr_max_avg_score = 0.0;
        let mut hcrr_max_success_rate = 0.0;

        /*
        TODO:
        add in timer to track how many nanoseconds pass for hill climb and hill climb rand restart for doing all of the tables
        */ 

        timer = SystemTime::now();

        // hill climb
        for table in &tables{
            let hc_table = hill_climb(&table);
    
            hc_avg_score += hc_table.score() as f64;
            if hc_table.score() == 0{
                hc_success_rate += 1.0;
            }
        }

       runtime = timer.elapsed().unwrap();

        hc_avg_score = hc_avg_score/(size as f64);
        hc_success_rate = (hc_success_rate/(size as f64)) * 100.0;

        println!("After Hill Climb (Avg Score): {}\nAfter Hill Climb (Success Rate): {}%\nHill Climb Runtime: {:?}\n", hc_avg_score, hc_success_rate, runtime);

        timer = SystemTime::now();
        // hill climb rand restart
        for table in &tables{
            let hcrr_table = hill_climb_rand_restart(&table);
    
            hcrr_avg_score += hcrr_table.score() as f64;
            if hcrr_table.score() == 0{
                hcrr_success_rate += 1.0;
            }
        }
        runtime = timer.elapsed().unwrap();

        hcrr_avg_score = hcrr_avg_score/(size as f64);
        hcrr_success_rate = (hcrr_success_rate/(size as f64)) * 100.0;
        println!("After Hill Climb Rand Restart (Avg Score): {}\nAfter Hill Climb Rand Restart (Success Rate): {}%\nHill Climb Rand Restart Runtime: {:?}\n", hcrr_avg_score, hcrr_success_rate, runtime);
        
        timer = SystemTime::now();
        // hill climb rand restart with max iteration limit
        for table in &tables{
            let hcrr_max_table = hill_climb_rand_restart_max(&table, max_iterations);
    
            hcrr_max_avg_score += hcrr_max_table.score() as f64;
            if hcrr_max_table.score() == 0{
                hcrr_max_success_rate += 1.0;
            }
        }
        runtime = timer.elapsed().unwrap();

        hcrr_max_avg_score = hcrr_max_avg_score/(size as f64);
        hcrr_max_success_rate = (hcrr_max_success_rate/(size as f64)) * 100.0;
        println!("After Hill Climb Rand Restart w/ max_iterations = {} (Avg Score): {}\nAfter Hill Climb Rand Restart w/ max_iterations = {} (Success Rate): {}%\nHill Climb Rand Restart w/ max_iterations = {} Runtime: {:?}\n", max_iterations, hcrr_max_avg_score, max_iterations, hcrr_max_success_rate, max_iterations, runtime);

        println!("\n---------------------------------------------------------------------------------\n");
    }
}
