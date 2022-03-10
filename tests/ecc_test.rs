use ecc::{FieldElement, ECCError};
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

// /// test the exponentiation of a FieldElement to a given exponent
// #[test]
// fn test_raise_field_element_to_power() {
//     // arrange
//     let exp = BigInt::from(-5_i32);
//     let fe_1 = FieldElement::new(5, 7).unwrap();
//     let expected = FieldElement::new(5, 7).unwrap();

//     // act
//     let pow = fe_1 ^ exp;

// }

// /// test the division of a FieldElement with another
// #[test]
// fn test_division_of_two_field_elements() {
//     let fe_1 = FieldElement::new(5, 19).unwrap();
//     let fe_2 = FieldElement::new(17, 19).unwrap();
//     let expected = FieldElement::new(7, 19).unwrap();

//     let div = fe_1 / fe_2;

//     match div {
//         Ok(res) => {
//             assert_eq!(res, expected);
//         },
//         Err(_e) => (),
//     }
// }