fn main() {
    println!("Hello, world!");

    let six = {
        let four = three() + 1;
        four + 1           // <- no semicolon here
    };

    let mytuple = (three(), six);
    another_function(mytuple);
}

fn another_function((x, y): (i32, i32)) {
    println!("The value of x is {}.", x);
    println!("The value of y is {}.", y);
}

fn three() -> i32 {
    3
}