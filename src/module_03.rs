#[cfg(test)]
mod module_03 {

    // Hello world
    #[test]
    fn hello_world() {
        // We are calling a Macro by using the '!' character
        println!("Hello, world!");
    }

    // 3.1. Variables and mutability

    #[test]
    fn immutable_variable() {
        let x: i32 = 5;
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

    // 3.2. Data Types

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
        println!(
            "z1 value is {}. z2 value is {}. face vaue is {}. ",
            z1, z2, face
        );
    }

    // Compound Types
    // They can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    #[test]
    fn compound_tuple() {
        //Destructuring: breaking a single tuple into parts
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;

        println!("tup values are x={}, y={}, z={}", x, y, z);
        println!("tup values are x={}, y={}, z={}", tup.0, tup.1, tup.2);
    }

    // Arrays
    // - Every element of an array must have the same type, and have a fixed length.
    // - They are useful when you want your data allocated on the stack rather than the heap
    #[test]
    fn compound_array() {
        let sequence: [i32; 5] = [1, 2, 3, 4, 5];
        let months: [&str; 12] = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        // Different syntax, similar arrays
        let arr_1: [i32; 5] = [3; 5];
        let arr_2: [i32; 5] = [3, 3, 3, 3, 3];

        // Accessing arrays
        // An array is a single chunk of memory allocated on the stack.
        println!("The first month of the year is {}", months[0]);

        // When you attempt to access an element using indexing, Rust will check that the index
        // you’ve specified is less than the array length. If the index is greater than or equal
        // to the array length, Rust will panic. Rust doesn't allow invalid memory to be accessed.
    }

    // 3.3. Functions

    // Rust doesn’t care where you define your functions, only that they’re defined somewhere.
    #[test]
    fn functions_definition() {
        fn print_greeting() {
            println!("Hello, world!");
        }

        fn print_age(age: u32) {
            println!("The age is: {}", age);
        }

        fn print_coordinates(x: i32, y: i32) {
            println!("The coordinates are ({},{})", x, y);
        }

        print_greeting();
        print_age(20);
        print_coordinates(-12, 45);
    }

    // Rust is an expression-based language
    // Statements: instructions that perform some action and do not return a value
    // Expressions: evaluate to a resulting value
    #[test]
    fn functions_body() {
        // Statements do not return values.
        let a = 6;

        // Expressions evaluate to something
        // - For instance: 5 + 6 is an expression that evaluates to the value 11
        // - Calling a function is an expression
        // - Calling a macro is an expression

        let x: i32 = 5;

        // This expression (block) evaluates to 4. Note the i + 1 line without a semicolon at the end
        // Expressions do not include ending semicolons. If you add a semicolon to the end of an
        // expression, you turn it into a statement, which will then not return a value
        let y: i32 = {
            let i: i32 = 3;
            i + 1
        };

        println!("The value of y is: {}", y);
    }

    // Return value
    // - We don’t name return values, but we do declare their type after an arrow (->)
    // - The return value of the function is synonymous with the value of the final expression
    //   in the block of the body of a function.
    // - You can return early from a function by using the return keyword and specifying a value
    #[test]
    fn functions_return_value() {

        // We’re using the return value of a function to initialize a variable
        fn five() -> i32 {
            5
        }
        let x: i32 = five();
        println!("The value of x is: {}", 5);

        fn plus_one(a: i32) -> i32 {
            a + 1
            // - If we place a semicolon at the end of the line containing a + 1, changing it
            //   from an expression to a statement, we’ll get an error: expected `i32`, found `()`
            // - The definition of the function plus_one says that it will return an i32,
            //   but statements don’t evaluate to a value, which is expressed by (), an empty tuple.

        }
        let y:i32 = plus_one(5);
    }

    // TODO: 3.5 Control Flow

}
