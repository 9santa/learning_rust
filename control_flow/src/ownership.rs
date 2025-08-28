fn main() {
    ownership();
}

fn heap_mem() {
    let s = String::from("hello"); // requests memory from the allocator. We need to pair exactly one allocate with exactly one free.

    // Can be mutated
    let mut s = String::from("hey"); // s is valid from this line
    s.push_str(", john!"); // push_str() appends a literal to a String
    println!("{s}");
}
// s goes out of scope, and so s is no longer valid. Rust caalls 'drop()'

/* In the case of a string literal, we know the contents at compile time, so the text is hardcoded
directly into the final executable. This is why string literals are fast and efficient. But these
properties only come from the string literal’s immutability. */

/* RAII Note: In C++, this pattern of deallocating resources at the end of an item’s lifetime is
sometimes called Resource Acquisition Is Initialization (RAII). The drop function in Rust will
be familiar to you if you’ve used RAII patterns. */

// Variables and Data interacting with Move
fn move_semantics() {
    let x = 5; // 5, on the stack
    let y = x; // 5, on the stack

    // Let's look at the String version
    let s1 = String::from("hello");
    let s2 = s1;

    // Length (size) is how much memory object is currently using, Capacity is the total memory object has received from the allocator
    // What happens above? The stack data is copied to s2 from s1, however the heap allocated actual String literal isn't copied
}
// When we go out of scope, drop() will be called twice for s1 and s2 to the same heap memory (double free error)
// To counter this, Rust considers s1 as no longer valid after 'let s2 = s2'
// s1 was moved into s2
// Shallow copy - we only copy the stack data (pointer, size, capacity)
// Deep copy - we copy everything, including the object on the heap itself

fn scope_assignment() {
    let mut s = String::from("hello");
    s = String::from("ahoy"); // we lose control of "hello", drop() is called

    println!("{s}, world!");
}

fn cloning() {
    // For deep copy use clone()
    let s1 = String::from("hello");
    let s2 = s1.clone(); // heap gets copied aswell

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x; // simple compile-time known types (sizes) are cloned automatically
    // They have what's called a 'Copy Trait'
    // all integers, booleans, floats, char, tuples (if they consist of those types)

    println!("x = {x}, y = {y}");
}

// Passing a variable to a function will move or copy
fn ownership() {
    let s = String::from("hello");
    takes_ownership(s); // s is no longer valid here

    let x = 5; // x is still valid
    makes_copy(x);

    // Return values cn also transfer ownership
    let s1 = gives_ownership(); // moves its return value into s1

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2); // s2 is moved into takes_and_gives_back, return value of that moves into s3

    // we can return multiple values using a tuple
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);

    println!("The Length of '{s2}' is: {len}.");
}

fn takes_ownership(some_string: String) {
    println!("move: {some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("made a copy of: {some_integer}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // some_string is returned and moved out to the calling function 
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moved out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}
