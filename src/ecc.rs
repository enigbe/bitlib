/// Module containing helper primitives for Elliptic Curve Cryptography (ECC)
use std::fmt::{Display, format, Formatter, Result as FmtResult};
use std::ops::{Add, Sub, Mul, Div};

/// A element belonging to a finite set of prime order
#[derive(Debug, PartialEq)]
pub struct FieldElement {
    pub num: i32,
    pub prime: i32,
}

/// An enumeration of ECC errors
#[derive(Debug)]
pub enum ECCError {
    TypeError(String),
    ValueError(String),
}

/// Implements display trait for FieldElement
impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.num, self.prime)
    }
}

/// Implements addition trait for a FieldElement
impl Add for FieldElement {
    type Output = Result<FieldElement, ECCError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot add two elements in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {
            let num = modulus(self.num + rhs.num, self.prime);
            Ok(FieldElement {
                num,
                prime: self.prime,
            })
        }
    }
}

/// Implements subtraction trait for FieldElement
impl Sub for FieldElement {
    type Output = Result<FieldElement, ECCError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot subtract two elements in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {
            let num = modulus(self.num - rhs.num, self.prime);
            Ok(FieldElement {
                num,
                prime: self.prime,
            })
        }
    }
}

/// Implement multiplication trait for FieldElement
impl Mul for FieldElement {
    type Output = Result<FieldElement, ECCError>;
    
    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot multiply two number in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {
            let num = modulus(self.num * rhs.num, self.prime);
            Ok(FieldElement {
                num,
                prime: self.prime,
            })
        }
    }
}

/// Implement division trait for FieldElement
impl Div for FieldElement {
    type Output = Result<FieldElement, ECCError>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot divide two number in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {
            let fermat_pos: u32 = (self.prime - 2).try_into().unwrap();
            let a: i32 = rhs.num.pow(fermat_pos);

            let divisor = modulus(a, self.prime);
            let dividend = self.num * divisor;
            let num = modulus(dividend, self.prime);

            Ok(FieldElement{
                num,
                prime: self.prime,
            })
        }
    }
}

/// Implement methods for FieldElement
impl FieldElement {
    /// Create a new FieldElement with integer num and prime
    pub fn new(num: i32, prime: i32) -> Result<Self, ECCError> {
        if num >= prime || num < 0 {
            let error_message = format(format_args!("Num {} not in the field range 0 to {}", num, prime - 1));
            return Err(ECCError::ValueError(error_message));
        } else {
            Ok(FieldElement {
                num,
                prime
            })
        }
    }

    /// Raise the power of the FieldElement to the given exponent
    pub fn pow(&self, exponent: i32) -> Result<Self, ECCError> {
        let n = modulus(exponent, self.prime - 1);
        let n_pos = n.try_into().unwrap();
        let num = modulus(self.num.pow(n_pos), self.prime);

        FieldElement::new(num, self.prime)
    }
}

/// Modulo workaround: Helper function to compute the modulo that wraps around
/// unlike the remainder that '%' computes
fn modulus(a: i32, b: i32) -> i32 {
    let modu = ((a % b) + b) % b;
    modu
}

