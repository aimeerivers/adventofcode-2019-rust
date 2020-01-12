mod file_reader;
mod fuel_counter_upper;

fn main() {
  day1();
}

fn day1() {
  let masses: Vec<u64> = file_reader::read_vec_u64_from_file("input/day1.txt");
  assert_eq!(3334297, fuel_counter_upper::total_fuel_needed_for_masses(masses));
}
