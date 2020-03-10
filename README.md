### Common Programming concepts in RUST

### What I learned

- [1 Variables and Mutability](#variables-and-mutability)
  - [1.1 Difference b/w variables and contants](#differences-between-variables-and-constants)
  - [1.2 Shadowing](#shadowing)

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

1.  You aren't allowed to use `mut` with contants. They are always immutable
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
