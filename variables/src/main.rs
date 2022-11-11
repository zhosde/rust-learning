const MAX_POINTS: u32 = 100_000; // constant

fn main() {
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    let y = 2;
    let y = y + 1; // shadowing
    println!("The value of x is {}", y);

    let spaces = "   ";
    let spaces = spaces.len(); // shadowing, type changable
    println!("{}", spaces);
    println!("{}", MAX_POINTS);

    let guess: u32 = "42".parse().expect("Not a number");
    println!("{}", guess);

    let tup: (i32, f64, u8) = (500, 6.4, 1); // compound type, tuple
    let (a, b, c) = tup;
    println!("{}, {}, {}", a, b, c );
    let one = tup.2;
    println!("{}", one);

    let _a: [i32; 5] = [1,2,3,4,5];
    let _b = [3; 5]; // [3,3,3,3,3]

    another_function(5); // argument
    example();

    let f = plus_five(5);
    println!("The value of f is {f}");

    let condition = true;
    let number = if condition {5} else {6}; // arms shall have compatible type, e.g. not "6"
    println!("The number value is {number}");

    let mut counter = 0;
    let result = loop {
        counter +=1;
        if counter ==10 {
            break counter * 2;
        }
    };
    println!("The result if {result}");

    let arr = [10, 20, 30];
    for ele in arr {
        println!("The value is {ele}");
    }

    count_down();
}

fn another_function(x: i32) { // parameter
    println!("The value of x is: {}", x);
}

fn example(){
    let _m = 6; // statement, 6 is expression
    let n = {
        let p = 1;
        p + 2 // no semicolon, expression has returned value; p+2; => statement will return ()
    };
    println!("The value of n is {n}")
}

fn plus_five(x: i32) -> i32 {
   x + 5
}

fn count_down(){
    for number in (1..4).rev(){
        println!("{number}");
    }
    println!("LIFTOFF!!")
}
