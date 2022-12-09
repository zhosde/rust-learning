struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32); // tuple struct

#[derive(Debug)] // for printing struct
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle { 
    fn area(&self) -> u32 { // method
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }

    fn square(size: u32) -> Rectangle { // doesn't have self as 1st parameter => associated function
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn _build_user(email: String, username: String) -> User {
    User {
        email, // field init shorthand syntax
        username, 
        active: true,
        sign_in_count: 0,
    }
}

fn main () {
    let user1 = User {
        email: String::from("abc@email.com"),
        username: String::from("Alice"),
        active: true,
        sign_in_count: 555,
    };

    let _user2 = User {
        email: String::from("efg@email.com"),
        username: String::from("Bett"),
        ..user1 // struct update syntax
    };

    let _black = Color(0,0,0);

    let rect = Rectangle {
        width: 30,
        length: 50,
    };

    //println!("{}", area(&rect));
    println!("{}", rect.area()); // 1500
    println!("{:#?}", rect);

    let rect1 = Rectangle {
        width: 30, 
        length: 50,
    };

    let rect2 = Rectangle {
        width: 10, 
        length: 40,
    };

    let rect3 = Rectangle {
        width: 35, 
        length: 55,
    };

    println!("{}", rect1.can_hold(&rect2)); // true
    println!("{}", rect1.can_hold(&rect3)); // false

    let _s = Rectangle::square(20); // calling associated function
}
/* 
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.length
}
*/
