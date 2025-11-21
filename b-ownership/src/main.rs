// A function that TAKES ownership of a String
fn take_ownership(s: String) {
    // `s` is now owned by this function
    println!("Inside take_ownership: {}", s);

    // When the function ends, `s` is dropped automatically
}

// A function that BORROWS a String (does not take ownership)
fn borrow_string(s: &String) {
    println!("Inside borrow_string: {}", s);

    // `s` is just a reference, so nothing is dropped here
}

// A function that RETURNS ownership of a String
fn give_ownership() -> String {
    let msg = String::from("I'm coming from give_ownership!");
    msg // ownership is returned to the caller
}

fn main() {
    // --------------------------------------
    // 1. Basic ownership move
    // --------------------------------------
    let s1 = String::from("hello");

    // Move: s1 is moved into s2 → s1 becomes invalid
    let s2 = s1;

    println!("s2 after move: {}", s2);

    // This would NOT compile:
    // println!("{}", s1);


    // --------------------------------------
    // 2. Clone to keep both
    // --------------------------------------
    let s3 = String::from("world");
    let s4 = s3.clone(); // deep copy
    println!("s3: {}, s4: {}", s3, s4);


    // --------------------------------------
    // 3. Functions & ownership
    // --------------------------------------
    let text = String::from("ownership demo");

    // This moves ownership into the function
    take_ownership(text);

    // This would NOT compile:
    // println!("{}", text);


    // --------------------------------------
    // 4. Borrowing instead of moving
    // --------------------------------------
    let message = String::from("borrowing example");

    borrow_string(&message);
    // message is still valid because we only borrowed it
    println!("After borrowing: {}", message);


    // --------------------------------------
    // 5. Function returning ownership
    // --------------------------------------
    let returned = give_ownership();
    println!("Returned value: {}", returned);


    // --------------------------------------
    // 6. Copy types do not move
    // --------------------------------------
    let a = 10;   // i32 implements Copy
    let b = a;    // a is still valid
    println!("a = {}, b = {}", a, b);
}
