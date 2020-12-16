/* crate::module */
use std::io;
/* crate::trait */
use rand::Rng;
/* Ordering = enum */
use std::cmp::Ordering;
/*
 * crate = library/package
 *  every crate has a root module that contains the code for that crate
 *  (corresponds to the name of the crate)
 * sub-modules can be defined under the root module
 */
fn main()
{
    println!("Guess the number!");

    /*
     * rand::thread_rng(): gives the random number generator to be used,
     * it is local to the current thread of execution & seeded by OS
     */
    let secret_number = rand::thread_rng().gen_range(1, 101);

    /*
     * loop: creates an infinite loop
     */
    loop {
        println!("Please input your guess.");

        /*
         * Define a mutable (can be changed) variable guess
         * that is of type string
        */
        let mut guess = String::new();

        /* 
        * Method
        * Functions attached to objects-- stdin() is part of the io object
        * Break up in multiple lines for clarity-- would be: 
        *  io::stdin().read_line(&mut guess).expect(" ")
        */
        io::stdin()
           /*
            * References are immutable by default-
            * need to write &mut guess rather than
            * &guess
            * read_line returns io::Result
            * Result types = enums
            * Returns Ok(T) or Err(E)
            */
            .read_line(&mut guess)
            /*
             * Expect
             * If io::Result returns Err, panic & output message. passed as an argument to expect
             * If io::Result returns Ok, expect takes value that
             * Ok is holding (# of bytes entered into stdio) and returns it
             * Don't call expect = warning
             */
            .expect("Failed to read line");

        /*
         * Shadow previously defined variable "guess"-- convert value from String to u32
         * trim method: elimate whitespaces at beginning and end
         * parse method: parses a string into some kind of number-- we need to declare
         *  the exact number type. returns Result
         * Type inference: u32 declaration here means Rust will infer that secret_number
         *  should also be u32
         * expect -> match = crashing on error -> handling error
         * "_" = catchall value: match all Err values no matter what info they have inside
         */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        /*
         * {} = placeholder
         */
        println!("You guessed: {}", guess);

        /*
         * cmp method: compares 2 values, takes a reference to variable
         *  returns a variant of Ordering
         * match: like a switch, made up of "arms"
         *  arms: pattern & code that should be run if value given to match matches
         */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

