use std::fs::read_to_string;

pub fn read_vec_u64_from_file(filename: &str) -> Vec<u64> {
  let input = read_to_string(filename).unwrap();
  let masses: Vec<u64> = input.split("\n")
    .map(|s: &str| s.parse::<u64>().unwrap())
    .collect();
  masses
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_read_vec_u64_from_file() {
    let expected: Vec<u64> = vec![12, 14, 1969, 100756];
    assert_eq!(expected, read_vec_u64_from_file("fixtures/day1.txt"));
  }

}
