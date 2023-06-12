#![allow(unused)]
use rand::Rng;

//Here's one of the most important functions right here.
pub fn explain() {
    println!("Hello, World!");
    
    explain_some_more();
}
//Don't look at me like that. We're pretending this is the main function, remember!
//The main function is called when your project's executable is ran.


//The common convention for functions is to name them like variables with lower_snake_case
fn explain_some_more() {
    println!("こんにちは、世界！");

    explain_parameters(rand::thread_rng().gen_range(1..=20), 
        rand::thread_rng().gen_range(1..=20));
}
//Also functions are hoisted, that's why we can call explain_some_more even though it's defined afterwards


//parameters MUST have their type defined
fn explain_parameters(num1: i32, num2: i32) {
    let multiplied = num1 * num2;
    let subtracted = num1 - num2;

    println!("Input: {{{num1}, {num2}}}");
    println!("Output: {{{}}}", multiplied + subtracted);

    explain_statements_and_expressions(10);
}


//Rust is an expression-based language.
fn explain_statements_and_expressions(num: i32) {
    //Everything inside a function's squiggly brackets is its BODY.
    //Rust function bodies are made up of some statements, and can end in an expression.

    //These are statements. They perform multiplcation with num, assing it to the vars and don't return anything.
    let square = num * num;
    let random_multiply = num * rand::thread_rng().gen_range(1..=num);
    /*
    By 'they don't return anything,' I mean you can't do something like:

    //let some_var = (let square = num * num);

    This isn't valid syntax, because the assignment to square doesn't return a value to give some_var, so it's like:
    let some_var = 
    */

    //The curly braces part of the assignment is an expression. It evaluates, and produces a value.
    let abstract_subtract = {
        let some_var = square - random_multiply;
        some_var - 5 //here's the value that's returned
    };
    //Calling a function, a macro, or creating a new scope block are all expressions.
    //Note that the last line of the curly braces doesn't have a semicolon:
    //adding a semicolon to the end of an expression turns it into a statement

    println!("The returned value is {}", explain_return());
}


//function return values must be specified
fn explain_return() -> f64 {
    3.141592653589793
}