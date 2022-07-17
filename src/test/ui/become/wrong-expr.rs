#![feature(become_stmt)]

fn b() -> u32 {
    become b() + 1;
    //~^ ERROR become must always call a function
}

fn main() {
    println!("{}", b());
}
