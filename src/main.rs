mod ecc;

fn main() {
    let field_element = ecc::FieldElement {
        num: 12,
        prime: 17,
    };

    let field_element_two = ecc::FieldElement {
        num: 12,
        prime: 19,
    };

    println!("{:?}", field_element);
    assert_eq!(field_element, field_element_two);
}
