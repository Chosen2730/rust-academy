### Summary of Rust Data Types

- **Rust's Statically Typed Nature**:

  - Rust must know the type of all variables at compile time.
  - The compiler can often infer the type but requires type annotations when ambiguous.

- **Scalar Types**: Represents single values.

  - **Integer Types**:

    - Can be **signed** (allow negative values) or **unsigned** (only non-negative).
    - Common types: `i8`, `u8`, `i32`, `u32`, `isize`, `usize`.
    - Default integer type is `i32`.
    - Literals: decimal, hex, octal, binary, byte (`u8` only).
    - **Integer Overflow**:
      - In debug mode, Rust panics on overflow.
      - In release mode, Rust wraps around using two's complement.
      - Explicit overflow handling methods: `wrapping_*`, `checked_*`, `overflowing_*`, `saturating_*`.

  - **Floating-Point Types**:

    - Types: `f32` and `f64` (default).
    - Follow IEEE-754 standard.

  - **Boolean Type**:

    - `true` and `false`.
    - Type: `bool`.

  - **Character Type**:
    - Type: `char`.
    - Represents Unicode Scalar Values (e.g., emoji, accented letters).
    - Char literals use single quotes.

- **Compound Types**: Group multiple values into one type.

  - **Tuples**:

    - Can hold values of different types.
    - Fixed length once declared.
    - Can destructure or access elements using index (`tup.0`).
    - The empty tuple `()` represents the unit type.

  - **Arrays**:
    - Fixed size, homogeneous type.
    - Allocated on the stack.
    - Syntax: `[type; length]`.
    - Can initialize all elements with the same value: `[value; count]`.
    - Invalid access (out-of-bounds) causes panic at runtime.
