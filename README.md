# Big Number

A scientific notation number type for idle/incremental games in Rust.

[![Crates.io](https://img.shields.io/crates/v/big_number.svg)](https://crates.io/crates/big_number)

## Overview

`big_number` is a lightweight library that provides a simple implementation of big numbers using scientific notation, specifically designed for incremental/idle games where extremely large numbers are common.

The library represents numbers in the form `mantissa * 10^exponent` where the mantissa is normalized to be between 1.0 and 10.0 (except for zero).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
big_number = "0.1.2"
```

## Usage

```rust
use big_number::BigNumber;

// Create big numbers
let a = BigNumber::new(2.5, 10); // 2.5e10
let b = BigNumber::new(3.0, 5);  // 3.0e5

// Basic operations
let sum = a.add(b);        // Addition
let difference = a.sub(b); // Subtraction
let product = a.mul(b);    // Multiplication
let quotient = a.div(b);   // Division

// Convert to string
println!("a = {}", a.to_string());       // "2.500e10"
println!("a + b = {}", sum.to_string()); // Result of addition

// Helper methods
let zero = BigNumber::zero();
let one = BigNumber::one();
```

## Features

- Simple and lightweight implementation
- Basic arithmetic operations: addition, subtraction, multiplication, division
- Automatic normalization of scientific notation
- String representation for display
- Optimized for performance in idle/incremental game scenarios

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you shall be dual licensed as above, without any additional terms or conditions.