use std::fs::read_to_string;

pub fn read_vec_i64_from_file(filename: &str) -> Vec<i64> {
  let input = read_to_string(filename).unwrap();
  let output: Vec<i64> = input.split("\n")
    .map(|s: &str| s.parse::<i64>().unwrap())
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

}
