use recase::Recase;

#[test]
fn repetitions() {
    assert!("Another_____Baker".to_snake_case() == "another_baker");
    assert!("Another------Baker".to_snake_case() == "another_baker");
    assert!("Another  Baker".to_snake_case() == "another_baker");
    assert!("Another    Baker".to_snake_case() == "another_baker");
    assert!("Another........Baker".to_snake_case() == "another_baker");
    assert!("Another_____Baker".to_camel_case() == "anotherBaker");
    assert!("Another------Baker".to_camel_case() == "anotherBaker");
    assert!("Another  Baker".to_camel_case() == "anotherBaker");
    assert!("Another    Baker".to_camel_case() == "anotherBaker");
    assert!("Another........Baker".to_camel_case() == "anotherBaker");
    assert!("Another_____Baker".to_kebab_case() == "another-baker");
    assert!("Another------Baker".to_kebab_case() == "another-baker");
    assert!("Another  Baker".to_kebab_case() == "another-baker");
    assert!("Another    Baker".to_kebab_case() == "another-baker");
    assert!("Another........Baker".to_kebab_case() == "another-baker");
}
