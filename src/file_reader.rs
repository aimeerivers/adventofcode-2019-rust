use std::fs::read_to_string;

pub fn read_vec_i64_from_file(filename: &str) -> Vec<i64> {
  let input = read_to_string(filename).unwrap();
  input.lines()
    .map(|s: &str| s.parse::<i64>().unwrap())
    .collect()
}

pub fn read_vec_usize_from_file(filename: &str) -> Vec<usize> {
  let input = read_to_string(filename).unwrap();
  input.split(",")
    .map(|s: &str| s.parse::<usize>().unwrap())
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_read_vec_i64_from_file() {
    assert_eq!(vec![12, 14, 1969, 100756],
      read_vec_i64_from_file("fixtures/day1.txt"));
  }

  #[test]
  fn test_read_vec_usize_from_file() {
    assert_eq!(vec![30,1,1,4,2,5,6,0,99],
      read_vec_usize_from_file("fixtures/day2.txt"));
  }

}
