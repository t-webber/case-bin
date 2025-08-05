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

fn to_new_case<F: Fn(&mut String, char) -> ()>(
    value: &str,
    is_first_capitalised: bool,
    one_word_begin: F,
) -> String {
    let mut output = String::with_capacity(value.len());
    let mut is_previous_char_non_alphabetic = is_first_capitalised;
    let mut is_previous_char_lowercase = false;
    for ch in value.chars() {
        if !ch.is_alphabetic() {
            is_previous_char_non_alphabetic = true;
            continue;
        }

        let is_current_upper = ch.is_uppercase();

        if is_previous_char_non_alphabetic || is_current_upper && is_previous_char_lowercase {
            one_word_begin(&mut output, ch);
        } else {
            push_lower(&mut output, ch);
        }

        is_previous_char_lowercase = !is_current_upper;
        is_previous_char_non_alphabetic = false;
    }
    output
}

pub trait Case {
    fn to_camel_case(&self) -> String;
    fn to_pascal_case(&self) -> String;
    fn to_snake_case(&self) -> String;
    fn to_kebab_case(&self) -> String;
    fn to_sentence_case(&self) -> String;
    fn to_capitalised_case(&self) -> String;
    fn to_dot_case(&self) -> String;
}

impl Case for str {
    fn to_camel_case(&self) -> String {
        to_new_case(self, false, push_upper)
    }

    fn to_pascal_case(&self) -> String {
        to_new_case(self, true, push_upper)
    }

    fn to_snake_case(&self) -> String {
        to_new_case(self, false, |output, ch| {
            output.push('_');
            push_lower(output, ch);
        })
    }

    fn to_kebab_case(&self) -> String {
        to_new_case(self, false, |output, ch| {
            output.push('-');
            push_lower(output, ch);
        })
    }

    fn to_sentence_case(&self) -> String {
        to_new_case(self, false, |output, ch| {
            output.push(' ');
            push_lower(output, ch);
        })
    }

    fn to_capitalised_case(&self) -> String {
        to_new_case(self, false, |output, ch| {
            output.push(' ');
            push_upper(output, ch);
        })
    }

    fn to_dot_case(&self) -> String {
        to_new_case(self, false, |output, ch| {
            output.push('.');
            push_lower(output, ch);
        })
    }
}
