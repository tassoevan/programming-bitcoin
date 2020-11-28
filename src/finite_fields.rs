#[derive(Debug, Clone, Copy)]
pub struct FiniteFieldElement {
  index: i64,
  prime: i64,
}

impl FiniteFieldElement {
  pub fn new(index: i64, prime: i64) -> FiniteFieldElement {
    if index >= prime || index < 0 {
      panic!("index {:?} not in field range 0 to {:?}", index, prime - 1);
    }
    FiniteFieldElement { index, prime }
  }
}

impl std::cmp::PartialEq for FiniteFieldElement {
  fn eq(&self, other: &Self) -> bool {
    self.index == other.index && self.prime == other.prime
  }
}

impl std::ops::Add for FiniteFieldElement {
  type Output = FiniteFieldElement;

  fn add(self, other: FiniteFieldElement) -> FiniteFieldElement {
    if self.prime != other.prime {
      panic!("Cannot add two elements from different finite fields");
    }

    let index = (self.index + other.index) % self.prime;
    FiniteFieldElement::new(index, self.prime)
  }
}

impl std::ops::Sub for FiniteFieldElement {
  type Output = FiniteFieldElement;

  fn sub(self, other: FiniteFieldElement) -> FiniteFieldElement {
    if self.prime != other.prime {
      panic!("Cannot subtract two elements from different finite fields");
    }

    let remainder = (self.index - other.index) % self.prime; // can be negative
    let index = (remainder + self.prime) % self.prime;
    FiniteFieldElement::new(index, self.prime)
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
}
