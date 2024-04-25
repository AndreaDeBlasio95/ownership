// OWNERSHIP RULES:
// 1. Each value in Rust has a variable that’s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

use core::str;

// keywords: move, drop, clone, 
fn main() {
    
    // String type - Heap allocated -------------------------------
    let s = String::from("hello");
    // the :: operator allows us to namespace this particular from function under the String type rather than using some sort of name like string_from
    println!("{}", s);

    let mut str = String::from("Hello");
    str.push_str(", World!"); // push_str() appends a literal to a String
    println!("{}", str);

    // Memory and Allocation ---------------------------------------
    // string literals are immutable and hardcoded into the binary, we know the contents at compile time, so the text is hardcoded directly into the final executable
    // String type is allocated on the heap, which means the memory must be requested from the operating system at runtime
    // The memory must be requested when we call String::from and the memory is automatically returned once the variable that owns it goes out of scope, so using drop, this is done by the Rust runtime when closing the curly braces of the scope
    // Memory lenght: how much memory, in bytes, the is currently used
    // Memory capacity: the total amount of memory, in bytes, that has received from the operating system

    // Ways Variables and Data Interact: Move -----------------------
    let x = 5;
    let y = x;

    // Double free error that Rust prevents
    let s1 = String::from("hello"); // s1 is moved to s2
    let s2 = s1;

    // Ways Variables and Data Interact: Clone ----------------------
    let my_s1 = String::from("hello");
    let my_s2 = s1.clone(); // deep copy
    println!("my_s1 = {}, my_s2 = {}", my_s1, my_s2);

    // Stack-Only Data: Copy ----------------------------------------
    // Types such as integers that have a known size at compile time are stored entirely on the stack, so copies of the actual values are quick to make
    let my_p = 5;
    let my_q = my_p;
    println!("my_p = {}, my_q = {}", my_p, my_q);

    // Ownership and Functions --------------------------------------
    let my_str = String::from("hello");

    takes_ownership(my_str); // my_str value is moved to the function

    let my_x = 5;
    makes_copy(my_x); // my_x value is copied to the function

    // Return Values and Scope
    let s_1 = gives_ownership();
    let s_2 = String::from("hello");
    let s_3 = takes_and_gives_back(s_2);

    // Returning ownership of parameters
    let s_4 = String::from("hello");
    let (s_5, len) = calculate_length(s_4);
    println!("The length of '{}' is {}", s_5, len);

    // References and Borrowing -------------------------------------
    let str_1 = String::from("hello");
    let len = calculate_length_ref(&str_1);
    println!("The length of '{}' is {}", str_1, len);

    // Mutable References -------------------------------------------
    let mut str_2 = String::from("hello");
    change(&mut str_2);

    let mut str_3 = String::from("hello");
    {
        let r1 = &mut str_3;
    } // r1 goes out of scope here, so we can make a new reference with no problems
    let r2 = &mut str_3;

    // This code is error:
    // let mut string_1 = String::from("hello");
    // let refer_1 = &string_1; // no problem
    // let refer2 = &string_1; // no problem
    // let refer3 = &mut string_1; // BIG PROBLEM
    // can't borrow as mutable because it is also borrowed as immutable

    // Dangling References -------------------------------------------
    // Rust ensures that references will never be dangling references, which would be a pointer to memory that may have been given to someone else, by ensuring that all borrows are valid
    

}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
} // some_string goes out of scope and `drop` is called. The backing memory is freed

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
} // some_integer goes out of scope. Nothing special happens


fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
} // some_string is returned and moves out to the calling function

fn takes_and_gives_back(a_string: String) -> String {
    a_string
} // a_string is returned and moves out to the calling function

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}

// & is a reference, which allows you to refer to some value without taking ownership of it
fn calculate_length_ref(s: &String) -> usize {
    s.len()
} // s goes out of scope, but because it does not have ownership of what it refers to, nothing happens
// Is you try to modify while borrowing, you will get a compile error
// As variables are immutable by default, so are references. You can make them mutable by using &mut

fn change(some_string: &mut String) {
    some_string.push_str(", world");
} // some_string is mutable, so the value can be changed
// mutable reference have one big restriction: you can only have one mutable reference to a particular piece of data in a particular scope


