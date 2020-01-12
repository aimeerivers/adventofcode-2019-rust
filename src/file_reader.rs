use std::fs::read_to_string;

pub fn read_vec_i64_from_file(filename: &str) -> Vec<i64> {
  let input = read_to_string(filename).unwrap();
  let output: Vec<i64> = input.split("\n")
    .map(|s: &str| s.parse::<i64>().unwrap())
    .collect();
  output
}

pub fn read_vec_usize_from_file(filename: &str) -> Vec<usize> {
  let input = read_to_string(filename).unwrap();
  let output: Vec<usize> = input.split(",")
    .map(|s: &str| s.parse::<usize>().unwrap())
    .collect();
  output
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_read_vec_i64_from_file() {
    let expected: Vec<i64> = vec![12, 14, 1969, 100756];
    assert_eq!(expected, read_vec_i64_from_file("fixtures/day1.txt"));
  }

  #[test]
  fn test_read_vec_usize_from_file() {
    let expected: Vec<usize> = vec![30,1,1,4,2,5,6,0,99];
    assert_eq!(expected, read_vec_usize_from_file("fixtures/day2.txt"));
  }

}
