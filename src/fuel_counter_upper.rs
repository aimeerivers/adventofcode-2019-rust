pub fn fuel_needed_for_mass(mass: i32) -> i32 {
  (mass / 3) - 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_with_mass_12() {
    assert_eq!(2, fuel_needed_for_mass(12));
  }

  #[test]
  fn test_with_mass_14() {
    assert_eq!(2, fuel_needed_for_mass(14));
  }

  #[test]
  fn test_with_mass_1969() {
    assert_eq!(654, fuel_needed_for_mass(1969));
  }

  #[test]
  fn test_with_mass_100756() {
    assert_eq!(33583, fuel_needed_for_mass(100756));
  }
}
