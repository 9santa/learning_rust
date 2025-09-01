pub fn main() {
    println!("This is slices.rs file");
    // Variable word is completely out of sync with string s
    // let mut s = String::from("Hello World");
    //
    // let word = first_word(&s);
    //
    // s.clear(); // empties the string, making it "", word still has the value 5, but its invalid since s no longer exists

    let s = String::from("Hello World");

    let word = first_word_slice(&s);

    // println!("{word}");

    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word_slice(&my_string[0..6]);
    let word = first_word_slice(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word = first_word_slice(&my_string);
    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word = first_word_slice(&my_string_literal[0..6]);
    let word = first_word_slice(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);

    array_slice();
}

//1 THE SLICE TYPE
// Slices let you reference a continious seq of elements in a collection
// A slice is a kind of reference, so it doesn't have ownership

// Find index of the end of the 1st word in a string
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // converting to an array of bytes

    // iter() is a method that returns each element in a collection
    // enumerate() wraps that and returns each element as a tuple (index, &value)
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn string_slices() {
    let s = String::from("hello world");

    // References to a portion of the string
    let hello = &s[0..5];
    let world = &s[6..11];

    let slice = &s[..2];
    let slice = &s[3..];
    let slice = &s[..];
}

// Let's rewrite first_word with string slices
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// String Literals are Slices already
fn string_literals() {
    let s = "Hello, world!"; // type of s is &str
}

// Array Slices
fn array_slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // stores a reference to the 1st element and a length
    assert_eq!(slice, &[2, 3]);
}
