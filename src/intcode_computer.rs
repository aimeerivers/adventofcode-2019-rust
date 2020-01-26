pub fn run_program(input_values: Vec<usize>) -> Vec<usize> {
  let mut memory = input_values;
  let mut instruction_pointer = 0;

  while memory[instruction_pointer] != 99 {
    let x = memory[instruction_pointer + 1];
    let y = memory[instruction_pointer + 2];
    let z = memory[instruction_pointer + 3];

    memory[z] = perform_instruction(memory[x], memory[y], memory[instruction_pointer]);
    instruction_pointer += 4;
  }

  memory
}

pub fn perform_instruction(noun: usize, verb: usize, opcode: usize) -> usize {
  if opcode == 1 {
    noun + verb
  } else if opcode == 2 {
    noun * verb
  } else {
    0
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_run_program_1() {
    let input_values = vec![1,0,0,0,99];
    assert_eq!(vec![2,0,0,0,99],
      run_program(input_values));
  }

  #[test]
  fn test_run_program_2() {
    let input_values = vec![2,3,0,3,99];
    assert_eq!(vec![2,3,0,6,99],
      run_program(input_values));
  }

  #[test]
  fn test_run_program_3() {
    let input_values = vec![2,4,4,5,99,0];
    assert_eq!(vec![2,4,4,5,99,9801],
      run_program(input_values));
  }

  #[test]
  fn test_run_program_4() {
    let input_values = vec![1,1,1,4,99,5,6,0,99];
    assert_eq!(vec![30,1,1,4,2,5,6,0,99],
      run_program(input_values));
  }

  #[test]
  fn test_run_program_5() {
    let input_values = vec![1,9,10,3,2,3,11,0,99,30,40,50];
    assert_eq!(vec![3500,9,10,70,2,3,11,0,99,30,40,50],
      run_program(input_values));
  }

  #[test]
  fn test_perform_addition_instruction() {
    assert_eq!(20, perform_instruction(10, 10, 1));
  }

  #[test]
  fn test_perform_multiplication_instruction() {
    assert_eq!(100, perform_instruction(10, 10, 2));
  }

  #[test]
  fn test_perform_unimplemented_instruction() {
    assert_eq!(0, perform_instruction(10, 10, 3));
  }

}
