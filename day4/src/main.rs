extern crate file_reader;

use std::env;
use std::collections::HashSet;


fn extract_ranges(section: &str) -> Vec<HashSet<u32>> {
    let mut assignments:  Vec<HashSet<u32>> = Vec::new();
    
    for range in section.split(',') {
        let extremes : Vec<&str> = range.split('-').collect();
        let start: u32 = extremes[0].parse().unwrap();
        let end: u32 = extremes[1].parse().unwrap();
        let assignment: HashSet<u32> = (start..=end).collect();
        assignments.push(assignment);
    }
    assignments
}

fn check_containment(sets : &[HashSet<u32>]) -> bool {
    sets[0].is_subset(&sets[1]) || sets[1].is_subset(&sets[0])
}

fn check_overlapping(sets : &[HashSet<u32>]) -> bool {
    let isect: HashSet<_> = sets[0].intersection(&sets[1]).collect();
    !isect.is_empty() 
}

fn get_redundant_assignments(assignments: &[String], policy_id: &str) -> u32 {

    let policy = match policy_id {
        "containment"   => { check_containment }
        "overlapping" => { check_overlapping }
        _ => { panic!("Unparsable policy!: {:?}", policy_id); }
    };

    let sections = assignments
        .iter()
        .fold(0u32, |acc, section| {
            let assignments = extract_ranges(&section);
            match policy(&assignments) {
                true => { acc + 1}
                false => { acc }
            }
        });
    sections
}

fn main() {
    // Get input file as argument
    let args: Vec<String> = env::args().collect();
    
    // Read file
    let path = file_reader::sanitize(&args);
    let inputs: Vec<String> = file_reader::read_file(&path);

    // Part 1
    println!("Redundant: {:?}", get_redundant_assignments(&inputs, "containment"));
    // Part 2
    println!("Redundant: {:?}", get_redundant_assignments(&inputs, "overlapping"));

}