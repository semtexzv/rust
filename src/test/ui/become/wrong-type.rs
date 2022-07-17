#![feature(become_stmt)]

fn a(v: u32) -> u32 {
    v
}

fn b() -> u32 {
    become a(0)
    //~^ ERROR cannot become `a` because it has different arguments
}

fn main() {
    println!("{}", b());
}
