// A function that BORROWS immutably
fn print_message(msg: &String) {
    println!("Message: {}", msg);
}

// A function that BORROWS mutably
fn add_suffix(msg: &mut String) {
    msg.push_str(" (modified)");
}

fn main() {
    // --------------------------------------
    // 1. Immutable borrowing
    // --------------------------------------
    let text = String::from("hello world");

    print_message(&text); // immutable borrow
    print_message(&text); // still fine

    // text is still valid because no ownership was taken
    println!("After immutable borrows: {}", text);


    // --------------------------------------
    // 2. Mutable borrowing
    // --------------------------------------
    let mut name = String::from("Alice");

    add_suffix(&mut name); // mutable borrow
    println!("After mutable borrow: {}", name);


    // --------------------------------------
    // 3. Rust preventing a data race
    // --------------------------------------
    let mut data = String::from("danger");

    // This is OK:
    let r1 = &data;   // first immutable borrow
    let r2 = &data;   // second immutable borrow

    println!("r1 = {}, r2 = {}", r1, r2);

    // But this would NOT compile:
    // let r3 = &mut data; 
    // ERROR:
    // cannot borrow `data` as mutable because it is 
    // also borrowed as immutable


    // --------------------------------------
    // 4. Rust also forbids multiple mutable borrows
    // --------------------------------------
    let mut value = String::from("abc");

    let m1 = &mut value;

    // This would NOT compile:
    // let m2 = &mut value;
    // ERROR:
    // cannot borrow `value` as mutable more than once at a time

    m1.push_str("!!!");
    println!("After single mutable borrow: {}", m1);


    // --------------------------------------
    // 5. How to fix borrowing conflicts
    // (limit the scope of the borrow)
    // --------------------------------------
    let mut message = String::from("fix me");

    {
        let tmp = &mut message; // mutable borrow inside a smaller scope
        tmp.push_str(" now");
    } // tmp ends here → mutable borrow ends

    // Now mutable borrow is over, so we can borrow again
    add_suffix(&mut message);
    println!("After fixing scope: {}", message);
}
