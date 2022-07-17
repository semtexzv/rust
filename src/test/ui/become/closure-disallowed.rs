#![feature(become_stmt)]

fn a(v: u32) -> u32 {
    let closure = |e: u32| -> u32 {
        become a(e)
        //~^ ERROR become statement cannot be used inside a closure
    };

    closure(v)
}

fn main() {
    println!("{}", a(0));
}
