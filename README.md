### Common Programming concepts in RUST

### What I learned

- [1 Variables and Mutability](#variables-and-mutability)
  - [1.1 Difference b/w variables and constants](#differences-between-variables-and-constants)
  - [1.2 Shadowing](#shadowing)
- [2 Data Types](#data-types)
  - [2.1 Scalar Types](#scalar-types)
    - [2.1.1 Integer Types](#integer-types)
    - [2.1.2 Floating-Point Types](#floating-point-types)
    - [2.1.3 Boolean Types](#boolean-types)
    - [2.1.4 The Character Type](#the-character-type)
  - [2.2 Compound Types](#compound-types)
    - [2.2.1 Tuple Type](#tuple-type)
    - [2.2.2 Array Type](#array-type)

## Variables and Mutability

Variables are by default immutable.

The following code error out with **cannot assign twice to immutable variable**

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

###### Making variables mutable

You can make them mutable by adding `mut` in front of the variable name.

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

#### Differences between variables and constants

Like immutable variables, constants are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

1.  You aren't allowed to use `mut` with constants. They are always immutable
2.  Constant is declared using `const` and must always be type annotated
3.  Constants can be declared in any scope, including the global scope
4.  Constants are set using only an expression. They cannot be set by a function's return value or any other computed value

```rust
#![allow(unused_variables)]
fn main() {
const MAX_POINTS: u32 = 100_000;
}
```

#### Shadowing

You can declare a new variable with the same name as a previous variable, and the new variable shadows the previous variable. First variable is `shadowed` by the second, which means that second variables value will be used.
`We can shadow a variable by using the same variable’s name and repeating the use of the let keyword`

Shadowing is different from marking a variable as `mut`, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword.

###### Changing type using shadowing

The other difference between `mut` and `shadowing` is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name.

```rust
#![allow(unused_variables)]
fn main() {
let spaces = "   ";
let spaces = spaces.len();
}
```

## Data Types

Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data. We’ll look at two data type subsets: `scalar` and `compound`. Rust is statically typed language and it must know all variable's types at compile time.
In cases when many types are possible, we must add a type annotation.

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

#### Scalar Types

A scalar type represents a single value. Rust has four primary scalar types.

1. Integers
2. Floating-point numbers
3. Booleans
4. Characters

##### Integer Types

An integer is a number without a fractional component. An example type is `u32` that indicates that the value it’s associated with should be an `unsigned` integer `(signed integer types start with i, instead of u)` that takes up 32 bits of space.
Integer types in Rust are:

| Length  | Signued | Unsigned |
| ------- | :-----: | -------: |
| 8-bit   |  `i8`   |     `u8` |
| 16-bit  |  `i16`  |    `u16` |
| 32-bit  |  `i32`  |    `u32` |
| 64-bit  |  `i64`  |    `u64` |
| 128-bit | `i128`  |   `u128` |
| arch    | `isize` |  `usize` |

`Signed` and `unsigned` refer to whether it’s possible for the number to be negative or positive. Whether the number needs to have a sign with it (signed) or whether it will only ever be positive.

**Each signed variant can store numbers from -`(2pow(n - 1))` to `2pow(n - 1) - 1` inclusive, where n is the number of bits that variant uses.**
So an `i8` can store numbers from `-(2pow(7))` to `2pow(7) - 1`, which equals `-128` to `127`.
The `isize` and `usize` types depend on the kind of computer your program is running on.

You can write integer literals in any of the forms including `Decimal`, `Hex`, `Octal`, `Binary` and `Byte(u8 only)`
**Default is `i32`**

##### Floating point types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points.Rust’s floating-point types are `f32` and `f64`, which are `32 bits` and `64 bits` in size, respectively. **default is f64**

##### Boolean types

Booleans are `one byte` in size. The `Boolean` type in Rust is specified using `bool`.

```rust
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

##### The Character Type

Rust’s `char` type is the language’s most primitive alphabetic type.
**char literals are specified with `single quotes`, as opposed to string literals, which use `double quotes`.**

```rust
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

**Rust’s char type is four bytes in size and represents a Unicode Scalar Value**

#### Compound Types

Compound types can group multiple values into one type. Rust has two primitive compound types: `tuples` and `arrays`.

##### Tuple Type

We create a tuple by writing a comma-separated list of values inside parentheses. We can destructure tuples.
`Tuples have a fixed length: once declared, they cannot grow or shrink in size.`

```rust
fn main() {
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {}", y);
}
```

```rust
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

##### Array Type

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.
**Arrays have a fixed length too**

```rust
#![allow(unused_variables)]
fn main() {
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
}
```

You would write an array’s type by using square brackets, and within the brackets include the type of each element, a semicolon, and then the number of elements in the array, like so: `let a: [i32; 5] = [1, 2, 3, 4, 5];`. Here, `i32` is the type of each element. After the semicolon, the number `5` indicates the array contains five elements.
If you want to create an array that contains the same value for each element, you can specify the initial value, followed by a semicolon, and then the length of the array in square brackets like `let a = [3; 5];`

###### Invalid Array element access

If you access an invalid index in element, it will end up in a `runtime error`.
