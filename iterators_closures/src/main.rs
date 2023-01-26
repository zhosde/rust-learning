use std::thread;
use std::time::Duration;

struct Cacher<T>
where T: Fn(u32) -> u32
{
    calculation: T, // closure
    value: Option<u32>,
}

impl<T> Cacher<T> 
where T: Fn(u32) -> u32,
{
fn new(calculation: T) -> Cacher<T> {
    Cacher {
        calculation,
        value: None,
    }
}
// no matter how many times called, the closure max. executed once
fn value(&mut self, arg: u32) -> u32 {
    match self.value {
        Some(v) => v,
        None => {
            let v = (self.calculation)(arg);
            self.value = Some(v);
            v
        }
    }
}
}

fn main() {
    let simulated_user_specified_value = 28;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}
/* => 1
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}
*/
fn generate_workout(intensity: u32, random_number: u32) {
    // defining a closure & storing definition of this closure in a variable
    let mut expensive_closure = Cacher::new(
        |num| { 
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
        });
    /* => 3
    |num|{ 
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    */
    //let expensive_result = simulated_expensive_calculation(intensity); => 2
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            //simulated_expensive_calculation(intensity) => 1
            //expensive_result // 2
            //expensive_closure(intensity) => 3
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            //simulated_expensive_calculation(intensity) => 1
            //expensive_result => 2
            //expensive_closure(intensity) => 3
            expensive_closure.value(intensity)
        )
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated");
        } else {
            println!(
                "Today, run for {} minutes!",
                //simulated_expensive_calculation(intensity) => 1
                //expensive_result => 2
                //expensive_closure(intensity) => 3
                expensive_closure.value(intensity)
            );
        }
    }

    let v1 = vec![1,2,3];
    let v1_iter = v1.iter(); // iterator stored in variable

    for val in v1_iter {
        println!("Got: {}", val);
    }
}
// limitation of this closure
#[cfg(test)]
mod tests {
    use std::vec;

/* 
    #[test]
    fn call_with_different_values(){
        let mut c = super::Cacher::new(|a|a);
        let v1 = c.value(1); // is 1
        let v2 = c.value(2); // is still 1

        assert_eq!(v2, 2); // fail
    }
*/
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1,2,3];
        let mut v1_iter = v1.iter();
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
    }
    #[test]
    fn iterator_sum() {
        let v1 = vec![1,2,3];
        let mut v1_iter = v1.iter();
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);
    }
    #[test]
    fn iterator_sum2() {
        let v1: Vec<i32> = vec![1,2,3];
        let v2: Vec<_> = v1.iter().map(|x| x+1).collect(); // lazy iterator adaptors need iterator consumed
        assert_eq!(v2, vec![2,3,4]);
    }
}
