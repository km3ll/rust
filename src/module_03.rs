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

    }

    #[test]
    fn template() {

    }

}