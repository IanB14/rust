/*
   * Variables
     * By default, variables in Rust are immutable. Rust will immediately catch
     mutability errors as soon as you type them and will refuse to compile if
     you try to build/run anyway.
     * You don't have to explicitly specify a type for variables, but the
     rust-analyzer will suggest one based on the value you assign. This is the
     type that Rust will infer for the type.
   * Constants
     * Constants are similar but different to immutable variables. The `mut`
     keyword doesn't work with the `const` keyword and constants must have a
     type specified.
     * Constant identifiers in Rust are ALL_CAPS_SNAKE_CASE.
     * Rust attempts to evaluate complicated constants at compile time - see
     https://doc.rust-lang.org/reference/const_eval.html for more.
     * Constants are valid for the entire runtime of the program WITHIN THE
     SCOPE THEY WERE DECLARED IN. This makes constants useful for unchanging
     values that multiple parts of your code may need to know about.
   * Shadowing
     * Variables can be shadowed by repeating the `let` keyword. Note that this
     is different from using the `mut` to denote the variable mutable and is NOT
     a compile-time error.
     * Shadowing within a scope is only valid within that scope - variables that
     are created in one scope and then shadowed in an enclosed scope will keep
     their value until the end of the scope at which point they take on the
     original value again (see example below).
     * Shadowing allows us to modify a variable but also keep the variable
     immutable once we're finished with it.
     * Shadowing also effectively allows us to change the type of a variable.
*/
fn main() {
    // Variables
    let x: i8 = 5; // This is an immutable variable - note the type specificity.
    println!("The value of x is {x}.");
    // Attempting `x = 6;` is an error - "cannot mutate immutable variable `x`".

    let mut y = 10; // This is a mutable variable - note no type is given.
    println!("The value of y is {y}.");
    y = 12;
    println!("The value of y is now {y}.");

    // Constants
    // Note the explicit type and the `const` keyword.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in equal to {THREE_HOURS_IN_SECONDS} seconds.");

    // Shadowing
    // Initial declaration/initialisation. Note that the compiler will throw a
    // warning here as we're not using this initial value.
    let z = 5;
    // Shadowing - z is equal to 10 from this point on.
    let z = 10;
    {
        let z = 20;
        // z = 20 at this point
        println!("The value of z within this scope is {z}.");
    }

    // z = 10 at this point
    println!("The value of z within this scope is {z}.");

    // Using shadowing to change a variable's type. Note that we can use the
    // original variable to construct the shadowed value.
    let spaces = "     ";
    let spaces = spaces.len(); // this would fail if we used `let mut`
    println!("There are {spaces} spaces in the string.")
}
