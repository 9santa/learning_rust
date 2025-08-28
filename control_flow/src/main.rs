use std::io;

fn main() {
    let mut n = String::new();

    println!("Please enter n to calculate n-th Fibonacci number: ");

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read number");

    let mut n: i64 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => -1,
    };

    let n_th = nth_fib(n);
    println!("The {n} Fibonacci number is: {n_th}!");
}

fn if_statements() {
    let number = 3;

    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }
}

fn multiple_conditions() {
    let number = 8;

    if number % 4 == 0 {
        println!("Divisible by 4");
    } else if number % 3 == 0 {
        println!("Divisible by 3");
    } else if number % 2 == 0 {
        println!("Divisible by 2");
    } else {
        println!("Not Divisible by 4, 3, or 2");
    }
}

fn if_in_a_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("{}", number);
}

fn loop_() {
    // loop keyword is an infinite loop
    // loop {
    //     println!("again!");
    // }

    // Returning values from Loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
            // would exit the current function
            // return counter * 2;
        }
    };

    println!("The result is: {result}");

    // Loop Labels to Distinguish. Loop labels begin with a single quote ' at the start
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // breaking an outer loop
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}

// While conditional loops
fn while_() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn for_() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Looping through a Collection with while
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    println!("\n");

    // for loop is better and faster for this purpose
    for element in a {
        println!("the value is: {element}");
    }

    // more efficient than while
    for number in (1..=3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!");
}

// Some practice
fn nth_fib(n: i64) -> i64 {
    if n < 0 {
        println!("Number should be >= 0!");
        return 0;
    }

    match n {
        0 => 0,
        1 => 0,
        2 => 1,
        _ => nth_fib(n - 1) + nth_fib(n - 2),
    }
}
