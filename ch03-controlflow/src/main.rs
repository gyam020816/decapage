fn main() {
    let x = -1;

    let w = if x == 1 {
        5

    } else {
        //"hey"
        10
    };
    println!("{}", w);

    let mut c = 0;
    let vee = loop {
        c += 1;

        if c > 100_000_000 {
            break "what";
        }
    };

    println!("{}", vee)
}
