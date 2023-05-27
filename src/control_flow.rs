#![allow(unused)]
use std::io;

const THE_MEANING_OF_LIFE: i32 = 42;

pub fn explain() /* explain_if_else_elseif */ {
    //Some stage setting
    println!("For the sake of demonstration, please enter an integer:");

    let mut response = String::new();
    
    io::stdin().read_line(&mut response).expect("Failed to read");

    let response: i32 = response.trim().parse().expect("Not an integer!");

    //if, else, and else if statements

    //the condition that allows a branch of the if statement to be run is called an arm
    if response > THE_MEANING_OF_LIFE {
        println!("Pretty good number!");
    } else if response < THE_MEANING_OF_LIFE {
        println!("Not a fan of this number.");
    } else {
        println!("Oh my god, you know the magic number!");
    }
    //it's just every other language but you don't need the parentheses (&& and || still exist)
    //Rust doesn't typecast conditions (NO TRUTHY/FALSY) - condition must return true or false

    //Using if for let assignments
    let coolness_level: u8 = if response == THE_MEANING_OF_LIFE {255} else {100};

    //IS THE SAME AS:

    //also you don't need the mut keyword here: coolness_level is only assigned to once
    let coolness_level: u8; 
    if response == THE_MEANING_OF_LIFE {
        coolness_level = 255;
    } else {
        coolness_level = 100;
    }
    //Note that both conditions MUST HAVE THE SAME TYPE: Rust needs to know this variable's type is

    //Okay, on to loops!
    explain_loops();
}


fn explain_loops() {
    //loop, for, and while loops

    //loop runs infinitely until you stop it with break
    //Use continue to skip the rest of the current loop iteration and move to the next
    let mut counter: u8 = 0;
    loop {
        counter += 1;

        if counter == 3 || counter == 7 {
            println!("Taking a brief break.");
            continue;
        }

        println!("CREO IS THE GREATEST!");
        
        if counter > 10 {
            break;
        }
    }

    //You can create and use a LOOP LABEL to specify which loop to break
    let mut inner_counter: usize = 0;
    let mut outer_counter: usize = 0;
    let what_the_hell: [[(i32, char); 2]; 3] = [
        [(1, 'a'), (2, 'b')], 
        [(3, 'c'), (4, 'd')], 
        [(5, 'e'), (6, 'f')]
    ];

    //here's how you make a loop label
    'outer: loop {
        println!("We are currently on index {outer_counter} of the outer array.");

        loop {
            let curr_pos = what_the_hell[outer_counter][inner_counter];

            println!("We got [{}: {}] at ({outer_counter}, {inner_counter})", 
                curr_pos.0, 
                curr_pos.1);

            if curr_pos.1 == 'c' {
                outer_counter += 1;
                inner_counter == 0;
                //here's how you use an loop label
                continue 'outer;
            }

            inner_counter += 1;

            if inner_counter > 1 {
                inner_counter = 0;
                break;
            }
        }

        outer_counter += 1;

        if outer_counter > 2 {
            break;
        }
    }


    //while loops work exactly as you expect, and can also use loop labels:
    let mut cownter: u8 = 5;
    let mut calfnter: u8 = 3;

    'cow: while cownter > 0 {
        println!("I got a cow!");

        while calfnter > 0 {
            println!("I got a calf!");

            if cownter == 3 && calfnter == 2 {
                println!("...actually I'm lying, I don't have any pets ;-;");
                break 'cow;
            }

            calfnter -= 1;
        }

        cownter -= 1;
        calfnter = 3;
    }

    //for loops are basically while loops for collections (like arrays)
    let trees: [char; 3] = ['木', '林', '森'];

    for tree in trees {
        println!("{tree}");
    }

    //you can even make a numerical range a collection with the Range collection to make a numeric for loop
    for i in 0..4 {
        println!("Look at me mom, I'm numeric! (Sent {i} days ago)");
    }
    //Note that this only prints numbers 0 1 2 3 - loop stops when you hit end of range
}