
use std::io;    // brings the standard i/o library into scope
use std::cmp::Ordering; // like Result, this is an enum with 3 variants, Less, Greater & Equal
use rand::Rng;  // Rng trait defines methods that random number generators impl

                // entry point into the program
fn main() {     // fn declares new function, main=name of function, ()=no args passed
    println!("Guess a number!");

    let secret_number = rand::thread_rng().gen_range(1, 10);
    /* initialise (immutable) var with a value equal to result of the above methods from rand
    thread_rng() is the specific rand gen to be used & gen_range() which takes 2 args
    these args are inclusive...exclusive */

    // println!("The secret number is: {}", secret_number); (removed to make the game challenging)

    loop { // loop on its own initialises an infinite loop

        println!("Please input your guess: ");  // calling println! macro to provide output to user

        let mut guess = String::new();
        /*  let statement, creates a new var
        mut=mutable, makes var mutable(vars=immut by default)
        String::new(), means assign the variable to a new String instance(UTF-8)
        :: in ::new indicates that 'new' is associated function of String type
        associated functions are run on a var type as opposed to a var instance (aka static methods)
        */

        io::stdin()
            // calls the stdin function from imported library
            // if not imported the same could be achieved by std::io::stdin

            .read_line(&mut guess)
            // ^ reads line of user input and place into the string as an argument
            // references (&) are immutable so must make mutable

            .expect("Failed to read line");
        /*  this could have been written:
        io::stdin().read_line(&mut guess).expect("Text here...");
        separate with white space to make more readable */

        // let guess: u32 = guess.trim().parse().expect("Please type a number!")

        /* shadowing the earlier 'guess' var allows you to convert value from 1 type to another
        shadowing enables the reuse of the guess variable name opposed to making unique similar ones
        'guess' refers to the original guess, trim() eliminates any whitespace at start or end
        once the user enters the number they press enter which introduces a newline
        trim() removes that newline, parse() on strings converts it into a number */

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, // Ok value of Result returned, containing the num version of string
            Err(_) => continue, // Err value of Result returned, and program executes 'continue'
            // continue tells program to go to next iteration of the loop to ask for a new guess
        };

        /* moving from 'expect' call to 'match' expression is generally how you handle errors
        opposed to crashing on an error - we are using parse() which returns a Result type
        and Result is an enum with variants Ok or Err - match expression is used here
        */

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {   // match expressions are made up of 'arms'
            Ordering::Less => println!("Too small!"),   // this is an example of an 'arm'
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {    // adding a break statement to this expression allows the loop to end
                println!("You win!");
                break
            }
        }
        /* the cmp method compares 2 values & can be called on anything to compare
        after bringing the Ordering enum into scope it can now be used
        the Ordering variant is returned depending on the result of cmp between
        'guess' var and '&secret_number' var reference */
    }

}
