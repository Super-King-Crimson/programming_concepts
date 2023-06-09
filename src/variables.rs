#![allow(unused)]
const YOULL_UNDERSTAND_WHY_THIS_IS_HERE_LATER: f32 = 55.5; //did you know 555 is 'lol' in Thai?

pub fn explain() {
    //let is the variable definition keyword, name convention is lower_snake_case
    let my_var = 10; 
    println!("my_var has a value of {my_var}");
    //my_var = 12;    //variables are immutable by default, so compiler throws an error
    
    let mut my_mut = 5;    //use mut to make a variable mutable so it can be changed
    println!("my_mut has a value of {my_mut}");
    my_mut = 10;
    println!("And now it's {my_mut}");

    //constants can be declared with const, name convention is UPPER_SNAKE_CASE
    const SPEED_OF_SOUND_CONST: f32 = 3.43 * 100.0;    //must have a data type annotation
    //this is the same speed as writing 343, compiler can evaluate basic expressions

    //constants can even be declared outside of a function! (let must be declared in function)
    println!("Do you understand? {}", SPEED_OF_SOUND_CONST * YOULL_UNDERSTAND_WHY_THIS_IS_HERE_LATER);

    //If let is used with a the same variable name as another one, it is shadowed
    //Since it's basically saving a completely different variable to the same name, it can even change type:
    let shadow = 10;
    println!("First, the variable is: {shadow}");
    {
        let shadow = 11.11;  //A shadow of a variable takes all uses of a variable for itself until:
        println!("Going in a block, the variable is: {shadow}");
        {
            let shadow = "twelve";  //the shadow itself is shadowed
            println!("In the innermost block the variable is: {shadow}");
        }
        println!("Coming out the block, the variable is: {shadow}");
    }   //or the scope where the shadow was created ends
    println!("Finally, the variable is: {shadow}");

    //Note that even though shadow is immutable, we can manipulate it's value using shadowing
    //That means we can still transform it, then have it go back to being immutable after
}