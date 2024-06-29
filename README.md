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
fdef square : number
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
 - Function return types
 - Functions
 - Comparisons between numbers
 - Signed / unsigned integers, floating point numbers
 - Strings (They don't serve any purpose for now)
 - Comments
 - Simple stack operations (drop, dup)

## Quirks
 - Any math operation on any integer type result is a f64:
 ```
 # Output: 4.0 #
 2 2 +
 ```

## TODO
 - Including files
 - Don't coerce all numbers to f64 after any math operation
 - More stack operations
 - Reading from files
 - Arrays
 - Escaped strings

