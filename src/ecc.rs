/// Module containing helper primitives for Elliptic Curve Cryptography (ECC)
use std::fmt::{Display, format, Formatter, Result as FmtResult};
use std::ops::{Add, Sub, Mul, Div};
use num_bigint::{BigInt, BigUint};
use num_integer::Integer;
use num_traits::Pow;

/// A element belonging to a finite set of prime order
#[derive(Clone, Debug, PartialEq)]
pub struct FieldElement<'prime> {
    pub num: BigInt,
    pub prime: &'prime BigInt,
}

/// An enumeration of ECC errors
#[derive(Debug)]
pub enum ECCError {
    TypeError(String),
    ValueError(String),
}

/// Implements display trait for FieldElement
impl<'prime> Display for FieldElement<'prime> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "({}, {})", self.num, self.prime)
    }
}

/// Implements addition trait for a FieldElement
impl<'prime> Add for FieldElement<'prime> {
    type Output = Result<FieldElement<'prime>, ECCError>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot add two elements in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {
            let num = self.num.add(rhs.num).mod_floor(&self.prime);

            Ok(FieldElement {
                num: num,
                prime: &self.prime,
            })
        }
    }
}

/// Implements subtraction trait for FieldElement
impl<'prime> Sub for FieldElement<'prime> {
    type Output = Result<FieldElement<'prime>, ECCError>;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot subtract two elements in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {
            let num = self.num.sub(rhs.num).mod_floor(&self.prime);
            
            Ok(FieldElement {
                num,
                prime: &self.prime,
            })
        }
    }
}

/// Implement multiplication trait for FieldElement
impl<'prime> Mul for FieldElement<'prime> {
    type Output = Result<FieldElement<'prime>, ECCError>;
    
    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot multiply two number in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {
            let num = self.num.mul(rhs.num).mod_floor(&self.prime);
            Ok(FieldElement {
                num,
                prime: &self.prime,
            })
        }
    }
}

/// Implement division trait for FieldElement
impl<'prime> Div for FieldElement<'prime> {
    type Output = Result<FieldElement<'prime>, ECCError>;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            let error_message = "Cannot divide two number in different fields".to_owned();
            Err(ECCError::TypeError(error_message))
        } else {

            let exponent = rhs.num.modpow(&(self.prime - BigInt::from(2_u32)), &self.prime);
            let num = self.num.mul(exponent).mod_floor(&self.prime);

            Ok(FieldElement{
                num,
                prime: self.prime,
            })
        }
    }
}


/// Implement methods for FieldElement
impl<'prime>  FieldElement<'prime> {
    /// Create a new FieldElement with integer num and prime
    pub fn new(num: BigInt, prime: &'prime BigInt) -> Result<Self, ECCError> {
        if num >= *prime || num < BigInt::from(0_u32) {
            let error_message = format(format_args!("Num {} not in the field range 0 to {}", num, prime - 1));
            return Err(ECCError::ValueError(error_message));
        } else {
            
            Ok(FieldElement {
                num: num,
                prime: &prime, 
            })
        }
    }

    // / Raise the power of the FieldElement to the given exponent
    pub fn pow(&self, exponent: BigInt) -> Self {
        let fermat_exp = self.prime - BigInt::from(1_u32);
        let n = exponent.mod_floor(&fermat_exp);
        let num = self.num.modpow(&n, &self.prime);

        FieldElement {
            num,
            prime: self.prime,
        }
    }
}

/// Modulo workaround: Helper function to compute the modulo that wraps around
/// unlike the remainder that '%' computes
fn modulus(a: BigInt, b: BigInt) -> BigInt {
    let modu = ((a % &b) + &b) % &b;
    modu
}

/// Point on an elliptic curve
#[derive(Debug, PartialEq)]
pub struct Point<'prime> {
    pub x: Option<FieldElement<'prime>>,
    pub y: Option<FieldElement<'prime>>,
    pub a: FieldElement<'prime>,
    pub b: FieldElement<'prime>,
}

/// Implements methods for Point
impl<'prime> Point<'prime> {
    /// Creates a new Point object
    /// # Args:
    /// x (FieldElement) representing the x-coordinate
    /// y (FieldElement) representing the y-coordinate
    /// a (FieldElement) representing the x-coordinate coefficient 'a'
    /// b (FieldElement) representing the constant 'b'
    /// # Returns:
    /// Point object
    pub fn new(x: FieldElement<'prime>, y: FieldElement<'prime>, a: FieldElement<'prime>, b: FieldElement<'prime>) -> Result<Self, ECCError> {
        // 1. Create point
        let point = Point{ 
            x: Some(x), 
            y: Some(y), 
            a, 
            b, 
        };
        // 2. Check if it point at infinity and is valid, return point if true
        if is_inf(&point) || is_valid(&point) {
            return Ok(point)
        } else {
            let err_msg = format!("Point ({}) is not on the curve", point);
            return Err(ECCError::ValueError(err_msg));
        }
    }
}

/// Implements display for Point
impl<'prime> Display for Point<'prime> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f, 
            "Point_[x: {:?}, y: {:?}]_[a: {}, b: {}]", 
            self.x, 
            self.y,
            self.a,
            self.b
        )
    }
}

/// Checks if the Point object is at the point of infinity
fn is_inf(point: &Point) -> bool {
    let mut xp = point.x.as_ref();
    let mut yp = point.y.as_ref();

    match xp {
        Some(_x) => (),
        None => { xp = None},
    }

    match yp {
        Some(_y) => (),
        None => { yp = None},
    }

    if xp == None && yp == None {
        true
    } else {
        false
    }
}

/// Checks if the Point object lies on the elliptic curve
fn is_valid(point: &Point) -> bool {
    let xp = point.x.clone();
    let yp = point.y.clone();

    match xp {
        Some(x) => {
            match yp {
                Some(y) => {
                    let y_sq = y.pow(BigInt::from(2_u32));
                    let ax = (point.a.clone() * x.clone()).unwrap();
                    let b = point.b.clone();
                    let x_cube = x.clone().pow(BigInt::from(3_u32));

                    if y_sq == addition(&addition(&x_cube, &ax), &b) {
                        true
                    } else {
                        false
                    }

                },
                None => {true},
            }
        },
        None => {true},
    }
}

/// FieldElement addition that returns only a FieldElement
/// # Args:
/// fe_1: FieldElement 
/// fe_2: FieldElement
/// 
/// # Returns:
/// fe_3: FieldElement that is the sum of fe_1 and fe_2
fn addition<'p>(fe_1: &'p FieldElement<'p>, fe_2: &'p FieldElement<'p>) -> FieldElement<'p> {
    let sum = (fe_1.clone() + fe_2.clone()).unwrap();

    sum
}