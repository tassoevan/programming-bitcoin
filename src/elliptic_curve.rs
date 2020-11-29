use std::cmp::*;
use std::ops::*;

pub trait FieldElement<T = Self>:
  PartialEq
  + Eq
  + Add<Output = T>
  + Sub<Output = T>
  + Neg<Output = T>
  + Mul<Output = T>
  + Div<Output = T>
  + std::fmt::Debug
  + Copy
  + Sized
{
}

impl<T> FieldElement for T where
  T: PartialEq
    + Eq
    + Add<Output = T>
    + Sub<Output = T>
    + Neg<Output = T>
    + Mul<Output = T>
    + Div<Output = T>
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

  pub fn new_check(x: T, y: T, a: T, b: T) -> Result<Self, ()> {
    if y * y == x * x * x + a * x + b {
      return Ok(Self::NonZero { x, y, a, b });
    }

    Err(())
  }

  pub fn zero(a: T, b: T) -> Self {
    Self::Zero { a, b }
  }
}

impl<T: FieldElement> Add for EllipticCurvePoint<T> {
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

impl<T: FieldElement> Mul<EllipticCurvePoint<T>> for u32 {
  type Output = EllipticCurvePoint<T>;

  fn mul(self, point: EllipticCurvePoint<T>) -> EllipticCurvePoint<T> {
    match self {
      0 => match point {
        EllipticCurvePoint::Zero { a, b } => EllipticCurvePoint::zero(a, b),
        EllipticCurvePoint::NonZero { x: _, y: _, a, b } => EllipticCurvePoint::zero(a, b),
      },
      i => point + (i - 1) * point,
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

  #[test]
  fn test_finite_field_validation() {
    use super::super::finite_field::FiniteFieldElement;

    let prime = 223;
    let a = FiniteFieldElement::new(0, prime);
    let b = FiniteFieldElement::new(7, prime);

    let pt = |x, y| {
      EllipticCurvePoint::new_check(
        FiniteFieldElement::new(x, prime),
        FiniteFieldElement::new(y, prime),
        a,
        b,
      )
    };

    let valid_points = vec![pt(192, 105), pt(17, 56), pt(1, 193)];
    for p in valid_points.into_iter() {
      assert!(p.is_ok());
    }

    let invalid_points = vec![pt(200, 119), pt(42, 99)];
    for p in invalid_points.into_iter() {
      assert!(p.is_err());
    }
  }

  #[test]
  fn test_finite_field_add() {
    use super::super::finite_field::FiniteFieldElement;

    let prime = 223;
    let a = FiniteFieldElement::new(0, prime);
    let b = FiniteFieldElement::new(7, prime);

    let pt = |x, y| {
      EllipticCurvePoint::new(
        FiniteFieldElement::new(x, prime),
        FiniteFieldElement::new(y, prime),
        a,
        b,
      )
    };

    let additions = vec![
      (pt(192, 105), pt(17, 56), pt(170, 142)),
      (pt(47, 71), pt(117, 141), pt(60, 139)),
      (pt(143, 98), pt(76, 66), pt(47, 71)),
    ];

    for (p1, p2, p3) in additions.into_iter() {
      assert_eq!(p1 + p2, p3);
    }
  }

  #[test]
  fn test_finite_field_rmul() {
    use super::super::finite_field::FiniteFieldElement;

    let prime = 223;
    let a = FiniteFieldElement::new(0, prime);
    let b = FiniteFieldElement::new(7, prime);

    let pt = |x, y| {
      EllipticCurvePoint::new(
        FiniteFieldElement::new(x, prime),
        FiniteFieldElement::new(y, prime),
        a,
        b,
      )
    };

    let multiplications = vec![
      (2, pt(192, 105), pt(49, 71)),
      (2, pt(143, 98), pt(64, 168)),
      (2, pt(47, 71), pt(36, 111)),
      (4, pt(47, 71), pt(194, 51)),
      (8, pt(47, 71), pt(116, 55)),
      (21, pt(47, 71), EllipticCurvePoint::zero(a, b)),
    ];

    for (s, p1, p2) in multiplications.into_iter() {
      assert_eq!(s * p1, p2);
    }
  }
}
