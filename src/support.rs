use std::fs;
use std::io;

pub fn csv_to_vec(line: &str) -> Vec<&str> {
    line.split(',').collect()
}

pub fn read_test_data(path: &str) -> Result<Vec<String>, io::Error> {
    let clean_path = fs::canonicalize(path)?;
    let s = fs::read_to_string(clean_path)?;

    let mut mv = Vec::new();

    for line in s.lines() {
        mv.push(String::from(line.trim()));
    }

    Ok(mv)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csv_to_vec() {
        assert_eq!(vec!["a"], csv_to_vec("a"));
        assert_eq!(vec!["a", "b"], csv_to_vec("a,b"));
        assert_eq!(vec!["a", "b", "c"], csv_to_vec("a,b,c"));
    }
}
