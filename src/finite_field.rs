use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FiniteFieldElement {
  index: u32,
  prime: u32,
}

impl FiniteFieldElement {
  pub fn new(index: u32, prime: u32) -> Self {
    if index >= prime {
      panic!("index {:?} not in field range 0 to {:?}", index, prime - 1);
    }

    Self { index, prime }
  }

  fn pow(self, exponent: i32) -> Self {
    match exponent {
      0 => Self::new(1, self.prime),
      1 => self,
      i if i < 0 => self.pow(self.prime as i32 - 1 + i),
      _ => self * self.pow(exponent - 1),
    }
  }
}

impl Add for FiniteFieldElement {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    if self.prime != other.prime {
      panic!("Cannot add two elements from different finite fields");
    }

    let index = (self.index + other.index).rem_euclid(self.prime);
    Self::new(index, self.prime)
  }
}

impl Sub for FiniteFieldElement {
  type Output = Self;

  fn sub(self, other: Self) -> Self {
    if self.prime != other.prime {
      panic!("Cannot subtract two elements from different finite fields");
    }

    let index = match self.index < other.index {
      true => (self.index + self.prime - other.index).rem_euclid(self.prime),
      false => self.index - other.index,
    };

    Self::new(index, self.prime)
  }
}

impl Neg for FiniteFieldElement {
  type Output = Self;

  fn neg(self) -> Self {
    Self::new(self.prime - self.index, self.prime)
  }
}

impl Mul for FiniteFieldElement {
  type Output = Self;

  fn mul(self, other: Self) -> Self {
    if self.prime != other.prime {
      panic!("Cannot multiply two elements from different finite fields");
    }

    let index = (self.index * other.index).rem_euclid(self.prime);
    Self::new(index, self.prime)
  }
}

impl Div for FiniteFieldElement {
  type Output = Self;

  fn div(self, other: Self) -> Self {
    if self.prime != other.prime {
      panic!("Cannot divide two elements from different finite fields");
    }

    self * other.pow(-1)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eq() {
    let a = FiniteFieldElement::new(7, 13);
    let b = FiniteFieldElement::new(6, 13);
    assert_ne!(a, b);
    assert_eq!(a, a);
  }

  #[test]
  fn test_ne() {
    let a = FiniteFieldElement::new(2, 31);
    let b = FiniteFieldElement::new(2, 31);
    let c = FiniteFieldElement::new(15, 31);
    assert_eq!(a, b);
    assert_ne!(a, c);
  }

  #[test]
  fn test_add() {
    FiniteFieldElement::new(2, 31);
    let a = FiniteFieldElement::new(2, 31);
    let b = FiniteFieldElement::new(15, 31);
    assert_eq!(a + b, FiniteFieldElement::new(17, 31));

    let a = FiniteFieldElement::new(17, 31);
    let b = FiniteFieldElement::new(21, 31);
    assert_eq!(a + b, FiniteFieldElement::new(7, 31));
  }

  #[test]
  fn test_sub() {
    let a = FiniteFieldElement::new(29, 31);
    let b = FiniteFieldElement::new(4, 31);
    assert_eq!(a - b, FiniteFieldElement::new(25, 31));

    let a = FiniteFieldElement::new(15, 31);
    let b = FiniteFieldElement::new(30, 31);
    assert_eq!(a - b, FiniteFieldElement::new(16, 31));
  }

  #[test]
  fn test_mul() {
    let a = FiniteFieldElement::new(24, 31);
    let b = FiniteFieldElement::new(19, 31);
    assert_eq!(a * b, FiniteFieldElement::new(22, 31));
  }

  #[test]
  fn test_pow() {
    let a = FiniteFieldElement::new(17, 31);
    assert_eq!(a.pow(3), FiniteFieldElement::new(15, 31));

    let a = FiniteFieldElement::new(5, 31);
    let b = FiniteFieldElement::new(18, 31);
    assert_eq!(a.pow(5) * b, FiniteFieldElement::new(16, 31));
  }

  #[test]
  fn test_div() {
    let a = FiniteFieldElement::new(3, 31);
    let b = FiniteFieldElement::new(24, 31);
    assert_eq!(a / b, FiniteFieldElement::new(4, 31));

    let a = FiniteFieldElement::new(17, 31);
    assert_eq!(a.pow(-3), FiniteFieldElement::new(29, 31));

    let a = FiniteFieldElement::new(4, 31);
    let b = FiniteFieldElement::new(11, 31);
    assert_eq!(a.pow(-4) * b, FiniteFieldElement::new(13, 31));
  }
}
