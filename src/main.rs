use alstroemeria;
extern crate pollster;

fn main() {
  pollster::block_on(alstroemeria::run());
  println!("Hello, world!");
}

