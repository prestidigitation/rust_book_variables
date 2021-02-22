fn main() {
    // Variables are immutable by default, so you must use the mut keyword in order to make them mutable.
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Variables can be shadowed by using the same name and the let keyword.
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}", y);
}
