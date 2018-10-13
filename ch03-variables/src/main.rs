fn main() {
    let tuple: (char, f32) = ('\u{FDFA}', 1.1);
    let (cval, _) = tuple;

    println!("The value of cval is: {}", cval);
}
