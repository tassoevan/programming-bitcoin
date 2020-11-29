mod elliptic_curve;
mod finite_field;

use finite_field::FiniteFieldElement;

fn main() {
  println!("Hello, world!");
  println!(
    "This is a finite field element: {:?}",
    FiniteFieldElement::new(0, 1)
  );
}
