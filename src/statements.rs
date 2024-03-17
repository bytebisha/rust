#![allow(unused_variables, dead_code, unused_must_use, unused_assignments)]
use std::mem::size_of_val;

pub fn _issue_19() {
    let s: i32 = sum(1, 2);
    assert_eq!(s, 3);

    println!("Success!");
}

pub fn sum(x: i32, y: i32) -> i32 {
     x + y
}

pub fn _issue_18() {
    let v = {
        let x = 3;
        x
    };

    assert!(v == 3);

    println!("Success!");
}

pub fn _issue_17() {
    let v = {
        let mut x: i32 = 1;
        x += 2
    };

    assert_eq!(v, ());

    println!("Success!");
}

pub fn _issue_16() {
    let x: u32 = 5u32;
    
    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        x_cube + x_squared + x
    };

    let z = {
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}

pub fn _issue_15() {
    let unit: () = ();
    assert!(size_of_val(&unit) == 0);

    println!("Success!");
}

pub fn _issue_14() {
    let _v: () = ();

    let v: (i32, i32) = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success!");
}

pub fn implicitly_ret_unit() {
    println!("I will return a ()");
}

pub fn _issue_13() {
    let f: bool = true;
    let t: bool = true && true;
    assert_eq!(t, f);

    println!("Success!");
}

pub fn _issue_12() {
    let _f: bool = false;

    let t = true;

    if t {
        println!("Success!");
    }
}

pub fn _issue_11(c: char) {
    println!("{}", c)
}

pub fn _issue_10 () {
    let c1: char = 'a';

    assert_eq!(size_of_val(&c1), 4);

    let c2: char = '漢';

    assert_eq!(size_of_val(&c2), 4);

    println!("Success!")
}