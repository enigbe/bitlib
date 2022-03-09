use ecc::{FieldElement};

/// test the creation of valid FieldElement
#[test]
fn test_valid_create_field_element() {
    let valid_fe_1 = FieldElement::new(5, 19);
    let invalid_fe_1 = FieldElement::new(20, 19);
    let invalid_fe_2 = FieldElement::new(-6, 21);

    match valid_fe_1 {
        Ok(res) => {
            assert_eq!(res.num, 5);
            assert_eq!(res.prime, 19);
        },
        Err(_e) => {
            println!("Invalid FieldElement");

        },
    }

    match invalid_fe_1 {
        Ok(_res) => {},
        Err(_e) => {
            println!("Invalid FieldElement");
        },
    }

    match invalid_fe_2 {
        Ok(_res) => {},
        Err(_e) => {
            println!("Invalid FieldElement");
        },
    }

}

/// test the equality of two field elements
#[test]
fn test_field_element_are_equal() {
    let field_element_one = FieldElement::new(5, 19).unwrap();
    let field_element_two = FieldElement::new(5, 19).unwrap();

    assert_eq!(field_element_one, field_element_two);
}

/// test the addition of two field elements
#[test]
fn test_add_two_field_element() {
    // set up
    let field_element_one = FieldElement::new(5, 19).unwrap();
    let field_element_two = FieldElement::new(5, 19).unwrap();

    let expected = FieldElement::new(10, 19).unwrap();

    // act
    let sum = field_element_one + field_element_two;

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
    // set up
    let fe_1 = FieldElement::new(5, 19).unwrap();
    let fe_2 = FieldElement::new(7, 19).unwrap();
    let expected = FieldElement::new(17, 19).unwrap();

    // act
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
    let fe_1 = FieldElement::new(12, 23).unwrap();
    let fe_2 = FieldElement::new(17, 23).unwrap();
    let expected = FieldElement::new(20, 23).unwrap();

    // act
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
    let exp = -5;
    let fe_1 = FieldElement::new(5, 7);
    let expected = FieldElement::new(5, 7).unwrap();

    match fe_1 {
        Ok(res) => {
            let power = res.pow(exp).unwrap();
            assert_eq!(power, expected);
        },
        Err(_e) => (),
    }

}

/// test the division of a FieldElement with another
#[test]
fn test_division_of_two_field_elements() {
    let fe_1 = FieldElement::new(5, 19).unwrap();
    let fe_2 = FieldElement::new(17, 19).unwrap();
    let expected = FieldElement::new(7, 19).unwrap();

    let div = fe_1 / fe_2;

    match div {
        Ok(res) => {
            assert_eq!(res, expected);
        },
        Err(_e) => (),
    }
}