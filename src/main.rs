mod elliptic_curve;
mod finite_field;

use elliptic_curve::EllipticCurvePoint;
use finite_field::FiniteFieldElement;

fn main() {
  println!("Hello, world!");
  println!(
    "This is a finite field element: {:?}",
    FiniteFieldElement::new(0, 1),
  );
  println!(
    "These are elliptic curve points: {:?} {:?}",
    EllipticCurvePoint::new(3, 7, 5, 7),
    EllipticCurvePoint::zero(5, 7)
  );
}
