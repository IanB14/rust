/* Things learned during this exercise:
   * rust-analyzer won't pick up rust files if you don't specifically open the
   directory containiner Cargo.toml - use File>Open and open the parent dir
   to fix this.
   * Rust includes a number of items automatically into every rust program.
   This is called the prelude - see here for an explanation and the contents:
   https://doc.rust-lang.org/std/prelude/index.html#prelude-contents
*/
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Note the use of an RNG which is local to the current thread and seeded by
    // the OS. Note also the range expression in the form `(start..=end)`.
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // The ! character denotes a macro instead of a function call.
        println!("Guess a number between 0 and 100.");

        /* Variables:
           * The `let` statement is used to create a variable. Variables in Rust
           are immutable by default - the `mut` keyword allows us to create a
           mutable variable. See the following for a discussion of variables and
           mutability:
           https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#variables-and-mutability
           * The `::new()` syntax after `String` indicates that `new()` is an
           associated function of the `String` type. This is common on types in
           Rust as it allows you to create a new instance of that type.
        */
        let mut guess = String::new();

        /* Function calls and error handling:
          * The `io` module was manually imported on line 9 above - it would
          also be possible to import modules in the body of the code using (for
          example) `std::io::stdin` here.
          * The `.read_line` syntax calls the method in the `io` module.
          * Note that we pass a mutable reference to the readline function. The
          argument must be mutable for the readline function to update it.
          * References are immutable by default - this is why we use `&mut
          guess` rather than `&guess` (which is valid if the variable is
          immutable).
          * The readline function returns a `Result` enum - this is a type that
          can be in one of multiple possible states. In this case, `Result` is
          used to encode error-handling information - it can return either `Ok`
          or `Err` and Result has an 'expect' method that can be called. In this
          case, the `.expect` syntax will return either an error or the size of
          the user's input in it's `Ok` value. We don't use the `Ok` value here,
          but we could if we wanted to.
          * Failing to include the `expect` method above will mean that the
          program compiles, but will throw an `(unused_must_use)` warning.
          * Note that while this function call takes place on multiple lines,
          but it would have been perfectly valid to write it without newlines
          like so:
            `io::stdin().read_line(&mut guess).expect("Failed to read guess.");`
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read guess.");

        /* Shadowing
          * The `guess` variable already exists as a String above. Rust allows us
          to shadow the variable - this is a common use case when type casting.
          * `trim()` will remove any leading/trailing whitespace from the input and
          will also trim the newline character the user inputs.
          * `parse()` is a fairly standard type cast in Rust - we tell it to create a
          variable, explicitly give it a type and then Rust knows what type we want
          to cast to.
          * Note that parse only works on numbers that can easily be converted to
          numbers, which makes it error-prone - hence the 'expect'.
          * Note the pattern matching here - more on this below. In this case,
          on an `Ok`, we assign the passed value to the variable and move on. On
          an `Err`, we `continue` to restart the loop.
          * The underscore in `Err` is a catch-all - any errors here will enter
          this arm.
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // The `{}` syntax in the string literal is a 'placeholder' - it can be used
        // to print a value. You can also add empty placeholders and follow the
        // string with a comma separate list of variables you want to print.
        println!("Your guess: {guess}");

        /* Pattern Matching:
          * The `match` syntax sets up a pattern match. Pattern matchers create a
          series of 'arms' - each arm consists of a pattern to match against.
          Whichever statement matches, that is the arm that will be executed. In
          this case, we're calling `cmp` (a method on the String class), passing it
          a reference to the secret number and then choosing an action to perform
          based on the outcome of the pattern match.
          * Rust pattern matching enforces every possible arm - excluding any of the
          `Less/Equal/Greater` arms is a compiler error.
          * The `Ordering` type is used to compare two values and return higher,
          lower or equals. This works on any type that can be compared.
        */
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too low - try again."),
            Ordering::Greater => println!("Too high - try again."),
            Ordering::Equal => {
                println!("Correct - the secret number was {secret_number}");
                break; // break exits the loop, which ends the program.
            }
        }
    }
}
