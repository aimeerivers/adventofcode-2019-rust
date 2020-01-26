mod file_reader;
mod fuel_counter_upper;
mod intcode_computer;

fn main() {
  day1_part1();
  day1_part2();
  day2_part1();
  day2_part2();
}

fn day1_part1() {
  let masses: Vec<i64> = file_reader::read_vec_i64_from_file("input/day1.txt");
  assert_eq!(3334297, fuel_counter_upper::total_fuel_needed_for_masses(masses));
}

fn day1_part2() {
  let masses: Vec<i64> = file_reader::read_vec_i64_from_file("input/day1.txt");
  assert_eq!(4998565, fuel_counter_upper::total_fuel_needed_for_masses_including_fuel(masses));
}

fn day2_part1() {
  let mut input_values: Vec<usize> = file_reader::read_vec_usize_from_file("input/day2.txt");
  input_values[1] = 12;
  input_values[2] = 2;
  let result: Vec<usize> = intcode_computer::run_program(input_values);
  assert_eq!(9581917, result[0]);
}

fn day2_part2() {
  let mut noun = 0;
  let mut verb = 0;
  let goal = 19690720;
  let input: Vec<usize> = file_reader::read_vec_usize_from_file("input/day2.txt");

  'outer_loop: for i in 0..100 {
    for j in 0..100 {
      let mut test = input.clone();
      test[1] = i;
      test[2] = j;
      let result: Vec<usize> = intcode_computer::run_program(test);
      if result[0] == goal {
        noun = i;
        verb = j;
        break 'outer_loop;
      }
    }
  }

  let answer = (100 * noun) + verb;
  assert_eq!(2505, answer);
}
