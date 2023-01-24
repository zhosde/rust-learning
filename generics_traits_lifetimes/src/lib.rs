use std::fmt::Display;
use std::fmt::Debug;

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub fn notify(item1: impl Summary + Display, item2: impl Summary) {
    println!("News!, {}", item1.summarize())
}

pub fn notify2<T: Summary + Display>(item1: T, item2: T) { // trait bound
    println!("News!, {}", item1.summarize())
}

pub fn notify3<T, U>(a: T, b: U) -> String // clear trait bounds with where clauses
    where 
    T: Summary + Display,
    U: Clone + Debug,
{ 
    format!("News!, {}", a.summarize())
}

pub fn notify4(s: &str) -> impl Summary { // returning type that implement traits
    NewsArticle {
        headline: String::from("Hello"),
        content: String::from("world"),
        author: String::from("me"),
        location: String::from("nowhere"),
    }
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl <T: Display + PartialOrd> Pair<T> { // conditionally implement methods
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}