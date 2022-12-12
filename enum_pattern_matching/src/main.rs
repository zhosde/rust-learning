/* 
enum IPAddrKind {
    V4,
    V6,
}
*/
enum IPAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

struct IpAddr {
    kind: IPAddrKind,
    address: String,
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
   /* 
   let four = IPAddrKind::V4;
   let six = IPAddrKind::V6;

   route(four);
   route(six);
   route(IPAddrKind::V6);
   */ 

   /*  
   let home = IpAddr {
    kind: IPAddrKind::V4,
    address: String::from("127.0.0.1"),
   };

   let loopback = IpAddr {
    kind: IPAddrKind::V6,
    address: String::from("::1"),
   };
   */
  let home = IPAddrKind::V4(127, 0, 0, 1);
  let loopback = IPAddrKind::V6(String::from("::1"));

  let q = Message::Quit;
  let m = Message::Move {x: 12, y:24};
  let w = Message::Write(String::from("Hello"));
  let c = Message::ChangeColor(0, 255, 255);

  m.call();

  let some_number = Some(5);
  let some_string = Some("A string");
  let absent_number: Option<i32> = None; // Option<T> vs. null

  let c = Coin::Quarter(UsState::Alaska);
  println!("{}", value_in_cents(c));

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  let v = Some(0u8);
  match v {
    Some(3) => println!("three"),
    _ => println!("others"), // using placeholder to be able to not meet catch-all pattern requirement
  }

  if let Some(3) = v { // less code for above match, when only one pattern needed for matching 
    println!("three");
  } else {
    println!("others");
  }
}

fn route(ip_kind: IPAddrKind) {
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}
