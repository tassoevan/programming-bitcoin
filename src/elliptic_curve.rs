pub trait FieldElement:
  std::cmp::PartialEq
  + std::cmp::Eq
  + std::ops::Add
  + std::ops::Sub
  + std::ops::Neg
  + std::ops::Mul
  + std::ops::Div
  + Sized
{
}

#[derive(Debug, Clone, Copy)]
pub struct EllipticCurvePoint<T: FieldElement> {
  x: Option<T>,
  y: Option<T>,
  a: T,
  b: T,
}

impl<T: FieldElement> EllipticCurvePoint<T> {
  pub fn new(x: T, y: T, a: T, b: T) -> Self {
    EllipticCurvePoint {
      x: Some(x),
      y: Some(y),
      a,
      b,
    }
  }

  pub fn zero(a: T, b: T) -> Self {
    EllipticCurvePoint {
      x: None,
      y: None,
      a,
      b,
    }
  }

  pub fn is_zero(&self) -> bool {
    self.x == None || self.y == None
  }
}

impl<T: FieldElement> std::cmp::PartialEq for EllipticCurvePoint<T> {
  fn eq(&self, other: &Self) -> bool {
    self.a == other.a && self.b == other.b && self.x == other.x && self.y == other.y
  }
}

impl<T: FieldElement> std::ops::Neg for EllipticCurvePoint<T> {
  type Output = Self;

  fn neg(self) -> Self {
    self
  }
}

impl<T> std::ops::Add for EllipticCurvePoint<T>
where
  T: FieldElement + std::fmt::Debug,
{
  type Output = EllipticCurvePoint<T>;

  fn add(self, other: Self) -> Self {
    if self.a != other.a || self.b != other.b {
      panic!("Points {:?} {:?} are not on the same curve", self, other);
    }

    if self.is_zero() {
      return other;
    }

    if other.is_zero() {
      return self;
    }

    if self.x == other.x && self.y != other.y {
      return Self::zero(self.a, self.b);
    }

    todo!();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  impl FieldElement for i64 {}

  #[test]
  fn test_eq() {
    let a = EllipticCurvePoint::new(3, 7, 5, 7);
    let b = EllipticCurvePoint::new(18, 77, 5, 7);
    assert_ne!(a, b);
    assert_eq!(a, a);
  }

  #[test]
  fn test_add0() {
    let a = EllipticCurvePoint::zero(5, 7);
    let b = EllipticCurvePoint::new(2, 5, 5, 7);
    let c = EllipticCurvePoint::new(2, -5, 5, 7);
    assert_eq!(a + b, b);
    assert_eq!(b + a, b);
    assert_eq!(b + c, a);
  }

  // #[test]
  // fn test_add1() {
  //   let a = EllipticCurvePoint::new(3, 7, 5, 7);
  //   let b = EllipticCurvePoint::new(-1, -1, 5, 7);
  //   assert_eq!(a + b, EllipticCurvePoint::new(2, -5, 5, 7));
  // }

  // #[test]
  // fn test_add2() {
  //   let a = EllipticCurvePoint::new(-1, -1, 5, 7);
  //   assert_eq!(a + a, EllipticCurvePoint::new(18, 77, 5, 7),);
  // }
}
