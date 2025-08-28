fn main() {
    let reference_to_nothing = dangle();
}

// A reference is like a pointer in that it's an address we can follow to access the data
// While the data is owned by some other variable
// However, unlike a pointer, a reference is guaranteed to point to a valid value for the life of that reference

fn references() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // passing a reference

    println!("The length of '{s1}' is: {len}.");

    let mut s = String::from("hello");
    change(&mut s); // only 1 mutable reference can be at a time, eliminates Data-Races
    println!("{s}"); // hello, world!
}

fn calculate_length(s: &String) -> usize {
    // action of creating a reference is called borrowing
    s.len()

    // references are immutable by default aswell
    // s.push_str(", world!"); // compile-error
}

fn change(a: &mut String) {
    // mutable reference
    a.push_str(", world!");
}
// We also cannot have a mutable reference while we have an immutable one to the same value
// However, multiple immutable references are allowed, since they just read data
// we can create a new scope, allowing for multiple mutable references
fn multiple() {
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
    } // r1 goes out of scope, so we can make a new reference
    let r2 = &mut s;

    let r3 = &s;
    let r4 = &s;
    println!("{r3} and {r4}");

    let r5 = &mut s; // this works, because r3 and r4 are no longer used later
    println!("{r5}");
}

// Dangling references (a pointer that references a location in memory that may have been given to someone else —
// by freeing some memory while preserving a pointer to that memory) are checked at compile-time

fn dangle() -> &String {
    let s = String::from("hello");

    &s
} // s is dropped, so reference would be invalid
