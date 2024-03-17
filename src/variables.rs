#![allow(unused_variables)]

pub fn _issue_1() {
    let x: i32 = 5;
    let _y: i32;

    assert_eq!(x, 5);
    println!("Success!");
}

pub fn _issue_2() {
    let mut x: i32 = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}

pub fn _issue_3() {
    let x: i32 = 10;
    let y: i32 = 5;
    {
        
        println!("The value of x is {} and the value of y is {}", x, y);
    }
    println!("The value of x is {} and the value of y is {}", x, y);
}

pub fn _issue_4() {
    let x: &str = "Hello";
    println!("{}, World!", x);
}

pub fn _issue_5() {
    let x: i32 = 5;

    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);

    let x = 42;
    println!("{}", x);
}

pub fn _issue_6() {
    let mut _x: i32 = 1;
    _x = 7;

    _x += 3;

    let _y: i32 = 4;

    let _y: &str = "I can also be bound to text!";

    println!("Success!");
}

pub fn _issue_7() {
    let x = 5;
}

pub fn _issue_8() {
    let (mut x, y) = (1, 2);
    x += 2;

    assert_eq!(x, 3);
    assert_eq!(y, 2);

    println!("Success!")
}

pub fn _issue_9() {
    let (x, y);

    (x, ..) = (3, 4);
    [.., y] = [1, 2];

    assert_eq!([x, y], [3, 2]);

    println!("Success!")
}