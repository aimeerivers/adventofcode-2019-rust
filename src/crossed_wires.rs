pub fn points_occupied_by_wire(wire: &str) -> Vec<(i16, i16)> {
  let mut x = 0;
  let mut y = 0;
  let mut points: Vec<(i16, i16)> = vec![];

  let pieces = wire.split(",");
  for piece in pieces {
    let direction = piece.chars().next().unwrap();
    let distance = piece[1..].parse::<i16>().unwrap();
    for _i in 0..distance {
      match direction {
        'R' => x += 1,
        'L' => x -= 1,
        'U' => y += 1,
        'D' => y -= 1,
        _ => println!("Invalid direction: {:?}", direction),
      }
      points.push((x, y));
    }
  }

  points
}

pub fn crossing_points(wire1: &str, wire2: &str) -> Vec<(i16, i16)> {
  let points1 = points_occupied_by_wire(wire1);
  let points2 = points_occupied_by_wire(wire2);
  let mut crossings: Vec<(i16, i16)> = vec![];

  for point in points1 {
    if points2.contains(&point) {
      crossings.push(point);
    }
  }
  crossings
}

pub fn closest_crossing_point(wire1: &str, wire2: &str) -> (i16, i16) {
  let crossings = crossing_points(wire1, wire2);
  let mut closest = crossings[0];
  for point in crossings {
    if point.0.abs() + point.1.abs() < closest.0.abs() + closest.1.abs() {
      closest = point;
    }
  }
  closest
}

pub fn manhatten_distance(point: (i16, i16)) -> i16 {
  point.0.abs() + point.1.abs()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_points_occupied_by_wire_simple_examples() {
    assert_eq!(vec![(1, 0), (2, 0), (2, 1)],
      points_occupied_by_wire("R2,U1"));
    assert_eq!(vec![(1, 0), (2, 0), (2, -1), (2, -2)],
      points_occupied_by_wire("R2,D2"));
    assert_eq!(vec![(-1, 0), (-2, 0), (-2, -1)],
      points_occupied_by_wire("L2,D1"));
  }

  #[test]
  fn test_points_occupied_by_wire() {
    assert_eq!(vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0), (8, 0),
      (8, 1), (8, 2), (8, 3), (8, 4), (8, 5),
      (7, 5), (6, 5), (5, 5), (4, 5), (3, 5),
      (3, 4), (3, 3), (3, 2)],
      points_occupied_by_wire("R8,U5,L5,D3"));
    assert_eq!(vec![(0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 7),
      (1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7),
      (6, 6), (6, 5), (6, 4), (6, 3),
      (5, 3), (4, 3), (3, 3), (2, 3)],
      points_occupied_by_wire("U7,R6,D4,L4"));
  }

  #[test]
  fn test_crossing_points_negative_coordinates() {
    let wire1 = "L1,D2,L1";
    let wire2 = "D1,L2,D1";
    assert_eq!(vec![(-1, -1), (-2, -2)], crossing_points(wire1, wire2));
  }

  #[test]
  fn test_crossing_points() {
    let wire1 = "R8,U5,L5,D3";
    let wire2 = "U7,R6,D4,L4";
    assert_eq!(vec![(6, 5), (3, 3)], crossing_points(wire1, wire2));
  }

  #[test]
  fn test_closest_crossing_point_negative_coordinates() {
    let wire1 = "L1,D2,L1";
    let wire2 = "D1,L2,D1";
    assert_eq!((-1, -1), closest_crossing_point(wire1, wire2));
  }

  #[test]
  fn test_closest_crossing_point() {
    let wire1 = "R8,U5,L5,D3";
    let wire2 = "U7,R6,D4,L4";
    assert_eq!((3, 3), closest_crossing_point(wire1, wire2));
  }

  #[test]
  fn test_manhatten_distance_negative_coordinates() {
    let wire1 = "L1,D2,L1";
    let wire2 = "D1,L2,D1";
    assert_eq!(2, manhatten_distance(closest_crossing_point(wire1, wire2)));
  }

  #[test]
  fn test_example_1() {
    let wire1 = "R8,U5,L5,D3";
    let wire2 = "U7,R6,D4,L4";
    assert_eq!(6, manhatten_distance(closest_crossing_point(wire1, wire2)));
  }

  #[test]
  fn test_example_2() {
    let wire1 = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    let wire2 = "U62,R66,U55,R34,D71,R55,D58,R83";
    assert_eq!(159, manhatten_distance(closest_crossing_point(wire1, wire2)));
  }

  #[test]
  fn test_example_3() {
    let wire1 = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    let wire2 = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    assert_eq!(135, manhatten_distance(closest_crossing_point(wire1, wire2)));
  }

}
