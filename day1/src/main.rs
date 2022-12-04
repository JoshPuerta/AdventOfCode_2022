extern crate file_reader;

use std::env;

fn get_max_calories(calories: &[String]) -> Vec<u32> {
    let mut max_cal_reg  = Vec::new();
    let num_calories = 
        calories.iter()
                .fold(0, |acc, s| {
                    match s.is_empty() {
                        true  => { max_cal_reg.push(acc); 0 }
                        false => { acc + s.parse::<u32>().unwrap() }
                    }
                });
    println!("Last calories count: {num_calories}");
    max_cal_reg.sort_by(|u, v| v.cmp(u));
    max_cal_reg
}


fn main() {
    // Get input file as argument
    let args: Vec<String> = env::args().collect();
    
    // Read file
    let path = file_reader::sanitize(&args);
    let inputs: Vec<String> = file_reader::read_file(&path);

    // Get calories
    let max_calories = get_max_calories(&inputs);

    //println!("{:?}", inputs);
    println!("Max calories [1]: {:?}", &max_calories[0]);
    println!("Max calories [3]: {:?}. Sum: {:?}", 
                &max_calories[0..3], &max_calories[0..3].iter().sum::<u32>());

}
