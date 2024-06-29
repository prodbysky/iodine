# Iodine
 - Stack based programming language

## Inspiration
 - Tsoding's [Porth](https://gitlab.com/tsoding/porth)

## Installation
```console
  cargo install iodine
```


## Usage
```console
  cargo build --release
  ./target/release/iodine -i <input.iod>
```

## Example
```
fdef square
    dup *
fend

1 1 +
420 0.5 *
420 dup -
420 69 > if "420 IS more than 69" print end
3 square 9 == if "3^2 is 9!" print end
```

## Features
 - Basic math operations (See [quirks](#quirks))
 - If statements
 - Functions
 - Comparisons between numbers
 - Signed / unsigned integers, floating point numbers
 - Strings (They don't serve any purpose for now)
 - Dumb comments (See [quirks](#quirks))
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

