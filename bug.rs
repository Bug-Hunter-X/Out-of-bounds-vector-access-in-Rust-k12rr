fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let first = vec[0];
    println!("First element: {}", first);
    //The below line will cause a panic because the vector is empty
    let second = vec[1];
    println!("Second element: {}", second);
}