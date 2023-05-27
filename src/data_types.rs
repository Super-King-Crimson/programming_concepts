#![allow(unused)]
use std::io;

pub fn explain() {
    //Rust is statically typed - must know every variable's type at compile time

    //compiler complains because it doesn't know what to parse 100 to: int8? int16? float32?
    let guess: u32 = "100".parse().expect("Not a number!");
    
    //Integers:
    
    //Two types: signed (i) and unsigned (u) integers
    //unsigned integers cannot represent negatives, and therefore can represent more positives

    //i8, i16, i32, i64, i128, isize (depends on computer architecture)
    //u8, u16, u32, u64, u128, usize

    //In general, a signed number with n bits can represent numbers between -2^(n - 1) and 2^(n - 1) - 1.
    //In general, an unigned number with n bits can represent numbers between 0 and 2^n

    //Type specifications and other ways to specify numbers
    let signed: i8 = 54i8;
    let unsigned: u8 = 250u8;
    let easier_to_read: u64 = 1_000_000_000_000_000_000; //1*10^18, one quintillion
    let hex: u32 = 0xffffffff; //2^32 in hex, max u32 number
    let binary: i8 = 0b1010;   //11 in binary

    //If you don't know what to use, use i32.


    //Floats:

    //Only two types: f32 (single), f64 (double)
    //f64 is default

    let float = 2.0;
    let float32: f32 = 3.0; 

    //integer division is real
    println!("You'll never get this one. 3 / 4 = {}", 3 / 4);
    

    //Booleans: exist. 
    let true_or_false: bool = true;


    //Char:
    
    //Declare with singular quotes: 4 bytes long and represents Unicode Scalar Value
    let power = '力';
    let a: char = 'A';
    let emoji = '絵';


    //Compound Types group multiple values into one type

    //Tuples

    //Group various types into one compound
    //Use dot indexing to get a value in a tuple
    let tuple: (i8, bool, u8) = (-127, false, 128);
    println!("Here's what's in the tuple: {} {} {}", tuple.0, tuple.1, tuple.2);

    //Use pattern matching to get individual values out of a tuple (destrucutring)
    let (first, second, third) = tuple;
    println!("Here's what's in the tuple, backwards: {third} {second} {first}");

    //A tuple without any values has a special name: unit 
    let unit: () = ();
    //represents an empty value or return type: expressions implicity return a unit if no other return value

    //Arrays

    //All must have the same type, fixed size
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
        "December"
    ];
    //When we learn about the stack vs the heap arrays will seem much more useful
    
    //Cool ways to initialize arrays
    let some_numbers: [i32; 5] = [32, 314, 899, -27, 9]; //declare an array's type with arrName: [type; size]
    let someone_likes_balls: [char; 8] = ['玉'; 8]; //creates an array with 8 玉 characters in it

    //Invalid array index access:
    let panic_array: [u8; 5] = [1, 2, 3, 4, 5];

    println!("Enter a positive integer: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Input wasn't a positive integer");

    //If index > 4 (max index), rust will panic at runtime and stop execution
    let element = panic_array[index];
    println!("[{index}]: {element}");

    //Other low level languages would just let this grab invalid memory
    //Rust practices memory safety!

    let t = ([1; 2], [3; 4]);
    let (a, b) = t;

    println!("{}, {}, {}.", t.1[3], b[2], a[1])

    //Does this compile?
    //If so, what does this print out?
}