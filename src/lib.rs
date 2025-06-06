/*!
# TS2IR - TypeScript to IR Converter

TS2IR is a Rust library for converting TypeScript code into IR (Intermediate Representation).

The library provides a set of traits and implementations to handle the conversion of
TypeScript language constructs into their IR counterparts.

## Architecture

The library is organized around the main trait:

* `IRFromTypeScript`: Converts TypeScript code into an IR structure

## Basic Example

```rust
use ir::IR;
use ts2ir::IRFromTypeScript;

// Parse TypeScript code
let input = r#"class Cat {
    energy: number;
    breath: number;
}"#;

// Convert TypeScript to IR
let ir = IR::from_typescript(input);
```
*/

pub mod conversions;
pub mod traits;

// Re-export main traits for public API
pub use traits::IRFromTypeScript;
