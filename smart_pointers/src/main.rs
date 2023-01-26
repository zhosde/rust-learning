use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

fn main() {
    let b = Box::new(5); // data stored in heap
    println!("b = {}", b);

    // using Box enable recursive types
    /* 
    let _list = Cons(1, // instead of Cons(1, Cons(2, Cons(3, Nil)));
        Box::new(Cons(2, 
            Box::new(Cons(3, 
                Box::new(Nil)))))); 
    */
    let x = 5;
    let y = MyBox::new(x); // let y = Box::new(x); and let y = &x; both works with *y
    assert_eq!(5, x);
    assert_eq!(5, *y); // error if without dereference, {integer} and &{integer} are different types
    
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello("Hey");
    hello(&(*m)[..]); // if deref coercion not added

    // using Rc<T> to share data (via immutable references) 
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    let _b = Cons(3, Rc::clone(&a)); // Rc::clone() not deep copy comparing with .clone()
    let _c = Cons(4, Rc::clone(&a)); // error if Cons(4, Box::new(a));
}
enum List {
    Cons(i32, Rc<List>), // instead of Cons(i32, List) => Box<List>
    Nil,
}
