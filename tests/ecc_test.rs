use ecc::{FieldElement, Point};
use num_bigint::{BigInt};

/// test the creation of valid FieldElement
#[test]
fn test_valid_create_field_element() {
    // arrange
    let num_1 = BigInt::from(5_u32);
    let prime_1 = BigInt::from(19_u32);
    let num_2 = BigInt::from(6_u32);
    let prime_2 = BigInt::from(24109_u32);

    // act
    let valid_fe_1 = FieldElement::new(num_1, &prime_1).unwrap();
    let valid_fe_2 = FieldElement::new(num_2, &prime_2).unwrap();

    // assert
    assert_eq!(valid_fe_1.num, BigInt::from(5_u32));
    assert_eq!(valid_fe_2.prime, &BigInt::from(24109_u32));
   
}

/// test the equality of two field elements
#[test]
fn test_field_element_are_equal() {
    // arrange
    let num_1 = BigInt::from(5_u32);
    let num_2 = BigInt::from(5_u32);
    let prime_1 = BigInt::from(19_u32);

    // act
    let valid_fe_1 = FieldElement::new(num_1, &prime_1).unwrap();
    let valid_fe_2 = FieldElement::new(num_2, &prime_1).unwrap();

    // assert
    assert_eq!(valid_fe_1, valid_fe_2);
}

/// test the addition of two field elements
#[test]
fn test_add_two_field_element() {
    // arrange
    let num_1 = BigInt::from(5_u32);
    let num_2 = BigInt::from(5_u32);
    let prime_1 = BigInt::from(19_u32);

    // act
    let fe_1 = FieldElement::new(num_1, &prime_1).unwrap();
    let fe_2 = FieldElement::new(num_2, &prime_1).unwrap();
    let expected = FieldElement::new(BigInt::from(10_u32), &prime_1).unwrap();

    let sum = fe_1 + fe_2;

    match sum {
        Ok(actual) => {
            assert_eq!(actual, expected);
        },
        Err(_e) => (),
    }
}

/// test the subtraction of two field elements
#[test]
fn test_subtract_two_field_element() {
   // arrange
   let num_1 = BigInt::from(5_u32);
   let num_2 = BigInt::from(5_u32);
   let prime_1 = BigInt::from(19_u32);

   // act
   let fe_1 = FieldElement::new(num_1, &prime_1).unwrap();
   let fe_2 = FieldElement::new(num_2, &prime_1).unwrap();
   let expected = FieldElement::new(BigInt::from(0_u32), &prime_1).unwrap();

   let diff = fe_1 - fe_2;

    match diff {
        Ok(res) => {
            assert_eq!(res, expected);
        },
        Err(_e) => (),
    }

}

/// test the multiplication of two field elements
#[test]
fn test_multiply_two_field_elements() {
    // arrange
    let num_1 = BigInt::from(5_u32);
    let num_2 = BigInt::from(5_u32);
    let prime_1 = BigInt::from(19_u32);

    // act
    let fe_1 = FieldElement::new(num_1, &prime_1).unwrap();
    let fe_2 = FieldElement::new(num_2, &prime_1).unwrap();
    let expected = FieldElement::new(BigInt::from(6_u32), &prime_1).unwrap();

    let mul = fe_1 * fe_2;

    match mul {
        Ok(res) => {
            // assert
            assert_eq!(res, expected);
        },
        Err(_e) => (),
    }
}

/// test the exponentiation of a FieldElement to a given exponent
#[test]
fn test_raise_field_element_to_power() {
    // arrange
    let num_1 = BigInt::from(5_u32);
    let num_2 = BigInt::from(5_u32);
    let prime_1 = BigInt::from(19_u32);

    // act
    let fe_1 = FieldElement::new(num_1, &prime_1).unwrap();
    let fe_2 = FieldElement::new(num_2, &prime_1).unwrap();
    let expected = FieldElement::new(BigInt::from(9_u32), &prime_1).unwrap();
    let exp = BigInt::from(500_u32);

    let pow = fe_1.pow(exp);

    // assert 
    assert_eq!(pow, expected);

}

/// test the division of a FieldElement with another
#[test]
fn test_division_of_two_field_elements() {
    // arrange
    let num_1 = BigInt::from(5_u32);
    let num_2 = BigInt::from(5_u32);
    let prime_1 = BigInt::from(19_u32);

    // act
    let fe_1 = FieldElement::new(num_1, &prime_1).unwrap();
    let fe_2 = FieldElement::new(num_2, &prime_1).unwrap();
    let expected = FieldElement::new(BigInt::from(1_u32), &prime_1).unwrap();

    let div = fe_1 / fe_2;

    match div {
        Ok(res) => {
        assert_eq!(res, expected);
    },
        Err(_e) => (),
    }
}

/// test the creation of a valid Point object
#[test]
fn test_create_valid_point() {
    let x = BigInt::from(192_u32);
    let y = BigInt::from(105_u32);
    let a = BigInt::from(0_u32);
    let b = BigInt::from(7_u32);
    let prime = BigInt::from(223_u32);

    let fe_x = FieldElement::new(x, &prime).unwrap();
    let fe_y = FieldElement::new(y, &prime).unwrap();
    let fe_a = FieldElement::new(a, &prime).unwrap();
    let fe_b = FieldElement::new(b, &prime).unwrap();

    let point = Point::new(Some(fe_x), Some(fe_y), fe_a, fe_b);

    match point {
        Ok(pt) => {
            assert_eq!(*pt.a.prime, BigInt::from(223_u32));
        },
        Err(_) => (),
    }
}

/// test the creation of the point at infinity
#[test] 
fn test_create_infinity_point() {
    let x = BigInt::from(192_u32);
    let y = BigInt::from(105_u32);
    let a = BigInt::from(0_u32);
    let b = BigInt::from(7_u32);
    let prime = BigInt::from(223_u32);

    let fe_x: Option<FieldElement> = None;
    let fe_y: Option<FieldElement> = None;
    let fe_a = FieldElement::new(a, &prime).unwrap();
    let fe_b = FieldElement::new(b, &prime).unwrap();

    let point = Point::new(None, None, fe_a, fe_b);

    match point {
        Ok(pt) => {
            let pt_a = pt.a;
            assert_eq!(*pt_a.prime, BigInt::from(223_u32));
            assert_eq!(pt_a.num, BigInt::from(0_u32));
        },
        Err(_) => (),
    }
}

/// test point equality
#[test]
fn test_point_equality() {
    let x = BigInt::from(192_u32);
    let y = BigInt::from(105_u32);
    let a = BigInt::from(0_u32);
    let b = BigInt::from(7_u32);
    let prime = BigInt::from(223_u32);

    let fe_x = FieldElement::new(x, &prime).unwrap();
    let fe_y = FieldElement::new(y, &prime).unwrap();
    let fe_a = FieldElement::new(a, &prime).unwrap();
    let fe_b = FieldElement::new(b, &prime).unwrap();

    let point_a = Point::new(Some(fe_x.clone()), Some(fe_y.clone()), fe_a.clone(), fe_b.clone()).unwrap();
    let point_b = Point::new(Some(fe_x.clone()), Some(fe_y.clone()), fe_a.clone(), fe_b.clone()).unwrap();

    assert_eq!(point_a, point_b);
}