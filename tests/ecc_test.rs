use ecc::{FieldElement};

/// test create valid FieldElement
#[test]
fn test_valid_create_field_element() {
    let valid_field_element = FieldElement::new(5, 19);
    let invalid_field_element = FieldElement::new(20, 19);

    match valid_field_element {
        Ok(result) => {
            assert_eq!(result.num, 5);
            assert_eq!(result.prime, 19);
            assert!(result.num < result.prime - 1);
        },
        Err(_e) => (),
    }

    match invalid_field_element {
        Ok(_) => (),
        Err(e) => {
            println!("Invalid FieldElement {:?}", e);
        },
    }
}

/// test the equality of two field elements
#[test]
fn test_field_element_are_equal() {
    let field_element_one = FieldElement::new(5, 19);
    let field_element_two = FieldElement::new(5, 19);

    match field_element_one {
        Ok(result) => {
            match field_element_two {
                Ok(result_two) => {
                    assert_eq!(result, result_two);
                },
                Err(_e) => ()
            }
        },
        Err(_e) => ()
    }
}

/// test the addition of two field elements
#[test]
fn test_add_two_field_element() {
    // set up
    let field_element_one = FieldElement::new(5, 19).expect("Can't create FieldElement");
    let field_element_two = FieldElement::new(5, 19).expect("Can't create FieldElement");

    let expected = FieldElement::new(10, 19);

    // act
    let sum = field_element_one + field_element_two;

    match sum {
        Ok(actual) => {
            match expected {
                Ok(expect) => {
                    // assert
                    assert_eq!(actual, expect);
                },
                Err(_e) => (),
            }
        },
        Err(_e) => (),
    }
}

