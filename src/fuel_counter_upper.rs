pub fn fuel_needed_for_mass(mass: i32) -> i32 {
  (mass / 3) - 2
}

pub fn total_fuel_needed_for_masses(masses: Vec<i32>) -> i32 {
  let mut total: i32 = 0;
  for mass in masses.iter() {
    total += fuel_needed_for_mass(*mass);
  }
  total
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_fuel_needed_for_mass_12() {
    assert_eq!(2, fuel_needed_for_mass(12));
  }

  #[test]
  fn test_fuel_needed_for_mass_14() {
    assert_eq!(2, fuel_needed_for_mass(14));
  }

  #[test]
  fn test_fuel_needed_for_mass_1969() {
    assert_eq!(654, fuel_needed_for_mass(1969));
  }

  #[test]
  fn test_fuel_needed_for_mass_100756() {
    assert_eq!(33583, fuel_needed_for_mass(100756));
  }

  #[test]
  fn test_total_fuel_needed_for_masses() {
    let masses: Vec<i32> = vec![12, 14, 1969, 100756];
    let expected: i32 = 2 + 2 + 654 + 33583;
    assert_eq!(expected, total_fuel_needed_for_masses(masses));
  }

}
