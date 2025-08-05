# recase

A fast and simple Rust library and CLI tool for converting strings between different case conventions.

## Features

- **Library**: Trait-based API for easy string case conversion
- **CLI Tool**: Command-line utility for batch processing and pipes
- **Multiple Cases**: Support for 8 different case conventions

## CLI Usage

Install it with:

```bash
cargo install recase
```

Then use it as such:

```bash
# Convert a single string
recase --snake "SomeVariableName"
# Output: some_variable_name

# Use with pipes
echo "hello_world" | recase --camel
# Output: helloWorld

# Process multiple lines
cat file.txt | recase --pascal

# Available options
recase --help
```

Help message:

```bash
Usage: recase [OPTIONS] [VALUE]

Arguments:
  [VALUE]  If no value is provided, reads from stdin (e.g. for pipes)

Options:
  -c, --camel        thisIsCamelCase
  -p, --pascal       ThisIsPascalCase
  -s, --snake        this_is_snake_case
  -u, --constant     THIS_IS_CONSTANT_CASE (or UPPER_CASE)
  -k, --kebab        this-is-kebab-case (or dashed-case)
  -a, --capitalised  This Is Capitalised Case
  -e, --sentence     This is sentence case
  -d, --dot          this.is.dot.case
  -h, --help         Print help
```

## Library Usage

Add it to your `Cargo.toml` or run:

```
cargo add recase
```

Then use it as such:

```rust
use recase::Recase;

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
use recase::Recase;

assert_eq!("XMLHttpRequest".to_snake_case(), "xml_http_request");
assert_eq!("linux    _Kernel".to_camel_case(), "linuxKernel");
assert_eq!("some--weird___input".to_pascal_case(), "SomeWeirdInput");
```
