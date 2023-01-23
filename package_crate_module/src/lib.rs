mod front_of_house;

mod serving {
    fn take_order(){}
    fn serve_order(){}
    fn take_payment(){}
}

pub use crate::front_of_house::hosting; // use & absolute path
// use front_of_house::hosting; // use & relative path

pub fn eat_at_restaurant(){
    crate::front_of_house::hosting::add_to_waitlist(); // absolute path
    front_of_house::hosting::add_to_waitlist(); // relative path

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    //meal.seasonal_fruit = String::from("blueberries"); //error cause seasonal_fruit is private field

    hosting::add_to_waitlist(); // use
}

fn serve_order(){}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order(){
        cook_order();
        super::serve_order(); // relative path
        crate::serve_order(); // absolute paht
    }
    fn cook_order(){}
}

