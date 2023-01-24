use std::collections::HashMap;

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    //let v: Vec<i32> = Vec::new();
    //let v = vec![1, 2, 3]; // often using initial value to do type annotation
    let mut v = Vec::new();
    v.push(1); // update a vector
    v.push(2);
    v.push(3);
    v.push(4);
    v.push(5);

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    match v.get(2) { // reading element of vector
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element"),
    }

    for i in &mut v {
        *i += 50;
        println!("{i}")
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
// create a new String
    let data = "initial contents";
    let mut s = data.to_string(); 
    let s1 = "initial contents".to_string();
    let s2 = String::from("initial contents");
//  update a String
    s.push_str(".");
    s.push_str(&s1);
    s.push('!');
    let s3 = s1 + &s2;
    format!("{s2}-{s3}");
//  slice a String
    let s4 = &data[0..6];

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // map.insert(&field_name, &field_value);
    // println!("{field_name}: {field_value}"); // error occur cause no more ownership, invalid at this point

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("{s}"),
        None => println!("team not exist"),
    }

    for (k, v) in &scores {
        println!("{k}: {v}");
    }

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Red")).or_insert(30);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map2 = HashMap::new();
    for word in text.split_whitespace() {
        let count = map2.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:#?}", map2);
}
