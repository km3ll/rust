fn main() {

    // We are calling a Macro by using the '!' character
    println!("Hello, world!");

    // Variables
    let x: i32 = 5;
    println!("The value of x is {}", x);

    let mut y: i32 = 10;
    println!("The value of y is {}", y);
    y = 15;
    println!("The value of y now {}", y);

    /*
    Constants may be set only to a constant expression, not the result of a function call
    or any other value that could only be computed at runtime.
    */
    const MAX_X: u32 = 100000;
    const MAX_Y: u32 = 100_000;
    println!("Max x is {}. Max y is {}.", MAX_X, MAX_Y);

    // Shadowing
    let z: i32 = 10;
    let z = z + 10;
    let z = z + 10;
    println!("The value of z is {}", z);

    /*
    A difference between mut and shadowing is that because we’re effectively creating a
    new variable when we use the let keyword again, we can change the type of the value
    but reuse the same name.
    */
    let spaces: &str = "   ";
    let spaces: usize = spaces.len();
    println!("spaces value is {}", spaces);

    /*
    This code doesn't compile. The error says we’re not allowed to mutate a variable’s type:
    let mut spaces: &str = "   ";
    spaces = spaces.len();
    */

    // Scalar Types
    // Integer
    let x_axis: i32 = -20;
    let y_axis: i32 = 20;
    println!("x_axis value is {} and y_axis value is {}", x_axis, y_axis);

    // Floating-Point
    let a: f64 = 2.0;
    let b: f32 = 3.0;
    println!("a value is {} and b value is {}", a, b);

    // Numeric operations
    // addition
    let sum = 5 + 10;
    println!("sum value is {}", sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference value is {}", difference);
    // multiplication
    let product = 4 * 30;
    println!("product value is {}", product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient value is {}", quotient);
    // remainder
    let remainder = 43 % 5;
    println!("remainder value is {}", remainder);

    // Boolean
    let is_active: bool = true;
    println!("is_active vaue is {}", is_active);

    // Character
    let z1: char = 'z';
    let z2: char = 'ℤ';
    let emoji: char = '😻';
    println!("z1 vaue is {}. z2 vaue is {}. emoji vaue is {}. ", z1, z2, emoji);

}