fn main() {
    // By default, variables are immutable in Rust.  As the doc says, "This
    // is one of many nudges in Rust that encourages you to write your code
    // in a way that takes advantage of the safety and easy concurrency
    // that Rust offers."
    let x = 5;
    println!("The value of x is: {}", x);

    // This will cause a compilation error! Re-assigning an immutable var
    // is prohibit.
    //x = 30;

    // Use the modifier "mut" to make a variable mutable
    let mut y = 5;
    y += 10;
    println!("The value of y is: {}", y);

    // Constants are always immutable. We use the keyword "const" instead
    // of "let" for constants. You can't use "mut" and you have to provide
    // the datatype, as well:
    const MAX_VALUE : u32 = 100_000;
    println!("MAX VALUE IS: {}", MAX_VALUE);

    // NOTE: Shadowing is different from making a variable mutable!
    // Read the docs for more info, if necessay :-)
}
