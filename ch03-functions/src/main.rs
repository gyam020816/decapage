fn main() {
    println!("Hello, world!");

    let mytuple = (3, 6);
    another_function(mytuple);
}

fn another_function((x, y): (i32, i32)) {
    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);
}