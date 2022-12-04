use std::fs;
use std::io::{self, BufRead};

pub fn sanitize(inp: &[String]) -> &str {
    
    if inp.len() > 2 {
        panic!("More than 1 argument given!");
    }
     &inp[1]
}

pub fn read_file(path: &str) -> Vec<String> {
    let file = fs::File::open(path).unwrap();
    let reader = io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file() {
        let expected = vec!["a".to_string(), "bb".to_string(), "ccc".to_string()];
        let result = read_file("test.txt");
        assert_eq!(result, expected);
    }

    #[test]
    fn test_sanitize() {
        let mockinput: Vec<String> = vec!["a".to_string(), "b".to_string()];
        let result = sanitize(&mockinput);
        assert_eq!(result, "b");
    }
}
