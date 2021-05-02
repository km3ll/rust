mod module_03;

fn main() {

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