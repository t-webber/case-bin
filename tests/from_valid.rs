use recase::Recase;

struct CaseTestEntry {
    camel: &'static str,
    pascal: &'static str,
    snake: &'static str,
    constant: &'static str,
    kebab: &'static str,
    capitalised: &'static str,
    sentence: &'static str,
    dot: &'static str,
}

impl CaseTestEntry {
    fn test_output<F: Fn(&str) -> String>(&self, convert: F, expected: &str) {
        assert_eq!(dbg!(convert(self.camel)), expected);
        assert_eq!(dbg!(convert(self.pascal)), expected);
        assert_eq!(convert(self.snake), expected);
        assert_eq!(convert(self.constant), expected);
        assert_eq!(convert(self.kebab), expected);
        assert_eq!(convert(self.capitalised), expected);
        assert_eq!(convert(self.sentence), expected);
        assert_eq!(convert(self.dot), expected);
    }

    fn test(&self) {
        self.test_output(Recase::to_camel_case, self.camel);
        self.test_output(Recase::to_pascal_case, self.pascal);
        self.test_output(Recase::to_snake_case, self.snake);
        self.test_output(Recase::to_constant_case, self.constant);
        self.test_output(Recase::to_kebab_case, self.kebab);
        self.test_output(Recase::to_capitalised_case, self.capitalised);
        self.test_output(Recase::to_sentence_case, self.sentence);
        self.test_output(Recase::to_dot_case, self.dot);
    }
}

#[test]
fn from_valid() {
    CaseTestEntry {
        camel: "someCaseExample",
        pascal: "SomeCaseExample",
        snake: "some_case_example",
        constant: "SOME_CASE_EXAMPLE",
        kebab: "some-case-example",
        capitalised: "Some Case Example",
        sentence: "Some case example",
        dot: "some.case.example",
    }
    .test();

    CaseTestEntry {
        camel: "anotherExample",
        pascal: "AnotherExample",
        snake: "another_example",
        constant: "ANOTHER_EXAMPLE",
        kebab: "another-example",
        capitalised: "Another Example",
        sentence: "Another example",
        dot: "another.example",
    }
    .test();

    CaseTestEntry {
        camel: "yetAnotherExample",
        pascal: "YetAnotherExample",
        snake: "yet_another_example",
        constant: "YET_ANOTHER_EXAMPLE",
        kebab: "yet-another-example",
        capitalised: "Yet Another Example",
        sentence: "Yet another example",
        dot: "yet.another.example",
    }
    .test();

    CaseTestEntry {
        camel: "exampleWithNumbers123",
        pascal: "ExampleWithNumbers123",
        snake: "example_with_numbers_123",
        constant: "EXAMPLE_WITH_NUMBERS_123",
        kebab: "example-with-numbers-123",
        capitalised: "Example With Numbers 123",
        sentence: "Example with numbers 123",
        dot: "example.with.numbers.123",
    }
    .test();
}
