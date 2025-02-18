use std::io;

fn main() {
    println!("Hello, world!");
    
    let arr = [1,2,3,4];
    
    let s = String::from("Hello World");

    for i in arr{
        println!("The element is {i}")
    }

    for i in 1..10 {
        println!("TLDR{i}")
    }

    fn calculate_length(s: &String) -> usize {
        return s.len();
    }
    let name = String::from("name");
    println!("{}", calculate_length(&name));

}

