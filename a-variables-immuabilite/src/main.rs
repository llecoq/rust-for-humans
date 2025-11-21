fn main() {
    // By default, variables in Rust are immutable
    let x = 5;
    println!("x is: {}", x);

    // x = 6; 

    // To make a variable mutable, you explicitly add `mut`
    let mut y = 10;
    println!("y starts as: {}", y);

    // Now this is allowed because `y` is mutable
    y = 20;
    println!("y is now: {}", y);

    // Shadowing: re-declaring a variable with the same name
    // This creates a NEW variable, it does not mutate the old one
    let z = 3;
    let z = z + 1; // shadowing
    let z = "now a string"; // shadowing can even change the type
    println!("z is: {}", z);
}
