mod file_reader;
mod fuel_counter_upper;
mod intcode_computer;

fn main() {
  day1_part1();
  day1_part2();
}

fn day1_part1() {
  let masses: Vec<i64> = file_reader::read_vec_i64_from_file("input/day1.txt");
  assert_eq!(3334297, fuel_counter_upper::total_fuel_needed_for_masses(masses));
}

fn day1_part2() {
  let masses: Vec<i64> = file_reader::read_vec_i64_from_file("input/day1.txt");
  assert_eq!(4998565, fuel_counter_upper::total_fuel_needed_for_masses_including_fuel(masses));
}
