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
 * */

 /**
  * Say hello from chapter 04
  */
pub fn greeting() {
    println!(">> Greetings from Ownership\n");
}