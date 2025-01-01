fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    //Safe way to access element: get() returns an Option
    match vec.get(1) {
        Some(second) => println!("Second element: {}", second),
        None => println!("Index out of bounds"),
    };
} 