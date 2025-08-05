fn push_lower(output: &mut String, ch: char) {
    for c in ch.to_lowercase() {
        output.push(c);
    }
}
fn push_upper(output: &mut String, ch: char) {
    for c in ch.to_uppercase() {
        output.push(c);
    }
}

#[derive(Debug)]
enum Type {
    Lowercase,
    CapitalOrNumber,
    Symbol,
    None,
}

impl From<char> for Type {
    fn from(value: char) -> Self {
        if !value.is_alphanumeric() {
            Self::Symbol
        } else if value.is_lowercase() {
            Self::Lowercase
        } else {
            Self::CapitalOrNumber
        }
    }
}

fn handle_func<F: Fn(&mut String, char), G: Fn(&mut String, char)>(
    output: &mut String,
    old: &Type,
    on_word_begin: F,
    on_normal: G,
    is_first_capitalised: bool,
    prev: char,
    next_is_lowercase: bool,
) -> Type {
    let new = Type::from(dbg!(prev));
    match dbg!((&old, &new)) {
        (Type::None, _) => {
            if is_first_capitalised {
                push_upper(output, prev)
            } else {
                push_lower(output, prev)
            }
        }
        (_, Type::None) => unreachable!(),
        (_, Type::Symbol) => (),
        (Type::Symbol, _) | (Type::Lowercase, Type::CapitalOrNumber) => on_word_begin(output, prev),
        (Type::CapitalOrNumber, Type::CapitalOrNumber) if next_is_lowercase => {
            on_word_begin(output, prev)
        }
        (Type::Lowercase, Type::Lowercase)
        | (Type::CapitalOrNumber, Type::CapitalOrNumber | Type::Lowercase) => {
            on_normal(output, prev)
        }
    }
    new
}

fn to_new_case<F: Fn(&mut String, char), G: Fn(&mut String, char)>(
    value: &str,
    is_first_capitalised: bool,
    on_word_begin: F,
    on_normal: G,
) -> String {
    let mut output = String::with_capacity(value.len());
    let mut old = Type::None;
    let mut current_char: Option<char> = None;
    for ch in value.chars() {
        if let Some(prev) = current_char {
            old = handle_func(
                &mut output,
                &old,
                &on_word_begin,
                &on_normal,
                is_first_capitalised,
                prev,
                ch.is_lowercase(),
            );
        }
        current_char = Some(ch);
    }
    if let Some(prev) = current_char {
        handle_func(
            &mut output,
            &old,
            &on_word_begin,
            &on_normal,
            is_first_capitalised,
            prev,
            false,
        );
    }

    output
}

pub trait Recase {
    fn to_camel_case(&self) -> String;
    fn to_pascal_case(&self) -> String;
    fn to_snake_case(&self) -> String;
    fn to_kebab_case(&self) -> String;
    fn to_sentence_case(&self) -> String;
    fn to_capitalised_case(&self) -> String;
    fn to_dot_case(&self) -> String;
    fn to_constant_case(&self) -> String;
}

impl Recase for str {
    fn to_camel_case(&self) -> String {
        to_new_case(self, false, push_upper, push_lower)
    }

    fn to_pascal_case(&self) -> String {
        to_new_case(self, true, push_upper, push_lower)
    }

    fn to_snake_case(&self) -> String {
        to_new_case(
            self,
            false,
            |output, ch| {
                output.push('_');
                push_lower(output, ch);
            },
            push_lower,
        )
    }

    fn to_kebab_case(&self) -> String {
        to_new_case(
            self,
            false,
            |output, ch| {
                output.push('-');
                push_lower(output, ch);
            },
            push_lower,
        )
    }

    fn to_sentence_case(&self) -> String {
        to_new_case(
            self,
            true,
            |output, ch| {
                output.push(' ');
                push_lower(output, ch);
            },
            push_lower,
        )
    }

    fn to_capitalised_case(&self) -> String {
        to_new_case(
            self,
            true,
            |output, ch| {
                output.push(' ');
                push_upper(output, ch);
            },
            push_lower,
        )
    }

    fn to_dot_case(&self) -> String {
        to_new_case(
            self,
            false,
            |output, ch| {
                output.push('.');
                push_lower(output, ch);
            },
            push_lower,
        )
    }

    fn to_constant_case(&self) -> String {
        to_new_case(
            self,
            true,
            |output, ch| {
                output.push('_');
                push_upper(output, ch);
            },
            push_upper,
        )
    }
}
