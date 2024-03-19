#![allow(dead_code)]

fn _issue_29() {
    let s: String = String::from("hello, ");
    let mut s1 = s;
    s1.push_str("world!");
    println!("Success!");
}

fn _issue_28() {
    let x: (i32, i32, (), &str) = (1, 2, (), "hello");
    let y: (i32, i32, (), &str) = x.clone();
    println!("{:?}, {:?}", x, y);
}

fn _issue_27() {
    let s = String::from("hello, world!");
    print_str(s.clone());
    println!("{}", s);
}

fn print_str(s: String) {
    println!("{}", s);
}

fn _issue_26() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("hello, world!");
    let _s = s.as_bytes();
    s
}


fn _issue_25() {
    let s1: String = String::from("hello, world!");
    let s2: String = take_ownership(s1);

    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn _issue_24() {
    let x: String = String::from("hello, world!");
    let y: String = x.clone();

    println!("{}, {}", x, y);
}
