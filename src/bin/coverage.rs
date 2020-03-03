extern crate rust_demo;
use crate::rust_demo::simulator::coverage_sim;
fn main() {
  let data = coverage_sim(5, 0.8, 10, 10000);
  println!("{:#?}", data);
}
