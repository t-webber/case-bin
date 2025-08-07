# caseify

A fast and simple Rust library and CLI tool for converting strings between different case conventions.

> This is a new crate, open to comments and improvements!

## Features

- **Library**: Trait-based API for easy string case conversion
- **CLI Tool**: Command-line utility for batch processing and pipes
- **Multiple Cases**: Support for 8 different case conventions

## CLI Usage

Install it with:

```bash
cargo install caseify
```

Then use it as such:

```bash
# Convert a single string
caseify --snake "SomeVariableName"
# Output: some_variable_name

# Use with pipes
echo "some text
some_snake_case
AndPascalCase" | caseify --camel
# Output:
# someText
# someSnakeCase
# AndPascalCase

# Process multiple lines
cat file.txt | caseify --pascal

# Available options
caseify --help
```

Help message:

```bash
Usage: caseify [OPTIONS] [VALUE]

Arguments:
  [VALUE]  If no value is provided, reads from stdin (e.g. for pipes)

Options:
  -c, --camel        `thisIsCamelCase`
  -a, --capitalised  `This Is Capitalised Case`
  -o, --constant     `THIS_IS_CONSTANT_CASE` (or `UPPER_CASE`)
  -d, --dot          `this.is.dot.case`
  -k, --kebab        `this-is-kebab-case` (or `dashed-case`)
  -p, --pascal       `ThisIsPascalCase`
  -e, --sentence     `This is sentence case`
  -s, --snake        `this_is_snake_case`
  -h, --help         Print help
```

## Library Usage

Add it to your `Cargo.toml` or run:

```bash
cargo add caseify
```

Then use it as such:

```rust
use caseify::Caseify;

let input = "some_snake_case_string";

assert_eq!(input.to_camel_case(), "someSnakeCaseString");
assert_eq!(input.to_pascal_case(), "SomeSnakeCaseString");
assert_eq!(input.to_kebab_case(), "some-snake-case-string");
assert_eq!(input.to_constant_case(), "SOME_SNAKE_CASE_STRING");
assert_eq!(input.to_sentence_case(), "Some snake case string");
assert_eq!(input.to_capitalised_case(), "Some Snake Case String");
assert_eq!(input.to_dot_case(), "some.snake.case.string");
```

The library intelligently handles various input formats:

```rust
use caseify::Caseify;

assert_eq!("XMLHttpRequest".to_snake_case(), "xml_http_request");
assert_eq!("linux    _Kernel".to_camel_case(), "linuxKernel");
assert_eq!("some--weird___input".to_pascal_case(), "SomeWeirdInput");
```
