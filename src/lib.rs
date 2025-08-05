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

fn to_new_case<F: Fn(&mut String, char), G: Fn(&mut String, char)>(
    value: &str,
    is_first_capitalised: bool,
    on_word_begin: F,
    on_normal: G,
) -> String {
    let mut output = String::with_capacity(value.len());
    let mut is_previous_char_non_alphabetic = is_first_capitalised;
    let mut is_previous_char_lowercase = false;
    let mut first = is_first_capitalised;
    for ch in value.chars() {
        println!(
            "{is_previous_char_non_alphabetic}\t{is_previous_char_lowercase}\t{first}\t{ch}: {output}"
        );
        if !ch.is_alphanumeric() {
            is_previous_char_non_alphabetic = true;
            continue;
        }

        let is_current_upper = matches!(ch, '0'..='9' | 'A'..='Z');

        if first {
            push_upper(&mut output, ch);
            first = false;
        } else if is_previous_char_non_alphabetic || is_current_upper && is_previous_char_lowercase
        {
            on_word_begin(&mut output, ch);
        } else {
            on_normal(&mut output, ch);
        }

        is_previous_char_lowercase = !is_current_upper;
        is_previous_char_non_alphabetic = false;
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
            false,
            |output, ch| {
                output.push('_');
                push_upper(output, ch);
            },
            push_upper,
        )
    }
}
