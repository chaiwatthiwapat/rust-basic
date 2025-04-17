#![allow(dead_code)]
#![allow(while_true)]
#![allow(unused_imports)]

mod error;
mod my_struct;
mod my_trait;
mod my_enum;

use std::collections::HashMap;
use error::print_error;
use my_struct::{person::Person, car::Car};
use my_trait::country::Country;
use my_enum::{color::Colors, grade::Grade};

fn main() {
    calc();
}

fn num() {
    let x: i8;
    x = 10;
    println!("{}", x);
}

fn str() {
    let x = String::from("Hello");
    let y = "World";
    let f = format!("{} {}", x, y);
    let bollow_str: &str = f.as_ref();
    print!("{}", bollow_str);
}

fn tuple() {
    let x = (1, 20.2, 1000);
    let y: f64 = (x.0 as f64) * x.1;
    println!("{}", y);
}

fn array() {
    let x: [i32; 5];
    x = [1, 2, 3, 4, 5];
    let y = [0; 5];
    println!("{:?}\n{:?}", x, y);
}

fn sum(a: i32, b: i32) -> i32 {
    let x = |a: i32, b: i32| -> i32 { a * b }; // lamda
    let y = a + b;
    let x = x(a, b);
    let result = y + x;
    result
}

fn use_if(scr: i32) -> String {
    let grd;

    if scr >= 80 {
        grd = "A";
    }
    else {
        grd = "F";
    }

    grd.to_string()
}

fn use_while() {
    let mut x = 0;

    while true {
        x += 1;
        if x < 3 { continue; }

        println!("x: {}", x);
        break;
    }
}

fn use_loop() {
    let mut x = 0;

    'l1: loop {
        x += 1;
        if x < 10 { continue; }

        loop {
            x += 1;
            if x % 2 == 1 { 
                break 'l1 
                println!("x: {}", x);
            }
        }
    }
}

fn use_for() {
    for i in 0..=10 {
        println!("{}", i);
    }

    let x = [1, 2, 3];
    for i in x.iter() {
        println!("{}", i);
    }
}

fn collect() {
    let mut x: Vec<i32> = Vec::new();
    x.push(10);
    x.push(20);
    x.push(30);
    // println!("{:?}", x.pop());
    // println!("{:?}", x.pop().unwrap());

    // let x = match x.get(5) {
    //     Some(v) => *v,
    //     None => 0
    // };

    let mut x = vec![1, 2, 3];
    x.push(10);

    let y = x.get(1).copied().unwrap_or(0);

    println!("{} {:?}", y, x);
}

fn use_hashmap() {
    // use std::collections::HashMap;

    let mut x: HashMap<&str, &str> = HashMap::new();
    x.insert("th", "Thailand");
    x.insert("us", "United state");
    let y = x.get("th");

    println!("{}", y.unwrap());
}

fn use_struct() {
    let p = Person {
        name: "Chaiwat".to_string(),
        age: 22
    };

    let c = Car::new("Honda".to_string(), "White".to_string());

    println!("person name: {} \n{:#?}", p.name, p);
    println!("car name: {} \n{:#?}", c.name(), c);
}

fn use_trait() {
    let p = Person {
        name: "Chaiwat".to_string(),
        age: 22
    };

    println!("{}", p.country());
}

fn grade(score: i32) -> Grade {
    let grade: String;
    if score < 0 || score > 100 {
        return Grade::Error(format!("Invalid score: {}", score))
    }
    else {
        grade = "A".to_string();
    }

    Grade::Value(grade)
}

fn grade2(score: i32) -> Option<String> {
    if score < 0 || score > 100 {
        return None;
    }

    Some("A".to_string())
}

fn grade3(score: i32) -> Result<String, String> {
    if score < 0 || score > 100 {
        return Err("Invalid score".to_string());
    }

    Ok("A".to_string())
}

fn use_enum() {
    let x = Colors::Red;
    let color = match x {
        Colors::Red => "Red",
        _ => "White"
    };
    let color = format!("color is: {}", color);
    println!("{}", color);

    let grade = grade(80);
    match grade {
        Grade::Error(e) => println!("{}", e),
        Grade::Value(g) => println!("{}", g)
    };
}

fn use_option() {
    let x = grade2(100);
    match x {
        Some(v) => println!("{}", v),
        None => println!("Invalid score")
    }
}

fn use_result() {
    let x = grade3(100);
    match x {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e)
    }
}

// Rust
fn calc() {
    let x = fn_x(10, 20, |a, b| a + b);
    let y = fn_y(10, 20, |a, b| a * b);

    println!("x: {}\ny: {}", x, y);
}

fn fn_x<F: Fn(i32, i32) -> i32>(a: i32, b: i32, f:F) -> i32 {
    f(a, b) + a - b
}

fn fn_y<F>(a: i32, b: i32, f:F) -> i32 
where F: Fn(i32, i32) -> i32 {
    f(a, b) + a * b
}

// answer ?
// A. x = 30, y = 200
// B. x = 230, y = 230
// C. x = 20, y = 400
// D. x = 60, y = 300