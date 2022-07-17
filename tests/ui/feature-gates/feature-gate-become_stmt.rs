fn other() {
    println!("Called recursively");
}

fn main() {
    become other(); //~ ERROR `become` statement is experimental
}
