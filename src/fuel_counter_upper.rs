pub fn fuel_needed_for_mass(mass: i64) -> i64 {
  let fuel_needed = (mass / 3) - 2;
  if fuel_needed <= 0 { 0 } else { fuel_needed }
}

pub fn fuel_needed_for_mass_including_fuel(mass: i64) -> i64 {
  let fuel_needed: i64 = fuel_needed_for_mass(mass);
  if fuel_needed <= 0 { 0 } else {
    fuel_needed + fuel_needed_for_mass_including_fuel(fuel_needed)
  }

}

pub fn total_fuel_needed_for_masses(masses: Vec<i64>) -> i64 {
  let mut total: i64 = 0;
  for mass in masses.iter() {
    total += fuel_needed_for_mass(*mass);
  }
  total
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_fuel_needed_for_mass() {
    assert_eq!(0, fuel_needed_for_mass(2));
    assert_eq!(2, fuel_needed_for_mass(12));
    assert_eq!(2, fuel_needed_for_mass(14));
    assert_eq!(654, fuel_needed_for_mass(1969));
    assert_eq!(33583, fuel_needed_for_mass(100756));
  }

  #[test]
  fn test_fuel_needed_for_mass_including_fuel() {
    assert_eq!(0, fuel_needed_for_mass_including_fuel(2));
    assert_eq!(2, fuel_needed_for_mass_including_fuel(14));
    assert_eq!(31 + 8, fuel_needed_for_mass_including_fuel(100));
    assert_eq!(966, fuel_needed_for_mass_including_fuel(1969));
    assert_eq!(50346, fuel_needed_for_mass_including_fuel(100756));
  }

  #[test]
  fn test_total_fuel_needed_for_masses() {
    let masses: Vec<i64> = vec![12, 14, 1969, 100756];
    let expected: i64 = 2 + 2 + 654 + 33583;
    assert_eq!(expected, total_fuel_needed_for_masses(masses));
  }

}
