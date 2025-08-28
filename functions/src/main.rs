fn main() {
    println!("Hello, world!");

    statements_expressions();

    println!("{}", five());
}

fn another_function(x: i32) {
    println!("The passed argument value is: {x}");
}

fn statements_expressions() {
    // Statements and Expressions
    // - Statements are instructions that perform some action and do not return a value.
    // - Expressions evaluate to a resultant value.
    let y = 6; // statement
    // let x = (let y = 6); // can't assign a statement to let, because it doesn't return a value

    //Calling a function is an expression. Calling
    // a macro is an expression. A new scope block created with curly brackets is an expression
    let y = {
        let x = 3; // now inside the {} is an expression
        x + 1 // expressions don't include ';'
    };
    println!("The value of y is: {y}");
}

// Return values
fn five() -> i32 {
    5
}
