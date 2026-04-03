use std::fmt;

enum List<T> {
    // Your code here
}

impl<T> List<T> {
    // Your code here
    // Define at least three functions here, new (creates empty list), append, length
}

impl<T: fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Your code here
    }
}

fn main() {
    // This should run without any errors

    let mut list = List::<i32>::new();
    list.append(10);
    list.append(20);
    list.append(30);
    list.append(40);
    list.append(50);
    println!("{}", list);
    println!("List Lenght :: {}", list.length());
}
