#![allow(dead_code)]
#![allow(while_true)]

mod dto;
mod error;

use std::collections::HashMap;
use dto::person::Person;
use dto::car::Car;
use crate::error::print_error;

fn main() {
    print_error();
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

