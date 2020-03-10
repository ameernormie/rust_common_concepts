fn main() {
    println!("\n\n****************Variables and Mutability*******************\n");
    variables();
    shadowed_variables();

    println!("\n\n****************Data Types*******************\n");
    change_same_var_type();
    floating_points();
    tuple_types();
    array_types();

    println!("\n\n****************Functions*******************\n");
    let sum = add_five(32);
    println!("I added five: {}", sum);

    println!("\n\n****************Control Flow*******************\n");
    condition_if();
    loop_that_returns_value();
    while_loop();
    for_loop();
    range_in_for_loop();
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

fn change_same_var_type() {
    let guess: u32 = "42".parse().expect("Not a numeber");
    println!("Explicitly changed type of variable: {}", guess);
}

fn floating_points() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
    println!(
        "f32 and f64 floating point values respectively: {} and {}",
        x, y
    );
}

fn tuple_types() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple is: {}, {}, {}", tup.0, tup.1, tup.2);
}

fn array_types() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!(
        "First and second element of array are {} and {}:",
        first, second
    );
}

fn add_five(a: i32) -> i32 {
    a + 5
}

fn condition_if() {
    let x = 5;

    if x < 7 {
        println!("Condition is true")
    } else {
        println!("Condition is false")
    }
}

fn loop_that_returns_value() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!(
        "Loop that returns a value and acts as an expression: {}",
        result
    );
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("Countdown: {}", number);
        number -= 1;
    }

    println!("LIFTOFF")
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn range_in_for_loop() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    println!("LIFTOFF");
}
