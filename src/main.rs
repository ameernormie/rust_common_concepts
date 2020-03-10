fn main() {
    println!("Hello, world!");

    variables();
    shadowed_variables();
}

fn variables() {
    // Immutable variable x (by default)
    let x = 5;
    println!("Immutable variable x: {}", x);
    // make a variable mutable by adding mut before variable name
    let mut y = 10;
    println!("original value of y: {}:", y);
    y = 5;
    println!("value of y after mutation: {}", y);
}

fn shadowed_variables() {
    let x = 5;

    let x = x + 10;
    println!("value of shadowed x: {}", x);

    // We can also use shadowing to change type of variable
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Changed type after shadowing: {}", spaces)
}
