// A function that TAKES ownership of a String


// A function that BORROWS a String (does not take ownership)


// A function that RETURNS ownership of a String


fn main() {
    // 1. Basic ownership move
    let s1 = String::from("hello");

    // Move: s1 is moved into s2 → s1 becomes invalid


    // 2. Clone to keep both
    let s3 = String::from("world");

    // 3. Functions & ownership
    let text = String::from("ownership demo");

    // This moves ownership into the function

    // 4. Borrowing instead of moving
    let message = String::from("borrowing example");

    // message is still valid because we only borrowed it


    // 5. Function returning ownership


    // 6. Copy types do not move
}
