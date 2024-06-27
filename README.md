# Iodine
 - Stack based programming language

## Inspiration
 - Tsoding's [Porth](https://gitlab.com/tsoding/porth)

## Usage
```console
  cargo build --release
  ./target/release/iodine -i <input.iod>
```

## Example
```
1 1 +
420 0.5 *
420 dup -
```

## Features
 - Basic math operations (See [quirks](##quirks))
 - Signed / unsigned integers, floating point numbers
 - Strings (They don't serve any purpose for now)
 - Dumb comments (See [quirks](##quirks))
 - Simple stack operations (drop, dup)

## Quirks
 - Comments are just strings that are immeadiately dropped
 ```
 "This is a comment" drop
 ```
 - Any math operation on any integer type result is a f64:
 ```
 "Output: 4.0" drop
 2 2 +
 ```

