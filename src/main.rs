mod file_reader;
mod intcode_computer;

fn main() {
  solve();
}

fn solve() {
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
