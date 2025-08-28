fn main() {
    data_types();
}

fn variables() {
    //1 immutable and mutable variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    //2 const variable, type has to be annotated
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{}", THREE_HOURS_IN_SECONDS);

    //3 shadowing. you can declare a variable with the same name
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("Inner scope y is: {y}");
    }
    println!("The value of y is: {y}");
}

fn data_types() {
    //1 Scalar and Compound types
    // Scalar type: integers, floats, booleans, characters

    // Handling integer overflows: wrapping_, checked_, overflowing_, saturating_ methods
    let x: u8 = 250;
    let y: u8 = 10;

    // Wrapping
    let sum = x.wrapping_add(y); // 250 + 10 = 260 -> wraps to 4 (mod 256)
    println!("{}", sum); // 4

    // Checked
    match x.checked_add(y) {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("Overflow occured!"),
    }

    // Overflowing. Returns a tuple (value, bool). value = result, bool = true if overflow
    let (sum, overflowed) = x.overflowing_add(y);
    println!("Sum is: {sum}, Overflowed? {overflowed}"); // Sum is: 4, Overflowed? true

    // Saturating. Saturates at min/max values instead of wrapping
    let sum = x.saturating_add(y); // caps at u8::MAX = 255
    println!("{sum}"); // 255

    //2 Floating-Point Types
    let x = 2.0;
    let y: f32 = 3.0;

    // Numeric Operations
    let sum = 5 + 10;
    let difference = 95.5 - 18.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // -1
    let remainder = 43 % 5;

    //3 Boolean Type. Takes 1 byte in size
    let t = true;
    let f: bool = false; // with explicit type annotation

    //4 Char Type. The most primitive type. Specified with single quotes!!! 4 bytes in size -> can represent emojis and other languages
    let c = 'z';
    let z: char = 'Z';

    // The Tuple Type. They can have elements of diff types, however they are fixed in size (can't grow or shrink)
    let tup: (i32, f64, char) = (500, 908.9, 'P'); // type annotations are optional

    // Pattern matching to destractre a tuple:
    let (x, y, z) = tup;

    // Accessing elements in a Tuple:
    let five_hundred = tup.0;
    let float_el = tup.1;
    let char_el = tup.2;
    println!("Accessing elements in a Tuple: {five_hundred}, {float_el}, {char_el}");

    // The Tuple without any values has a special name - unit
    // Expressions implicitly return the unit value if they don’t return any other value.

    // The Array Type. Elements have to be one type. Fixed length
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Arrays are preffered over vectors when the number of elements won't change
    // For example names of the months:
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let a = [3; 5]; // array of 5 elements, all set to 3

    // Accessing elements of an Array
    let second = a[1];
    // Invalid array indexing results in runtime error (panic)
}
