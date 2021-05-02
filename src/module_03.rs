#[cfg(test)]
mod tests {

    // Hello world
    #[test]
    fn hello_world() {

        // We are calling a Macro by using the '!' character
        println!("Hello, world!");

    }

    // Variables and mutability
    #[test]
    fn immutable_variable() {

        let x:i32 = 5;
        println!("The value of x is {}", x);
        assert_eq!(x, 5)

    }

    #[test]
    fn mutable_variable() {

        let mut y: i32 = 10;
        println!("The value of y is {}", y);
        assert_eq!(y, 10)

    }

    // Constants
    // - They may be set only to a constant expression, not the result of a function call
    //    or any other value that could only be computed at runtime.
    // - They are valid for the entire time a program runs, within the scope they were declared in
    #[test]
    fn constant() {

        const MAX_X: u32 = 100000;
        const MAX_Y: u32 = 100_000;
        println!("Max x is {}. Max y is {}.", MAX_X, MAX_Y);

    }

    // Shadowing
    // - You can declare a new variable with the same name as a previous variable, and the new
    //   variable shadows the previous variable. The first variable is shadowed by the second
    // - Shadowing is different from marking a variable as mut
    // - By using 'let', we can perform a few transformations on a value but have the variable
    //   be immutable after those transformations have been completed.
    // - The other difference between mut and shadowing is that because we’re effectively creating
    //   a new variable when we use the let keyword again, we can change the type of the value
    //   but reuse the same name.
    #[test]
    fn shadowing() {

        let z: i32 = 10;
        let z = z + 10;
        let z = z + 10;
        println!("The value of z is {}", z);
        assert_eq!(z, 30);

        let spaces: &str = "   ";
        let spaces: usize = spaces.len();
        assert_eq!(spaces, 3);
        println!("spaces value is {}", spaces);

        // This code doesn't compile. The error says we’re not allowed to mutate a variable’s type:
        // let mut spaces: &str = "   ";
        // spaces = spaces.len();

    }

    // Rust is a statically typed language,
    // If we don’t add the type annotation here, Rust will display an error, meaning that the
    // compiler needs more information from us to know which type we want to use.
    #[test]
    fn function_parse() {

        let guess: u32 = "42".parse().expect("Not a number!");

    }

    // Scalar Types
    // A scalar type represents a single value. Rust has four primary scalar types:
    // integers, floating-point numbers, Booleans, and characters.

    // Integer
    // An integer is a number without a fractional component.
    // Each variant can be either signed or unsigned and has an explicit size.
    #[test]
    fn scalar_integer() {

        let age: u32 = 27;
        let balance: i32 = -33;
        println!("values age={}, balance={}", age, balance);

    }

    // Floating point
    // The default type is f64 because on modern CPUs it’s roughly the same speed as f32
    // but is capable of more precision.
    #[test]
    fn scalar_floating() {

        let a: f64 = 2.0;
        let b: f32 = 3.0;
        println!("values a={}, b={}", a, b);

    }

    #[test]
    fn scalar_numeric_operations() {

        let addition = 5 + 10;
        println!("addition value is {}", addition);

        let difference = 95.5 - 4.3;
        println!("difference value is {}", difference);

        let product = 4 * 30;
        println!("product value is {}", product);

        let quotient = 56.7 / 32.2;
        println!("quotient value is {}", quotient);

        let remainder = 43 % 5;
        println!("remainder value is {}", remainder);

    }

    // Boolean
    // They are one byte in size.
    #[test]
    fn scalar_boolean() {

        let is_active: bool = true;
        println!("is_active value is {}", is_active);

    }

    // Character
    // char literals are specified with single quotes, as opposed to string literals,
    // which use double quotes. Rust’s char type is four bytes in size and represents a Unicode
    // Scalar Value, which means it can represent a lot more than just ASCII.
    #[test]
    fn scalar_character() {

        let z1: char = 'z';
        let z2: char = 'ℤ';
        let face: char = '😻';
        println!("z1 value is {}. z2 value is {}. face vaue is {}. ", z1, z2, face);

    }

    #[test]
    fn template() {

    }

}