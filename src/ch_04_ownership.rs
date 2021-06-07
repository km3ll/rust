/*
 * Ownership
 * 
 * The Stack
 * The stack stores values in the order it gets them and removes the values in the opposite order.
 * This is referred to as last in, first out. All data stored on the stack must have a known, fixed size
 * Adding data is called pushing onto the stack, and removing data is called popping off the stack.
 * 
 * The Heap
 * Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
 * 
 * Allocating ( on the heap )( bookkeeping )
 * The heap is less organized: when you put data on the heap, you request a certain amount of space.
 * The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use,
 * and returns a pointer, which is the address of that location.
 * Pushing values onto the stack is not considered allocating. Because the pointer is a known, fixed size,
 * you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.
 * 
 * Scope
 * A scope is the range within a program for which an item is valid.
 * 
 * String
 * This type is allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
 * Why can String be mutated but literals cannot?
 *  - In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable.
 * The memory is automatically returned once the variable that owns it goes out of scope
 * 
 * RAII
 * In C++, this pattern of deallocating resources at the end of an item’s lifetime is sometimes called
 * Resource Acquisition Is Initialization (RAII). The drop function in Rust will be familiar to you if you’ve used RAII patterns.
 * 
 * Double free error
 * This is a problem: when s2 and s1 go out of scope, they will both try to free the same memory.
 * This is known as a double free error.
 * - shallow copy & deep copy
 * */

pub fn fn_chapter_04() {
  
  let name: String = String::from("mr roboto");
  println!("domo arigato {}", name);


  // bind the value 5 to x; then make a copy of the value in x and bind it to y.
  let _x: i16 = 5;
  let _y: i16 = _x;
  println!("values x:{} and y:{} pushed onto the stack", _x, _y);


  // move
  // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length,
  // and the capacitythat are on the stack. We do not copy the data on the heap that the pointer refers to.
  // String: pointer, length:5, and capacity:5
  let s1 = String::from("hello");
  let s2 = s1;
  println!("pointers s1 and s2:{} refer the same object", s2);


  // clone
  // If we do want to deeply copy the heap data of the String, not just the stack data
  let s3 = String::from("hakuna");
  let s4 = s3.clone();
  println!("heap data for s3:{} and s4:{} copied", s3, s4);


  // copy
  // Types such as integers that have a known size at compile time are stored entirely on the stack,
  // so copies of the actual values are quick to make.
  let n1: i16 = 10;
  let n2: i16 = n1;
  println!("stack values n1:{} and n2:{} copied", n1, n2);


  // ownership and functions
  // 1. msg comes into scope
  let msg: String = String::from("hola");
  // 2. msg's value moves into the function
  takes_ownership(msg);
  // 3. So, msg is no longer valid. Error: borrow of moved value
  // println!("msg's value is {}", msg);

  // 4. age comes into scope
  let age: i32 = 30;
  // 5. i32 is Copy
  makes_copy(age);
  // 6. So, it's safe to use age afterward
  println!("age's value is {}", age);
  

  // return values and scope
  // Returning values can also transfer ownership
  // 1. gives_ownership moves its return value into greeting
  let greeting: String = gives_ownership();
  println!("{}", greeting);
  // 2. name comes into scope
  let name: String = String::from("John Wick");
  // 3. name is moved into takes_and_gives_back, which also moves its return value into full_name
  let full_name: String = takes_and_gives_back(name);
  println!("My name is {}", full_name);

  // tuples
  let code_name: String = String::from("master miller");
  let (code, length) = calculate_length(code_name);
  println!("The length of code:{} is {}", code, length);
  

}

// ownership and functions
fn takes_ownership(some_string: String) { // some_string comes into scope
  println!("took ownership of String {}", some_string);
} // some_string goes out of scope and `drop` is called (memory is freed)

fn makes_copy(some_integer: i32) { // some_integer comes into scope
  println!("made copy of i32 {}", some_integer);
} // some_integer goes out of scope. Nothing special happens


// return values and scope
fn gives_ownership() -> String {
  // some_string comes into scope
  let some_string: String = String::from("Good evening.");
  // some_string is returned and moves out to the calling function
  some_string
}

fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
  // a_String is returned and moves out to the calling function
  a_string
}


// tuples
fn calculate_length(a_string: String) -> (String, usize) {
  let length: usize = a_string.len();
  (a_string, length)
}