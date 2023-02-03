mod test;
fn main() {
    println!("Hello, world!");
    println!("How does Rust Work");
    testing();
    test::function_testing();
}

fn testing() {
    let x = &["Hello", "This", "Sucks"];
    for text in x {
        println!("{}", text);
    }
    println!("I don't think this works");
    println!("I think Vim is Fun")
}
