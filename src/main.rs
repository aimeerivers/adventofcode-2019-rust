use std::fs::read_to_string;

mod crossed_wires;

fn main() {
  solve();
}

fn solve() {
  let file = read_to_string("input/day3.txt").unwrap();
  let mut lines = file.lines();

  let wire1 = lines.next().unwrap();
  let wire2 = lines.next().unwrap();

  println!("wire1: {:?}", wire1);
  println!("wire2: {:?}", wire2);
  println!("{:?}", crossed_wires::points_occupied_by_wire(wire2).len());
  println!("distance: {:?}", crossed_wires::manhatten_distance(crossed_wires::closest_crossing_point(wire1, wire2)));
}
