fn main() {
    let tuple: (char, f32) = ('\u{FDFA}', 1.1);
    let (cval, _) = tuple;

    println!("The value of cval is: {}", cval);

    let a = [1, 2, 3, 4];
    let x = 1;
    let y = 4;
    let n = a[x + y];
}
