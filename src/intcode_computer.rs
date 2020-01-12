pub fn run_program(input: Vec<usize>) -> Vec<usize> {
  let mut output = input;
  let mut position = 0;

  while output[position] != 99 {
    let x = output[position + 1];
    let y = output[position + 2];
    let z = output[position + 3];

    output[z] = perform_operaton(output[x], output[y], output[position]);
    position += 4;
  }

  output
}

pub fn perform_operaton(x: usize, y: usize, operation: usize) -> usize {
  if operation == 1 {
    x + y
  } else if operation == 2 {
    x * y
  } else {
    0
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_run_program_1() {
    let input = vec![1,0,0,0,99];
    let expected = vec![2,0,0,0,99];
    assert_eq!(expected, run_program(input));
  }

  #[test]
  fn test_run_program_2() {
    let input = vec![2,3,0,3,99];
    let expected = vec![2,3,0,6,99];
    assert_eq!(expected, run_program(input));
  }

  #[test]
  fn test_run_program_3() {
    let input = vec![2,4,4,5,99,0];
    let expected = vec![2,4,4,5,99,9801];
    assert_eq!(expected, run_program(input));
  }

  #[test]
  fn test_run_program_4() {
    let input = vec![1,1,1,4,99,5,6,0,99];
    let expected = vec![30,1,1,4,2,5,6,0,99];
    assert_eq!(expected, run_program(input));
  }

  #[test]
  fn test_perform_addition_operation() {
    assert_eq!(20, perform_operaton(10, 10, 1));
  }

  #[test]
  fn test_perform_multiplication_operation() {
    assert_eq!(100, perform_operaton(10, 10, 2));
  }

  #[test]
  fn test_perform_unimplemented_operation() {
    assert_eq!(0, perform_operaton(10, 10, 3));
  }

}
