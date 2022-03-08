/// Module containing helper primitives for Elliptic Curve Cryptography (ECC)
use std::fmt::{Display, format, Formatter, Result as FmtResult};
use std::ops::Add;

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

/// Implements display for a FieldElement
impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.num, self.prime)
    }
}

/// Implements addition for a FieldElement
impl Add for FieldElement {
    type Output = Result<FieldElement, ECCError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot add two elements in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {
            let num = (self.num + rhs.num) % self.prime;
            Ok(FieldElement {
                num,
                prime: self.prime,
            })
        }
    }
}

impl FieldElement {
    /// Create a new FieldElement
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
}

