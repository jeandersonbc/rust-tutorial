fn main() {

    // Important to remember: Rust is STATICALLY TYPED; however, when it is
    // not explicitly declared and there is enough information on the current
    // context, the Rust compiler is able to infer the type of a variable
    //
    // Remember: being statically typed means that all types must be known at
    // compile time.
    let whichtype = "Must be a string captain obvious!";

    // Without the 32-bits integer annotation (i32), the compiler
    // can't infer the type because there are multiple results for the string
    // parsing
    let guess: i32 = "47".parse().expect("Can't convert to number");

    // In Rust there are Scalar and Compound types!
    // All numeric types have a signed and unsigned variant
    let num : i32 = 34_423;
    let num : u32 = 34_423;

    // Sizes: 8, 16, 32, 64, and architecture-dependent (i/usize):
    let num : isize = 100_000;

    // Other scalars (not required here but types are explicitly annotated):
    let other : char = 'a';
    let other : bool = false;
    let other : f32 = 3.4214;
}
