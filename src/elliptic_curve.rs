pub trait FieldElement<T = Self>:
  std::cmp::PartialEq
  + std::cmp::Eq
  + std::ops::Add<Output = T>
  + std::ops::Sub<Output = T>
  + std::ops::Neg<Output = T>
  + std::ops::Mul<Output = T>
  + std::ops::Div<Output = T>
  + std::fmt::Debug
  + Copy
  + Sized
{
}

impl<T> FieldElement for T where
  T: std::cmp::PartialEq
    + std::cmp::Eq
    + std::ops::Add<Output = T>
    + std::ops::Sub<Output = T>
    + std::ops::Neg<Output = T>
    + std::ops::Mul<Output = T>
    + std::ops::Div<Output = T>
    + std::fmt::Debug
    + Copy
{
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EllipticCurvePoint<T: FieldElement> {
  Zero { a: T, b: T },
  NonZero { x: T, y: T, a: T, b: T },
}

impl<T: FieldElement> EllipticCurvePoint<T> {
  pub fn new(x: T, y: T, a: T, b: T) -> Self {
    assert!(
      y * y == x * x * x + a * x + b,
      "({:?}, {:?}) is not in the elliptic curve y^2 == xˆ3 + {:?} * x + {:?}",
      x,
      y,
      a,
      b
    );
    Self::NonZero { x, y, a, b }
  }

  pub fn zero(a: T, b: T) -> Self {
    Self::Zero { a, b }
  }
}

impl<T: FieldElement> std::ops::Add for EllipticCurvePoint<T> {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    match (self, other) {
      (Self::Zero { a, b }, Self::Zero { a: oa, b: ob }) => {
        assert!(
          a == oa && b == ob,
          "cannot add points that are not on the same elliptic curve"
        );
        Self::Zero { a, b }
      }

      (Self::Zero { a, b }, Self::NonZero { x, y, a: oa, b: ob })
      | (Self::NonZero { x, y, a, b }, Self::Zero { a: oa, b: ob }) => {
        assert!(
          a == oa && b == ob,
          "cannot add points that are not on the same elliptic curve"
        );
        Self::NonZero { x, y, a, b }
      }

      (
        Self::NonZero { x: sx, y: sy, a, b },
        Self::NonZero {
          x: ox,
          y: oy,
          a: oa,
          b: ob,
        },
      ) => {
        assert!(
          a == oa && b == ob,
          "cannot add points that are not on the same elliptic curve"
        );

        if sx == ox && sy != oy {
          return Self::Zero { a, b };
        }

        let y_zero = sy - sy;

        if sx != ox {
          let s = (oy - sy) / (ox - sx);
          let x = s * s - sx - ox;
          let y = s * (sx - x) - sy;
          return Self::NonZero { x, y, a, b };
        }

        if self == other && sy == y_zero {
          return Self::Zero { a, b };
        }

        let s = ((sx * sx) + (sx * sx) + (sx * sx) + a) / (sy + sy);
        let x = s * s - sx - sx;
        let y = s * (sx - x) - sy;
        Self::NonZero { x, y, a, b }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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

  #[test]
  fn test_add1() {
    let a = EllipticCurvePoint::new(3, 7, 5, 7);
    let b = EllipticCurvePoint::new(-1, -1, 5, 7);
    assert_eq!(a + b, EllipticCurvePoint::new(2, -5, 5, 7));
  }

  #[test]
  fn test_add2() {
    let a = EllipticCurvePoint::new(-1, -1, 5, 7);
    assert_eq!(a + a, EllipticCurvePoint::new(18, 77, 5, 7));
  }
}
