# binexp

[![CI](https://github.com/Mirch/binexp/actions/workflows/ci.yml/badge.svg)](https://github.com/Mirch/binexp/actions/workflows/ci.yml)

**binexp** provides a `PowerOfTwo` struct that you can create from any valid power of two integer.  
You can use `PowerOfTwo` as a safe way to represent powers of two in your code.
    
# Examples
```
use std::convert::TryFrom;
use binexp::PowerOfTwo;

assert!(PowerOfTwo::try_from(1).is_ok());
assert!(PowerOfTwo::try_from(2).is_ok());
assert!(PowerOfTwo::try_from(3).is_err());

let four = PowerOfTwo::try_from(4).unwrap();
assert_eq!(four.to_string(), "2^2 (4)");

let eight: PowerOfTwo = 8i8.try_into().unwrap();
assert_eq!(eight.to_string(), "2^3 (8)");
```

# License
This project is licensed under the [MIT license](LICENSE).
