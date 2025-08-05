use recase::Recase;

struct OneWayCaseTestEntry {
    input: &'static str,
    camel: &'static str,
    pascal: &'static str,
    snake: &'static str,
    constant: &'static str,
    kebab: &'static str,
    capitalised: &'static str,
    sentence: &'static str,
    dot: &'static str,
}

fn test(entry: &OneWayCaseTestEntry) {
    assert_eq!(dbg!(entry.input.to_camel_case()), entry.camel);
    assert_eq!(dbg!(entry.input.to_pascal_case()), entry.pascal);
    assert_eq!(dbg!(entry.input.to_snake_case()), entry.snake);
    assert_eq!(dbg!(entry.input.to_constant_case()), entry.constant);
    assert_eq!(dbg!(entry.input.to_kebab_case()), entry.kebab);
    assert_eq!(dbg!(entry.input.to_capitalised_case()), entry.capitalised);
    assert_eq!(dbg!(entry.input.to_sentence_case()), entry.sentence);
    assert_eq!(dbg!(entry.input.to_dot_case()), entry.dot);
}

#[test]
fn single_char() {
    test(&OneWayCaseTestEntry {
        input: "aB",
        camel: "aB",
        pascal: "AB",
        snake: "a_b",
        constant: "A_B",
        kebab: "a-b",
        capitalised: "A B",
        sentence: "A b",
        dot: "a.b",
    });

    test(&OneWayCaseTestEntry {
        input: "a-b",
        camel: "aB",
        pascal: "AB",
        snake: "a_b",
        constant: "A_B",
        kebab: "a-b",
        capitalised: "A B",
        sentence: "A b",
        dot: "a.b",
    });

    test(&OneWayCaseTestEntry {
        input: "A",
        camel: "a",
        pascal: "A",
        snake: "a",
        constant: "A",
        kebab: "a",
        capitalised: "A",
        sentence: "A",
        dot: "a",
    });

    test(&OneWayCaseTestEntry {
        input: "X-Y-Z",
        camel: "xYZ",
        pascal: "XYZ",
        snake: "x_y_z",
        constant: "X_Y_Z",
        kebab: "x-y-z",
        capitalised: "X Y Z",
        sentence: "X y z",
        dot: "x.y.z",
    });
}
