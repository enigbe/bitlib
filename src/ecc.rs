/// Module containing helper primitives for Elliptic Curve Cryptography (ECC)
use std::fmt::{Display, Formatter, Result as FmtResult};

/// FieldElement: A element belonging to a finite set
#[derive(Debug, PartialEq)]
pub struct FieldElement {
    pub num: i32,
    pub prime: i32,
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.num, self.prime)
    }
}

