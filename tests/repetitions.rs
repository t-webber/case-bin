use caseify::Caseify;

#[test]
fn repetitions() {
    assert!("Another_____Word".to_snake_case() == "another_word");
    assert!("Another------Word".to_snake_case() == "another_word");
    assert!("Another  Word".to_snake_case() == "another_word");
    assert!("Another    Word".to_snake_case() == "another_word");
    assert!("Another........Word".to_snake_case() == "another_word");
    assert!("Another_____Word".to_camel_case() == "anotherWord");
    assert!("Another------Word".to_camel_case() == "anotherWord");
    assert!("Another  Word".to_camel_case() == "anotherWord");
    assert!("Another    Word".to_camel_case() == "anotherWord");
    assert!("Another........Word".to_camel_case() == "anotherWord");
    assert!("Another_____Word".to_kebab_case() == "another-word");
    assert!("Another------Word".to_kebab_case() == "another-word");
    assert!("Another  Word".to_kebab_case() == "another-word");
    assert!("Another    Word".to_kebab_case() == "another-word");
    assert!("Another........Word".to_kebab_case() == "another-word");

    // Readme tests
    assert_eq!("XMLHttpRequest".to_snake_case(), "xml_http_request");
    assert_eq!("linux    _Kernel".to_camel_case(), "linuxKernel");
    assert_eq!("some--weird___input".to_pascal_case(), "SomeWeirdInput");
}
