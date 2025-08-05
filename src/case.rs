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

fn to_pascal_camel(value: &str, is_first_capitalised: bool) -> String {
    let mut output = String::with_capacity(value.len());
    let mut is_previous_char_non_alphabetic = is_first_capitalised;
    let mut is_previous_char_lowercase = false;
    for ch in value.chars() {
        if !ch.is_alphabetic() {
            is_previous_char_non_alphabetic = true;
            continue;
        }

        let is_current_upper = ch.is_uppercase();

        if is_previous_char_non_alphabetic {
            push_upper(&mut output, ch);
        } else if is_current_upper && is_previous_char_lowercase {
            output.push(ch);
        } else {
            push_lower(&mut output, ch);
        }

        is_previous_char_lowercase = !is_current_upper;
        is_previous_char_non_alphabetic = false;
    }
    output
}

pub fn to_camel(value: &str) -> String {
    to_pascal_camel(value, false)
}

pub fn to_pascal(value: &str) -> String {
    to_pascal_camel(value, true)
}
