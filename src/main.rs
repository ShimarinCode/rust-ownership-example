fn main() {
    // Make a String type
    // This is not a string literal so we ask the OS to allocate memory
    // Also, we want to append a value later (in takes_ownership) but
    // it doesn't need to be mutable here because the reference is
    // `moved` into the function/into a new mutable variable
    let s = String::from("hello");
    takes_ownership(s); // The reference to "hello" is moved to the function
                        // Thus s is no longer valid

    let x = 5;
    makes_copy(x);      // value of x moves into the function
                        // x is still valid because the value is copied from
                        // the stack (because i32 is Copy)

    // Prefix s1 and s3 with _ to make the compiler be quiet about not using the variables
    let _s1 = gives_ownership();  // Return value is moved into s1
    let s2 = String::from("hello");
    let _s3 = takes_and_gives_back(s2);  // s2 is moved into the function, then moved into s3
                                        // but it is annoying to have to move ownership
                                        // back and forth if we want to use the value that
                                        // we gave the function in the first place.

    // Instead, we can use references!
    let s4 = String::from("Hello");
    let len = calculate_length(&s4);    // Refer to s4 without giving ownership
    println!("The length of '{}' is {}", s4, len);

    // Try passing a mutable String as a reference
    let mut s5 = String::from("Hello");
    change_string(&mut s5); // Need to pass the reference to mutable s5
    println!("{}", s5);
    // Note: Can only have ONE mutable reference to the same data in order to prevent
    // a data race condition

    // Here, we reference a mutable variable twice but we make immutable references
    // so it's ok. Think read-only reference without taking ownership
    let s6 = &s4;
    let s7 = &s4;
    // CANNOT mix with mutable reference!!
    // let s8 = &mut s4;    <-- will not compile
}

fn takes_ownership(mut some_string: String) {
    some_string.push_str(", world!");   // Appends string literal to String
    println!("{}", some_string);
}   // 'drop' is called on some_string because it is out of scope
    // so memory holding "hello" is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {    // Function that returns a String
    let some_string = String::from("hello");
    some_string // return some_string (Note: no semicolon to return value)
}

// Function that takes ownership of a string and then returns it back
fn takes_and_gives_back(a_string: String) -> String {
    a_string    // Return the taken String
}

fn calculate_length(s: &String) -> usize {  // s is a reference to a String
    s.len()
}

fn change_string(s: &mut String) {  // s is a reference to a mutable String
    s.push_str(", world!");
}
