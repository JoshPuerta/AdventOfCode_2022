extern crate file_reader;

use std::env;
use std::collections::HashSet;

const LOWERCASE_MAP : u8 = 'a' as u8 - 1;
const UPPERCASE_MAP : u8 = 'A' as u8 - 1;
const OFFSET : u8 = 26;

fn char2n_map(c: &char) -> u8 {
    match *c {
        'a'..='z' => { *c as u8 - LOWERCASE_MAP }
        'A'..='Z' => { *c as u8 - UPPERCASE_MAP + OFFSET }
        _ => panic!("fault_finder>> non expected character: {:?}", *c)
    }
}

fn iter_intersection(sets: &[HashSet<char>]) -> HashSet<char> {
    if sets.is_empty() { 
        return HashSet::new(); 
    }

    let mut result = sets[0].clone();

    for set in &sets[1..] {
        result = result.intersection(set).cloned().collect();
    }
    result
}

fn priority_finder(pload: &Vec<HashSet<char>>) -> u32 {

    let isect: HashSet<char> = iter_intersection(&pload);

    let mut isec_num: Vec<u8> = isect
        .iter()
        .map(|c| { char2n_map(c) })
        .collect();
    isec_num.pop().unwrap() as u32
}

fn priority_sum_perline(rucksacks: &[String]) -> u32 {
    let priority: u32 = rucksacks
        .iter()
        .fold(0u32, |acc, s| {
            let hhalf:  HashSet<char> = 
                s.chars().take(s.len() / 2).collect();
            let lhalf:  HashSet<char> = 
                s.chars().skip(s.len() / 2).collect();
            let payload = vec![hhalf, lhalf];
            acc + priority_finder(&payload) as u32
        });
    priority
}

fn priority_sum_nlines(rucksacks: &[String], n: u32) -> u32 {
    let mut idx = 0u32;
    let mut payload: Vec<HashSet<char>> = Vec::new();

    let priority: u32 = rucksacks
        .iter()
        .fold( 0u32,| acc, s | {
            let mut delta = 0u32;
            match idx % n {
                x if x == n - 1  => { 
                    let set: HashSet<char> = s.chars().collect();
                    payload.push(set);
                    delta = priority_finder(&payload);
                    payload.clear();
                }
                _ => { 
                    let set: HashSet<char> = s.chars().collect();
                    payload.push(set);
                }
            } 
            idx += 1;
            acc + delta
        });
    priority
}

fn main() {
    // Get input file as argument
    let args: Vec<String> = env::args().collect();
    
    // Read file
    let path = file_reader::sanitize(&args);
    let inputs: Vec<String> = file_reader::read_file(&path);

    // Part 1
    println!("Priority each line: {:?}", priority_sum_perline(&inputs));

    // Part 2
    println!("Priority with the 3 lines rule: {:?}", priority_sum_nlines(&inputs, 3));

}