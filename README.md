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
- [3 Functions](#functions)
  - [3.1 Function bodies have Statements and Expressions](#function-bodies-have-statements-and-expressions)
    - [3.1.1 Statements](#statements)
    - [3.1.2 Expressions](#expressions)
  - [3.2 Functions with return values](#functions-with-return-values)
- [4 Control Flow](#control-flow)
  - [4.1 `If` Expressions](#if-expressions)
    - [4.1.1 Using if in a `let` statement](#using-if-in-a-let-statement)
  - [4.2 Repetition with Loops](#repetition-with-loops)
    - [4.2.1 loop](#loop)
    - [4.2.2 while](#while)
    - [4.2.3 for](#for)

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

> `Integer Overflow `
> Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, integer overflow will occur. Rust has some interesting rules involving this behavior. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error; we’ll discuss panics in more depth in the “Unrecoverable Errors with panic!” section in Chapter 9.

When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, 256 becomes 0, 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error. If you want to wrap explicitly, you can use the standard library type Wrapping.

You can write integer literals in any of the forms including `Decimal`, `Hex`, `Octal`, `Binary` and `Byte(u8 only)`
**Default is `i32`**

| Number Literals |    Example    |
| --------------- | :-----------: |
| Decimal         |   `98_222`    |
| Hex             |    `0xff`     |
| Octal           |    `0o77`     |
| Binary          | `0b1111_0000` |
| Byte (u8 only)  |    `b'A'`     |
|                 |               |
|                 |               |

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

## Functions

In function signatures, you must declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what you mean.

#### Function bodies have Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression.**Rust is an expression-based language**, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what `statements` and `expressions` are and how their differences affect the bodies of functions.

##### Statements

`Statements are instructions that perform some action and do not return a value.`
Creating a variable and assigning a value to it with the let keyword is a statement. `let y = 6;` is a statement.
Function definitions are also statements; the entire preceding example is a statement in itself.
Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:

```rust
let x = (let y = 6);
```

##### Expressions

`Expressions evaluate to a resulting value.`
Expressions evaluate to something and make up most of the rest of the code that you’ll write in Rust. Consider a simple math operation, such as `5 + 6`, which is an expression that evaluates to the value `11`. **Expressions can be part of statements:** . The `6` in the statement `let y = 6;` is an expression that evaluates to the value `6`. `Calling a function is an expression`. `Calling a macro is an expression`. `The block that we use to create new scopes, {}, is an expression`, for example:

```rust
fn main() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

This expression:

```
{
    let x = 3;
    x + 1
}
```

`Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, which will then not return a value.`

#### Functions with return values

Functions can return values to the code that calls them. We don’t name return values, but we do declare their type after an `arrow (->)`.
You can return early from a function by using the `return` keyword and specifying a value, but most functions return the last expression implicitly.

## Control Flow

#### If expressions

```rust
fn main() {
    let number = 3;

    if number < 2 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}
```

`Rust will not automatically try to convert non-Boolean types to a Boolean. You must be explicit and always provide if with a Boolean as its condition.`

###### else-if

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

##### Using if in a let Statement

Because `if` is an expression, we can use it on the right side of a `let` statement.

```rust
fn main() {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}
```

#### Repetition with Loops

It’s often useful to execute a block of code more than once. For this task, Rust provides several loops.
Rust has three kinds of loops:

1. loop
2. while
3. for

##### loop

The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
Rust provides another, more reliable way to break out of a loop. You can place the break keyword within the loop to tell the program when to stop executing the loop.

###### Returning values from loop

One of the uses of a loop is to retry an operation you know might fail, such as checking whether a thread has completed its job. However, you might need to pass the result of that operation to the rest of your code.

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

##### while

While the condition is true, the loop runs. When the condition ceases to be true, the program calls `break`, stopping the loop.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}
```

##### for

You can use a for loop and execute some code for each item in a collection.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```
