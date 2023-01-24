use generics_traits_lifetimes::Summary;
use generics_traits_lifetimes::Tweet;
use std::fmt::Display;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { // fn largest(list: &[i32]) -> i32
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

struct Point<T, U> { // Point<T>
    x: T,
    y: U,
}
/* 
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    fn x1(&self) -> i32 {
        &self.x
    }
}
*/
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

struct ImportantExcerpt<'a> {
    part: &'a str, // reference type needs lifetime annotation
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let number_list = vec![102, 34, 6000, 89, 54, 2];
    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['Y', 'N', 'Z'];
    let result = largest(&char_list);
    println!("The largest char is {result}");

    let integer = Point {x: 5, y: 2.3};
    let float = Point {x: 0.5, y: 1.0};

    let p1 = Point {x: 5, y: 4};
    let p2 = Point {x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    let tweet = Tweet {
        username: String::from("ebook"),
        content: String::from("of course"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let string1 = String::from("abcd");
    {
    let string2 = "xyz"; // static lifetime
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
    }

    let novel = String::from("Here is the story. Long time ago...");
    let first_sentence = novel.split('.').next().expect("Could not found a '.'");
    let i = ImportantExcerpt {
        part: first_sentence
    };
}
// references with an explicit lifetime
// since borrow checker cannot determine the returned ref valid or not
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
/* 
fn longest<'a>(x: &'a str, y: &'a str) -> String {
    let result = String::from("abc");
    result
}
*/
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}