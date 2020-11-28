mod finite_fields;

use finite_fields::{
  FiniteFieldElement
};

fn main() {
  println!("Hello, world!");
  println!("This is a finite field element: {:?}", FiniteFieldElement::new(0, 1));
}
