fn main() {
    let mut s = String::from("hello"); // allocate data on heap
    s.push_str(", world!"); // append literal to String, string literal immutable
    println!("{s}");
    not_working();
    clone();
    copy();
    function();
    return_value();
    not_take_ownership();
    reference();
    let first = first_word(&s);
    println!("{first}");
    /* 
    let _hello = &s[..5]; 
    let _world = &[6..];
    let _whole_world = &s[..];
    */
    let my_string = String::from("Hello world");
    let _word = first_word(&my_string[..]);
    let my_string_literal = "Hello World";
    let _word = first_word(my_string_literal);
} // scope over, drop() called, s is no longer valid

fn not_working(){
    let s1 = String::from("hello");
    let _s2 = s1;
    //println!("{}, world", s1); error for s1, since it's already not valid 
}

fn clone(){
    let s1 = String::from("hello");
    let s2 = s1.clone(); // expensive, heap level copy, deep copy
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn copy(){
    let x = 5; // stack-only data
    let y = x;
    println!("{}, {}", x, y);
}

fn function(){
    let s = String::from("hello");
    takes_ownership(s); // s is moved to function, no longer valid
    let x = 5; // i32 type
    makes_copy(x); 
    println!("x: {x}"); // can still use x within this function
}

fn takes_ownership(str: String){
    println!("{str}");
}

fn makes_copy(integer: i32){
    println!("{integer}")
}

fn return_value(){
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 moved
    println!("s1: {}, s3: {}", s1, s3) // yours, hello
}

fn gives_ownership() -> String {
    let str = String::from("yours");
    str
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn not_take_ownership(){
    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}", s2, len);    
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn reference(){ // better way for not taking ownership
    let s1 = String::from("hello");
    let len = calculate_length_new(&s1); // let len = calculate_length_new(&mut s1); => mutable, but one time borrow
    println!("The length of '{}' is {}", s1, len);    
}

fn calculate_length_new(s: &String) -> usize {
    s.len() // immutable
}

fn first_word(s: &str) -> &str { // using string slice instead of &String / usize
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}