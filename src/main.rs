mod elliptic_curve;
mod finite_field;

use elliptic_curve::EllipticCurvePoint;
use finite_field::FiniteFieldElement;

#[allow(non_snake_case)]
fn main() {
  let prime = 223;
  let a = FiniteFieldElement::new(0, prime);
  let b = FiniteFieldElement::new(7, prime);

  let G = EllipticCurvePoint::new(
    FiniteFieldElement::new(15, prime),
    FiniteFieldElement::new(86, prime),
    a,
    b,
  );
  let nG = EllipticCurvePoint::zero(a, b);
  let mut i = 1;
  while i * G != nG {
    println!("{}G = {}", i, i * G);
    i = i + 1;
  }
  println!("{}G = {}", i, i * G);
  println!("the order is {}", i);

  println!("{}G = {}", 100000, 100000 * G);
}
