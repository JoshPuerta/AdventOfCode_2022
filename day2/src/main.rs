extern crate file_reader;

use std::env;

fn policy_2col_move(tactic: &str) -> u32 {
    match tactic {
        "A X" => { 1 + 3 }
        "A Y" => { 2 + 6 }
        "A Z" => { 3 + 0 }
        "B X" => { 1 + 0 }
        "B Y" => { 2 + 3 }
        "B Z" => { 3 + 6 }
        "C X" => { 1 + 6 }
        "C Y" => { 2 + 0 }
        "C Z" => { 3 + 3 }
        _ => { panic!("Unparsable tactic!: {:?}", tactic); }
    }
}

fn policy_2col_result(tactic: &str) -> u32 {
    match tactic {
        "A X" => { 3 + 0 }
        "A Y" => { 1 + 3 }
        "A Z" => { 2 + 6 }
        "B X" => { 1 + 0 }
        "B Y" => { 2 + 3 }
        "B Z" => { 3 + 6 }
        "C X" => { 2 + 0 }
        "C Y" => { 3 + 3 }
        "C Z" => { 1 + 6 }
        _ => { panic!("Unparsable tactic!: {:?}", tactic); }
    }
}

fn follow_strategy(strategy: &[String], policy_id: &str) -> u32 {
    let policy = match policy_id {
        "move"   => { policy_2col_move }
        "result" => { policy_2col_result }
        _ => { panic!("Unparsable policy!: {:?}", policy_id); }
    };

    let global_score =
        strategy.iter()
                .fold(0, |acc, s| {
                    acc + policy(s)
                });
    global_score
}

fn main() {
    // Get input file as argument
    let args: Vec<String> = env::args().collect();
    
    // Read file
    let path = file_reader::sanitize(&args);
    let inputs: Vec<String> = file_reader::read_file(&path);

    // Follow given strategy
    let global_score = follow_strategy(&inputs, "move");
    println!("Global score with move: {:?}", &global_score);

    let global_score = follow_strategy(&inputs, "result");
    println!("Global score with result: {:?}", &global_score);


}
