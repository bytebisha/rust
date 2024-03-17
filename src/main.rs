#![allow(unused)]

mod variables;
mod statements;

fn main() {
}

pub fn _issue_22(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };

    never_return_fn();
}

fn never_return_fn() -> ! {
    todo!("This function never returns!")
}

pub fn _issue_21() {
    never_return();

    println!("Failed!");
}

pub fn never_return() -> ! {
    panic!("This function never returns!");
}

pub fn _issue_20() {
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);

    println!("Success!")
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}