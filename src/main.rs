fn main() {
    // Variables are immutable by default, so you must use the mut keyword in order to make them mutable.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants can only be a constant expression, not the result of a function  call or any other 
    // value that gets computed a runtime.
    // Constants are available everywhere in the program, and must be type annotated.
    const MAX_POINTS: u32 = 100_000;
    println!("Max points: {}", MAX_POINTS);

    // Variables can be shadowed by using the same name and the let keyword.
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);

    // Unlike mut, shadowing allows you to change the type of value but reuse the name.
    let spaces = "     ";
    let spaces = spaces.len();
    println!("Number of spaces in string is: {}", spaces);
}
